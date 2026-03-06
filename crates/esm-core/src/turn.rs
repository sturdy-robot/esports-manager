use serde::{Deserialize, Serialize};

use crate::game_state::GameState;
use esm_models::activity::ActivityScheduler;

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

        // Process daily schedules for all players (stamina, morale, etc.)
        Self::process_daily_schedules(state);

        // Advance the calendar
        state.advance_day();

        Ok(TurnResult {
            day_elapsed: state.calendar().days_elapsed(),
            is_weekly_tick: state.calendar().is_weekly_tick(),
        })
    }

    fn process_daily_schedules(state: &mut GameState) {
        for team in state.teams_mut() {
            for player in team.roster_mut() {
                let schedule = player.schedule();
                let effect = ActivityScheduler::compute_daily_effect(schedule);
                
                let net_stamina_change = (BASE_DAILY_STAMINA_RECOVERY as i16) 
                    + (effect.stamina_recovery as i16) 
                    - (effect.stamina_cost as i16);
                
                let state = player.state_mut();
                if net_stamina_change > 0 {
                    state.stamina.increase(net_stamina_change as u8);
                } else if net_stamina_change < 0 {
                    let decrease = net_stamina_change.unsigned_abs() as u8;
                    state.stamina.decrease(decrease);
                    
                    // Penalty: if stamina drops to 0, player loses morale and satisfaction
                    if state.stamina.value() == 0 {
                        state.morale.decrease(5);
                        state.satisfaction.decrease(2);
                    }
                }
            }
        }
    }
}
