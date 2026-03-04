use serde::{Deserialize, Serialize};

use crate::player::BoundedAttribute;
use super::player::{MobaPlayer, MobaRole};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobaTeam {
    name: String,
    tag: String,
    roster: Vec<MobaPlayer>,
    synergy: BoundedAttribute,
    reputation: BoundedAttribute,
}

impl MobaTeam {
    pub fn new(name: String, tag: String, roster: Vec<MobaPlayer>) -> Self {
        Self {
            name,
            tag,
            roster,
            synergy: BoundedAttribute::new(0),
            reputation: BoundedAttribute::new(50),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn roster(&self) -> &[MobaPlayer] {
        &self.roster
    }

    pub fn roster_mut(&mut self) -> &mut [MobaPlayer] {
        &mut self.roster
    }

    pub fn synergy(&self) -> BoundedAttribute {
        self.synergy
    }

    pub fn synergy_mut(&mut self) -> &mut BoundedAttribute {
        &mut self.synergy
    }

    pub fn reputation(&self) -> BoundedAttribute {
        self.reputation
    }

    pub fn player_by_primary_role(&self, role: MobaRole) -> Option<&MobaPlayer> {
        self.roster.iter().find(|p| p.roles().primary() == role)
    }

    pub fn player_by_primary_role_mut(&mut self, role: MobaRole) -> Option<&mut MobaPlayer> {
        self.roster.iter_mut().find(|p| p.roles().primary() == role)
    }

    /// Compute an aggregate team power rating from all players' key attributes.
    pub fn compute_power(&self) -> u32 {
        let mut total: u32 = 0;
        for player in &self.roster {
            let a = player.attributes();
            total += a.endurance.value() as u32;
            total += a.reaction_time.value() as u32;
            total += a.decision_making.value() as u32;
            total += a.clutch.value() as u32;
            total += a.mechanics.value() as u32;
            total += a.teamfighting.value() as u32;
        }
        total
    }
}
