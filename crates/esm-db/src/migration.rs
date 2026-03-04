use rusqlite::{Connection, Result as SqlResult};

/// A single database migration with a version number and SQL to execute.
#[derive(Debug, Clone)]
pub struct Migration {
    version: u32,
    description: String,
    sql: String,
}

impl Migration {
    pub fn new(version: u32, description: &str, sql: &str) -> Self {
        Self {
            version,
            description: description.to_string(),
            sql: sql.to_string(),
        }
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn sql(&self) -> &str {
        &self.sql
    }
}

/// Manages database schema migrations.
///
/// Tracks applied migrations in a `_migrations` table and applies new ones
/// in version order. Designed for forward-only, append-only migration scripts
/// that prioritize backwards compatibility with previous saves.
pub struct MigrationRunner;

impl MigrationRunner {
    /// Create the `_migrations` tracking table if it doesn't exist.
    pub fn initialize(conn: &Connection) -> SqlResult<()> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS _migrations (
                version     INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                applied_at  TEXT NOT NULL DEFAULT (datetime('now'))
            );",
        )?;
        Ok(())
    }

    /// Return the highest migration version that has been applied, or 0 if none.
    pub fn current_version(conn: &Connection) -> SqlResult<u32> {
        let version: u32 = conn
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM _migrations",
                [],
                |row| row.get(0),
            )?;
        Ok(version)
    }

    /// Apply all migrations whose version is greater than the current DB version.
    ///
    /// Returns the number of newly applied migrations.
    /// Migrations are applied inside a transaction — if any migration fails,
    /// all changes in this batch are rolled back.
    pub fn run(conn: &Connection, migrations: &[Migration]) -> SqlResult<usize> {
        let current = Self::current_version(conn)?;

        let pending: Vec<&Migration> = migrations
            .iter()
            .filter(|m| m.version() > current)
            .collect();

        if pending.is_empty() {
            return Ok(0);
        }

        let tx = conn.unchecked_transaction()?;

        for migration in &pending {
            tx.execute_batch(migration.sql())?;
            tx.execute(
                "INSERT INTO _migrations (version, description) VALUES (?1, ?2)",
                rusqlite::params![migration.version(), migration.description()],
            )?;
        }

        tx.commit()?;
        Ok(pending.len())
    }
}
