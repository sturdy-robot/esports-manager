use esm_core::rng::GameRng;
use serde::{Deserialize, Serialize};

use super::event::MatchEvent;
use super::game_state::{MatchGameState, MatchPlayerSimulationData};
use super::player_state::MatchPlayerState;
use super::sim_events::{all_events, enabled_events_with_tactics, pick_event};
use super::state::TeamSide;
use super::tactics::MatchTactics;

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobaMatchConfig {
    pub players_per_side: usize,
}

impl Default for MobaMatchConfig {
    fn default() -> Self {
        Self {
            players_per_side: 5,
        }
    }
}

// ---------------------------------------------------------------------------
// Result
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobaMatchResult {
    pub winner: TeamSide,
    pub duration_minutes: u32,
    pub events: Vec<MatchEvent>,
    pub blue_players: Vec<MatchPlayerState>,
    pub red_players: Vec<MatchPlayerState>,
    pub blue_team_gold: u32,
    pub red_team_gold: u32,
}

// ---------------------------------------------------------------------------
// Engine — event-driven simulation loop
// ---------------------------------------------------------------------------

pub struct MobaMatchEngine;

impl MobaMatchEngine {
    /// Run a full match simulation.
    ///
    /// `blue_sim_data` and `red_sim_data`: each is a slice of `MatchPlayerSimulationData` structs.
    /// Each struct contains base attributes, stamina, morale, and mastery multiplier.
    ///
    /// The engine loop:
    /// 1. Collect all enabled events based on current game state.
    /// 2. Pick one event via weighted random selection.
    /// 3. Let the event process itself (mutate state, produce MatchEvent).
    /// 4. Record the event.
    /// 5. Repeat until a winner is determined.
    ///
    /// There is **no time limit**. Games end naturally when a nexus falls.
    /// Run a full match simulation with default (balanced) tactics.
    pub fn simulate(
        rng: &mut GameRng,
        blue_sim_data: &[MatchPlayerSimulationData],
        red_sim_data: &[MatchPlayerSimulationData],
        config: &MobaMatchConfig,
    ) -> MobaMatchResult {
        Self::simulate_with_tactics(
            rng,
            blue_sim_data,
            red_sim_data,
            config,
            &MatchTactics::default(),
        )
    }

    /// Run a full match simulation with the given tactical modifiers.
    pub fn simulate_with_tactics(
        rng: &mut GameRng,
        blue_sim_data: &[MatchPlayerSimulationData],
        red_sim_data: &[MatchPlayerSimulationData],
        config: &MobaMatchConfig,
        tactics: &MatchTactics,
    ) -> MobaMatchResult {
        let mut state = MatchGameState::new(
            blue_sim_data.to_vec(),
            red_sim_data.to_vec(),
            config.players_per_side,
        );

        let events = all_events();

        while !state.is_over() {
            let enabled = enabled_events_with_tactics(&state, &events, tactics);

            if enabled.is_empty() {
                state.tick_minute();
                continue;
            }

            let event_idx = pick_event(rng, &enabled);
            let match_event = events[event_idx].process(&mut state, rng);
            state.record_event(match_event);

            if state.minute >= 20
                && !state.map.objectives().baron_alive()
                && state.baron_spawn_timer == 0
            {
                state.map.objectives_mut().spawn_baron();
            }
        }

        let blue_gold = state.blue_total_gold();
        let red_gold = state.red_total_gold();
        let winner = state.winner.unwrap();
        let duration = state.minute;

        MobaMatchResult {
            winner,
            duration_minutes: duration,
            events: state.events,
            blue_players: state.blue_players,
            red_players: state.red_players,
            blue_team_gold: blue_gold,
            red_team_gold: red_gold,
        }
    }
}
