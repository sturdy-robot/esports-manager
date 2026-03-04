use rusqlite::Connection;

use crate::migration::migrations;

/// Wrapper around a rusqlite `Connection` that auto-applies migrations on open.
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Open an in-memory database and apply all migrations.
    pub fn open_in_memory() -> Result<Self, Box<dyn std::error::Error>> {
        let mut conn = Connection::open_in_memory()?;
        migrations::runner().run(&mut conn)?;
        Ok(Self { conn })
    }

    /// Open (or create) a file-backed database and apply all migrations.
    pub fn open(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut conn = Connection::open(path)?;
        migrations::runner().run(&mut conn)?;
        Ok(Self { conn })
    }

    /// Access the underlying rusqlite connection.
    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    /// Check if a table exists in the database.
    pub fn table_exists(&self, name: &str) -> Result<bool, rusqlite::Error> {
        let exists: bool = self.conn.query_row(
            "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name=?1",
            [name],
            |row| row.get(0),
        )?;
        Ok(exists)
    }

    /// Return the current schema version (highest applied migration).
    pub fn schema_version(&self) -> Result<i64, rusqlite::Error> {
        let version: i64 = self.conn.query_row(
            "SELECT COALESCE(MAX(version), 0) FROM refinery_schema_history",
            [],
            |row| row.get(0),
        )?;
        Ok(version)
    }
}
