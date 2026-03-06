use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Time-of-day slot for scheduling activities.
/// Morning, Afternoon, Evening — 3 slots per day.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TimeSlot {
    Morning,
    Afternoon,
    Evening,
}

impl TimeSlot {
    pub const ALL: [TimeSlot; 3] = [TimeSlot::Morning, TimeSlot::Afternoon, TimeSlot::Evening];

    pub fn as_str(&self) -> &'static str {
        match self {
            TimeSlot::Morning => "Morning",
            TimeSlot::Afternoon => "Afternoon",
            TimeSlot::Evening => "Evening",
        }
    }

    pub fn index(self) -> usize {
        match self {
            TimeSlot::Morning => 0,
            TimeSlot::Afternoon => 1,
            TimeSlot::Evening => 2,
        }
    }

    pub fn next(self) -> Option<TimeSlot> {
        match self {
            TimeSlot::Morning => Some(TimeSlot::Afternoon),
            TimeSlot::Afternoon => Some(TimeSlot::Evening),
            TimeSlot::Evening => None,
        }
    }
}

impl fmt::Display for TimeSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for TimeSlot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Morning" => Ok(TimeSlot::Morning),
            "Afternoon" => Ok(TimeSlot::Afternoon),
            "Evening" => Ok(TimeSlot::Evening),
            _ => Err(format!("Unknown TimeSlot: '{s}'")),
        }
    }
}
