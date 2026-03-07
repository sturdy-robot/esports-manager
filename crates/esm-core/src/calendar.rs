use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use esm_models::time::TimeSlot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DayPhase {
    Morning,
    Afternoon,
    Evening,
}

impl From<DayPhase> for TimeSlot {
    fn from(phase: DayPhase) -> Self {
        match phase {
            DayPhase::Morning => TimeSlot::Morning,
            DayPhase::Afternoon => TimeSlot::Afternoon,
            DayPhase::Evening => TimeSlot::Evening,
        }
    }
}

impl From<TimeSlot> for DayPhase {
    fn from(slot: TimeSlot) -> Self {
        match slot {
            TimeSlot::Morning => DayPhase::Morning,
            TimeSlot::Afternoon => DayPhase::Afternoon,
            TimeSlot::Evening => DayPhase::Evening,
        }
    }
}

impl DayPhase {
    pub fn as_str(&self) -> &'static str {
        match self {
            DayPhase::Morning => "Morning",
            DayPhase::Afternoon => "Afternoon",
            DayPhase::Evening => "Evening",
        }
    }

    fn next(self) -> Option<DayPhase> {
        match self {
            DayPhase::Morning => Some(DayPhase::Afternoon),
            DayPhase::Afternoon => Some(DayPhase::Evening),
            DayPhase::Evening => None,
        }
    }
}

impl fmt::Display for DayPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for DayPhase {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Morning" => Ok(DayPhase::Morning),
            "Afternoon" => Ok(DayPhase::Afternoon),
            "Evening" => Ok(DayPhase::Evening),
            _ => Err(format!("Unknown DayPhase: '{s}'")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calendar {
    year: u32,
    month: u32,
    day: u32,
    phase: DayPhase,
    days_elapsed: u32,
}

impl Calendar {
    pub fn new(year: u32, month: u32, day: u32) -> Self {
        Self {
            year,
            month,
            day,
            phase: DayPhase::Morning,
            days_elapsed: 0,
        }
    }

    pub fn year(&self) -> u32 {
        self.year
    }

    pub fn month(&self) -> u32 {
        self.month
    }

    pub fn day(&self) -> u32 {
        self.day
    }

    pub fn phase(&self) -> DayPhase {
        self.phase
    }

    pub fn days_elapsed(&self) -> u32 {
        self.days_elapsed
    }

    pub fn advance_phase(&mut self) {
        match self.phase.next() {
            Some(next_phase) => self.phase = next_phase,
            None => {
                self.phase = DayPhase::Morning;
                self.increment_day();
            }
        }
    }

    pub fn advance_day(&mut self) {
        self.phase = DayPhase::Morning;
        self.increment_day();
    }

    pub fn is_weekly_tick(&self) -> bool {
        self.days_elapsed > 0 && self.days_elapsed % 7 == 0
    }

    /// Returns the day of the week as 0=Sunday, 1=Monday, ..., 6=Saturday.
    /// Uses Tomohiko Sakamoto's algorithm.
    pub fn day_of_week(&self) -> u32 {
        let mut y = self.year as i32;
        let m = self.month as i32;
        let d = self.day as i32;
        const T: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
        if m < 3 {
            y -= 1;
        }
        ((y + y / 4 - y / 100 + y / 400 + T[(m - 1) as usize] + d) % 7) as u32
    }

    /// Returns the short name of the current day of the week.
    pub fn day_of_week_name(&self) -> &'static str {
        const NAMES: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
        NAMES[self.day_of_week() as usize]
    }

    fn increment_day(&mut self) {
        self.days_elapsed += 1;
        let max_day = days_in_month(self.year, self.month);
        if self.day < max_day {
            self.day += 1;
        } else {
            self.day = 1;
            if self.month < 12 {
                self.month += 1;
            } else {
                self.month = 1;
                self.year += 1;
            }
        }
    }
}

fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => panic!("invalid month: {month}"),
    }
}
