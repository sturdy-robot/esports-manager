use esm_core::rng::GameRng;

use super::super::event::{Commentary, MatchEvent, MatchEventKind, MobaMatchPhase};
use super::super::game_state::MatchGameState;
use super::super::sim_event::SimEvent;

// ---------------------------------------------------------------------------
// HeraldFight
// ---------------------------------------------------------------------------

pub struct HeraldFight;

impl SimEvent for HeraldFight {
    fn name(&self) -> &'static str {
        "HeraldFight"
    }

    fn is_enabled(&self, state: &MatchGameState) -> bool {
        state.herald_available()
    }

    fn weight(&self, state: &MatchGameState) -> f64 {
        match state.phase() {
            MobaMatchPhase::Early => 10.0,
            MobaMatchPhase::Mid => 5.0,
            MobaMatchPhase::Late => 0.0,
        }
    }

    fn process(&self, state: &mut MatchGameState, rng: &mut GameRng) -> MatchEvent {
        let killer = state.resolve_winner(rng);
        state.map.objectives_mut().consume_herald();

        let mut event = MatchEvent::new(
            state.minute,
            state.phase(),
            MatchEventKind::HeraldKill { killer },
        );
        event.set_commentary(Commentary::for_herald(&state.team_name(killer)));
        event
    }
}
