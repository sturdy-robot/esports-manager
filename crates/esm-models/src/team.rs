use serde::{Deserialize, Serialize};

use crate::player::{BoundedAttribute, Player, Role};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    name: String,
    tag: String,
    roster: Vec<Player>,
    synergy: BoundedAttribute,
    reputation: BoundedAttribute,
}

impl Team {
    pub fn new(name: String, tag: String, roster: Vec<Player>) -> Self {
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

    pub fn roster(&self) -> &[Player] {
        &self.roster
    }

    pub fn roster_mut(&mut self) -> &mut [Player] {
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

    pub fn player_by_role(&self, role: Role) -> Option<&Player> {
        self.roster.iter().find(|p| p.role() == role)
    }

    pub fn player_by_role_mut(&mut self, role: Role) -> Option<&mut Player> {
        self.roster.iter_mut().find(|p| p.role() == role)
    }

    pub fn apply_match_result(&mut self, is_win: bool) {
        if is_win {
            self.reputation.increase(1);
        } else {
            self.reputation.decrease(1);
        }
        for player in &mut self.roster {
            player.state_mut().apply_match_result(is_win);
        }
    }
}
