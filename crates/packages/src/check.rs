//! Checks across entities: unique ids, references that resolve, text that
//! exists, and content the generator can always use (packages and world
//! generation specs). A package that passes never makes New Game fail.

use std::collections::{BTreeMap, BTreeSet};

use domain::id::ContentId;
use domain::world::{Competition, NamePool, Nationality, Region, Team};
use esports::moba::content::{GenerationParams, PinnedPlayer, RosterRules};
use esports::moba::roles::{Role, RoleSet};
use fluent_syntax::ast;

use crate::convert::{Located, Manifest};
use app::saves::ContentText;

use crate::parse::{Entry, Parsed};
use crate::report::{NameKind, PackageError, Problem};

/// Everything that converted cleanly, by kind.
pub struct Content<'a> {
    pub regions: &'a [Located<Region>],
    pub nationalities: &'a [Located<Nationality>],
    pub name_pools: &'a [Located<NamePool>],
    pub nicknames: &'a [Located<Vec<String>>],
    pub competitions: &'a [Located<Competition>],
    pub teams: &'a [Located<Team>],
    pub players: &'a [Located<PinnedPlayer>],
    pub roles: Option<&'a Located<(RoleSet, RosterRules)>>,
    pub generation: Option<&'a Located<GenerationParams>>,
}

pub fn check(
    parsed: &Parsed,
    content: &Content<'_>,
    manifest: Option<&Manifest>,
    errors: &mut Vec<PackageError>,
) {
    duplicates(parsed, errors);
    references(content, errors);
    if let Some(manifest) = manifest {
        text(&parsed.texts, content, manifest, errors);
    }
    mixes_and_pools(content, errors);
    if let Some(roles) = content.roles
        && rosters(content, roles, errors)
        && let Some(generation) = content.generation
    {
        nickname_pool(
            content,
            roles.value.1,
            &roles.value.0,
            &generation.value,
            errors,
        );
    }
}

fn duplicates(parsed: &Parsed, errors: &mut Vec<PackageError>) {
    fn ids<T>(
        entries: &[Entry<T>],
        id: impl Fn(&T) -> &str,
    ) -> impl Iterator<Item = (&str, &str, String)> {
        entries
            .iter()
            .map(move |entry| (id(&entry.def), entry.file.as_str(), entry.at("id")))
    }
    let all = ids(&parsed.competitions, |d| &d.id)
        .chain(ids(&parsed.generations, |d| &d.id))
        .chain(ids(&parsed.name_pools, |d| &d.id))
        .chain(ids(&parsed.nationalities, |d| &d.id))
        .chain(ids(&parsed.nicknames, |d| &d.id))
        .chain(ids(&parsed.players, |d| &d.id))
        .chain(ids(&parsed.regions, |d| &d.id))
        .chain(ids(&parsed.role_sets, |d| &d.id))
        .chain(ids(&parsed.teams, |d| &d.id));
    let mut seen: BTreeMap<&str, &str> = BTreeMap::new();
    for (text, file, path) in all {
        let Ok(id) = ContentId::parse(text) else {
            continue;
        };
        match seen.get(text) {
            Some(first) => errors.push(PackageError::new(
                file,
                path,
                Problem::DuplicateId {
                    id,
                    other_file: (*first).to_owned(),
                },
            )),
            None => {
                seen.insert(text, file);
            }
        }
    }
}

fn references(content: &Content<'_>, errors: &mut Vec<PackageError>) {
    let regions: BTreeSet<&ContentId> = content.regions.iter().map(|r| &r.value.id).collect();
    let nationalities: BTreeSet<&ContentId> =
        content.nationalities.iter().map(|n| &n.value.id).collect();
    let teams: BTreeSet<&ContentId> = content.teams.iter().map(|t| &t.value.id).collect();
    let roles: Option<BTreeSet<&ContentId>> = content
        .roles
        .map(|roles| roles.value.0.roles().iter().map(Role::id).collect());

    let mut require = |set: &BTreeSet<&ContentId>,
                       at: &dyn Fn(Problem) -> PackageError,
                       field: &str,
                       id: &ContentId| {
        if !set.contains(id) {
            errors.push(at(Problem::UnresolvedReference {
                field: field.to_owned(),
                id: id.to_string(),
            }));
        }
    };

    for region in content.regions {
        for (nationality, _) in &region.value.nationalities {
            let field = format!("nationalities.{nationality}");
            require(
                &nationalities,
                &|p| region.error(&field, p),
                &field,
                nationality,
            );
        }
    }
    for pool in content.name_pools {
        require(
            &nationalities,
            &|p| pool.error("nationality", p),
            "nationality",
            &pool.value.nationality,
        );
    }
    for competition in content.competitions {
        let value = &competition.value;
        require(
            &regions,
            &|p| competition.error("region", p),
            "region",
            &value.region,
        );
        for (index, team) in value.teams.iter().enumerate() {
            let field = format!("teams[{index}]");
            require(&teams, &|p| competition.error(&field, p), &field, team);
        }
    }
    for team in content.teams {
        require(
            &regions,
            &|p| team.error("region", p),
            "region",
            &team.value.region,
        );
    }
    for player in content.players {
        let value = &player.value;
        require(&teams, &|p| player.error("team", p), "team", &value.team);
        if let Some(roles) = &roles {
            require(roles, &|p| player.error("role", p), "role", &value.role);
        }
        if let Some(nationality) = &value.nationality {
            require(
                &nationalities,
                &|p| player.error("nationality", p),
                "nationality",
                nationality,
            );
        }
    }
}

fn text(
    texts: &[ContentText],
    content: &Content<'_>,
    manifest: &Manifest,
    errors: &mut Vec<PackageError>,
) {
    let mut messages: BTreeMap<&str, BTreeSet<String>> = BTreeMap::new();
    for file in texts {
        if !manifest.locales.contains(&file.locale) {
            errors.push(PackageError::new(
                &file.file,
                "",
                Problem::UnknownLocale {
                    locale: file.locale.clone(),
                },
            ));
            continue;
        }
        let resource = match fluent_syntax::parser::parse(file.source.as_str()) {
            Ok(resource) => resource,
            Err((resource, problems)) => {
                for problem in problems {
                    let line = file.source[..problem.pos.start.min(file.source.len())]
                        .matches('\n')
                        .count()
                        + 1;
                    errors.push(PackageError::new(
                        &file.file,
                        "",
                        Problem::TextSyntax { line },
                    ));
                }
                resource
            }
        };
        let ids = messages.entry(file.locale.as_str()).or_default();
        for entry in resource.body {
            if let ast::Entry::Message(message) = entry {
                ids.insert(message.id.name.to_owned());
            }
        }
    }

    let Some(first) = manifest.locales.first() else {
        return;
    };
    let empty = BTreeSet::new();
    let defined = messages.get(first.as_str()).unwrap_or(&empty);
    let mut require = |id: &ContentId, at: &dyn Fn(Problem) -> PackageError| {
        let message = id.message_id();
        if !defined.contains(&message) {
            errors.push(at(Problem::MissingText {
                message,
                locale: first.clone(),
            }));
        }
    };
    for region in content.regions {
        require(&region.value.id, &|p| region.error("id", p));
    }
    for nationality in content.nationalities {
        require(&nationality.value.id, &|p| nationality.error("id", p));
    }
    for competition in content.competitions {
        require(&competition.value.id, &|p| competition.error("id", p));
    }
    for team in content.teams {
        require(&team.value.id, &|p| team.error("id", p));
    }
    if let Some(roles) = content.roles {
        for (index, role) in roles.value.0.roles().iter().enumerate() {
            require(role.id(), &|p| {
                roles.error(&format!("roles[{index}].id"), p)
            });
        }
    }
}

fn mixes_and_pools(content: &Content<'_>, errors: &mut Vec<PackageError>) {
    let mut drawable = BTreeSet::new();
    for region in content.regions {
        let mix = &region.value.nationalities;
        let total: u64 = mix.iter().map(|&(_, weight)| u64::from(weight)).sum();
        if total == 0 {
            errors.push(region.error(
                "nationalities",
                Problem::InvalidMix {
                    region: region.value.id.clone(),
                },
            ));
        }
        drawable.extend(
            mix.iter()
                .filter(|(_, weight)| *weight > 0)
                .map(|(id, _)| id),
        );
    }
    for nationality in content.nationalities {
        let id = &nationality.value.id;
        if !drawable.contains(id) {
            continue;
        }
        let pools = content
            .name_pools
            .iter()
            .filter(|pool| &pool.value.nationality == id);
        let (mut given, mut family) = (false, false);
        for pool in pools {
            given |= !pool.value.given.is_empty();
            family |= !pool.value.family.is_empty();
        }
        for (has, names) in [(given, NameKind::Given), (family, NameKind::Family)] {
            if !has {
                errors.push(nationality.error(
                    "id",
                    Problem::EmptyNamePool {
                        nationality: id.clone(),
                        names,
                    },
                ));
            }
        }
    }
}

/// Checks the roster shape, pinned nicknames and pinned rosters. Returns
/// whether the roster shape itself is usable.
fn rosters(
    content: &Content<'_>,
    located: &Located<(RoleSet, RosterRules)>,
    errors: &mut Vec<PackageError>,
) -> bool {
    let (role_set, rules) = &located.value;
    let role_count = role_set.roles().len();
    if usize::from(rules.subs) > role_count {
        errors.push(located.error(
            "subs",
            Problem::TooManySubs {
                subs: rules.subs,
                roles: role_count,
            },
        ));
        return false;
    }

    let mut nicknames: BTreeMap<String, &ContentId> = BTreeMap::new();
    for player in content.players {
        let Some(nickname) = &player.value.nickname else {
            continue;
        };
        match nicknames.get(&nickname.to_lowercase()) {
            Some(&other) => errors.push(player.error(
                "nickname",
                Problem::DuplicateNickname {
                    nickname: nickname.clone(),
                    other: other.clone(),
                },
            )),
            None => {
                nicknames.insert(nickname.to_lowercase(), &player.value.id);
            }
        }
    }

    for team in content.teams {
        let mut counts: BTreeMap<&ContentId, usize> = BTreeMap::new();
        for player in content
            .players
            .iter()
            .filter(|p| p.value.team == team.value.id)
        {
            *counts.entry(&player.value.role).or_default() += 1;
        }
        let doubled = counts.values().filter(|&&count| count >= 2).count();
        let crowded = counts.values().any(|&count| count > 2);
        if crowded || doubled > usize::from(rules.subs) {
            let missing_roles = role_set
                .roles()
                .iter()
                .map(Role::id)
                .filter(|id| !counts.contains_key(id))
                .cloned()
                .collect();
            errors.push(team.error(
                "id",
                Problem::IncompletableRoster {
                    team: team.value.id.clone(),
                    missing_roles,
                },
            ));
        }
    }
    true
}

fn nickname_pool(
    content: &Content<'_>,
    rules: RosterRules,
    role_set: &RoleSet,
    generation: &GenerationParams,
    errors: &mut Vec<PackageError>,
) {
    let pinned: BTreeSet<String> = content
        .players
        .iter()
        .filter_map(|player| player.value.nickname.as_ref())
        .map(|nickname| nickname.to_lowercase())
        .collect();
    let distinct = content
        .nicknames
        .iter()
        .flat_map(|pool| &pool.value)
        .map(|nickname| nickname.to_lowercase())
        .filter(|nickname| !pinned.contains(nickname))
        .collect::<BTreeSet<_>>()
        .len();
    let roster_size = role_set.roles().len() + usize::from(rules.subs);
    let generated: usize = content
        .teams
        .iter()
        .map(|team| {
            let pinned = content
                .players
                .iter()
                .filter(|p| p.value.team == team.value.id)
                .count();
            roster_size.saturating_sub(pinned)
        })
        .sum();
    let factor = usize::try_from(generation.nickname_pool_factor).unwrap_or(usize::MAX);
    let needed = generated.saturating_mul(factor);
    if distinct < needed {
        let file = content
            .nicknames
            .first()
            .map_or_else(|| "nicknames/".to_owned(), |pool| pool.file.clone());
        errors.push(PackageError::new(
            file,
            "",
            Problem::NicknamePoolTooSmall { distinct, needed },
        ));
    }
}
