use esm_core::rng::GameRng;

use super::super::event::{Commentary, MatchEvent, MatchEventKind, MobaMatchPhase};
use super::super::game_state::MatchGameState;
use super::super::sim_event::SimEvent;
use super::super::state::TeamSide;

// ---------------------------------------------------------------------------
// Teamfight
// ---------------------------------------------------------------------------

pub struct Teamfight;

impl SimEvent for Teamfight {
    fn name(&self) -> &'static str {
        "Teamfight"
    }

    fn is_enabled(&self, state: &MatchGameState) -> bool {
        state.alive_count(TeamSide::Blue) >= 2 && state.alive_count(TeamSide::Red) >= 2
    }

    fn weight(&self, state: &MatchGameState) -> f64 {
        match state.phase() {
            MobaMatchPhase::Early => 5.0,
            MobaMatchPhase::Mid => 20.0,
            MobaMatchPhase::Late => 30.0,
        }
    }

    fn process(&self, state: &mut MatchGameState, rng: &mut GameRng) -> MatchEvent {
        let winner_side = state.resolve_winner(rng);
        let kills_won = rng.range_u32(1, 5);
        let kills_lost = rng.range_u32(0, 3);
        let death_timer = 15 + state.minute;
        let players_per_side = state.players(winner_side).len();

        for i in 0..players_per_side {
            state.players_mut(winner_side)[i].reset_fight_kills();
            state.players_mut(winner_side.opposite())[i].reset_fight_kills();
        }

        for _ in 0..kills_won {
            if let Some(killer_idx) = state.pick_alive_player(rng, winner_side) {
                let gold = rng.range_u32(200, 400);
                state.players_mut(winner_side)[killer_idx].record_fight_kill(gold);
            }
            if let Some(victim_idx) = state.pick_alive_player(rng, winner_side.opposite()) {
                state.players_mut(winner_side.opposite())[victim_idx].record_death(death_timer);
            }
        }

        for _ in 0..kills_lost {
            if let Some(victim_idx) = state.pick_alive_player(rng, winner_side) {
                state.players_mut(winner_side)[victim_idx].record_death(death_timer);
            }
            if let Some(killer_idx) = state.pick_alive_player(rng, winner_side.opposite()) {
                let gold = rng.range_u32(200, 400);
                state.players_mut(winner_side.opposite())[killer_idx].record_fight_kill(gold);
            }
        }

        if !state.first_blood_claimed && (kills_won > 0 || kills_lost > 0) {
            state.first_blood_claimed = true;
        }

        let mut sub_events = Vec::new();
        for side in [winner_side, winner_side.opposite()] {
            for i in 0..players_per_side {
                let player = &state.players(side)[i];
                if let Some(mk) = player.current_multi_kill() {
                    let tier_label = mk.label().to_string();
                    let mut mk_event = MatchEvent::new(
                        state.minute,
                        state.phase(),
                        MatchEventKind::MultiKill {
                            side,
                            player_idx: i,
                            tier: tier_label.clone(),
                        },
                    );
                    mk_event.set_commentary(Commentary::for_multi_kill(
                        &state.player_name(side, i),
                        &tier_label,
                    ));
                    sub_events.push(mk_event);
                }
                if let Some(spree) = player.spree_label() {
                    let mut sp_event = MatchEvent::new(
                        state.minute,
                        state.phase(),
                        MatchEventKind::KillingSpree {
                            side,
                            player_idx: i,
                            label: spree.to_string(),
                        },
                    );
                    sp_event.set_commentary(Commentary::for_killing_spree(
                        &state.player_name(side, i),
                        spree,
                    ));
                    sub_events.push(sp_event);
                }
            }
        }
        for ev in sub_events {
            state.record_event(ev);
        }

        let (kb, kr) = match winner_side {
            TeamSide::Blue => (kills_won, kills_lost),
            TeamSide::Red => (kills_lost, kills_won),
        };

        let mut event = MatchEvent::new(
            state.minute,
            state.phase(),
            MatchEventKind::Teamfight {
                winning_side: winner_side,
                kills_blue: kb,
                kills_red: kr,
            },
        );
        event.set_commentary(Commentary::for_teamfight(
            &state.team_name(winner_side),
            kills_won,
            kills_lost,
        ));
        event
    }
}
