//! The save port (saves spec): what the application needs from storage. The
//! `db` crate implements it with one SQLite file per save.

use domain::date::GameDate;
use domain::id::ContentId;
use domain::rng::WorldSeed;
use domain::world::{Competition, Nationality, Region, Team};
use esports::Genre;
use esports::moba::roles::RoleSet;
use esports::moba::world::{Player, World};

/// Real time, in whole seconds since the Unix epoch. Only adapters read the
/// clock; the domain never sees this (ADR 0003).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(pub i64);

/// A package a save was built from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageRecord {
    pub id: String,
    pub version: String,
    pub content_hash: [u8; 32],
}

/// A `.ftl` file of a package's own text, copied into the save. The UI
/// loads it into its Fluent bundles.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ContentText {
    pub locale: String,
    /// Its path inside the package, such as `locales/en/teams.ftl`.
    pub file: String,
    pub source: String,
}

/// What every save records about itself (`save_meta`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveMeta {
    /// The name the user typed.
    pub name: String,
    pub manager_name: String,
    pub team: ContentId,
    pub genre: Genre,
    pub seed: WorldSeed,
    /// The game version that created the save.
    pub game_version: String,
    pub game_date: GameDate,
    pub created_at: Timestamp,
    pub last_played_at: Timestamp,
    pub packages: Vec<PackageRecord>,
}

/// Everything a new save holds.
#[derive(Debug, Clone, Copy)]
pub struct NewSave<'a> {
    pub meta: &'a SaveMeta,
    pub world: &'a World,
    pub texts: &'a [ContentText],
}

/// Why a save file can't be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Damage {
    NotASave,
    /// Made by a newer version of the game.
    NewerVersion,
    /// Another process has it open.
    InUse,
}

/// One file in the saves folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveListing {
    /// The file name, such as `ana-s-tigers.esmsave`. It identifies the save.
    pub file: String,
    pub save: Result<SaveMeta, Damage>,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SaveError {
    #[error("a save named `{0}` already exists")]
    NameTaken(String),
    #[error("there is no save `{0}`")]
    NotFound(String),
    #[error("the save can't be used: {0:?}")]
    Damaged(Damage),
    #[error("storage failed: {0}")]
    Storage(String),
}

/// Where saves live.
pub trait SaveStore {
    type Save: OpenSave;

    /// Writes a new save and returns its file name.
    ///
    /// # Errors
    ///
    /// When the name is taken (ignoring case) or storage fails. A failed
    /// creation leaves nothing behind.
    fn create(&self, save: NewSave<'_>) -> Result<String, SaveError>;

    /// Every save in the folder, newest first; damaged files included.
    ///
    /// # Errors
    ///
    /// When the folder itself can't be read.
    fn list(&self) -> Result<Vec<SaveListing>, SaveError>;

    /// Opens a save for this process alone and records `now` as its last
    /// played time.
    ///
    /// # Errors
    ///
    /// When it doesn't exist, is damaged, newer than the game, or open
    /// elsewhere.
    fn open(&self, file: &str, now: Timestamp) -> Result<Self::Save, SaveError>;
}

/// A save this process holds open.
pub trait OpenSave {
    fn meta(&self) -> &SaveMeta;

    /// # Errors
    ///
    /// When storage fails.
    fn team(&mut self, id: &ContentId) -> Result<Option<Team>, SaveError>;

    /// The players of a team, in the order the world was generated.
    ///
    /// # Errors
    ///
    /// When storage fails.
    fn players_of(&mut self, team: &ContentId) -> Result<Vec<Player>, SaveError>;

    /// # Errors
    ///
    /// When storage fails.
    fn player(&mut self, id: &ContentId) -> Result<Option<Player>, SaveError>;

    /// # Errors
    ///
    /// When storage fails.
    fn roles(&mut self) -> Result<RoleSet, SaveError>;

    /// # Errors
    ///
    /// When storage fails.
    fn regions(&mut self) -> Result<Vec<Region>, SaveError>;

    /// # Errors
    ///
    /// When storage fails.
    fn nationalities(&mut self) -> Result<Vec<Nationality>, SaveError>;

    /// # Errors
    ///
    /// When storage fails.
    fn competitions(&mut self) -> Result<Vec<Competition>, SaveError>;

    /// # Errors
    ///
    /// When storage fails.
    fn texts(&mut self) -> Result<Vec<ContentText>, SaveError>;
}
