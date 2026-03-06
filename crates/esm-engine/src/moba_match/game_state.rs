use esm_core::rng::GameRng;
use serde::{Deserialize, Serialize};

use super::event::{MatchEvent, MobaMatchPhase};
use super::map::{Lane, MapState, TowerTier};
use super::player_state::MatchPlayerState;
use super::state::TeamSide;

const ALL_LANES: [Lane; 3] = [Lane::Top, Lane::Mid, Lane::Bot];
const DRAGON_SPAWN_MINUTE: u32 = 5;
const HERALD_DESPAWN_MINUTE: u32 = 20;
const BARON_SPAWN_MINUTE: u32 = 20;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchPlayerSimulationData {
    pub attributes: [u8; 9],
    pub stamina: u8,
    pub morale: u8,
    pub mastery_multiplier: f64,
}

impl MatchPlayerSimulationData {
    /// Compute effective power including stamina penalty, morale boost, and mastery.
    pub fn effective_power(&self) -> f64 {
        let base_power: f64 = self.attributes.iter().map(|&v| v as f64).sum();
        
        let stamina_factor = (self.stamina as f64) / 100.0;
        let morale_factor = 0.8 + ((self.morale as f64) / 100.0) * 0.4; // 0.8 at 0 morale, 1.2 at 100
        
        base_power * stamina_factor * morale_factor * self.mastery_multiplier
    }
}

/// The full mutable game state that events operate on.
/// This is the single source of truth during a match simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchGameState {
    pub minute: u32,
    pub map: MapState,
    pub blue_players: Vec<MatchPlayerState>,
    pub red_players: Vec<MatchPlayerState>,
    pub blue_sim_data: Vec<MatchPlayerSimulationData>,
    pub red_sim_data: Vec<MatchPlayerSimulationData>,
    pub events: Vec<MatchEvent>,
    pub winner: Option<TeamSide>,
    pub first_blood_claimed: bool,
    pub baron_spawn_timer: u32,
    pub dragon_timer: u32,
}

impl MatchGameState {
    pub fn new(blue_sim_data: Vec<MatchPlayerSimulationData>, red_sim_data: Vec<MatchPlayerSimulationData>, players_per_side: usize) -> Self {
        Self {
            minute: 0,
            map: MapState::new(),
            blue_players: (0..players_per_side).map(MatchPlayerState::new).collect(),
            red_players: (0..players_per_side).map(MatchPlayerState::new).collect(),
            blue_sim_data,
            red_sim_data,
            events: Vec::new(),
            winner: None,
            first_blood_claimed: false,
            baron_spawn_timer: 0,
            dragon_timer: 0,
        }
    }

    pub fn is_over(&self) -> bool {
        self.winner.is_some()
    }

    pub fn phase(&self) -> MobaMatchPhase {
        MobaMatchPhase::from_minute(self.minute)
    }

    pub fn blue_total_gold(&self) -> u32 {
        self.blue_players.iter().map(|p| p.gold()).sum()
    }

    pub fn red_total_gold(&self) -> u32 {
        self.red_players.iter().map(|p| p.gold()).sum()
    }

    pub fn gold_delta(&self) -> i64 {
        self.blue_total_gold() as i64 - self.red_total_gold() as i64
    }

    pub fn team_power(&self, side: TeamSide) -> f64 {
        let sim_data = match side {
            TeamSide::Blue => &self.blue_sim_data,
            TeamSide::Red => &self.red_sim_data,
        };
        sim_data.iter().map(|d| d.effective_power()).sum()
    }

    pub fn alive_count(&self, side: TeamSide) -> usize {
        let players = match side {
            TeamSide::Blue => &self.blue_players,
            TeamSide::Red => &self.red_players,
        };
        players.iter().filter(|p| !p.is_dead()).count()
    }

    pub fn players(&self, side: TeamSide) -> &[MatchPlayerState] {
        match side {
            TeamSide::Blue => &self.blue_players,
            TeamSide::Red => &self.red_players,
        }
    }

    pub fn players_mut(&mut self, side: TeamSide) -> &mut [MatchPlayerState] {
        match side {
            TeamSide::Blue => &mut self.blue_players,
            TeamSide::Red => &mut self.red_players,
        }
    }

    pub fn pick_alive_player(&self, rng: &mut GameRng, side: TeamSide) -> Option<usize> {
        let players = self.players(side);
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

    /// Resolve which side wins a contest based on team power, gold, alive count, and clutch.
    pub fn resolve_winner(&self, rng: &mut GameRng) -> TeamSide {
        let blue_power = self.team_power(TeamSide::Blue);
        let red_power = self.team_power(TeamSide::Red);
        let total = blue_power + red_power;
        if total == 0.0 {
            return if rng.check_probability(0.5) {
                TeamSide::Blue
            } else {
                TeamSide::Red
            };
        }

        let mut blue_prob = blue_power / total;

        // Gold advantage: every 1000 gold shifts ~3%
        let gold_shift = self.gold_delta() as f64 / 1000.0 * 0.03;
        blue_prob += gold_shift;

        // Alive advantage
        let blue_alive = self.alive_count(TeamSide::Blue) as f64;
        let red_alive = self.alive_count(TeamSide::Red) as f64;
        if blue_alive + red_alive > 0.0 {
            blue_prob += (blue_alive - red_alive) * 0.05;
        }

        // Clutch factor for the behind team (index 3 = clutch attribute)
        if self.gold_delta() < -2000 {
            let clutch_avg: f64 = self.blue_sim_data.iter().map(|d| d.attributes[3] as f64).sum::<f64>()
                / self.blue_sim_data.len().max(1) as f64;
            blue_prob += (clutch_avg / 100.0) * 0.05;
        } else if self.gold_delta() > 2000 {
            let clutch_avg: f64 = self.red_sim_data.iter().map(|d| d.attributes[3] as f64).sum::<f64>()
                / self.red_sim_data.len().max(1) as f64;
            blue_prob -= (clutch_avg / 100.0) * 0.05;
        }

        blue_prob = blue_prob.clamp(0.10, 0.90);

        if rng.check_probability(blue_prob) {
            TeamSide::Blue
        } else {
            TeamSide::Red
        }
    }

    /// Advance the game clock by one minute. Tick death timers, inhibitor respawns.
    pub fn tick_minute(&mut self) {
        self.minute += 1;
        for p in self
            .blue_players
            .iter_mut()
            .chain(self.red_players.iter_mut())
        {
            p.tick_death_timer(60);
        }
        self.map.tick_inhibitors(60);

        // Baron respawn timer
        if self.baron_spawn_timer > 0 {
            self.baron_spawn_timer = self.baron_spawn_timer.saturating_sub(1);
            if self.baron_spawn_timer == 0 && self.minute >= BARON_SPAWN_MINUTE {
                self.map.objectives_mut().spawn_baron();
            }
        }

        // Dragon respawn timer
        if self.dragon_timer > 0 {
            self.dragon_timer = self.dragon_timer.saturating_sub(1);
        }
    }

    /// Check if dragon is available to contest.
    pub fn dragon_available(&self) -> bool {
        self.minute >= DRAGON_SPAWN_MINUTE
            && self.dragon_timer == 0
            && !self.map.objectives().has_dragon_soul(TeamSide::Blue)
            && !self.map.objectives().has_dragon_soul(TeamSide::Red)
    }

    /// Check if elder dragon is available.
    pub fn elder_available(&self) -> bool {
        self.map.objectives().elder_available() && self.dragon_timer == 0
    }

    /// Check if herald is available.
    pub fn herald_available(&self) -> bool {
        self.minute >= 8
            && self.minute < HERALD_DESPAWN_MINUTE
            && self.map.objectives().herald_available()
    }

    /// Check if baron is alive and contestable.
    pub fn baron_available(&self) -> bool {
        self.minute >= BARON_SPAWN_MINUTE && self.map.objectives().baron_alive()
    }

    /// Find the next vulnerable tower for an attacker to siege against a defender.
    pub fn next_vulnerable_tower(&self, defender: TeamSide) -> Option<(Lane, TowerTier)> {
        let tiers = [TowerTier::Outer, TowerTier::Inner, TowerTier::Inhibitor];
        for &lane in &ALL_LANES {
            for &tier in &tiers {
                if self.map.is_tower_vulnerable(defender, lane, tier) {
                    return Some((lane, tier));
                }
            }
        }
        None
    }

    /// Find a lane where the inhibitor is vulnerable.
    pub fn vulnerable_inhibitor_lane(&self, defender: TeamSide) -> Option<Lane> {
        ALL_LANES
            .iter()
            .find(|&&lane| self.map.is_inhibitor_vulnerable(defender, lane))
            .copied()
    }

    /// Check if any nexus tower on the defender's side can be targeted.
    pub fn nexus_towers_standing(&self, defender: TeamSide) -> bool {
        self.map
            .towers(defender)
            .iter()
            .any(|t| t.tier() == TowerTier::Nexus && t.is_standing())
    }

    /// Record an event and push it to history.
    pub fn record_event(&mut self, event: MatchEvent) {
        self.events.push(event);
    }
}
