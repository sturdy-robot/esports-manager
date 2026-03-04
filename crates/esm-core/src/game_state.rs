use serde::{Deserialize, Serialize};

use crate::calendar::Calendar;
use crate::rng::GameRng;
use esm_models::manager::Manager;
use esm_models::team::Team;

/// Central game state that orchestrates all core systems.
///
/// Holds the calendar, deterministic RNG, manager, and all teams.
/// This is the top-level object that gets serialized for save/load.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    calendar: Calendar,
    rng: GameRng,
    manager: Manager,
    player_team_index: usize,
    teams: Vec<Team>,
}

impl GameState {
    /// Create a new game starting January 1st of `start_year`.
    ///
    /// `player_team_index` identifies which team in `teams` the player manages.
    pub fn new(
        start_year: u32,
        seed: u64,
        manager: Manager,
        player_team_index: usize,
        teams: Vec<Team>,
    ) -> Self {
        Self {
            calendar: Calendar::new(start_year, 1, 1),
            rng: GameRng::from_seed(seed),
            manager,
            player_team_index,
            teams,
        }
    }

    pub fn calendar(&self) -> &Calendar {
        &self.calendar
    }

    pub fn rng(&self) -> &GameRng {
        &self.rng
    }

    pub fn rng_mut(&mut self) -> &mut GameRng {
        &mut self.rng
    }

    pub fn manager(&self) -> &Manager {
        &self.manager
    }

    pub fn manager_mut(&mut self) -> &mut Manager {
        &mut self.manager
    }

    pub fn player_team_index(&self) -> usize {
        self.player_team_index
    }

    pub fn player_team(&self) -> &Team {
        &self.teams[self.player_team_index]
    }

    pub fn player_team_mut(&mut self) -> &mut Team {
        &mut self.teams[self.player_team_index]
    }

    pub fn teams(&self) -> &[Team] {
        &self.teams
    }

    pub fn teams_mut(&mut self) -> &mut [Team] {
        &mut self.teams
    }

    pub fn team_by_name(&self, name: &str) -> Option<&Team> {
        self.teams.iter().find(|t| t.name() == name)
    }

    /// Advance the calendar by one phase (Morning → Afternoon → Evening → next day Morning).
    pub fn advance_phase(&mut self) {
        self.calendar.advance_phase();
    }

    /// Advance the calendar by one full day, resetting to Morning.
    pub fn advance_day(&mut self) {
        self.calendar.advance_day();
    }
}
