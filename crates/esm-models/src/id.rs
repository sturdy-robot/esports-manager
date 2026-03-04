use serde::{Deserialize, Serialize};
use std::fmt;

/// A unique identifier for any game entity (Player, Team, Champion, Staff, etc).
///
/// Uses a simple `u64` counter — deterministic and reproducible given the same
/// game setup sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(u64);

impl EntityId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Deterministic sequential ID generator.
///
/// The generator is stored in the `GameState` and serialized with it,
/// ensuring that new entities created after loading a save file continue
/// the sequence without collisions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdGenerator {
    counter: u64,
}

impl IdGenerator {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    /// Create a generator starting from an offset (e.g., after loading a save).
    pub fn from_offset(offset: u64) -> Self {
        Self { counter: offset }
    }

    /// Issue the next unique ID.
    pub fn next(&mut self) -> EntityId {
        self.counter += 1;
        EntityId(self.counter)
    }

    /// Return the last issued counter value (0 if none issued yet).
    pub fn current(&self) -> u64 {
        self.counter
    }
}

impl Default for IdGenerator {
    fn default() -> Self {
        Self::new()
    }
}
