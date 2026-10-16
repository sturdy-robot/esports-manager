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
pub mod schemas;

use std::path::Path;

use esports::moba::world::WorldTemplate;

pub use app::content::{Manifest, NameKind, Package, PackageError, Problem};
pub use files::PackageFiles;

use convert::Located;
use parse::{Entry, MANIFEST, Parsed, WORLD};

/// Loads the package in the directory `root`.
///
/// # Errors
///
/// Every problem found, sorted by file.
pub fn load(root: &Path) -> Result<Package, Vec<PackageError>> {
    let unreadable = |file: String, error: &std::io::Error| {
        PackageError::new(
            file,
            "",
            Problem::Unreadable {
                reason: error.to_string(),
            },
        )
    };
    let (files, problems) = PackageFiles::read_dir_reporting(root)
        .map_err(|error| vec![unreadable(root.display().to_string(), &error)])?;
    if !problems.is_empty() {
        return Err(problems
            .iter()
            .map(|(file, error)| unreadable(file.clone(), error))
            .collect());
    }
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

    let manifest = required(&parsed, parsed.manifest.as_ref(), MANIFEST, &mut errors)
        .and_then(|entry| convert::manifest(entry, &mut errors));
    let world = required(&parsed, parsed.world.as_ref(), WORLD, &mut errors)
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
        convert::all(&parsed.role_sets, &mut errors, convert::role_set),
        &mut errors,
    );
    let generation = exactly_one(
        &parsed.generations,
        "generation/",
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
            start_date: world.map(|world| world.start_date),
        },
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

/// A required root file. It is missing only if the package has no such
/// file; one that is there but broken has already been reported.
fn required<'a, T>(
    parsed: &Parsed,
    entry: Option<&'a Entry<T>>,
    file: &'static str,
    errors: &mut Vec<PackageError>,
) -> Option<&'a Entry<T>> {
    if entry.is_none() && !parsed.present.contains(&file) {
        errors.push(PackageError::new(file, "", Problem::MissingFile));
    }
    entry
}

/// The one entity of a kind a package must define exactly once.
fn exactly_one<D, V>(
    entries: &[Entry<D>],
    folder: &str,
    converted: Vec<Located<V>>,
    errors: &mut Vec<PackageError>,
) -> Option<Located<V>> {
    if entries.len() != 1 {
        let file = entries.get(1).map_or(folder, |entry| entry.file.as_str());
        errors.push(PackageError::new(
            file,
            "",
            Problem::WrongCount {
                folder: folder.to_owned(),
                count: entries.len(),
            },
        ));
        return None;
    }
    converted.into_iter().next()
}

/// The package in a directory, as the application's content source.
#[derive(Debug, Clone)]
pub struct PackageDir(pub std::path::PathBuf);

impl app::content::ContentSource for PackageDir {
    fn load(&self) -> Result<Package, Vec<PackageError>> {
        load(&self.0)
    }
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
