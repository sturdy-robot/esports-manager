use esm_core::rng::GameRng;

use super::super::event::{Commentary, MatchEvent, MatchEventKind, MobaMatchPhase};
use super::super::game_state::MatchGameState;
use super::super::sim_event::SimEvent;
use super::super::state::TeamSide;

// ---------------------------------------------------------------------------
// TowerSiege — destroy the next vulnerable tower
// ---------------------------------------------------------------------------

pub struct TowerSiege;

impl SimEvent for TowerSiege {
    fn name(&self) -> &'static str {
        "TowerSiege"
    }

    fn is_enabled(&self, state: &MatchGameState) -> bool {
        state.minute >= 5
            && (state.next_vulnerable_tower(TeamSide::Blue).is_some()
                || state.next_vulnerable_tower(TeamSide::Red).is_some())
    }

    fn weight(&self, state: &MatchGameState) -> f64 {
        match state.phase() {
            MobaMatchPhase::Early => 4.0,
            MobaMatchPhase::Mid => 20.0,
            MobaMatchPhase::Late => 15.0,
        }
    }

    fn process(&self, state: &mut MatchGameState, rng: &mut GameRng) -> MatchEvent {
        let attacker = state.resolve_winner(rng);
        let defender = attacker.opposite();

        let target = state
            .next_vulnerable_tower(defender)
            .or_else(|| state.next_vulnerable_tower(attacker));

        if let Some((lane, tier)) = target {
            let actual_defender = if state.next_vulnerable_tower(defender).is_some() {
                defender
            } else {
                attacker
            };

            state.map.destroy_tower(actual_defender, lane, tier);

            let gold = 550;
            if let Some(idx) = state.pick_alive_player(rng, actual_defender.opposite()) {
                state.players_mut(actual_defender.opposite())[idx].add_gold(gold);
            }

            let actual_attacker = actual_defender.opposite();
            let mut event = MatchEvent::new(
                state.minute,
                state.phase(),
                MatchEventKind::TowerDestroyed {
                    attacker: actual_attacker,
                    lane,
                },
            );
            event.set_commentary(Commentary::for_tower_destroy(
                &state.team_name(actual_attacker),
                lane,
            ));
            event
        } else {
            MatchEvent::new(state.minute, state.phase(), MatchEventKind::FarmTick)
        }
    }
}
