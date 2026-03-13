use esm_core::rng::GameRng;

use super::super::event::{Commentary, MatchEvent, MatchEventKind, MobaMatchPhase};
use super::super::game_state::MatchGameState;
use super::super::sim_event::SimEvent;
use super::super::state::TeamSide;

const INHIB_RESPAWN_SECONDS: u32 = 300;

// ---------------------------------------------------------------------------
// InhibSiege — destroy a vulnerable inhibitor
// ---------------------------------------------------------------------------

pub struct InhibSiege;

impl SimEvent for InhibSiege {
    fn name(&self) -> &'static str {
        "InhibSiege"
    }

    fn is_enabled(&self, state: &MatchGameState) -> bool {
        state.minute >= 15
            && (state.vulnerable_inhibitor_lane(TeamSide::Blue).is_some()
                || state.vulnerable_inhibitor_lane(TeamSide::Red).is_some())
    }

    fn weight(&self, state: &MatchGameState) -> f64 {
        match state.phase() {
            MobaMatchPhase::Early => 0.0,
            MobaMatchPhase::Mid => 5.0,
            MobaMatchPhase::Late => 15.0,
        }
    }

    fn process(&self, state: &mut MatchGameState, rng: &mut GameRng) -> MatchEvent {
        let attacker = state.resolve_winner(rng);
        let defender = attacker.opposite();

        let lane = state
            .vulnerable_inhibitor_lane(defender)
            .or_else(|| state.vulnerable_inhibitor_lane(attacker));

        if let Some(lane) = lane {
            let actual_defender = if state.vulnerable_inhibitor_lane(defender).is_some() {
                defender
            } else {
                attacker
            };
            let actual_attacker = actual_defender.opposite();

            state
                .map
                .destroy_inhibitor(actual_defender, lane, INHIB_RESPAWN_SECONDS);

            let mut event = MatchEvent::new(
                state.minute,
                state.phase(),
                MatchEventKind::InhibitorDestroyed {
                    attacker: actual_attacker,
                    lane,
                },
            );
            event.set_commentary(Commentary::for_inhibitor_destroy(
                &state.team_name(actual_attacker),
                lane,
            ));
            event
        } else {
            MatchEvent::new(state.minute, state.phase(), MatchEventKind::FarmTick)
        }
    }
}
