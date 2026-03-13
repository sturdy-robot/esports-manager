use esm_core::rng::GameRng;

use super::super::event::{Commentary, MatchEvent, MatchEventKind, MobaMatchPhase};
use super::super::game_state::MatchGameState;
use super::super::map::Lane;
use super::super::sim_event::SimEvent;
use super::super::state::TeamSide;

const ALL_LANES: [Lane; 3] = [Lane::Top, Lane::Mid, Lane::Bot];

// ---------------------------------------------------------------------------
// SoloKill
// ---------------------------------------------------------------------------

pub struct SoloKill;

impl SimEvent for SoloKill {
    fn name(&self) -> &'static str {
        "SoloKill"
    }

    fn is_enabled(&self, state: &MatchGameState) -> bool {
        state.alive_count(TeamSide::Blue) > 0 && state.alive_count(TeamSide::Red) > 0
    }

    fn weight(&self, state: &MatchGameState) -> f64 {
        match state.phase() {
            MobaMatchPhase::Early => 25.0,
            MobaMatchPhase::Mid => 15.0,
            MobaMatchPhase::Late => 8.0,
        }
    }

    fn process(&self, state: &mut MatchGameState, rng: &mut GameRng) -> MatchEvent {
        let winner_side = state.resolve_winner(rng);
        let loser_side = winner_side.opposite();

        let killer_idx = state.pick_alive_player(rng, winner_side).unwrap();
        let victim_idx = state.pick_alive_player(rng, loser_side).unwrap();

        let lane = ALL_LANES[rng.range_u32(0, 3) as usize];
        let is_first_blood = !state.first_blood_claimed;
        if is_first_blood {
            state.first_blood_claimed = true;
        }

        let bounty = state.players(loser_side)[victim_idx].bounty();
        let death_timer = 10 + state.minute;

        state.players_mut(winner_side)[killer_idx].record_kill(bounty);
        state.players_mut(loser_side)[victim_idx].record_death(death_timer);

        let mut event = MatchEvent::new(
            state.minute,
            state.phase(),
            MatchEventKind::SoloKill {
                killer_idx,
                victim_idx,
                lane,
            },
        );
        let killer_name = state.player_name(winner_side, killer_idx);
        let victim_name = state.player_name(loser_side, victim_idx);
        event.set_commentary(Commentary::for_solo_kill(
            &killer_name,
            &victim_name,
            lane,
            is_first_blood,
        ));
        event
    }
}
