use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::player::BoundedAttribute;

// ---------------------------------------------------------------------------
// Manager Archetype
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManagerArchetype {
    TacticalGenius,
    PlayerDeveloper,
    Motivator,
    Analyst,
    Balanced,
}

impl ManagerArchetype {
    pub fn as_str(&self) -> &'static str {
        match self {
            ManagerArchetype::TacticalGenius => "TacticalGenius",
            ManagerArchetype::PlayerDeveloper => "PlayerDeveloper",
            ManagerArchetype::Motivator => "Motivator",
            ManagerArchetype::Analyst => "Analyst",
            ManagerArchetype::Balanced => "Balanced",
        }
    }
}

impl fmt::Display for ManagerArchetype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ManagerArchetype {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TacticalGenius" => Ok(ManagerArchetype::TacticalGenius),
            "PlayerDeveloper" => Ok(ManagerArchetype::PlayerDeveloper),
            "Motivator" => Ok(ManagerArchetype::Motivator),
            "Analyst" => Ok(ManagerArchetype::Analyst),
            "Balanced" => Ok(ManagerArchetype::Balanced),
            _ => Err(format!("Unknown ManagerArchetype: '{s}'")),
        }
    }
}

// ---------------------------------------------------------------------------
// Manager Entity
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manager {
    nickname: String,
    first_name: String,
    last_name: String,
    nationality: String,
    archetype: ManagerArchetype,
    reputation: BoundedAttribute,
}

impl Manager {
    pub fn new(
        nickname: String,
        first_name: String,
        last_name: String,
        nationality: String,
        archetype: ManagerArchetype,
    ) -> Self {
        Self {
            nickname,
            first_name,
            last_name,
            nationality,
            archetype,
            reputation: BoundedAttribute::new(50),
        }
    }

    pub fn nickname(&self) -> &str {
        &self.nickname
    }

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    pub fn nationality(&self) -> &str {
        &self.nationality
    }

    pub fn archetype(&self) -> ManagerArchetype {
        self.archetype
    }

    pub fn reputation(&self) -> BoundedAttribute {
        self.reputation
    }

    pub fn reputation_mut(&mut self) -> &mut BoundedAttribute {
        &mut self.reputation
    }
}
