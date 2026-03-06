pub mod scrim;
pub mod solo_queue;
pub mod validation;

use serde::{Deserialize, Serialize};

use esm_models::time::TimeSlot;

// ---------------------------------------------------------------------------
// Schedule entry: what occupies a team's time slot
// ---------------------------------------------------------------------------

/// Identifies a scrim by its unique ID (assigned at creation).
pub type ScrimId = u32;

/// Focus direction for solo queue practice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SoloQueueFocus {
    Champions,
    Tactics,
    Mechanics,
    Mentality,
}

/// A single time-slot entry in the team's schedule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScheduleEntry {
    /// A scrim block — all 5 players participate.
    Scrim { scrim_id: ScrimId },
    /// Solo queue — specific players are assigned (indices into roster).
    SoloQueue {
        players: Vec<usize>,
        focus: SoloQueueFocus,
    },
    /// Explicit rest — no activities.
    Rest,
}

// ---------------------------------------------------------------------------
// Team daily schedule
// ---------------------------------------------------------------------------

const SLOTS_PER_DAY: usize = 3;

/// A single day's schedule for a team (Morning / Afternoon / Evening).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TeamDailySchedule {
    slots: [Option<ScheduleEntry>; SLOTS_PER_DAY],
}

impl TeamDailySchedule {
    pub fn new() -> Self {
        Self {
            slots: [None, None, None],
        }
    }

    pub fn get(&self, slot: TimeSlot) -> Option<&ScheduleEntry> {
        self.slots[slot.index()].as_ref()
    }

    pub fn set(&mut self, slot: TimeSlot, entry: ScheduleEntry) {
        self.slots[slot.index()] = Some(entry);
    }

    pub fn clear(&mut self, slot: TimeSlot) {
        self.slots[slot.index()] = None;
    }

    pub fn is_free(&self, slot: TimeSlot) -> bool {
        self.slots[slot.index()].is_none()
    }

    pub fn slots(&self) -> &[Option<ScheduleEntry>; SLOTS_PER_DAY] {
        &self.slots
    }

    /// Count how many scrim entries exist in this day.
    pub fn scrim_count(&self) -> usize {
        self.slots
            .iter()
            .filter(|s| matches!(s, Some(ScheduleEntry::Scrim { .. })))
            .count()
    }
}

// ---------------------------------------------------------------------------
// Team weekly schedule (7 days)
// ---------------------------------------------------------------------------

const DAYS_PER_WEEK: usize = 7;

/// A full week of team scheduling, indexed by day offset (0 = Monday .. 6 = Sunday).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamWeeklySchedule {
    days: [TeamDailySchedule; DAYS_PER_WEEK],
}

impl TeamWeeklySchedule {
    pub fn new() -> Self {
        Self {
            days: std::array::from_fn(|_| TeamDailySchedule::new()),
        }
    }

    pub fn day(&self, day_index: usize) -> &TeamDailySchedule {
        &self.days[day_index]
    }

    pub fn day_mut(&mut self, day_index: usize) -> &mut TeamDailySchedule {
        &mut self.days[day_index]
    }

    pub fn days(&self) -> &[TeamDailySchedule; DAYS_PER_WEEK] {
        &self.days
    }

    /// Total scrims scheduled across the entire week.
    pub fn total_scrims(&self) -> usize {
        self.days.iter().map(|d| d.scrim_count()).sum()
    }

    /// Count how many slots are occupied (non-free) across the week.
    pub fn occupied_slots(&self) -> usize {
        self.days
            .iter()
            .flat_map(|d| d.slots().iter())
            .filter(|s| s.is_some())
            .count()
    }

    /// Maximum possible slots in a week (3 per day × 7 days).
    pub fn total_slots(&self) -> usize {
        DAYS_PER_WEEK * SLOTS_PER_DAY
    }
}

impl Default for TeamWeeklySchedule {
    fn default() -> Self {
        Self::new()
    }
}
