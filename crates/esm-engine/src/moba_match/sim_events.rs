use esm_core::rng::GameRng;

use super::event::{Commentary, MatchEvent, MatchEventKind, MobaMatchPhase};
use super::game_state::MatchGameState;
use super::map::Lane;
use super::state::TeamSide;
use super::tactics::MatchTactics;

const ALL_LANES: [Lane; 3] = [Lane::Top, Lane::Mid, Lane::Bot];
const INHIB_RESPAWN_SECONDS: u32 = 300;

// ---------------------------------------------------------------------------
// SimEvent trait
// ---------------------------------------------------------------------------

/// Each simulation event is an independent object that knows:
/// - Whether it is enabled given the current game state.
/// - Its probability of occurring.
/// - How to process itself (mutate game state + return a MatchEvent).
pub trait SimEvent {
    /// A short identifier for logging/debugging.
    fn name(&self) -> &'static str;

    /// Is this event eligible to fire given the current game state?
    fn is_enabled(&self, state: &MatchGameState) -> bool;

    /// Relative weight of this event occurring (not necessarily 0..1).
    /// Higher weight = more likely to be picked among enabled events.
    fn weight(&self, state: &MatchGameState) -> f64;

    /// Execute the event: mutate game state, return the resulting MatchEvent.
    fn process(&self, state: &mut MatchGameState, rng: &mut GameRng) -> MatchEvent;
}

// ---------------------------------------------------------------------------
// Helper: side name for commentary
// ---------------------------------------------------------------------------

fn side_name(side: TeamSide) -> &'static str {
    match side {
        TeamSide::Blue => "Blue",
        TeamSide::Red => "Red",
    }
}

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
        let killer_name = format!("Player{}", killer_idx + 1);
        let victim_name = format!("Player{}", victim_idx + 1);
        event.set_commentary(Commentary::for_solo_kill(
            &killer_name,
            &victim_name,
            lane,
            is_first_blood,
        ));
        event
    }
}

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

        // Reset fight kills for all participants before the fight
        for i in 0..players_per_side {
            state.players_mut(winner_side)[i].reset_fight_kills();
            state.players_mut(winner_side.opposite())[i].reset_fight_kills();
        }

        // Winning side gets kills on losing side
        for _ in 0..kills_won {
            if let Some(killer_idx) = state.pick_alive_player(rng, winner_side) {
                let gold = rng.range_u32(200, 400);
                state.players_mut(winner_side)[killer_idx].record_fight_kill(gold);
            }
            if let Some(victim_idx) = state.pick_alive_player(rng, winner_side.opposite()) {
                state.players_mut(winner_side.opposite())[victim_idx].record_death(death_timer);
            }
        }

        // Losing side gets some trade kills
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

        // Collect multi-kill and killing spree sub-events (avoid borrow conflict)
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
                        &format!("Player{}", i + 1),
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
                        &format!("Player{}", i + 1),
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
            side_name(winner_side),
            kills_won,
            kills_lost,
        ));
        event
    }
}

// ---------------------------------------------------------------------------
// TowerSiege — destroy the next vulnerable tower
// ---------------------------------------------------------------------------

pub struct TowerSiege;

impl SimEvent for TowerSiege {
    fn name(&self) -> &'static str {
        "TowerSiege"
    }

    fn is_enabled(&self, state: &MatchGameState) -> bool {
        // At least one side must have a vulnerable tower to siege
        state.next_vulnerable_tower(TeamSide::Blue).is_some()
            || state.next_vulnerable_tower(TeamSide::Red).is_some()
    }

    fn weight(&self, state: &MatchGameState) -> f64 {
        match state.phase() {
            MobaMatchPhase::Early => 12.0,
            MobaMatchPhase::Mid => 20.0,
            MobaMatchPhase::Late => 15.0,
        }
    }

    fn process(&self, state: &mut MatchGameState, rng: &mut GameRng) -> MatchEvent {
        let attacker = state.resolve_winner(rng);
        let defender = attacker.opposite();

        // Try the defender side first (attacker pushes into defender's base)
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

            // Gold reward
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
                side_name(actual_attacker),
                lane,
            ));
            event
        } else {
            // Fallback: farm tick if somehow no tower available (shouldn't happen due to is_enabled)
            MatchEvent::new(state.minute, state.phase(), MatchEventKind::FarmTick)
        }
    }
}

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
        // Increase weight if a team is close to soul
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

        // Team-wide gold reward
        for p in state.players_mut(killer).iter_mut() {
            p.add_gold(50);
        }

        // Set dragon respawn cooldown (5 minutes)
        state.dragon_timer = 5;

        let mut event = MatchEvent::new(
            state.minute,
            state.phase(),
            MatchEventKind::DragonKill { killer },
        );
        event.set_commentary(Commentary::for_dragon(side_name(killer), count));
        event
    }
}

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
        event.set_commentary(Commentary::for_herald(side_name(killer)));
        event
    }
}

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

        // Team-wide baron gold
        for p in state.players_mut(killer).iter_mut() {
            p.add_gold(300);
        }

        // Baron respawn cooldown (6 minutes)
        state.baron_spawn_timer = 6;

        let mut event = MatchEvent::new(
            state.minute,
            state.phase(),
            MatchEventKind::BaronKill { killer },
        );
        event.set_commentary(Commentary::for_baron(side_name(killer)));
        event
    }
}

// ---------------------------------------------------------------------------
// InhibSiege — destroy a vulnerable inhibitor
// ---------------------------------------------------------------------------

pub struct InhibSiege;

impl SimEvent for InhibSiege {
    fn name(&self) -> &'static str {
        "InhibSiege"
    }

    fn is_enabled(&self, state: &MatchGameState) -> bool {
        state.vulnerable_inhibitor_lane(TeamSide::Blue).is_some()
            || state.vulnerable_inhibitor_lane(TeamSide::Red).is_some()
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

        // Prefer attacking the defender's inhibitor
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
                side_name(actual_attacker),
                lane,
            ));
            event
        } else {
            MatchEvent::new(state.minute, state.phase(), MatchEventKind::FarmTick)
        }
    }
}

// ---------------------------------------------------------------------------
// NexusSiege — attempt to end the game
// ---------------------------------------------------------------------------

pub struct NexusSiege;

impl SimEvent for NexusSiege {
    fn name(&self) -> &'static str {
        "NexusSiege"
    }

    fn is_enabled(&self, state: &MatchGameState) -> bool {
        state.map.is_nexus_vulnerable(TeamSide::Blue)
            || state.map.is_nexus_vulnerable(TeamSide::Red)
    }

    fn weight(&self, state: &MatchGameState) -> f64 {
        // Nexus siege is very high weight when available — this naturally ends games
        let base = match state.phase() {
            MobaMatchPhase::Early => 0.0,
            MobaMatchPhase::Mid => 10.0,
            MobaMatchPhase::Late => 35.0,
        };
        // Scale up over time to guarantee games end
        let time_pressure = ((state.minute as f64 - 20.0) / 10.0).max(0.0);
        base + time_pressure * 5.0
    }

    fn process(&self, state: &mut MatchGameState, rng: &mut GameRng) -> MatchEvent {
        // Determine which nexus is vulnerable and who is attacking
        let blue_nexus_open = state.map.is_nexus_vulnerable(TeamSide::Blue);
        let red_nexus_open = state.map.is_nexus_vulnerable(TeamSide::Red);

        let (attacker, defender) = if blue_nexus_open && red_nexus_open {
            // Both open — stronger team pushes
            let winner = state.resolve_winner(rng);
            (winner, winner.opposite())
        } else if blue_nexus_open {
            (TeamSide::Red, TeamSide::Blue)
        } else {
            (TeamSide::Blue, TeamSide::Red)
        };

        // The attacker must win a fight to take the nexus.
        // Probability influenced by gold lead, alive count, and game time.
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
            // Nexus falls!
            state.winner = Some(attacker);
            let mut event = MatchEvent::new(
                state.minute,
                state.phase(),
                MatchEventKind::NexusDestroyed { winner: attacker },
            );
            event.set_commentary(Commentary::for_nexus_destroy(side_name(attacker)));
            event
        } else {
            // Failed push — a teamfight occurs instead, defender holds
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
                side_name(defender),
                kills_for_defender,
                0,
            ));
            event
        }
    }
}

// ---------------------------------------------------------------------------
// Event registry: all events in one place
// ---------------------------------------------------------------------------

/// Returns a Vec of all simulation events.
pub fn all_events() -> Vec<Box<dyn SimEvent>> {
    vec![
        Box::new(FarmTick),
        Box::new(SoloKill),
        Box::new(Teamfight),
        Box::new(TowerSiege),
        Box::new(DragonFight),
        Box::new(HeraldFight),
        Box::new(BaronFight),
        Box::new(InhibSiege),
        Box::new(NexusSiege),
    ]
}

/// Given the current game state, return enabled events with their weights.
pub fn enabled_events(state: &MatchGameState, events: &[Box<dyn SimEvent>]) -> Vec<(usize, f64)> {
    events
        .iter()
        .enumerate()
        .filter(|(_, e)| e.is_enabled(state))
        .map(|(i, e)| (i, e.weight(state)))
        .filter(|(_, w)| *w > 0.0)
        .collect()
}

/// Return the tactical weight multiplier for a given event name.
fn tactical_mult(name: &str, tactics: &MatchTactics) -> f64 {
    match name {
        "FarmTick" => tactics.farm_mult(),
        "SoloKill" => tactics.solo_kill_mult(),
        "Teamfight" => tactics.teamfight_mult(),
        "TowerSiege" | "InhibSiege" | "NexusSiege" => tactics.tower_mult(),
        "DragonFight" | "HeraldFight" | "BaronFight" => tactics.objective_mult(),
        _ => 1.0,
    }
}

/// Like `enabled_events` but applies tactical weight multipliers.
pub fn enabled_events_with_tactics(
    state: &MatchGameState,
    events: &[Box<dyn SimEvent>],
    tactics: &MatchTactics,
) -> Vec<(usize, f64)> {
    events
        .iter()
        .enumerate()
        .filter(|(_, e)| e.is_enabled(state))
        .map(|(i, e)| {
            let base = e.weight(state);
            let mult = tactical_mult(e.name(), tactics);
            (i, base * mult)
        })
        .filter(|(_, w)| *w > 0.0)
        .collect()
}

/// Pick an event index from the enabled events using weighted random selection.
pub fn pick_event(rng: &mut GameRng, enabled: &[(usize, f64)]) -> usize {
    let total_weight: f64 = enabled.iter().map(|(_, w)| w).sum();
    if total_weight <= 0.0 {
        return enabled[0].0;
    }

    let roll = rng.range_u32(0, 10000) as f64 / 10000.0 * total_weight;
    let mut cumulative = 0.0;
    for &(idx, weight) in enabled {
        cumulative += weight;
        if roll <= cumulative {
            return idx;
        }
    }
    // Fallback to last
    enabled.last().unwrap().0
}
