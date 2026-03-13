use esm_core::rng::GameRng;

use super::super::event::{Commentary, MatchEvent, MatchEventKind, MobaMatchPhase};
use super::super::game_state::MatchGameState;
use super::super::sim_event::SimEvent;
use super::super::state::TeamSide;

// ---------------------------------------------------------------------------
// NexusSiege — attempt to end the game
// ---------------------------------------------------------------------------

pub struct NexusSiege;

impl SimEvent for NexusSiege {
    fn name(&self) -> &'static str {
        "NexusSiege"
    }

    fn is_enabled(&self, state: &MatchGameState) -> bool {
        state.minute >= 20
            && (state.map.is_nexus_vulnerable(TeamSide::Blue)
                || state.map.is_nexus_vulnerable(TeamSide::Red))
    }

    fn weight(&self, state: &MatchGameState) -> f64 {
        let base = match state.phase() {
            MobaMatchPhase::Early => 0.0,
            MobaMatchPhase::Mid => 10.0,
            MobaMatchPhase::Late => 35.0,
        };
        let time_pressure = ((state.minute as f64 - 20.0) / 10.0).max(0.0);
        base + time_pressure * 5.0
    }

    fn process(&self, state: &mut MatchGameState, rng: &mut GameRng) -> MatchEvent {
        let blue_nexus_open = state.map.is_nexus_vulnerable(TeamSide::Blue);
        let red_nexus_open = state.map.is_nexus_vulnerable(TeamSide::Red);

        let (attacker, defender) = if blue_nexus_open && red_nexus_open {
            let winner = state.resolve_winner(rng);
            (winner, winner.opposite())
        } else if blue_nexus_open {
            (TeamSide::Red, TeamSide::Blue)
        } else {
            (TeamSide::Blue, TeamSide::Red)
        };

        let gold_dir = if attacker == TeamSide::Blue {
            1.0
        } else {
            -1.0
        };
        let gold_advantage = state.gold_delta() as f64 * gold_dir;
        let gold_bonus = (gold_advantage / 20_000.0).max(0.0);
        let time_factor = ((state.minute as f64 - 24.0) / 10.0).max(0.0).min(1.0);
        let alive_advantage =
            state.alive_count(attacker) as f64 - state.alive_count(defender) as f64;
        let alive_bonus = (alive_advantage * 0.08).max(0.0);

        let nexus_prob = (0.15 + time_factor * 0.25 + gold_bonus + alive_bonus).min(0.85);

        if rng.check_probability(nexus_prob) {
            state.winner = Some(attacker);
            let mut event = MatchEvent::new(
                state.minute,
                state.phase(),
                MatchEventKind::NexusDestroyed { winner: attacker },
            );
            event.set_commentary(Commentary::for_nexus_destroy(&state.team_name(attacker)));
            event
        } else {
            let kills_for_defender = rng.range_u32(1, 4);
            let death_timer = 15 + state.minute;
            for _ in 0..kills_for_defender {
                if let Some(victim_idx) = state.pick_alive_player(rng, attacker) {
                    state.players_mut(attacker)[victim_idx].record_death(death_timer);
                }
                if let Some(killer_idx) = state.pick_alive_player(rng, defender) {
                    let gold = rng.range_u32(200, 400);
                    state.players_mut(defender)[killer_idx].record_kill(gold);
                }
            }

            let (kb, kr) = match defender {
                TeamSide::Blue => (kills_for_defender, 0),
                TeamSide::Red => (0, kills_for_defender),
            };

            let mut event = MatchEvent::new(
                state.minute,
                state.phase(),
                MatchEventKind::Teamfight {
                    winning_side: defender,
                    kills_blue: kb,
                    kills_red: kr,
                },
            );
            event.set_commentary(Commentary::for_teamfight(
                &state.team_name(defender),
                kills_for_defender,
                0,
            ));
            event
        }
    }
}
