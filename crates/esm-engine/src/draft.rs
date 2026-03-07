use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::match_sim::TeamSide;

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DraftPhase {
    Ban,
    Pick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DraftFormat {
    ThreeBan,
    FiveBan,
    Fearless,
}

impl DraftFormat {
    /// Return the ordered sequence of (phase, team) slots for this format.
    ///
    /// 3-ban: B-B-B R-R-R then standard snake pick (10 picks) = 16 total
    /// 5-ban: B-B-B R-R-R B-B R-R then standard snake pick (10 picks) = 20 total
    /// Fearless: same as 5-ban for now (future: unique per series game)
    pub fn sequence(&self) -> Vec<DraftSlot> {
        match self {
            DraftFormat::ThreeBan => three_ban_sequence(),
            DraftFormat::FiveBan | DraftFormat::Fearless => five_ban_sequence(),
        }
    }
}

fn three_ban_sequence() -> Vec<DraftSlot> {
    let mut seq = Vec::with_capacity(16);
    // 3 blue bans, 3 red bans
    for _ in 0..3 {
        seq.push(DraftSlot {
            phase: DraftPhase::Ban,
            team: TeamSide::Blue,
        });
    }
    for _ in 0..3 {
        seq.push(DraftSlot {
            phase: DraftPhase::Ban,
            team: TeamSide::Red,
        });
    }
    // Snake pick: B R R B B R R B B R
    let pick_order = [
        TeamSide::Blue,
        TeamSide::Red,
        TeamSide::Red,
        TeamSide::Blue,
        TeamSide::Blue,
        TeamSide::Red,
        TeamSide::Red,
        TeamSide::Blue,
        TeamSide::Blue,
        TeamSide::Red,
    ];
    for &team in &pick_order {
        seq.push(DraftSlot {
            phase: DraftPhase::Pick,
            team,
        });
    }
    seq
}

fn five_ban_sequence() -> Vec<DraftSlot> {
    let mut seq = Vec::with_capacity(20);
    // Phase 1 bans: 3 blue, 3 red
    for _ in 0..3 {
        seq.push(DraftSlot {
            phase: DraftPhase::Ban,
            team: TeamSide::Blue,
        });
    }
    for _ in 0..3 {
        seq.push(DraftSlot {
            phase: DraftPhase::Ban,
            team: TeamSide::Red,
        });
    }
    // Phase 1 picks: B R R B B R (6 picks)
    let phase1_picks = [
        TeamSide::Blue,
        TeamSide::Red,
        TeamSide::Red,
        TeamSide::Blue,
        TeamSide::Blue,
        TeamSide::Red,
    ];
    for &team in &phase1_picks {
        seq.push(DraftSlot {
            phase: DraftPhase::Pick,
            team,
        });
    }
    // Phase 2 bans: 2 blue, 2 red
    for _ in 0..2 {
        seq.push(DraftSlot {
            phase: DraftPhase::Ban,
            team: TeamSide::Blue,
        });
    }
    for _ in 0..2 {
        seq.push(DraftSlot {
            phase: DraftPhase::Ban,
            team: TeamSide::Red,
        });
    }
    // Phase 2 picks: R B B R (4 picks)
    let phase2_picks = [TeamSide::Red, TeamSide::Blue, TeamSide::Blue, TeamSide::Red];
    for &team in &phase2_picks {
        seq.push(DraftSlot {
            phase: DraftPhase::Pick,
            team,
        });
    }
    seq
}

// ---------------------------------------------------------------------------
// DraftSlot
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DraftSlot {
    pub phase: DraftPhase,
    pub team: TeamSide,
}

// ---------------------------------------------------------------------------
// DraftAction
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DraftAction {
    SelectChampion(String), // Legacy for quick select
    HoverChampion(String),
    LockChampion,
}

// ---------------------------------------------------------------------------
// DraftError
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DraftError {
    DraftComplete,
    ChampionAlreadySelected(String),
    NoChampionHovered,
}

// ---------------------------------------------------------------------------
// Draft State Machine
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Draft {
    format: DraftFormat,
    sequence: Vec<DraftSlot>,
    current_step: usize,
    blue_bans: Vec<String>,
    red_bans: Vec<String>,
    blue_picks: Vec<String>,
    red_picks: Vec<String>,
    all_selected: HashSet<String>,
    active_hover: Option<String>,
}

impl Draft {
    pub fn new(format: DraftFormat) -> Self {
        let sequence = format.sequence();
        Self {
            format,
            sequence,
            current_step: 0,
            blue_bans: Vec::new(),
            red_bans: Vec::new(),
            blue_picks: Vec::new(),
            red_picks: Vec::new(),
            all_selected: HashSet::new(),
            active_hover: None,
        }
    }

    pub fn current_step(&self) -> usize {
        self.current_step
    }

    pub fn is_complete(&self) -> bool {
        self.current_step >= self.sequence.len()
    }

    pub fn current_slot(&self) -> Option<&DraftSlot> {
        self.sequence.get(self.current_step)
    }

    pub fn apply_action(&mut self, action: DraftAction) -> Result<(), DraftError> {
        if self.is_complete() {
            return Err(DraftError::DraftComplete);
        }

        let slot = self.sequence[self.current_step];

        match action {
            DraftAction::SelectChampion(ref name) => {
                if self.all_selected.contains(name) {
                    return Err(DraftError::ChampionAlreadySelected(name.clone()));
                }
                self.lock_name(name.clone(), slot);
                self.current_step += 1;
                self.active_hover = None;
            }
            DraftAction::HoverChampion(ref name) => {
                if self.all_selected.contains(name) {
                    return Err(DraftError::ChampionAlreadySelected(name.clone()));
                }
                self.active_hover = Some(name.clone());
            }
            DraftAction::LockChampion => {
                if let Some(name) = self.active_hover.take() {
                    self.lock_name(name, slot);
                    self.current_step += 1;
                } else {
                    return Err(DraftError::NoChampionHovered);
                }
            }
        }

        Ok(())
    }

    fn lock_name(&mut self, name: String, slot: DraftSlot) {
        self.all_selected.insert(name.clone());

        match (slot.phase, slot.team) {
            (DraftPhase::Ban, TeamSide::Blue) => self.blue_bans.push(name),
            (DraftPhase::Ban, TeamSide::Red) => self.red_bans.push(name),
            (DraftPhase::Pick, TeamSide::Blue) => self.blue_picks.push(name),
            (DraftPhase::Pick, TeamSide::Red) => self.red_picks.push(name),
        }
    }

    pub fn blue_bans(&self) -> &[String] {
        &self.blue_bans
    }

    pub fn red_bans(&self) -> &[String] {
        &self.red_bans
    }

    pub fn blue_picks(&self) -> &[String] {
        &self.blue_picks
    }

    pub fn red_picks(&self) -> &[String] {
        &self.red_picks
    }

    pub fn all_selected(&self) -> &HashSet<String> {
        &self.all_selected
    }

    pub fn active_hover(&self) -> Option<&str> {
        self.active_hover.as_deref()
    }

    pub fn format(&self) -> DraftFormat {
        self.format
    }

    /// Swap two picks on the blue side. Returns Err if indices are out of bounds.
    pub fn swap_blue_picks(&mut self, a: usize, b: usize) -> Result<(), DraftError> {
        if a >= self.blue_picks.len() || b >= self.blue_picks.len() {
            return Err(DraftError::DraftComplete); // reuse: invalid swap indices
        }
        self.blue_picks.swap(a, b);
        Ok(())
    }

    /// Swap two picks on the red side. Returns Err if indices are out of bounds.
    pub fn swap_red_picks(&mut self, a: usize, b: usize) -> Result<(), DraftError> {
        if a >= self.red_picks.len() || b >= self.red_picks.len() {
            return Err(DraftError::DraftComplete); // reuse: invalid swap indices
        }
        self.red_picks.swap(a, b);
        Ok(())
    }
}
