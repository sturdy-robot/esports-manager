use esm_core::rng::GameRng;
use serde::{Deserialize, Serialize};

use super::event::{Commentary, MatchEvent, MatchEventKind, MobaMatchPhase};
use super::map::{Lane, MapState, TowerTier};
use super::player_state::MatchPlayerState;
use super::state::TeamSide;

const ALL_LANES: [Lane; 3] = [Lane::Top, Lane::Mid, Lane::Bot];

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobaMatchConfig {
    pub players_per_side: usize,
    pub max_minutes: u32,
}

impl Default for MobaMatchConfig {
    fn default() -> Self {
        Self {
            players_per_side: 5,
            max_minutes: 55,
        }
    }
}

// ---------------------------------------------------------------------------
// Result
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobaMatchResult {
    pub winner: TeamSide,
    pub duration_minutes: u32,
    pub events: Vec<MatchEvent>,
    pub blue_players: Vec<MatchPlayerState>,
    pub red_players: Vec<MatchPlayerState>,
    pub blue_team_gold: u32,
    pub red_team_gold: u32,
}

// ---------------------------------------------------------------------------
// Internal match state
// ---------------------------------------------------------------------------

struct LiveMatch {
    minute: u32,
    map: MapState,
    blue: Vec<MatchPlayerState>,
    red: Vec<MatchPlayerState>,
    events: Vec<MatchEvent>,
    winner: Option<TeamSide>,
    first_blood: bool,
    blue_attrs: Vec<[u8; 9]>,
    red_attrs: Vec<[u8; 9]>,
}

impl LiveMatch {
    fn new(blue_attrs: &[[u8; 9]], red_attrs: &[[u8; 9]], players_per_side: usize) -> Self {
        Self {
            minute: 0,
            map: MapState::new(),
            blue: (0..players_per_side).map(MatchPlayerState::new).collect(),
            red: (0..players_per_side).map(MatchPlayerState::new).collect(),
            events: Vec::new(),
            winner: None,
            first_blood: false,
            blue_attrs: blue_attrs.to_vec(),
            red_attrs: red_attrs.to_vec(),
        }
    }

    fn phase(&self) -> MobaMatchPhase {
        MobaMatchPhase::from_minute(self.minute)
    }

    fn blue_total_gold(&self) -> u32 {
        self.blue.iter().map(|p| p.gold()).sum()
    }

    fn red_total_gold(&self) -> u32 {
        self.red.iter().map(|p| p.gold()).sum()
    }

    fn gold_delta(&self) -> i64 {
        self.blue_total_gold() as i64 - self.red_total_gold() as i64
    }

    fn team_power(&self, side: TeamSide) -> u32 {
        let attrs = match side {
            TeamSide::Blue => &self.blue_attrs,
            TeamSide::Red => &self.red_attrs,
        };
        attrs
            .iter()
            .map(|a| a.iter().map(|&v| v as u32).sum::<u32>())
            .sum()
    }

    fn alive_count(&self, side: TeamSide) -> usize {
        let players = match side {
            TeamSide::Blue => &self.blue,
            TeamSide::Red => &self.red,
        };
        players.iter().filter(|p| !p.is_dead()).count()
    }

    fn pick_alive_player(&self, rng: &mut GameRng, side: TeamSide) -> Option<usize> {
        let players = match side {
            TeamSide::Blue => &self.blue,
            TeamSide::Red => &self.red,
        };
        let alive: Vec<usize> = players
            .iter()
            .enumerate()
            .filter(|(_, p)| !p.is_dead())
            .map(|(i, _)| i)
            .collect();
        if alive.is_empty() {
            return None;
        }
        Some(alive[rng.range_u32(0, alive.len() as u32) as usize])
    }
}

// ---------------------------------------------------------------------------
// Engine
// ---------------------------------------------------------------------------

pub struct MobaMatchEngine;

impl MobaMatchEngine {
    /// Run a full match simulation.
    ///
    /// `blue_attrs` and `red_attrs`: each is a Vec of player attribute arrays.
    /// Each array has 9 values: [endurance, reaction_time, decision_making,
    /// clutch, discipline, tilt_resistance, mechanics, vision_control, teamfighting].
    pub fn simulate(
        rng: &mut GameRng,
        blue_attrs: &[[u8; 9]],
        red_attrs: &[[u8; 9]],
        config: &MobaMatchConfig,
    ) -> MobaMatchResult {
        let mut m = LiveMatch::new(blue_attrs, red_attrs, config.players_per_side);

        while m.winner.is_none() && m.minute < config.max_minutes {
            m.minute += 1;
            let phase = m.phase();

            // Tick death timers
            for p in m.blue.iter_mut().chain(m.red.iter_mut()) {
                p.tick_death_timer(60); // 1 minute = 60 seconds
            }
            // Tick inhibitor respawns
            m.map.tick_inhibitors(60);

            // Passive gold & CS per minute
            Self::passive_income(&mut m, rng);

            // Roll for events based on phase
            let event_prob = match phase {
                MobaMatchPhase::Early => 0.35,
                MobaMatchPhase::Mid => 0.50,
                MobaMatchPhase::Late => 0.65,
            };

            if rng.check_probability(event_prob) {
                Self::resolve_event(rng, &mut m);
            }

            // Objective spawns
            if m.minute == 5 && m.map.objectives().herald_available() {
                // Herald is available from start, first dragon at 5 min
            }
            if m.minute >= 20 && !m.map.objectives().baron_alive() && rng.check_probability(0.15) {
                m.map.objectives_mut().spawn_baron();
            }

            // Late game nexus push pressure
            if phase == MobaMatchPhase::Late && m.winner.is_none() {
                Self::check_nexus_push(rng, &mut m);
            }
        }

        // Timeout: team with gold lead wins
        if m.winner.is_none() {
            let winner = if m.gold_delta() >= 0 {
                TeamSide::Blue
            } else {
                TeamSide::Red
            };
            m.winner = Some(winner);
            let mut event = MatchEvent::new(
                m.minute,
                m.phase(),
                MatchEventKind::NexusDestroyed { winner },
            );
            let name = if winner == TeamSide::Blue {
                "Blue"
            } else {
                "Red"
            };
            event.set_commentary(Commentary::for_nexus_destroy(name));
            m.events.push(event);
        }

        let blue_gold = m.blue_total_gold();
        let red_gold = m.red_total_gold();

        MobaMatchResult {
            winner: m.winner.unwrap(),
            duration_minutes: m.minute,
            events: m.events,
            blue_players: m.blue,
            red_players: m.red,
            blue_team_gold: blue_gold,
            red_team_gold: red_gold,
        }
    }

    fn passive_income(m: &mut LiveMatch, rng: &mut GameRng) {
        for p in m.blue.iter_mut().chain(m.red.iter_mut()) {
            if !p.is_dead() {
                p.add_gold(rng.range_u32(80, 120));
                p.add_cs(rng.range_u32(5, 9));
            }
        }
    }

    fn resolve_winner(rng: &mut GameRng, m: &LiveMatch) -> TeamSide {
        let blue_power = m.team_power(TeamSide::Blue) as f64;
        let red_power = m.team_power(TeamSide::Red) as f64;
        let total = blue_power + red_power;
        if total == 0.0 {
            return if rng.check_probability(0.5) {
                TeamSide::Blue
            } else {
                TeamSide::Red
            };
        }

        let mut blue_prob = blue_power / total;

        // Gold advantage shifts probability
        let gold_shift = m.gold_delta() as f64 / 1000.0 * 0.03;
        blue_prob = (blue_prob + gold_shift).clamp(0.10, 0.90);

        // Alive player advantage
        let blue_alive = m.alive_count(TeamSide::Blue) as f64;
        let red_alive = m.alive_count(TeamSide::Red) as f64;
        if blue_alive + red_alive > 0.0 {
            let alive_shift = (blue_alive - red_alive) * 0.05;
            blue_prob = (blue_prob + alive_shift).clamp(0.10, 0.90);
        }

        // Clutch factor: behind team gets a small bonus from clutch attributes
        if m.gold_delta() < -2000 {
            let clutch_avg: f64 =
                m.blue_attrs.iter().map(|a| a[3] as f64).sum::<f64>() / m.blue_attrs.len() as f64;
            blue_prob += (clutch_avg / 100.0) * 0.05;
        } else if m.gold_delta() > 2000 {
            let clutch_avg: f64 =
                m.red_attrs.iter().map(|a| a[3] as f64).sum::<f64>() / m.red_attrs.len() as f64;
            blue_prob -= (clutch_avg / 100.0) * 0.05;
        }

        blue_prob = blue_prob.clamp(0.10, 0.90);

        if rng.check_probability(blue_prob) {
            TeamSide::Blue
        } else {
            TeamSide::Red
        }
    }

    fn resolve_event(rng: &mut GameRng, m: &mut LiveMatch) {
        let roll = rng.range_u32(0, 100);
        let phase = m.phase();

        match phase {
            MobaMatchPhase::Early => match roll {
                0..=35 => Self::solo_kill_event(rng, m),
                36..=55 => Self::tower_event(rng, m),
                56..=75 => Self::dragon_event(rng, m),
                76..=90 => Self::herald_event(rng, m),
                _ => Self::solo_kill_event(rng, m),
            },
            MobaMatchPhase::Mid => match roll {
                0..=20 => Self::teamfight_event(rng, m),
                21..=40 => Self::tower_event(rng, m),
                41..=60 => Self::dragon_event(rng, m),
                61..=75 => Self::baron_event(rng, m),
                76..=90 => Self::solo_kill_event(rng, m),
                _ => Self::tower_event(rng, m),
            },
            MobaMatchPhase::Late => match roll {
                0..=30 => Self::teamfight_event(rng, m),
                31..=45 => Self::baron_event(rng, m),
                46..=60 => Self::tower_event(rng, m),
                61..=75 => Self::dragon_event(rng, m),
                76..=85 => Self::inhibitor_event(rng, m),
                _ => Self::teamfight_event(rng, m),
            },
        }
    }

    fn solo_kill_event(rng: &mut GameRng, m: &mut LiveMatch) {
        let winner_side = Self::resolve_winner(rng, m);
        let loser_side = winner_side.opposite();

        let killer_idx = match m.pick_alive_player(rng, winner_side) {
            Some(i) => i,
            None => return,
        };
        let victim_idx = match m.pick_alive_player(rng, loser_side) {
            Some(i) => i,
            None => return,
        };

        let lane = ALL_LANES[rng.range_u32(0, 3) as usize];
        let is_first_blood = !m.first_blood;
        if is_first_blood {
            m.first_blood = true;
        }

        let bounty = match loser_side {
            TeamSide::Blue => m.blue[victim_idx].bounty(),
            TeamSide::Red => m.red[victim_idx].bounty(),
        };

        let death_timer = 10 + m.minute; // Death timer scales with game time

        match winner_side {
            TeamSide::Blue => {
                m.blue[killer_idx].record_kill(bounty);
                m.red[victim_idx].record_death(death_timer);
            }
            TeamSide::Red => {
                m.red[killer_idx].record_kill(bounty);
                m.blue[victim_idx].record_death(death_timer);
            }
        }

        let mut event = MatchEvent::new(
            m.minute,
            m.phase(),
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
        m.events.push(event);
    }

    fn teamfight_event(rng: &mut GameRng, m: &mut LiveMatch) {
        let winner_side = Self::resolve_winner(rng, m);
        let kills_won = rng.range_u32(1, 5);
        let kills_lost = rng.range_u32(0, 3);

        let death_timer = 15 + m.minute;

        // Apply kills to winning side players
        for _ in 0..kills_won {
            if let Some(killer_idx) = m.pick_alive_player(rng, winner_side) {
                let gold = rng.range_u32(200, 400);
                match winner_side {
                    TeamSide::Blue => m.blue[killer_idx].record_kill(gold),
                    TeamSide::Red => m.red[killer_idx].record_kill(gold),
                }
            }
            if let Some(victim_idx) = m.pick_alive_player(rng, winner_side.opposite()) {
                match winner_side.opposite() {
                    TeamSide::Blue => m.blue[victim_idx].record_death(death_timer),
                    TeamSide::Red => m.red[victim_idx].record_death(death_timer),
                }
            }
        }

        // Apply deaths to winning side (they lost some too)
        for _ in 0..kills_lost {
            if let Some(victim_idx) = m.pick_alive_player(rng, winner_side) {
                match winner_side {
                    TeamSide::Blue => m.blue[victim_idx].record_death(death_timer),
                    TeamSide::Red => m.red[victim_idx].record_death(death_timer),
                }
            }
            if let Some(killer_idx) = m.pick_alive_player(rng, winner_side.opposite()) {
                let gold = rng.range_u32(200, 400);
                match winner_side.opposite() {
                    TeamSide::Blue => m.blue[killer_idx].record_kill(gold),
                    TeamSide::Red => m.red[killer_idx].record_kill(gold),
                }
            }
        }

        let (kb, kr) = match winner_side {
            TeamSide::Blue => (kills_won, kills_lost),
            TeamSide::Red => (kills_lost, kills_won),
        };

        let mut event = MatchEvent::new(
            m.minute,
            m.phase(),
            MatchEventKind::Teamfight {
                winning_side: winner_side,
                kills_blue: kb,
                kills_red: kr,
            },
        );
        let team_name = if winner_side == TeamSide::Blue {
            "Blue"
        } else {
            "Red"
        };
        event.set_commentary(Commentary::for_teamfight(team_name, kills_won, kills_lost));
        m.events.push(event);

        if !m.first_blood && (kills_won > 0 || kills_lost > 0) {
            m.first_blood = true;
        }
    }

    fn tower_event(rng: &mut GameRng, m: &mut LiveMatch) {
        let attacker = Self::resolve_winner(rng, m);
        let defender = attacker.opposite();
        let lane = ALL_LANES[rng.range_u32(0, 3) as usize];

        // Find the first vulnerable tower in this lane
        let tiers = [TowerTier::Outer, TowerTier::Inner, TowerTier::Inhibitor];
        let target_tier = tiers
            .iter()
            .find(|&&tier| m.map.is_tower_vulnerable(defender, lane, tier));

        if let Some(&tier) = target_tier {
            m.map.destroy_tower(defender, lane, tier);

            // Gold reward to attacker
            let gold = 550;
            if let Some(idx) = m.pick_alive_player(rng, attacker) {
                match attacker {
                    TeamSide::Blue => m.blue[idx].add_gold(gold),
                    TeamSide::Red => m.red[idx].add_gold(gold),
                }
            }

            let mut event = MatchEvent::new(
                m.minute,
                m.phase(),
                MatchEventKind::TowerDestroyed { attacker, lane },
            );
            let team_name = if attacker == TeamSide::Blue {
                "Blue"
            } else {
                "Red"
            };
            event.set_commentary(Commentary::for_tower_destroy(team_name, lane));
            m.events.push(event);
        }
    }

    fn dragon_event(rng: &mut GameRng, m: &mut LiveMatch) {
        let killer = Self::resolve_winner(rng, m);
        m.map.objectives_mut().add_dragon(killer);
        let count = m.map.objectives().dragon_count(killer);

        // Gold reward to all team members
        let players = if killer == TeamSide::Blue {
            &mut m.blue
        } else {
            &mut m.red
        };
        for p in players.iter_mut() {
            p.add_gold(50);
        }

        let mut event = MatchEvent::new(m.minute, m.phase(), MatchEventKind::DragonKill { killer });
        let team_name = if killer == TeamSide::Blue {
            "Blue"
        } else {
            "Red"
        };
        event.set_commentary(Commentary::for_dragon(team_name, count));
        m.events.push(event);
    }

    fn herald_event(rng: &mut GameRng, m: &mut LiveMatch) {
        if !m.map.objectives().herald_available() {
            return;
        }
        let killer = Self::resolve_winner(rng, m);
        m.map.objectives_mut().consume_herald();

        let mut event = MatchEvent::new(m.minute, m.phase(), MatchEventKind::HeraldKill { killer });
        let team_name = if killer == TeamSide::Blue {
            "Blue"
        } else {
            "Red"
        };
        event.set_commentary(Commentary::for_herald(team_name));
        m.events.push(event);
    }

    fn baron_event(rng: &mut GameRng, m: &mut LiveMatch) {
        if m.minute < 20 {
            return;
        }
        let killer = Self::resolve_winner(rng, m);
        if m.map.objectives().baron_alive() {
            m.map.objectives_mut().kill_baron(killer);
        } else {
            m.map.objectives_mut().spawn_baron();
            m.map.objectives_mut().kill_baron(killer);
        }

        // Baron buff gold to all team members
        let players = if killer == TeamSide::Blue {
            &mut m.blue
        } else {
            &mut m.red
        };
        for p in players.iter_mut() {
            p.add_gold(300);
        }

        let mut event = MatchEvent::new(m.minute, m.phase(), MatchEventKind::BaronKill { killer });
        let team_name = if killer == TeamSide::Blue {
            "Blue"
        } else {
            "Red"
        };
        event.set_commentary(Commentary::for_baron(team_name));
        m.events.push(event);
    }

    fn inhibitor_event(rng: &mut GameRng, m: &mut LiveMatch) {
        let attacker = Self::resolve_winner(rng, m);
        let defender = attacker.opposite();
        let lane = ALL_LANES[rng.range_u32(0, 3) as usize];

        if m.map.is_inhibitor_vulnerable(defender, lane) {
            m.map.destroy_inhibitor(defender, lane, 300);

            let mut event = MatchEvent::new(
                m.minute,
                m.phase(),
                MatchEventKind::InhibitorDestroyed { attacker, lane },
            );
            let team_name = if attacker == TeamSide::Blue {
                "Blue"
            } else {
                "Red"
            };
            event.set_commentary(Commentary::for_inhibitor_destroy(team_name, lane));
            m.events.push(event);
        }
    }

    fn check_nexus_push(rng: &mut GameRng, m: &mut LiveMatch) {
        for side in [TeamSide::Blue, TeamSide::Red] {
            let defender = side;
            let attacker = side.opposite();

            if m.map.is_nexus_vulnerable(defender) {
                let nexus_prob = 0.04 * (m.minute as f64 - 24.0) / 10.0;
                let gold_bonus = (m.gold_delta().abs() as f64) / 20_000.0;
                let attacker_is_blue = attacker == TeamSide::Blue;
                let gold_dir = if attacker_is_blue { 1.0 } else { -1.0 };
                let gold_advantage = m.gold_delta() as f64 * gold_dir;
                let advantage_bonus = if gold_advantage > 0.0 {
                    gold_bonus
                } else {
                    0.0
                };

                let total_prob = (nexus_prob + advantage_bonus).min(0.4);

                if rng.check_probability(total_prob) {
                    m.winner = Some(attacker);
                    let mut event = MatchEvent::new(
                        m.minute,
                        m.phase(),
                        MatchEventKind::NexusDestroyed { winner: attacker },
                    );
                    let team_name = if attacker == TeamSide::Blue {
                        "Blue"
                    } else {
                        "Red"
                    };
                    event.set_commentary(Commentary::for_nexus_destroy(team_name));
                    m.events.push(event);
                    return;
                }
            }
        }
    }
}
