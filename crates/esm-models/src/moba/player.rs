use serde::{Deserialize, Serialize};

use crate::player::{BoundedAttribute, PlayerState};
use crate::champion::ChampionPool;

// ---------------------------------------------------------------------------
// MobaRole
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MobaRole {
    Top,
    Jungle,
    Mid,
    Bot,
    Support,
}

// ---------------------------------------------------------------------------
// RoleAssignment — primary + secondary roles with skill modifiers
// ---------------------------------------------------------------------------

const SECONDARY_ROLE_MODIFIER: f64 = 0.80;
const UNASSIGNED_ROLE_MODIFIER: f64 = 0.55;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleAssignment {
    primary: MobaRole,
    secondary: Vec<MobaRole>,
}

impl RoleAssignment {
    pub fn new(primary: MobaRole, secondary: Vec<MobaRole>) -> Self {
        Self { primary, secondary }
    }

    pub fn primary(&self) -> MobaRole {
        self.primary
    }

    pub fn secondary(&self) -> &[MobaRole] {
        &self.secondary
    }

    pub fn can_play(&self, role: MobaRole) -> bool {
        self.primary == role || self.secondary.contains(&role)
    }

    /// Return the skill modifier when playing a given role.
    /// Primary = 1.0, Secondary = 0.80, Unassigned = 0.55.
    pub fn skill_modifier(&self, role: MobaRole) -> f64 {
        if role == self.primary {
            1.0
        } else if self.secondary.contains(&role) {
            SECONDARY_ROLE_MODIFIER
        } else {
            UNASSIGNED_ROLE_MODIFIER
        }
    }
}

// ---------------------------------------------------------------------------
// MobaPlayerAttributes — MOBA-specific attributes (flat, no sub-groups)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobaPlayerAttributes {
    pub endurance: BoundedAttribute,
    pub reaction_time: BoundedAttribute,
    pub decision_making: BoundedAttribute,
    pub clutch: BoundedAttribute,
    pub discipline: BoundedAttribute,
    pub tilt_resistance: BoundedAttribute,
    pub mechanics: BoundedAttribute,
    pub vision_control: BoundedAttribute,
    pub teamfighting: BoundedAttribute,
}

// ---------------------------------------------------------------------------
// MobaPlayer Entity
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobaPlayer {
    nickname: String,
    first_name: String,
    last_name: String,
    roles: RoleAssignment,
    attributes: MobaPlayerAttributes,
    state: PlayerState,
    champion_pool: ChampionPool,
}

impl MobaPlayer {
    pub fn new(
        nickname: String,
        first_name: String,
        last_name: String,
        roles: RoleAssignment,
        attributes: MobaPlayerAttributes,
    ) -> Self {
        Self {
            nickname,
            first_name,
            last_name,
            roles,
            attributes,
            state: PlayerState::default(),
            champion_pool: ChampionPool::default(),
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

    pub fn roles(&self) -> &RoleAssignment {
        &self.roles
    }

    pub fn attributes(&self) -> &MobaPlayerAttributes {
        &self.attributes
    }

    pub fn attributes_mut(&mut self) -> &mut MobaPlayerAttributes {
        &mut self.attributes
    }

    pub fn state(&self) -> &PlayerState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut PlayerState {
        &mut self.state
    }

    pub fn champion_pool(&self) -> &ChampionPool {
        &self.champion_pool
    }

    pub fn champion_pool_mut(&mut self) -> &mut ChampionPool {
        &mut self.champion_pool
    }

    /// Compute the effective value of an attribute when playing a specific role.
    /// Applies the role skill modifier to the base attribute value.
    pub fn effective_attribute(&self, attr: BoundedAttribute, playing_role: MobaRole) -> u8 {
        let modifier = self.roles.skill_modifier(playing_role);
        (attr.value() as f64 * modifier).round() as u8
    }
}
