pub mod processor;
pub mod scrim;
pub mod scrim_manager;
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

const MIN_SCHEDULE_DAYS: usize = 7;

/// A calendar-backed team schedule indexed by absolute day offset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamWeeklySchedule {
    days: Vec<TeamDailySchedule>,
}

impl TeamWeeklySchedule {
    pub fn new() -> Self {
        Self::with_total_days(MIN_SCHEDULE_DAYS)
    }

    pub fn with_total_days(total_days: usize) -> Self {
        Self {
            days: vec![TeamDailySchedule::new(); total_days.max(MIN_SCHEDULE_DAYS)],
        }
    }

    pub fn day(&self, day_index: usize) -> &TeamDailySchedule {
        &self.days[day_index]
    }

    pub fn day_mut(&mut self, day_index: usize) -> &mut TeamDailySchedule {
        if day_index >= self.days.len() {
            self.days.resize_with(day_index + 1, TeamDailySchedule::new);
        }
        &mut self.days[day_index]
    }

    pub fn days(&self) -> &[TeamDailySchedule] {
        &self.days
    }

    /// Total scrims scheduled across the entire calendar.
    pub fn total_scrims(&self) -> usize {
        self.days.iter().map(|d| d.scrim_count()).sum()
    }

    /// Count how many slots are occupied (non-free) across the entire calendar.
    pub fn occupied_slots(&self) -> usize {
        self.days
            .iter()
            .flat_map(|d| d.slots().iter())
            .filter(|s| s.is_some())
            .count()
    }

    /// Maximum possible slots in the current calendar span.
    pub fn total_slots(&self) -> usize {
        self.days.len() * SLOTS_PER_DAY
    }
}

impl Default for TeamWeeklySchedule {
    fn default() -> Self {
        Self::new()
    }
}
