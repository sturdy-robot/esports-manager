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
