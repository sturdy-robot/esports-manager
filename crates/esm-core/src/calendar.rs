use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DayPhase {
    Morning,
    Afternoon,
    Evening,
}

impl DayPhase {
    fn next(self) -> Option<DayPhase> {
        match self {
            DayPhase::Morning => Some(DayPhase::Afternoon),
            DayPhase::Afternoon => Some(DayPhase::Evening),
            DayPhase::Evening => None,
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
