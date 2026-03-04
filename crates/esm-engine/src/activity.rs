use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Activity types (DESIGN.md §6)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Activity {
    ScrimBlock,
    SoloQueue,
    RestDay,
}

/// The numerical impact of performing a single activity slot.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct ActivityEffect {
    pub stamina_cost: u8,
    pub stamina_recovery: u8,
    pub mastery_gain: u8,
    pub attribute_growth: u8,
}

impl Activity {
    /// Return the base effect of this activity type per DESIGN.md §6 impact matrix.
    pub fn effect(self) -> ActivityEffect {
        match self {
            Activity::ScrimBlock => ActivityEffect {
                stamina_cost: 18,
                stamina_recovery: 0,
                mastery_gain: 8,
                attribute_growth: 6,
            },
            Activity::SoloQueue => ActivityEffect {
                stamina_cost: 8,
                stamina_recovery: 0,
                mastery_gain: 4,
                attribute_growth: 2,
            },
            Activity::RestDay => ActivityEffect {
                stamina_cost: 0,
                stamina_recovery: 30,
                mastery_gain: 0,
                attribute_growth: 0,
            },
        }
    }
}

// ---------------------------------------------------------------------------
// Daily Schedule (4 time slots per DESIGN.md §6)
// ---------------------------------------------------------------------------

const SLOTS_PER_DAY: usize = 4;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailySchedule {
    slots: [Option<Activity>; SLOTS_PER_DAY],
}

impl DailySchedule {
    pub fn new() -> Self {
        Self {
            slots: [None; SLOTS_PER_DAY],
        }
    }

    pub fn slots(&self) -> &[Option<Activity>] {
        &self.slots
    }

    /// Set an activity in the given slot. Returns `false` if index is out of bounds.
    pub fn set_slot(&mut self, index: usize, activity: Activity) -> bool {
        if index >= SLOTS_PER_DAY {
            return false;
        }
        self.slots[index] = Some(activity);
        true
    }

    pub fn clear_slot(&mut self, index: usize) {
        if index < SLOTS_PER_DAY {
            self.slots[index] = None;
        }
    }
}

impl Default for DailySchedule {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// ActivityScheduler: aggregates effects for a full day
// ---------------------------------------------------------------------------

pub struct ActivityScheduler;

impl ActivityScheduler {
    /// Sum up the effects of all filled slots in a daily schedule.
    pub fn compute_daily_effect(schedule: &DailySchedule) -> ActivityEffect {
        let mut total = ActivityEffect::default();
        for slot in schedule.slots() {
            if let Some(activity) = slot {
                let e = activity.effect();
                total.stamina_cost = total.stamina_cost.saturating_add(e.stamina_cost);
                total.stamina_recovery = total.stamina_recovery.saturating_add(e.stamina_recovery);
                total.mastery_gain = total.mastery_gain.saturating_add(e.mastery_gain);
                total.attribute_growth = total.attribute_growth.saturating_add(e.attribute_growth);
            }
        }
        total
    }
}
