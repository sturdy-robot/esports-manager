//! The walking skeleton's use cases: New Game, Load Game, the roster and the
//! player profile (new game and roster specs).

use std::collections::BTreeMap;

use domain::attributes::{CoreTrait, HiddenAttribute};
use domain::id::ContentId;
use domain::rng::WorldSeed;
use domain::world::{Competition, Nationality, Team};
use esports::moba::attributes::MobaSkill;
use esports::moba::generation::{GenerationError, generate};
use esports::moba::roles::RoleSet;
use esports::moba::world::Player;

use crate::content::{ContentSource, Package, PackageError};
use crate::messages::{Message, package_error};
use crate::rng::RngProvider;
use crate::saves::{
    Damage, NewSave, OpenSave, PackageRecord, SaveError, SaveMeta, SaveStore, Timestamp,
};
use crate::views::{
    AttributeView, Crest, DamageView, NameOrderView, NationalityView, NewGameOptions,
    PlayerProfile, RegionOption, RosterPlayer, RosterView, SaveSummary, TeamHeader, TeamOption,
    hidden_message, name_message, skill_message, strength_stars, trait_message,
};

pub const MANAGER_NAME_MAX: u8 = 32;
pub const SAVE_NAME_MAX: u8 = 48;

/// What New Game offers: every region with its league and teams.
///
/// # Errors
///
/// The package's problems, when it doesn't load.
pub fn new_game_options(content: &impl ContentSource) -> Result<NewGameOptions, Vec<PackageError>> {
    let package = content.load()?;
    let template = &package.template;
    let regions = template
        .regions
        .iter()
        .map(|region| {
            let league = template.competitions.iter().find(|c| c.region == region.id);
            let teams = template
                .teams
                .iter()
                .filter(|team| team.region == region.id)
                .map(|team| TeamOption {
                    id: team.id.to_string(),
                    name_message: name_message(&team.id),
                    crest: Crest::of(team),
                    strength: strength_stars(team.reputation.value()),
                })
                .collect();
            RegionOption {
                id: region.id.to_string(),
                name_message: name_message(&region.id),
                league_id: league.map(|c| c.id.to_string()),
                league_message: league.map(|c| name_message(&c.id)),
                teams,
            }
        })
        .collect();
    Ok(NewGameOptions { regions })
}

/// What the user asks New Game for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewGameRequest {
    pub manager_name: String,
    pub team: String,
    pub save_name: String,
    /// Drawn by the adapter from OS entropy, unless the user gave one.
    pub seed: u64,
    pub now: Timestamp,
}

/// A request field that can't be used.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestProblem {
    EmptyManagerName,
    ManagerNameTooLong,
    EmptySaveName,
    SaveNameTooLong,
    ControlCharacters { field: &'static str },
    UnknownTeam { team: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NewGameError {
    Invalid(Vec<RequestProblem>),
    Package(Vec<PackageError>),
    Generation(GenerationError),
    Save(SaveError),
}

/// The new save.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewGameCreated {
    pub file: String,
    pub team: ContentId,
}

/// Builds a world from the content and writes it as a new save.
///
/// # Errors
///
/// Every problem with the request, the package's problems, or the save's.
/// Nothing is written unless the whole game is created.
pub fn new_game(
    content: &impl ContentSource,
    store: &impl SaveStore,
    request: &NewGameRequest,
) -> Result<NewGameCreated, NewGameError> {
    let package = content.load().map_err(NewGameError::Package)?;
    let (manager_name, save_name, team) =
        validate(request, &package).map_err(NewGameError::Invalid)?;
    let world = generate(
        &package.template,
        &RngProvider::new(WorldSeed::new(request.seed)),
    )
    .map_err(NewGameError::Generation)?;
    let meta = SaveMeta {
        name: save_name,
        manager_name,
        team: team.clone(),
        genre: package.manifest.genre,
        seed: WorldSeed::new(request.seed),
        game_version: crate::about().version,
        game_date: package.template.settings.start_date,
        created_at: request.now,
        last_played_at: request.now,
        packages: vec![PackageRecord {
            id: package.manifest.id.clone(),
            version: package.manifest.version.clone(),
            content_hash: package.content_hash,
        }],
    };
    let file = store
        .create(NewSave {
            meta: &meta,
            world: &world,
            texts: &package.texts,
        })
        .map_err(NewGameError::Save)?;
    Ok(NewGameCreated { file, team })
}

fn validate(
    request: &NewGameRequest,
    package: &Package,
) -> Result<(String, String, ContentId), Vec<RequestProblem>> {
    let mut problems = Vec::new();
    let manager = request.manager_name.trim();
    let save = request.save_name.trim();
    let mut check = |text: &str, field: &'static str, empty, long, max: u8| {
        let length = text.chars().count();
        if length == 0 {
            problems.push(empty);
        } else if length > usize::from(max) {
            problems.push(long);
        }
        if text.chars().any(char::is_control) {
            problems.push(RequestProblem::ControlCharacters { field });
        }
    };
    check(
        manager,
        "manager",
        RequestProblem::EmptyManagerName,
        RequestProblem::ManagerNameTooLong,
        MANAGER_NAME_MAX,
    );
    check(
        save,
        "save",
        RequestProblem::EmptySaveName,
        RequestProblem::SaveNameTooLong,
        SAVE_NAME_MAX,
    );
    let team = ContentId::parse(&request.team)
        .ok()
        .filter(|id| package.template.teams.iter().any(|team| &team.id == id));
    if team.is_none() {
        problems.push(RequestProblem::UnknownTeam {
            team: request.team.clone(),
        });
    }
    match team {
        Some(team) if problems.is_empty() => Ok((manager.to_owned(), save.to_owned(), team)),
        _ => Err(problems),
    }
}

/// Every save, for Load Game.
///
/// # Errors
///
/// When the saves folder can't be read.
pub fn list_saves(store: &impl SaveStore) -> Result<Vec<SaveSummary>, SaveError> {
    Ok(store
        .list()?
        .into_iter()
        .map(|listing| match listing.save {
            Ok(meta) => summary(listing.file, &meta),
            Err(damage) => SaveSummary::Damaged {
                file: listing.file,
                damage: damage_view(damage),
            },
        })
        .collect())
}

/// The Load Game entry of a readable save.
#[must_use]
pub fn summary(file: String, meta: &SaveMeta) -> SaveSummary {
    SaveSummary::Ready {
        file,
        name: meta.name.clone(),
        manager_name: meta.manager_name.clone(),
        team_message: name_message(&meta.team),
        game_date: meta.game_date.to_string(),
        last_played_at: meta.last_played_at.0,
    }
}

const fn damage_view(damage: Damage) -> DamageView {
    match damage {
        Damage::NotASave => DamageView::NotASave,
        Damage::NewerVersion => DamageView::NewerVersion,
        Damage::InUse => DamageView::InUse,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryError {
    UnknownTeam(String),
    UnknownPlayer(String),
    Save(SaveError),
}

impl From<SaveError> for QueryError {
    fn from(error: SaveError) -> Self {
        Self::Save(error)
    }
}

/// Everything the roster and profile need besides the players themselves.
struct Context {
    roles: RoleSet,
    nationalities: BTreeMap<ContentId, Nationality>,
    competitions: Vec<Competition>,
    today: domain::date::GameDate,
}

impl Context {
    fn load(save: &mut impl OpenSave) -> Result<Self, SaveError> {
        Ok(Self {
            roles: save.roles()?,
            nationalities: save
                .nationalities()?
                .into_iter()
                .map(|n| (n.id.clone(), n))
                .collect(),
            competitions: save.competitions()?,
            today: save.meta().game_date,
        })
    }

    fn header(&self, team: &Team) -> TeamHeader {
        TeamHeader {
            id: team.id.to_string(),
            name_message: name_message(&team.id),
            crest: Crest::of(team),
            region_message: name_message(&team.region),
            league_message: self
                .competitions
                .iter()
                .find(|c| c.teams.contains(&team.id))
                .map(|c| name_message(&c.id)),
        }
    }

    fn role_position(&self, role: &ContentId) -> usize {
        self.roles
            .roles()
            .iter()
            .position(|r| r.id() == role)
            .unwrap_or(usize::MAX)
    }

    fn row(&self, player: &Player) -> RosterPlayer {
        let nationality = self.nationalities.get(&player.nationality);
        RosterPlayer {
            id: player.id.to_string(),
            nickname: player.nickname.clone(),
            given_name: player.given_name.clone(),
            family_name: player.family_name.clone(),
            name_order: nationality.map_or(NameOrderView::GivenFirst, |n| n.name_order.into()),
            role_id: player.role.to_string(),
            role_message: name_message(&player.role),
            nationality: NationalityView {
                id: player.nationality.to_string(),
                code: nationality.map_or_else(String::new, |n| n.code.as_str().to_owned()),
                name_message: name_message(&player.nationality),
            },
            age: player.birth_date.age_on(self.today).unwrap_or(0),
            overall: self
                .roles
                .get(&player.role)
                .map_or(0, |role| role.overall(&player.attributes).display()),
        }
    }
}

/// A team's roster: starters, then subs, each in role-set order, players in
/// the same role by nickname. The team defaults to the user's.
///
/// # Errors
///
/// When the team doesn't exist or storage fails.
pub fn roster(save: &mut impl OpenSave, team: Option<&str>) -> Result<RosterView, QueryError> {
    let team_id = match team {
        Some(text) => {
            ContentId::parse(text).map_err(|_| QueryError::UnknownTeam(text.to_owned()))?
        }
        None => save.meta().team.clone(),
    };
    let team = save
        .team(&team_id)?
        .ok_or_else(|| QueryError::UnknownTeam(team_id.to_string()))?;
    let context = Context::load(save)?;
    let mut players = save.players_of(&team.id)?;
    players.sort_by(|a, b| {
        context
            .role_position(&a.role)
            .cmp(&context.role_position(&b.role))
            .then_with(|| a.nickname.cmp(&b.nickname))
    });
    let (starters, subs): (Vec<&Player>, Vec<&Player>) = players.iter().partition(|p| p.starter);
    Ok(RosterView {
        team: context.header(&team),
        starters: starters.into_iter().map(|p| context.row(p)).collect(),
        subs: subs.into_iter().map(|p| context.row(p)).collect(),
    })
}

/// One player in detail. Hidden attributes carry no value (attributes spec).
///
/// # Errors
///
/// When the player doesn't exist or storage fails.
pub fn player_profile(save: &mut impl OpenSave, id: &str) -> Result<PlayerProfile, QueryError> {
    let unknown = || QueryError::UnknownPlayer(id.to_owned());
    let player_id = ContentId::parse_stored(id).map_err(|_| unknown())?;
    let player = save.player(&player_id)?.ok_or_else(unknown)?;
    let team = save
        .team(&player.team)?
        .ok_or_else(|| QueryError::UnknownTeam(player.team.to_string()))?;
    let context = Context::load(save)?;
    let attributes = &player.attributes;
    Ok(PlayerProfile {
        player: context.row(&player),
        starter: player.starter,
        team: context.header(&team),
        birth_date: player.birth_date.to_string(),
        skills: MobaSkill::ALL
            .iter()
            .map(|&skill| AttributeView {
                name_message: skill_message(skill).to_owned(),
                value: Some(attributes.skills.get(skill).display()),
            })
            .collect(),
        traits: CoreTrait::ALL
            .iter()
            .map(|&core| AttributeView {
                name_message: trait_message(core).to_owned(),
                value: Some(attributes.core.get_trait(core).display()),
            })
            .collect(),
        hidden: HiddenAttribute::ALL
            .iter()
            .map(|&hidden| AttributeView {
                name_message: hidden_message(hidden).to_owned(),
                value: None,
            })
            .collect(),
    })
}

/// The messages describing a New Game failure.
#[must_use]
pub fn new_game_messages(error: &NewGameError) -> Vec<Message> {
    match error {
        NewGameError::Invalid(problems) => problems.iter().map(request_message).collect(),
        NewGameError::Package(errors) => errors.iter().map(package_error).collect(),
        NewGameError::Generation(_) => vec![Message::new("error-generation")],
        NewGameError::Save(error) => vec![save_message(error)],
    }
}

fn request_message(problem: &RequestProblem) -> Message {
    match problem {
        RequestProblem::EmptyManagerName => Message::new("error-manager-name-empty"),
        RequestProblem::ManagerNameTooLong => {
            Message::new("error-manager-name-long").number("max", MANAGER_NAME_MAX)
        }
        RequestProblem::EmptySaveName => Message::new("error-save-name-empty"),
        RequestProblem::SaveNameTooLong => {
            Message::new("error-save-name-long").number("max", SAVE_NAME_MAX)
        }
        RequestProblem::ControlCharacters { field } => {
            Message::new("error-control-characters").text("field", *field)
        }
        RequestProblem::UnknownTeam { team } => {
            Message::new("error-unknown-team").text("team", team.clone())
        }
    }
}

/// The message describing a storage failure.
#[must_use]
pub fn save_message(error: &SaveError) -> Message {
    match error {
        SaveError::NameTaken(name) => {
            Message::new("error-save-name-taken").text("name", name.clone())
        }
        SaveError::NotFound(file) => {
            Message::new("error-save-not-found").text("file", file.clone())
        }
        SaveError::Damaged(Damage::NotASave) => Message::new("error-save-not-a-save"),
        SaveError::Damaged(Damage::NewerVersion) => Message::new("error-save-newer"),
        SaveError::Damaged(Damage::InUse) => Message::new("error-save-in-use"),
        SaveError::Storage(detail) => Message::new("error-storage").text("detail", detail.clone()),
    }
}

/// The message describing a query failure.
#[must_use]
pub fn query_message(error: &QueryError) -> Message {
    match error {
        QueryError::UnknownTeam(team) => {
            Message::new("error-unknown-team").text("team", team.clone())
        }
        QueryError::UnknownPlayer(player) => {
            Message::new("error-unknown-player").text("player", player.clone())
        }
        QueryError::Save(error) => save_message(error),
    }
}

#[cfg(test)]
#[path = "game_tests.rs"]
mod tests;
