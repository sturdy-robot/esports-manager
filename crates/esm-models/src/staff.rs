use serde::{Deserialize, Serialize};

use crate::player::BoundedAttribute;

// ---------------------------------------------------------------------------
// StaffRole enum
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StaffRole {
    AssistantCoach,
    DraftAnalyst,
    PositionalCoach,
    SportsPsychologist,
    Scout,
    FinancialOfficer,
}

// ---------------------------------------------------------------------------
// Staff Entity
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Staff {
    nickname: String,
    first_name: String,
    last_name: String,
    role: StaffRole,
    skill: BoundedAttribute,
}

impl Staff {
    pub fn new(
        nickname: String,
        first_name: String,
        last_name: String,
        role: StaffRole,
        skill: u8,
    ) -> Self {
        Self {
            nickname,
            first_name,
            last_name,
            role,
            skill: BoundedAttribute::new(skill),
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

    pub fn role(&self) -> StaffRole {
        self.role
    }

    pub fn skill(&self) -> BoundedAttribute {
        self.skill
    }

    pub fn skill_mut(&mut self) -> &mut BoundedAttribute {
        &mut self.skill
    }
}
