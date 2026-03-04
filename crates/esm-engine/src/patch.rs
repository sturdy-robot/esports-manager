use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// MetaTier
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetaTier {
    S,
    A,
    B,
    C,
    D,
}

impl MetaTier {
    pub fn multiplier(self) -> f64 {
        match self {
            MetaTier::S => 1.5,
            MetaTier::A => 1.2,
            MetaTier::B => 1.0,
            MetaTier::C => 0.8,
            MetaTier::D => 0.6,
        }
    }
}

// ---------------------------------------------------------------------------
// PatchModifier
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchModifier {
    champion_name: String,
    tier: MetaTier,
}

impl PatchModifier {
    pub fn new(champion_name: String, tier: MetaTier) -> Self {
        Self {
            champion_name,
            tier,
        }
    }

    pub fn champion_name(&self) -> &str {
        &self.champion_name
    }

    pub fn tier(&self) -> MetaTier {
        self.tier
    }
}

// ---------------------------------------------------------------------------
// Patch
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patch {
    version: String,
    modifiers: Vec<PatchModifier>,
}

impl Patch {
    pub fn new(version: String, modifiers: Vec<PatchModifier>) -> Self {
        Self { version, modifiers }
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn modifiers(&self) -> &[PatchModifier] {
        &self.modifiers
    }

    pub fn tier_for(&self, champion_name: &str) -> Option<MetaTier> {
        self.modifiers
            .iter()
            .find(|m| m.champion_name() == champion_name)
            .map(|m| m.tier())
    }

    /// Return the tier for a champion, defaulting to B-tier if unlisted.
    pub fn tier_for_or_default(&self, champion_name: &str) -> MetaTier {
        self.tier_for(champion_name).unwrap_or(MetaTier::B)
    }
}

// ---------------------------------------------------------------------------
// PatchCycle
// ---------------------------------------------------------------------------

/// Manages the scheduled progression of patches throughout a season.
///
/// Per DESIGN.md §1, patches apply every 2-4 in-game weeks, shifting the
/// meta and forcing adaptation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchCycle {
    patches: Vec<Patch>,
    current_index: usize,
    interval_days: u32,
    next_change_day: u32,
}

impl PatchCycle {
    pub fn new(patches: Vec<Patch>, interval_days: u32) -> Self {
        Self {
            patches,
            current_index: 0,
            interval_days,
            next_change_day: interval_days,
        }
    }

    pub fn current_patch(&self) -> &Patch {
        &self.patches[self.current_index]
    }

    pub fn current_index(&self) -> usize {
        self.current_index
    }

    pub fn interval_days(&self) -> u32 {
        self.interval_days
    }

    /// Check if the patch should advance given the current day (days_elapsed).
    /// Returns `true` if a new patch was activated.
    pub fn check_advance(&mut self, days_elapsed: u32) -> bool {
        if self.current_index + 1 >= self.patches.len() {
            return false;
        }
        if days_elapsed >= self.next_change_day {
            self.current_index += 1;
            self.next_change_day += self.interval_days;
            return true;
        }
        false
    }
}
