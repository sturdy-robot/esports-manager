use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChampionClass {
    Tank,
    Fighter,
    Assassin,
    Mage,
    Marksman,
    Support,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChampionScaling {
    Early,
    Mid,
    Late,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChampionTag {
    Knockup,
    Engage,
    Poke,
    Splitpush,
    Waveclear,
    Peel,
    Burst,
    Sustain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MasteryLevel {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
    Master,
    Challenger,
}

impl MasteryLevel {
    pub fn multiplier(self) -> f64 {
        match self {
            MasteryLevel::Bronze => 0.5,
            MasteryLevel::Silver => 0.7,
            MasteryLevel::Gold => 0.85,
            MasteryLevel::Platinum => 1.0,
            MasteryLevel::Diamond => 1.15,
            MasteryLevel::Master => 1.3,
            MasteryLevel::Challenger => 1.5,
        }
    }
}

// ---------------------------------------------------------------------------
// Champion Entity
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Champion {
    name: String,
    class: ChampionClass,
    scaling: ChampionScaling,
    tags: Vec<ChampionTag>,
}

impl Champion {
    pub fn new(
        name: String,
        class: ChampionClass,
        scaling: ChampionScaling,
        tags: Vec<ChampionTag>,
    ) -> Self {
        Self {
            name,
            class,
            scaling,
            tags,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn class(&self) -> ChampionClass {
        self.class
    }

    pub fn scaling(&self) -> ChampionScaling {
        self.scaling
    }

    pub fn tags(&self) -> &[ChampionTag] {
        &self.tags
    }

    pub fn has_tag(&self, tag: &ChampionTag) -> bool {
        self.tags.contains(tag)
    }
}

// ---------------------------------------------------------------------------
// Champion Pool
// ---------------------------------------------------------------------------

pub type ChampionId = i64;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChampionPool {
    masteries: HashMap<ChampionId, MasteryLevel>,
}

impl ChampionPool {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_mastery(&self, champion_id: ChampionId) -> MasteryLevel {
        *self.masteries.get(&champion_id).unwrap_or(&MasteryLevel::Bronze)
    }

    pub fn set_mastery(&mut self, champion_id: ChampionId, level: MasteryLevel) {
        self.masteries.insert(champion_id, level);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ChampionId, &MasteryLevel)> {
        self.masteries.iter()
    }
}
