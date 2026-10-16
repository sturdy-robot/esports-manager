//! One SQLite file per save (saves spec, ADR 0004).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use app::saves::{
    ContentText, Damage, NewSave, OpenSave, PackageRecord, SaveError, SaveListing, SaveMeta,
    SaveStore, Timestamp,
};
use diesel::migration::MigrationSource;
use diesel::prelude::*;
use diesel::query_dsl::methods::OrderDsl;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use domain::id::ContentId;
use domain::rng::WorldSeed;
use domain::world::{Competition, Nationality, Region, Team};
use esports::moba::attributes::OverallAttribute;
use esports::moba::roles::RoleSet;
use esports::moba::world::{Player, World};

use crate::rows::{
    self, CompetitionRow, CompetitionTeamRow, NationalityRow, PackageRow, PlayerRow,
    RegionNationalityRow, RegionRow, RoleRow, SaveMetaRow, TeamRow, TextRow,
};
use crate::schema::{
    competition_teams, competitions, content_texts, nationalities, packages, players,
    region_nationalities, regions, roles, save_meta, teams,
};
use crate::slug::{free_stem, slug};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// The extension of save files.
pub const EXTENSION: &str = "esmsave";
const TEMPORARY: &str = "tmp";

/// Saves in one folder.
#[derive(Debug, Clone)]
pub struct SqliteSaveStore {
    dir: PathBuf,
}

/// A save this process holds open, exclusively.
pub struct SqliteSave {
    connection: SqliteConnection,
    meta: SaveMeta,
}

impl SqliteSaveStore {
    /// The store for `dir`. Leftovers of crashed creations are removed.
    #[must_use]
    pub fn new(dir: PathBuf) -> Self {
        let store = Self { dir };
        store.remove_leftovers();
        store
    }

    fn path(&self, file: &str) -> PathBuf {
        self.dir.join(file)
    }

    fn files(&self, extension: &str) -> Result<Vec<String>, SaveError> {
        let entries =
            std::fs::read_dir(&self.dir).map_err(|e| SaveError::Storage(e.to_string()))?;
        let mut files: Vec<String> = entries
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_ok_and(|t| t.is_file()))
            .filter_map(|entry| entry.file_name().into_string().ok())
            .filter(|name| Path::new(name).extension().is_some_and(|e| e == extension))
            .collect();
        files.sort();
        Ok(files)
    }

    /// Removes `.tmp` files no other process holds open.
    fn remove_leftovers(&self) {
        for file in self.files(TEMPORARY).unwrap_or_default() {
            let path = self.path(&file);
            // Only a lock held by another process keeps the file; anything
            // else, including a file that isn't a database yet, is a leftover.
            let free = connect(&path).is_ok_and(|mut connection| {
                match diesel::sql_query("BEGIN EXCLUSIVE").execute(&mut connection) {
                    Ok(_) => diesel::sql_query("ROLLBACK")
                        .execute(&mut connection)
                        .is_ok(),
                    Err(error) => !is_locked(&error),
                }
            });
            if free {
                let _ = std::fs::remove_file(&path);
            }
        }
    }

    fn read_listing(&self, file: &str) -> SaveListing {
        let save = connect(&self.path(file))
            .map_err(|_| Damage::NotASave)
            .and_then(|mut connection| read_meta(&mut connection));
        SaveListing {
            file: file.to_owned(),
            save,
        }
    }
}

static ATTEMPTS: AtomicU64 = AtomicU64::new(0);

impl SaveStore for SqliteSaveStore {
    type Save = SqliteSave;

    fn create(&self, save: NewSave<'_>) -> Result<String, SaveError> {
        let wanted = save.meta.name.to_lowercase();
        let taken = self.list()?.into_iter().any(|listing| {
            listing
                .save
                .is_ok_and(|meta| meta.name.to_lowercase() == wanted)
        });
        if taken {
            return Err(SaveError::NameTaken(save.meta.name.clone()));
        }
        let stem = free_stem(&slug(&save.meta.name), |stem| {
            self.path(&format!("{stem}.{EXTENSION}")).exists()
        });
        let file = format!("{stem}.{EXTENSION}");
        let attempt = ATTEMPTS.fetch_add(1, Ordering::Relaxed);
        let temporary = self.path(&format!(
            "{stem}.{}-{attempt}.{TEMPORARY}",
            std::process::id()
        ));

        let written = write_new(&temporary, save)
            .and_then(|()| commit(&temporary, &self.path(&file), &save.meta.name));
        if written.is_err() {
            let _ = std::fs::remove_file(&temporary);
        }
        written.map(|()| file)
    }

    fn list(&self) -> Result<Vec<SaveListing>, SaveError> {
        let mut listings: Vec<SaveListing> = self
            .files(EXTENSION)?
            .iter()
            .map(|file| self.read_listing(file))
            .collect();
        listings.sort_by(|a, b| match (&a.save, &b.save) {
            (Ok(x), Ok(y)) => y
                .last_played_at
                .cmp(&x.last_played_at)
                .then_with(|| x.name.cmp(&y.name))
                .then_with(|| a.file.cmp(&b.file)),
            (Ok(_), Err(_)) => std::cmp::Ordering::Less,
            (Err(_), Ok(_)) => std::cmp::Ordering::Greater,
            (Err(_), Err(_)) => a.file.cmp(&b.file),
        });
        Ok(listings)
    }

    fn open(&self, file: &str, now: Timestamp) -> Result<SqliteSave, SaveError> {
        let is_save_file = Path::new(file).extension().is_some_and(|e| e == EXTENSION)
            && !file.contains(['/', '\\']);
        let path = self.path(file);
        if !is_save_file || !path.is_file() {
            return Err(SaveError::NotFound(file.to_owned()));
        }
        let mut connection = connect(&path).map_err(|_| SaveError::Damaged(Damage::NotASave))?;
        diesel::sql_query("PRAGMA locking_mode = EXCLUSIVE")
            .execute(&mut connection)
            .map_err(storage)?;
        read_meta(&mut connection).map_err(SaveError::Damaged)?;
        connection
            .run_pending_migrations(MIGRATIONS)
            .map_err(|e| SaveError::Storage(e.to_string()))?;
        // The first write takes the exclusive lock, held until the save closes.
        diesel::update(save_meta::table)
            .set(save_meta::last_played_at.eq(now.0))
            .execute(&mut connection)
            .map_err(storage)?;
        let meta = read_meta(&mut connection).map_err(SaveError::Damaged)?;
        Ok(SqliteSave { connection, meta })
    }
}

/// Opens a connection that fails at once, rather than waiting, when another
/// process holds the file.
fn connect(path: &Path) -> Result<SqliteConnection, SaveError> {
    let url = path
        .to_str()
        .ok_or_else(|| SaveError::Storage(format!("{} is not a UTF-8 path", path.display())))?;
    let mut connection =
        SqliteConnection::establish(url).map_err(|e| SaveError::Storage(e.to_string()))?;
    diesel::sql_query("PRAGMA busy_timeout = 0")
        .execute(&mut connection)
        .map_err(storage)?;
    diesel::sql_query("PRAGMA foreign_keys = ON")
        .execute(&mut connection)
        .map_err(storage)?;
    Ok(connection)
}

fn is_locked(error: &diesel::result::Error) -> bool {
    error.to_string().contains("locked")
}

#[expect(clippy::needless_pass_by_value, reason = "passed to `map_err` by name")]
fn storage(error: diesel::result::Error) -> SaveError {
    if is_locked(&error) {
        SaveError::Damaged(Damage::InUse)
    } else {
        SaveError::Storage(error.to_string())
    }
}

/// Reads `save_meta` without changing the file, checking that it is a save
/// this version of the game can open.
fn read_meta(connection: &mut SqliteConnection) -> Result<SaveMeta, Damage> {
    let damage = |error: &diesel::result::Error| {
        if is_locked(error) {
            Damage::InUse
        } else {
            Damage::NotASave
        }
    };
    let row = save_meta::table
        .select(SaveMetaRow::as_select())
        .first(connection)
        .map_err(|e| damage(&e))?;
    if is_newer(connection)? {
        return Err(Damage::NewerVersion);
    }
    let packages: Vec<PackageRow> =
        load_ordered(connection, packages::table, packages::position.asc())
            .map_err(|_| Damage::NotASave)?;
    meta_from_rows(row, packages).map_err(|_| Damage::NotASave)
}

/// Whether the save has a migration this version of the game doesn't know.
fn is_newer(connection: &mut SqliteConnection) -> Result<bool, Damage> {
    #[derive(QueryableByName)]
    struct Version {
        #[diesel(sql_type = diesel::sql_types::Text)]
        version: String,
    }
    let applied: Vec<Version> =
        diesel::sql_query("SELECT version FROM __diesel_schema_migrations ORDER BY version")
            .load(connection)
            .map_err(|_| Damage::NotASave)?;
    let known: Vec<String> = MigrationSource::<diesel::sqlite::Sqlite>::migrations(&MIGRATIONS)
        .map_err(|_| Damage::NotASave)?
        .iter()
        .map(|migration| migration.name().version().to_string())
        .collect();
    Ok(applied.iter().any(|row| !known.contains(&row.version)))
}

fn meta_from_rows(row: SaveMetaRow, packages: Vec<PackageRow>) -> Result<SaveMeta, SaveError> {
    let SaveMetaRow {
        id: _,
        name,
        manager_name,
        team_id,
        genre,
        seed,
        game_version,
        game_date,
        created_at,
        last_played_at,
    } = row;
    let packages = packages
        .into_iter()
        .map(
            |PackageRow {
                 id,
                 position: _,
                 version,
                 content_hash,
             }| {
                let content_hash: [u8; 32] = content_hash
                    .try_into()
                    .map_err(|_| rows::corrupt("content hash", &id))?;
                Ok(PackageRecord {
                    id,
                    version,
                    content_hash,
                })
            },
        )
        .collect::<Result<_, SaveError>>()?;
    Ok(SaveMeta {
        name,
        manager_name,
        team: rows::id(&team_id)?,
        genre: rows::genre_from_db(&genre)?,
        seed: WorldSeed::new(rows::seed_from_db(seed)),
        game_version,
        game_date: rows::date(&game_date)?,
        created_at: Timestamp(created_at),
        last_played_at: Timestamp(last_played_at),
        packages,
    })
}

/// Every query that returns a collection goes through here, so its order is
/// always spelled out (ADR 0003). Review checks that no query bypasses it.
fn load_ordered<'q, Q, O, T>(
    connection: &mut SqliteConnection,
    query: Q,
    order: O,
) -> QueryResult<Vec<T>>
where
    O: diesel::Expression,
    Q: OrderDsl<O>,
    Q::Output: diesel::query_dsl::LoadQuery<'q, SqliteConnection, T>,
{
    query.order(order).load(connection)
}

fn write_new(path: &Path, save: NewSave<'_>) -> Result<(), SaveError> {
    let mut connection = connect(path)?;
    // Held until the connection closes, so a cleanup in another process
    // never removes a file still being written.
    diesel::sql_query("PRAGMA locking_mode = EXCLUSIVE")
        .execute(&mut connection)
        .map_err(storage)?;
    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|e| SaveError::Storage(e.to_string()))?;
    connection
        .transaction(|connection| insert_world(connection, save))
        .map_err(storage)
}

fn insert_world(connection: &mut SqliteConnection, save: NewSave<'_>) -> QueryResult<()> {
    let to_query =
        |error: SaveError| diesel::result::Error::QueryBuilderError(error.to_string().into());
    let NewSave { meta, world, texts } = save;
    insert_places(connection, world)?;
    insert_teams(connection, world)?;
    insert_people(connection, world)?;
    let SaveMeta {
        name,
        manager_name,
        team,
        genre,
        seed,
        game_version,
        game_date,
        created_at,
        last_played_at,
        packages: package_records,
    } = meta;
    diesel::insert_into(save_meta::table)
        .values(SaveMetaRow {
            id: 1,
            name: name.clone(),
            manager_name: manager_name.clone(),
            team_id: team.to_string(),
            genre: rows::genre_to_db(*genre).to_owned(),
            seed: rows::seed_to_db(seed.value()),
            game_version: game_version.clone(),
            game_date: game_date.to_string(),
            created_at: created_at.0,
            last_played_at: last_played_at.0,
        })
        .execute(connection)?;

    let package_rows = package_records
        .iter()
        .enumerate()
        .map(|(index, record)| {
            Ok(PackageRow {
                id: record.id.clone(),
                position: rows::position(index)?,
                version: record.version.clone(),
                content_hash: record.content_hash.to_vec(),
            })
        })
        .collect::<Result<Vec<_>, SaveError>>()
        .map_err(to_query)?;
    diesel::insert_into(packages::table)
        .values(&package_rows)
        .execute(connection)?;

    let text_rows: Vec<TextRow> = texts
        .iter()
        .map(
            |ContentText {
                 locale,
                 file,
                 source,
             }| TextRow {
                locale: locale.clone(),
                file: file.clone(),
                source: source.clone(),
            },
        )
        .collect();
    diesel::insert_into(content_texts::table)
        .values(&text_rows)
        .execute(connection)?;
    Ok(())
}

fn insert_places(connection: &mut SqliteConnection, world: &World) -> QueryResult<()> {
    let to_query =
        |error: SaveError| diesel::result::Error::QueryBuilderError(error.to_string().into());
    let nationality_rows = world
        .nationalities
        .iter()
        .enumerate()
        .map(|(index, nationality)| {
            Ok(NationalityRow {
                id: nationality.id.to_string(),
                position: rows::position(index)?,
                code: nationality.code.as_str().to_owned(),
                name_order: rows::name_order_to_db(nationality.name_order).to_owned(),
            })
        })
        .collect::<Result<Vec<_>, SaveError>>()
        .map_err(to_query)?;
    diesel::insert_into(nationalities::table)
        .values(&nationality_rows)
        .execute(connection)?;

    let mut region_rows = Vec::new();
    let mut mix_rows = Vec::new();
    for (index, region) in world.regions.iter().enumerate() {
        region_rows.push(RegionRow {
            id: region.id.to_string(),
            position: rows::position(index).map_err(to_query)?,
            level: region.level,
        });
        for (nationality, weight) in &region.nationalities {
            mix_rows.push(RegionNationalityRow {
                region_id: region.id.to_string(),
                nationality_id: nationality.to_string(),
                weight: i32::try_from(*weight)
                    .map_err(|_| to_query(rows::corrupt("weight", &weight.to_string())))?,
            });
        }
    }
    diesel::insert_into(regions::table)
        .values(&region_rows)
        .execute(connection)?;
    diesel::insert_into(region_nationalities::table)
        .values(&mix_rows)
        .execute(connection)?;
    Ok(())
}

fn insert_teams(connection: &mut SqliteConnection, world: &World) -> QueryResult<()> {
    let to_query =
        |error: SaveError| diesel::result::Error::QueryBuilderError(error.to_string().into());
    let team_rows = world
        .teams
        .iter()
        .enumerate()
        .map(|(index, team)| TeamRow::from_domain(team, index))
        .collect::<Result<Vec<_>, _>>()
        .map_err(to_query)?;
    diesel::insert_into(teams::table)
        .values(&team_rows)
        .execute(connection)?;

    let mut competition_rows = Vec::new();
    let mut entrant_rows = Vec::new();
    for (index, competition) in world.competitions.iter().enumerate() {
        competition_rows.push(CompetitionRow {
            id: competition.id.to_string(),
            position: rows::position(index).map_err(to_query)?,
            region_id: competition.region.to_string(),
        });
        for (slot, team) in competition.teams.iter().enumerate() {
            entrant_rows.push(CompetitionTeamRow {
                competition_id: competition.id.to_string(),
                position: rows::position(slot).map_err(to_query)?,
                team_id: team.to_string(),
            });
        }
    }
    diesel::insert_into(competitions::table)
        .values(&competition_rows)
        .execute(connection)?;
    diesel::insert_into(competition_teams::table)
        .values(&entrant_rows)
        .execute(connection)?;
    Ok(())
}

fn insert_people(connection: &mut SqliteConnection, world: &World) -> QueryResult<()> {
    let to_query =
        |error: SaveError| diesel::result::Error::QueryBuilderError(error.to_string().into());
    let role_rows = world
        .roles
        .roles()
        .iter()
        .enumerate()
        .map(|(index, role)| RoleRow::from_domain(role, index))
        .collect::<Result<Vec<_>, _>>()
        .map_err(to_query)?;
    diesel::insert_into(roles::table)
        .values(&role_rows)
        .execute(connection)?;

    let player_rows = world
        .players
        .iter()
        .enumerate()
        .map(|(index, player)| PlayerRow::from_domain(player, index))
        .collect::<Result<Vec<_>, _>>()
        .map_err(to_query)?;
    diesel::insert_into(players::table)
        .values(&player_rows)
        .execute(connection)?;

    Ok(())
}

/// Moves a finished save into place without ever replacing a file.
fn commit(temporary: &Path, target: &Path, name: &str) -> Result<(), SaveError> {
    commit_with(
        |from, to| std::fs::hard_link(from, to),
        temporary,
        target,
        name,
    )
}

/// `commit`, with the hard link as a parameter so the fallback is testable.
fn commit_with(
    link: impl Fn(&Path, &Path) -> std::io::Result<()>,
    temporary: &Path,
    target: &Path,
    name: &str,
) -> Result<(), SaveError> {
    // A hard link fails if the target exists, so it never replaces a save.
    if link(temporary, target).is_ok() {
        let _ = std::fs::remove_file(temporary);
        return Ok(());
    }
    if target.exists() {
        return Err(SaveError::NameTaken(name.to_owned()));
    }
    // Some file systems (FAT on a USB stick, say) have no hard links.
    std::fs::rename(temporary, target).map_err(|e| SaveError::Storage(e.to_string()))
}

impl OpenSave for SqliteSave {
    fn meta(&self) -> &SaveMeta {
        &self.meta
    }

    fn team(&mut self, id: &ContentId) -> Result<Option<Team>, SaveError> {
        teams::table
            .filter(teams::id.eq(id.as_str()))
            .select(TeamRow::as_select())
            .first(&mut self.connection)
            .optional()
            .map_err(storage)?
            .map(TeamRow::into_domain)
            .transpose()
    }

    fn players_of(&mut self, team: &ContentId) -> Result<Vec<Player>, SaveError> {
        let query = players::table
            .filter(players::team_id.eq(team.as_str()))
            .select(PlayerRow::as_select());
        let rows: Vec<PlayerRow> =
            load_ordered(&mut self.connection, query, players::position.asc()).map_err(storage)?;
        rows.into_iter().map(PlayerRow::into_domain).collect()
    }

    fn player(&mut self, id: &ContentId) -> Result<Option<Player>, SaveError> {
        players::table
            .filter(players::id.eq(id.as_str()))
            .select(PlayerRow::as_select())
            .first(&mut self.connection)
            .optional()
            .map_err(storage)?
            .map(PlayerRow::into_domain)
            .transpose()
    }

    fn roles(&mut self) -> Result<RoleSet, SaveError> {
        let rows: Vec<RoleRow> = load_ordered(
            &mut self.connection,
            roles::table.select(RoleRow::as_select()),
            roles::position.asc(),
        )
        .map_err(storage)?;
        let definitions = rows
            .into_iter()
            .map(|row| {
                let (id, weights) = row.into_parts()?;
                Ok((
                    id,
                    OverallAttribute::ALL
                        .into_iter()
                        .zip(weights)
                        .collect::<BTreeMap<_, _>>(),
                ))
            })
            .collect::<Result<Vec<_>, SaveError>>()?;
        RoleSet::new(definitions).map_err(|_| rows::corrupt("role set", "weights"))
    }

    fn regions(&mut self) -> Result<Vec<Region>, SaveError> {
        let region_rows: Vec<RegionRow> = load_ordered(
            &mut self.connection,
            regions::table.select(RegionRow::as_select()),
            regions::position.asc(),
        )
        .map_err(storage)?;
        let mix_rows: Vec<RegionNationalityRow> = load_ordered(
            &mut self.connection,
            region_nationalities::table.select(RegionNationalityRow::as_select()),
            (
                region_nationalities::region_id.asc(),
                region_nationalities::nationality_id.asc(),
            ),
        )
        .map_err(storage)?;
        region_rows
            .into_iter()
            .map(
                |RegionRow {
                     id,
                     position: _,
                     level,
                 }| {
                    let nationalities = mix_rows
                        .iter()
                        .filter(|mix| mix.region_id == id)
                        .map(|mix| {
                            let weight = u32::try_from(mix.weight)
                                .map_err(|_| rows::corrupt("weight", &mix.weight.to_string()))?;
                            Ok((rows::id(&mix.nationality_id)?, weight))
                        })
                        .collect::<Result<_, SaveError>>()?;
                    Ok(Region {
                        id: rows::id(&id)?,
                        level,
                        nationalities,
                    })
                },
            )
            .collect()
    }

    fn nationalities(&mut self) -> Result<Vec<Nationality>, SaveError> {
        let rows: Vec<NationalityRow> = load_ordered(
            &mut self.connection,
            nationalities::table.select(NationalityRow::as_select()),
            nationalities::position.asc(),
        )
        .map_err(storage)?;
        rows.into_iter().map(NationalityRow::into_domain).collect()
    }

    fn competitions(&mut self) -> Result<Vec<Competition>, SaveError> {
        let competition_rows: Vec<CompetitionRow> = load_ordered(
            &mut self.connection,
            competitions::table.select(CompetitionRow::as_select()),
            competitions::position.asc(),
        )
        .map_err(storage)?;
        let entrants: Vec<CompetitionTeamRow> = load_ordered(
            &mut self.connection,
            competition_teams::table.select(CompetitionTeamRow::as_select()),
            (
                competition_teams::competition_id.asc(),
                competition_teams::position.asc(),
            ),
        )
        .map_err(storage)?;
        competition_rows
            .into_iter()
            .map(
                |CompetitionRow {
                     id,
                     position: _,
                     region_id,
                 }| {
                    let teams = entrants
                        .iter()
                        .filter(|entrant| entrant.competition_id == id)
                        .map(|entrant| rows::id(&entrant.team_id))
                        .collect::<Result<_, _>>()?;
                    Ok(Competition {
                        id: rows::id(&id)?,
                        region: rows::id(&region_id)?,
                        teams,
                    })
                },
            )
            .collect()
    }

    fn texts(&mut self) -> Result<Vec<ContentText>, SaveError> {
        let rows: Vec<TextRow> = load_ordered(
            &mut self.connection,
            content_texts::table.select(TextRow::as_select()),
            (content_texts::locale.asc(), content_texts::file.asc()),
        )
        .map_err(storage)?;
        Ok(rows
            .into_iter()
            .map(
                |TextRow {
                     locale,
                     file,
                     source,
                 }| ContentText {
                    locale,
                    file,
                    source,
                },
            )
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn folder(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("esm-db-unit-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn creation_never_replaces_an_existing_save() {
        // spec: SAVE-018
        // Given a finished temporary save, and a save that appeared under
        // its final name in the meantime
        let dir = folder("commit");
        let (temporary, target) = (dir.join("tigers.1-0.tmp"), dir.join("tigers.esmsave"));
        std::fs::write(&temporary, b"new").unwrap();
        std::fs::write(&target, b"existing").unwrap();
        // When
        let result = commit(&temporary, &target, "Tigers");
        // Then
        assert_eq!(result, Err(SaveError::NameTaken("Tigers".to_owned())));
        assert_eq!(std::fs::read(&target).unwrap(), b"existing");
    }

    #[test]
    fn without_hard_links_commit_renames_but_never_replaces() {
        let dir = folder("commit-fat");
        let no_links = |_: &Path, _: &Path| Err(std::io::Error::other("no hard links here"));
        let (temporary, target) = (dir.join("a.1-0.tmp"), dir.join("a.esmsave"));
        std::fs::write(&temporary, b"new").unwrap();
        assert_eq!(commit_with(no_links, &temporary, &target, "A"), Ok(()));
        assert_eq!(std::fs::read(&target).unwrap(), b"new");
        std::fs::write(&temporary, b"newer").unwrap();
        assert_eq!(
            commit_with(no_links, &temporary, &target, "A"),
            Err(SaveError::NameTaken("A".to_owned()))
        );
        assert_eq!(std::fs::read(&target).unwrap(), b"new");
        let missing = dir.join("missing.tmp");
        assert!(matches!(
            commit_with(no_links, &missing, &dir.join("b.esmsave"), "B"),
            Err(SaveError::Storage(_))
        ));
    }

    #[test]
    fn commit_moves_the_temporary_file_into_place() {
        let dir = folder("commit-ok");
        let (temporary, target) = (dir.join("a.1-0.tmp"), dir.join("a.esmsave"));
        std::fs::write(&temporary, b"new").unwrap();
        assert_eq!(commit(&temporary, &target, "A"), Ok(()));
        assert_eq!(std::fs::read(&target).unwrap(), b"new");
        assert!(!temporary.exists());
    }

    #[test]
    fn a_temporary_file_in_use_is_kept() {
        let dir = folder("in-use-tmp");
        let path = dir.join("busy.1-0.tmp");
        let mut holder = connect(&path).unwrap();
        diesel::sql_query("BEGIN EXCLUSIVE")
            .execute(&mut holder)
            .unwrap();
        diesel::sql_query("CREATE TABLE t (x INTEGER)")
            .execute(&mut holder)
            .unwrap();
        drop(SqliteSaveStore::new(dir.clone()));
        assert!(path.exists());
        drop(holder);
        drop(SqliteSaveStore::new(dir));
        assert!(!path.exists());
    }

    #[test]
    fn non_utf8_paths_are_refused() {
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;
            let path = Path::new(std::ffi::OsStr::from_bytes(b"/tmp/\xff.esmsave"));
            assert!(matches!(connect(path), Err(SaveError::Storage(_))));
        }
    }
}
