//! Loading and validation of `.esm` content packages (ADR 0005, packages
//! spec).
//!
//! A package goes through four steps: its files are read ([`PackageFiles`]),
//! each file is parsed into its definition type, each definition is converted
//! into its domain type, and the whole is checked across entities. Every
//! problem from every step is reported together.

mod check;
mod convert;
pub mod defs;
mod files;
mod parse;
mod report;
pub mod schemas;

use std::path::Path;

use esports::moba::world::WorldTemplate;

pub use convert::Manifest;
pub use files::PackageFiles;
pub use parse::TextFile;
pub use report::{NameKind, PackageError, Problem};

use convert::Located;
use parse::{Entry, MANIFEST, WORLD};

/// A package that passed every check.
#[derive(Debug, Clone)]
pub struct Package {
    pub manifest: Manifest,
    /// SHA-256 of the package's files; saves record it (saves spec).
    pub content_hash: [u8; 32],
    /// The world the package defines, ready for the generator.
    pub template: WorldTemplate,
    pub texts: Vec<TextFile>,
}

/// Loads the package in the directory `root`.
///
/// # Errors
///
/// Every problem found, sorted by file.
pub fn load(root: &Path) -> Result<Package, Vec<PackageError>> {
    let files = PackageFiles::read_dir(root).map_err(|error| {
        vec![PackageError::new(
            "",
            "",
            Problem::Unreadable {
                reason: error.to_string(),
            },
        )]
    })?;
    load_files(&files)
}

/// Loads a package from files in memory.
///
/// # Errors
///
/// Every problem found, sorted by file.
pub fn load_files(files: &PackageFiles) -> Result<Package, Vec<PackageError>> {
    let mut errors = Vec::new();
    let parsed = parse::parse(files, &mut errors);

    let manifest = required(parsed.manifest.as_ref(), MANIFEST, &mut errors)
        .and_then(|entry| convert::manifest(entry, &mut errors));
    let world = required(parsed.world.as_ref(), WORLD, &mut errors)
        .and_then(|entry| convert::world(entry, &mut errors));
    let regions = convert::all(&parsed.regions, &mut errors, convert::region);
    let nationalities = convert::all(&parsed.nationalities, &mut errors, convert::nationality);
    let name_pools = convert::all(&parsed.name_pools, &mut errors, convert::name_pool);
    let nicknames = convert::all(&parsed.nicknames, &mut errors, convert::nickname_pool);
    let competitions = convert::all(&parsed.competitions, &mut errors, convert::competition);
    let teams = convert::all(&parsed.teams, &mut errors, convert::team);
    let players = convert::all(&parsed.players, &mut errors, convert::player);
    let roles = exactly_one(
        &parsed.role_sets,
        "roles/",
        "role_set",
        convert::all(&parsed.role_sets, &mut errors, convert::role_set),
        &mut errors,
    );
    let generation = exactly_one(
        &parsed.generations,
        "generation/",
        "generation",
        convert::all(&parsed.generations, &mut errors, convert::generation),
        &mut errors,
    );

    check::check(
        &parsed,
        &check::Content {
            regions: &regions,
            nationalities: &nationalities,
            name_pools: &name_pools,
            nicknames: &nicknames,
            competitions: &competitions,
            teams: &teams,
            players: &players,
            roles: roles.as_ref(),
            generation: generation.as_ref(),
        },
        manifest.as_ref(),
        &mut errors,
    );

    let (Some(manifest), Some(world), Some(roles), Some(generation), true) =
        (manifest, world, roles, generation, errors.is_empty())
    else {
        errors.sort_by(|a, b| a.file.cmp(&b.file));
        return Err(errors);
    };
    let (roles, roster) = roles.value;
    Ok(Package {
        manifest,
        content_hash: files.content_hash(),
        template: WorldTemplate {
            settings: world,
            regions: values(regions),
            nationalities: values(nationalities),
            name_pools: values(name_pools),
            nicknames: merge_nicknames(nicknames),
            competitions: values(competitions),
            teams: values(teams),
            players: values(players),
            roles,
            roster,
            generation: generation.value,
        },
        texts: parsed.texts,
    })
}

fn required<'a, T>(
    entry: Option<&'a Entry<T>>,
    file: &str,
    errors: &mut Vec<PackageError>,
) -> Option<&'a Entry<T>> {
    if entry.is_none() {
        errors.push(PackageError::new(file, "", Problem::MissingFile));
    }
    entry
}

/// The one entity of a kind a package must define exactly once.
fn exactly_one<D, V>(
    entries: &[Entry<D>],
    folder: &str,
    kind: &str,
    converted: Vec<Located<V>>,
    errors: &mut Vec<PackageError>,
) -> Option<Located<V>> {
    if entries.len() != 1 {
        let file = entries.get(1).map_or(folder, |entry| entry.file.as_str());
        errors.push(PackageError::new(
            file,
            "",
            Problem::WrongCount {
                kind: kind.to_owned(),
                count: entries.len(),
            },
        ));
        return None;
    }
    converted.into_iter().next()
}

fn values<T>(located: Vec<Located<T>>) -> Vec<T> {
    located.into_iter().map(|located| located.value).collect()
}

fn merge_nicknames(pools: Vec<Located<Vec<String>>>) -> Vec<String> {
    let mut seen = std::collections::BTreeSet::new();
    pools
        .into_iter()
        .flat_map(|pool| pool.value)
        .filter(|nickname| seen.insert(nickname.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests;
