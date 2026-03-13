use esm_core::rng::GameRng;

use super::super::event::{MatchEvent, MatchEventKind, MobaMatchPhase};
use super::super::game_state::MatchGameState;
use super::super::sim_event::SimEvent;

// ---------------------------------------------------------------------------
// FarmTick — passive gold/CS accumulation, always enabled
// ---------------------------------------------------------------------------

pub struct FarmTick;

impl SimEvent for FarmTick {
    fn name(&self) -> &'static str {
        "FarmTick"
    }

    fn is_enabled(&self, _state: &MatchGameState) -> bool {
        true
    }

    fn weight(&self, state: &MatchGameState) -> f64 {
        match state.phase() {
            MobaMatchPhase::Early => 50.0,
            MobaMatchPhase::Mid => 30.0,
            MobaMatchPhase::Late => 15.0,
        }
    }

    fn process(&self, state: &mut MatchGameState, rng: &mut GameRng) -> MatchEvent {
        for p in state
            .blue_players
            .iter_mut()
            .chain(state.red_players.iter_mut())
        {
            if !p.is_dead() {
                p.add_gold(rng.range_u32(80, 120));
                p.add_cs(rng.range_u32(5, 9));
            }
        }
        state.tick_minute();

        MatchEvent::new(state.minute, state.phase(), MatchEventKind::FarmTick)
    }
}
