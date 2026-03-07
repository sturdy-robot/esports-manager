use std::fs;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::database::Database;
use crate::game_session::GameSession;
use esm_core::game_state::GameState;

// ---------------------------------------------------------------------------
// SaveEntry — one entry in the saves.json index
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveEntry {
    pub name: String,
    pub checksum: String,
}

// ---------------------------------------------------------------------------
// FNV-1a 64-bit hash (no external crate needed)
// ---------------------------------------------------------------------------

fn fnv1a_64(data: &[u8]) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x00000100000001B3;
    let mut hash = FNV_OFFSET;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

fn file_checksum(path: &Path) -> io::Result<String> {
    let bytes = fs::read(path)?;
    Ok(format!("{:016x}", fnv1a_64(&bytes)))
}

// ---------------------------------------------------------------------------
// SaveManager
// ---------------------------------------------------------------------------

const INDEX_FILE: &str = "saves.json";

pub struct SaveManager;

impl SaveManager {
    /// Create (or overwrite) a save file and update the index.
    pub fn create_save(dir: &Path, name: &str, state: &GameState) -> io::Result<()> {
        Self::create_save_full(dir, name, state, "", "[]")
    }

    /// Create (or overwrite) a save file with tournament and moba teams data.
    pub fn create_save_full(
        dir: &Path,
        name: &str,
        state: &GameState,
        tournament_json: &str,
        moba_teams_json: &str,
    ) -> io::Result<()> {
        Self::create_save_all(
            dir,
            name,
            state,
            tournament_json,
            moba_teams_json,
            "[]",
            "{\"scrims\":[],\"next_id\":1}",
        )
    }

    /// Create (or overwrite) a save file with all ancillary data.
    pub fn create_save_all(
        dir: &Path,
        name: &str,
        state: &GameState,
        tournament_json: &str,
        moba_teams_json: &str,
        schedules_json: &str,
        scrims_json: &str,
    ) -> io::Result<()> {
        fs::create_dir_all(dir)?;

        let db_path = dir.join(format!("{name}.db"));

        // Remove existing file so we get a fresh DB
        if db_path.exists() {
            fs::remove_file(&db_path)?;
        }

        // Open a new SQLite DB at the target path, run migrations, save state
        let db_path_str = db_path
            .to_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Non-UTF-8 path"))?;
        let db = Database::open(db_path_str)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to open DB: {e}")))?;
        GameSession::save_all(
            db.conn(),
            state,
            tournament_json,
            moba_teams_json,
            schedules_json,
            scrims_json,
        )
        .map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to save session: {e}"))
        })?;

        // Force WAL checkpoint so all data is in the main file
        let _ = db.conn().execute_batch("PRAGMA wal_checkpoint(TRUNCATE);");
        drop(db);

        // Compute checksum of the written file
        let checksum = file_checksum(&db_path)?;

        // Update index
        let mut entries = Self::read_index(dir)?;
        entries.retain(|e| e.name != name);
        entries.push(SaveEntry {
            name: name.to_string(),
            checksum,
        });
        Self::write_index(dir, &entries)?;

        Ok(())
    }

    /// List all saves from the index file.
    pub fn list_saves(dir: &Path) -> io::Result<Vec<SaveEntry>> {
        Self::read_index(dir)
    }

    /// Load a GameState from a save file, validating the checksum.
    pub fn load_save(dir: &Path, name: &str) -> io::Result<GameState> {
        Self::load_save_full(dir, name).map(|(gs, _, _)| gs)
    }

    /// Load GameState + tournament JSON + moba teams JSON from a save file.
    pub fn load_save_full(dir: &Path, name: &str) -> io::Result<(GameState, String, String)> {
        Self::load_save_all(dir, name).map(|(gs, t, m, _, _)| (gs, t, m))
    }

    /// Load GameState + all ancillary JSON from a save file.
    pub fn load_save_all(
        dir: &Path,
        name: &str,
    ) -> io::Result<(GameState, String, String, String, String)> {
        let entries = Self::read_index(dir)?;
        let entry = entries.iter().find(|e| e.name == name).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("Save '{name}' not found in index"),
            )
        })?;

        let db_path = dir.join(format!("{name}.db"));

        // Validate checksum
        let actual_checksum = file_checksum(&db_path)?;
        if actual_checksum != entry.checksum {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Checksum mismatch for '{name}': expected {}, got {actual_checksum}",
                    entry.checksum
                ),
            ));
        }

        let db_path_str = db_path
            .to_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Non-UTF-8 path"))?;
        let db = Database::open(db_path_str)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to open DB: {e}")))?;

        let (state, tournament_json, moba_teams_json, schedules_json, scrims_json) =
            GameSession::load_all(db.conn())
                .map_err(|e| {
                    io::Error::new(io::ErrorKind::Other, format!("Failed to load session: {e}"))
                })?
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Save '{name}' DB has no session data"),
                    )
                })?;

        Ok((
            state,
            tournament_json,
            moba_teams_json,
            schedules_json,
            scrims_json,
        ))
    }

    /// Delete a save file and remove it from the index.
    pub fn delete_save(dir: &Path, name: &str) -> io::Result<()> {
        let db_path = dir.join(format!("{name}.db"));
        if db_path.exists() {
            fs::remove_file(&db_path)?;
        }
        // Also remove any WAL/SHM files
        let wal_path = dir.join(format!("{name}.db-wal"));
        let shm_path = dir.join(format!("{name}.db-shm"));
        if wal_path.exists() {
            fs::remove_file(&wal_path)?;
        }
        if shm_path.exists() {
            fs::remove_file(&shm_path)?;
        }

        let mut entries = Self::read_index(dir)?;
        entries.retain(|e| e.name != name);
        Self::write_index(dir, &entries)?;

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------------

    fn read_index(dir: &Path) -> io::Result<Vec<SaveEntry>> {
        let index_path = dir.join(INDEX_FILE);
        if !index_path.exists() {
            return Ok(Vec::new());
        }
        let contents = fs::read_to_string(&index_path)?;
        let entries: Vec<SaveEntry> = serde_json::from_str(&contents).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid saves.json: {e}"),
            )
        })?;
        Ok(entries)
    }

    fn write_index(dir: &Path, entries: &[SaveEntry]) -> io::Result<()> {
        let index_path = dir.join(INDEX_FILE);
        let json = serde_json::to_string_pretty(entries).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("JSON serialize error: {e}"))
        })?;
        fs::write(&index_path, json)?;
        Ok(())
    }
}
