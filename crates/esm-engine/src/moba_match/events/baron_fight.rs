use esm_core::rng::GameRng;

use super::super::event::{Commentary, MatchEvent, MatchEventKind, MobaMatchPhase};
use super::super::game_state::MatchGameState;
use super::super::sim_event::SimEvent;

// ---------------------------------------------------------------------------
// BaronFight
// ---------------------------------------------------------------------------

pub struct BaronFight;

impl SimEvent for BaronFight {
    fn name(&self) -> &'static str {
        "BaronFight"
    }

    fn is_enabled(&self, state: &MatchGameState) -> bool {
        state.baron_available()
    }

    fn weight(&self, state: &MatchGameState) -> f64 {
        match state.phase() {
            MobaMatchPhase::Early => 0.0,
            MobaMatchPhase::Mid => 12.0,
            MobaMatchPhase::Late => 20.0,
        }
    }

    fn process(&self, state: &mut MatchGameState, rng: &mut GameRng) -> MatchEvent {
        let killer = state.resolve_winner(rng);
        state.map.objectives_mut().kill_baron(killer);

        for p in state.players_mut(killer).iter_mut() {
            p.add_gold(300);
        }

        state.baron_spawn_timer = 6;

        let mut event = MatchEvent::new(
            state.minute,
            state.phase(),
            MatchEventKind::BaronKill { killer },
        );
        event.set_commentary(Commentary::for_baron(&state.team_name(killer)));
        event
    }
}
