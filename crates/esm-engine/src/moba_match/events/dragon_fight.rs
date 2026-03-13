use esm_core::rng::GameRng;

use super::super::event::{Commentary, MatchEvent, MatchEventKind, MobaMatchPhase};
use super::super::game_state::MatchGameState;
use super::super::sim_event::SimEvent;
use super::super::state::TeamSide;

// ---------------------------------------------------------------------------
// DragonFight
// ---------------------------------------------------------------------------

pub struct DragonFight;

impl SimEvent for DragonFight {
    fn name(&self) -> &'static str {
        "DragonFight"
    }

    fn is_enabled(&self, state: &MatchGameState) -> bool {
        state.dragon_available()
    }

    fn weight(&self, state: &MatchGameState) -> f64 {
        let base = match state.phase() {
            MobaMatchPhase::Early => 8.0,
            MobaMatchPhase::Mid => 15.0,
            MobaMatchPhase::Late => 12.0,
        };
        let max_dragons = state
            .map
            .objectives()
            .dragon_count(TeamSide::Blue)
            .max(state.map.objectives().dragon_count(TeamSide::Red));
        if max_dragons >= 3 {
            base * 1.5
        } else {
            base
        }
    }

    fn process(&self, state: &mut MatchGameState, rng: &mut GameRng) -> MatchEvent {
        let killer = state.resolve_winner(rng);
        state.map.objectives_mut().add_dragon(killer);
        let count = state.map.objectives().dragon_count(killer);

        for p in state.players_mut(killer).iter_mut() {
            p.add_gold(50);
        }

        state.dragon_timer = 5;

        let mut event = MatchEvent::new(
            state.minute,
            state.phase(),
            MatchEventKind::DragonKill { killer },
        );
        event.set_commentary(Commentary::for_dragon(&state.team_name(killer), count));
        event
    }
}
