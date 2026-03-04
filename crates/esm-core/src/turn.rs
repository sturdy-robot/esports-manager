use serde::{Deserialize, Serialize};

use crate::game_state::GameState;

// ---------------------------------------------------------------------------
// Turn processing constants
// ---------------------------------------------------------------------------

/// Base stamina recovery per day (rest day equivalent from DESIGN.md is +30,
/// but a normal day recovery is a smaller passive tick).
const BASE_DAILY_STAMINA_RECOVERY: u8 = 5;

// ---------------------------------------------------------------------------
// Result types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TurnError {
    BlockingMessages,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnResult {
    pub day_elapsed: u32,
    pub is_weekly_tick: bool,
}

// ---------------------------------------------------------------------------
// TurnProcessor
// ---------------------------------------------------------------------------

pub struct TurnProcessor;

impl TurnProcessor {
    /// Process end-of-day: validates no blocking messages, auto-resolves
    /// actionable messages, applies daily recovery, and advances the calendar.
    pub fn end_day(state: &mut GameState) -> Result<TurnResult, TurnError> {
        // Gate: cannot end day with unresolved HardBlock messages
        if !state.can_continue() {
            return Err(TurnError::BlockingMessages);
        }

        // Auto-resolve unresolved RequiresResponse messages with default outcome
        state.inbox_mut().resolve_all_actionable();

        // Apply daily stamina recovery to all players on all teams
        Self::apply_daily_stamina_recovery(state);

        // Advance the calendar
        state.advance_day();

        Ok(TurnResult {
            day_elapsed: state.calendar().days_elapsed(),
            is_weekly_tick: state.calendar().is_weekly_tick(),
        })
    }

    fn apply_daily_stamina_recovery(state: &mut GameState) {
        for team in state.teams_mut() {
            for player in team.roster_mut() {
                player
                    .state_mut()
                    .stamina
                    .increase(BASE_DAILY_STAMINA_RECOVERY);
            }
        }
    }
}
