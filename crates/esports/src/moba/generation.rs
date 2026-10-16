//! The world generator (world generation spec).
//!
//! Each team draws from its own stream (`generator:team:<team id>`), each
//! pinned player from theirs (`generator:player:<id>`), and each nickname
//! from `generator:nickname:<player id>`. Every player consumes the same
//! number of draws whatever is pinned, so pinning a field never shifts
//! another draw.

use std::collections::{BTreeMap, BTreeSet};

use domain::date::GameDate;
use domain::id::ContentId;
use domain::random::{index, index_excluding, normal, round_clamped, weighted_index};
use domain::rating::Rating;
use domain::rng::{StreamLabel, StreamSource};
use domain::world::{Region, Team};
use rand_core::Rng;

use super::attributes::{MobaAttributes, OverallAttribute};
use super::content::{GenerationParams, PinnedAttributes, PinnedPlayer};
use super::world::{Player, World, WorldTemplate};

/// Why a template can't be generated. The package loader rules all of
/// these out, so they only happen with a template built by hand.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum GenerationError {
    #[error("`{0}` refers to a region, role or nationality that doesn't exist")]
    Unknown(ContentId),
    #[error("region `{0}` has no nationality to draw")]
    EmptyMix(ContentId),
    #[error("nationality `{0}` has no given or family names")]
    EmptyNames(ContentId),
    #[error("team `{0}` can't complete its roster around its pinned players")]
    IncompletableRoster(ContentId),
    #[error("the nickname pool ran out")]
    NicknamesExhausted,
}

/// Builds the world: every team's roster, around its pinned players.
///
/// # Errors
///
/// When the template refers to something it doesn't define, or can't
/// supply a draw. A template from a package that loaded never fails.
pub fn generate(
    template: &WorldTemplate,
    streams: &impl StreamSource,
) -> Result<World, GenerationError> {
    let mut teams = template.teams.clone();
    teams.sort_by(|a, b| a.id.cmp(&b.id));
    let generator = Generator::new(template);

    let mut players = Vec::new();
    for team in &teams {
        players.extend(generator.team(team, streams)?);
    }
    assign_nicknames(template, &mut players, streams)?;
    pick_starters(template, &mut players);

    Ok(World {
        settings: template.settings,
        regions: template.regions.clone(),
        nationalities: template.nationalities.clone(),
        competitions: template.competitions.clone(),
        teams,
        roles: template.roles.clone(),
        players,
    })
}

/// Generates players from one template.
struct Generator<'a> {
    template: &'a WorldTemplate,
    params: &'a GenerationParams,
    /// Given and family names by nationality, every pool merged in order.
    names: BTreeMap<&'a ContentId, (Vec<&'a str>, Vec<&'a str>)>,
}

impl<'a> Generator<'a> {
    fn new(template: &'a WorldTemplate) -> Self {
        let mut names: BTreeMap<&ContentId, (Vec<&str>, Vec<&str>)> = BTreeMap::new();
        for pool in &template.name_pools {
            let entry = names.entry(&pool.nationality).or_default();
            entry.0.extend(pool.given.iter().map(String::as_str));
            entry.1.extend(pool.family.iter().map(String::as_str));
        }
        Self {
            template,
            params: &template.generation,
            names,
        }
    }

    fn team(
        &self,
        team: &Team,
        streams: &impl StreamSource,
    ) -> Result<Vec<Player>, GenerationError> {
        let region = self
            .template
            .regions
            .iter()
            .find(|region| region.id == team.region)
            .ok_or_else(|| GenerationError::Unknown(team.region.clone()))?;
        let strength = Strength::of(self.params, team.reputation.value(), region.level);

        let mut pinned: Vec<&PinnedPlayer> = self
            .template
            .players
            .iter()
            .filter(|p| p.team == team.id)
            .collect();
        pinned.sort_by(|a, b| a.id.cmp(&b.id));

        let mut rng = streams.stream(&StreamLabel::new(format!("generator:team:{}", team.id)));
        let slots = self.slots(team, &pinned, &mut rng)?;

        let mut players = Vec::new();
        for player in pinned {
            let mut own =
                streams.stream(&StreamLabel::new(format!("generator:player:{}", player.id)));
            players.push(self.player(
                &mut own,
                player.id.clone(),
                team,
                region,
                strength,
                &player.role,
                Some(player),
            )?);
        }
        for (number, role) in (1..).zip(slots) {
            let id = ContentId::generated("player", &team.id, number)
                .map_err(|_| GenerationError::Unknown(team.id.clone()))?;
            players.push(self.player(&mut rng, id, team, region, strength, &role, None)?);
        }
        Ok(players)
    }

    /// The roles of the players the generator adds to `team`: every role no
    /// pinned player covers, in role-set order, then subs in different roles
    /// drawn from the team's stream.
    fn slots(
        &self,
        team: &Team,
        pinned: &[&PinnedPlayer],
        rng: &mut impl Rng,
    ) -> Result<Vec<ContentId>, GenerationError> {
        let roles = self.template.roles.roles();
        let mut counts = vec![0_usize; roles.len()];
        for player in pinned {
            let position = roles
                .iter()
                .position(|role| role.id() == &player.role)
                .ok_or_else(|| GenerationError::Unknown(player.role.clone()))?;
            counts[position] += 1;
        }
        let missing: Vec<usize> = (0..roles.len()).filter(|&i| counts[i] == 0).collect();
        for &position in &missing {
            counts[position] = 1;
        }
        let size = roles.len() + usize::from(self.template.roster.subs);
        let remaining = size
            .checked_sub(pinned.len() + missing.len())
            .ok_or_else(|| GenerationError::IncompletableRoster(team.id.clone()))?;
        if counts.iter().any(|&count| count > 2) {
            return Err(GenerationError::IncompletableRoster(team.id.clone()));
        }

        // Subs go to roles that have exactly one player so far.
        let mut taken: BTreeSet<usize> = (0..roles.len()).filter(|&i| counts[i] != 1).collect();
        let mut subs = Vec::new();
        for _ in 0..remaining {
            let position = index_excluding(rng, roles.len(), &taken)
                .ok_or_else(|| GenerationError::IncompletableRoster(team.id.clone()))?;
            taken.insert(position);
            subs.push(position);
        }
        Ok(missing
            .into_iter()
            .chain(subs)
            .map(|position| roles[position].id().clone())
            .collect())
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "one call site, each argument distinct"
    )]
    fn player(
        &self,
        rng: &mut impl Rng,
        id: ContentId,
        team: &Team,
        region: &Region,
        strength: Strength,
        role_id: &ContentId,
        pinned: Option<&PinnedPlayer>,
    ) -> Result<Player, GenerationError> {
        let p = self.params;
        let role = self
            .template
            .roles
            .get(role_id)
            .ok_or_else(|| GenerationError::Unknown(role_id.clone()))?;

        // Every draw happens in this fixed order, pinned or not.
        let weights: Vec<u32> = region
            .nationalities
            .iter()
            .map(|&(_, weight)| weight)
            .collect();
        let drawn = weighted_index(rng, &weights)
            .ok_or_else(|| GenerationError::EmptyMix(region.id.clone()))?;
        let nationality = pinned
            .and_then(|p| p.nationality.clone())
            .unwrap_or_else(|| region.nationalities[drawn].0.clone());
        let (given_pool, family_pool) = self
            .names
            .get(&nationality)
            .ok_or_else(|| GenerationError::EmptyNames(nationality.clone()))?;
        let given = index(rng, given_pool.len())
            .ok_or_else(|| GenerationError::EmptyNames(nationality.clone()))?;
        let family = index(rng, family_pool.len())
            .ok_or_else(|| GenerationError::EmptyNames(nationality.clone()))?;
        let birth_date = self.birth_date(rng);
        let base = normal(rng, strength.mu, strength.sigma);
        let mut attributes = MobaAttributes::uniform(Rating::MIN);
        for attribute in OverallAttribute::ALL {
            let value = normal(
                rng,
                base + role.bias(attribute, p.bias_scale),
                p.attribute_spread,
            );
            set(&mut attributes, attribute, Rating::clamped(value));
        }
        attributes.core.aggression = Rating::clamped(normal(rng, p.style_mean, p.style_spread));
        attributes.core.work_ethic = Rating::clamped(normal(rng, p.style_mean, p.style_spread));
        attributes.core.consistency =
            Rating::clamped(normal(rng, p.consistency_mean, p.consistency_spread));
        let potential_noise = normal(rng, 0.0, p.potential_spread);

        let mut player = Player {
            id,
            team: team.id.clone(),
            role: role_id.clone(),
            starter: false,
            nickname: String::new(),
            given_name: given_pool[given].to_owned(),
            family_name: family_pool[family].to_owned(),
            nationality,
            birth_date,
            attributes,
        };
        if let Some(pinned) = pinned {
            apply_pins(&mut player, pinned);
        }
        if pinned.and_then(|p| p.attributes.potential).is_none() {
            let overall = role.overall(&player.attributes).points();
            let age = player
                .birth_date
                .age_on(self.template.settings.start_date)
                .unwrap_or(0);
            let youth = (p.potential_peak_age - f64::from(age)).max(0.0);
            let potential = overall + p.potential_per_year * youth + potential_noise;
            player.attributes.core.potential = Rating::clamped(potential.max(overall));
        }
        Ok(player)
    }

    /// A birth date that gives an age drawn from N(`age_mean`, `age_spread`),
    /// clamped to the allowed ages, on the start date.
    fn birth_date(&self, rng: &mut impl Rng) -> GameDate {
        let p = self.params;
        let start = self.template.settings.start_date;
        let age = round_clamped(
            normal(rng, p.age_mean, p.age_spread),
            i32::from(p.age_min),
            i32::from(p.age_max),
        );
        let age = u16::try_from(age).unwrap_or(0);
        // Born on `latest` turns `age` on the start date; one day before
        // `earliest`, they would already be a year older.
        let latest = start.years_before(age).unwrap_or(GameDate::MIN);
        let earliest = start
            .years_before(age + 1)
            .and_then(|date| date.add_days(1))
            .unwrap_or(GameDate::MIN);
        let span = latest.day_number() - earliest.day_number() + 1;
        let offset = domain::random::below(rng, u64::try_from(span).unwrap_or(1)).unwrap_or(0);
        earliest
            .add_days(i64::try_from(offset).unwrap_or(0))
            .unwrap_or(latest)
    }
}

/// A team's player-base distribution: N(`mu`, `sigma`).
#[derive(Debug, Clone, Copy, PartialEq)]
struct Strength {
    mu: f64,
    sigma: f64,
}

impl Strength {
    /// A higher reputation gives a higher mean and a lower spread; the
    /// region's level shifts the mean.
    fn of(params: &GenerationParams, reputation: u8, level: i32) -> Self {
        let reputation = f64::from(reputation);
        Self {
            mu: params.mu_base + params.mu_per_reputation * reputation + f64::from(level),
            sigma: (params.sigma_base - params.sigma_per_reputation * reputation)
                .max(params.sigma_min),
        }
    }
}

fn set(attributes: &mut MobaAttributes, attribute: OverallAttribute, rating: Rating) {
    use super::attributes::MobaSkill;
    let skills = &mut attributes.skills;
    match attribute {
        OverallAttribute::Skill(MobaSkill::Mechanics) => skills.mechanics = rating,
        OverallAttribute::Skill(MobaSkill::Laning) => skills.laning = rating,
        OverallAttribute::Skill(MobaSkill::GameSense) => skills.game_sense = rating,
        OverallAttribute::Skill(MobaSkill::Awareness) => skills.awareness = rating,
        OverallAttribute::Skill(MobaSkill::Positioning) => skills.positioning = rating,
        OverallAttribute::Skill(MobaSkill::ChampionPoolBreadth) => {
            skills.champion_pool_breadth = rating;
        }
        OverallAttribute::Skill(MobaSkill::Shotcalling) => skills.shotcalling = rating,
        OverallAttribute::Teamwork => attributes.core.teamwork = rating,
        OverallAttribute::Composure => attributes.core.composure = rating,
    }
}

fn apply_pins(player: &mut Player, pinned: &PinnedPlayer) {
    if let Some(name) = &pinned.given_name {
        player.given_name.clone_from(name);
    }
    if let Some(name) = &pinned.family_name {
        player.family_name.clone_from(name);
    }
    if let Some(date) = pinned.birth_date {
        player.birth_date = date;
    }
    let PinnedAttributes {
        mechanics,
        laning,
        game_sense,
        awareness,
        positioning,
        champion_pool_breadth,
        shotcalling,
        teamwork,
        aggression,
        composure,
        work_ethic,
        consistency,
        potential,
    } = pinned.attributes;
    let skills = &mut player.attributes.skills;
    let core = &mut player.attributes.core;
    let pins = [
        (mechanics, &mut skills.mechanics),
        (laning, &mut skills.laning),
        (game_sense, &mut skills.game_sense),
        (awareness, &mut skills.awareness),
        (positioning, &mut skills.positioning),
        (champion_pool_breadth, &mut skills.champion_pool_breadth),
        (shotcalling, &mut skills.shotcalling),
        (teamwork, &mut core.teamwork),
        (aggression, &mut core.aggression),
        (composure, &mut core.composure),
        (work_ethic, &mut core.work_ethic),
        (consistency, &mut core.consistency),
        (potential, &mut core.potential),
    ];
    for (pin, field) in pins {
        if let Some(rating) = pin {
            *field = rating;
        }
    }
}

/// Gives every player without a pinned nickname one from the pool, unique in
/// the world ignoring case.
fn assign_nicknames(
    template: &WorldTemplate,
    players: &mut [Player],
    streams: &impl StreamSource,
) -> Result<(), GenerationError> {
    let pinned: BTreeMap<&ContentId, &str> = template
        .players
        .iter()
        .filter_map(|p| p.nickname.as_deref().map(|nickname| (&p.id, nickname)))
        .collect();
    let lowered: Vec<String> = template
        .nicknames
        .iter()
        .map(|n| n.to_lowercase())
        .collect();
    let mut taken: BTreeSet<usize> = BTreeSet::new();
    let pinned_lowered: BTreeSet<String> = pinned.values().map(|n| n.to_lowercase()).collect();
    for (position, nickname) in lowered.iter().enumerate() {
        if pinned_lowered.contains(nickname) {
            taken.insert(position);
        }
    }
    for player in players {
        if let Some(nickname) = pinned.get(&player.id) {
            player.nickname = (*nickname).to_owned();
            continue;
        }
        let mut rng = streams.stream(&StreamLabel::new(format!(
            "generator:nickname:{}",
            player.id
        )));
        let position = index_excluding(&mut rng, template.nicknames.len(), &taken)
            .ok_or(GenerationError::NicknamesExhausted)?;
        taken.insert(position);
        player.nickname.clone_from(&template.nicknames[position]);
    }
    Ok(())
}

/// For each team and role, the player with the highest overall starts; ties
/// go to the lower id.
fn pick_starters(template: &WorldTemplate, players: &mut [Player]) {
    let mut candidates: BTreeMap<(&ContentId, &ContentId), Vec<(Rating, &ContentId)>> =
        BTreeMap::new();
    for player in players.iter() {
        if let Some(role) = template.roles.get(&player.role) {
            candidates
                .entry((&player.team, &player.role))
                .or_default()
                .push((role.overall(&player.attributes), &player.id));
        }
    }
    let starters: BTreeSet<ContentId> = candidates
        .values()
        .filter_map(|group| {
            group
                .iter()
                .max_by(|a, b| a.0.cmp(&b.0).then_with(|| b.1.cmp(a.1)))
                .map(|&(_, id)| id.clone())
        })
        .collect();
    for player in players {
        player.starter = starters.contains(&player.id);
    }
}

#[cfg(test)]
#[path = "generation_tests.rs"]
mod tests;
