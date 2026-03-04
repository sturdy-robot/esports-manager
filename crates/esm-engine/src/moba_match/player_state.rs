use serde::{Deserialize, Serialize};

const STARTING_GOLD: u32 = 500;
const BASE_BOUNTY: u32 = 300;
const BOUNTY_PER_STREAK: u32 = 150;

/// Per-player in-match state: gold, farm, KDA, streaks, death timer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchPlayerState {
    player_index: usize,
    gold: u32,
    cs: u32,
    kills: u32,
    deaths: u32,
    assists: u32,
    kill_streak: u32,
    death_streak: u32,
    death_timer: u32,
}

impl MatchPlayerState {
    pub fn new(player_index: usize) -> Self {
        Self {
            player_index,
            gold: STARTING_GOLD,
            cs: 0,
            kills: 0,
            deaths: 0,
            assists: 0,
            kill_streak: 0,
            death_streak: 0,
            death_timer: 0,
        }
    }

    pub fn player_index(&self) -> usize {
        self.player_index
    }

    pub fn gold(&self) -> u32 {
        self.gold
    }

    pub fn cs(&self) -> u32 {
        self.cs
    }

    pub fn kills(&self) -> u32 {
        self.kills
    }

    pub fn deaths(&self) -> u32 {
        self.deaths
    }

    pub fn assists(&self) -> u32 {
        self.assists
    }

    pub fn kill_streak(&self) -> u32 {
        self.kill_streak
    }

    pub fn death_streak(&self) -> u32 {
        self.death_streak
    }

    pub fn death_timer(&self) -> u32 {
        self.death_timer
    }

    pub fn is_dead(&self) -> bool {
        self.death_timer > 0
    }

    pub fn add_gold(&mut self, amount: u32) {
        self.gold += amount;
    }

    pub fn add_cs(&mut self, amount: u32) {
        self.cs += amount;
    }

    pub fn record_kill(&mut self, gold_reward: u32) {
        self.kills += 1;
        self.gold += gold_reward;
        self.kill_streak += 1;
        self.death_streak = 0;
    }

    pub fn record_death(&mut self, death_timer_seconds: u32) {
        self.deaths += 1;
        self.kill_streak = 0;
        self.death_streak += 1;
        self.death_timer = death_timer_seconds;
    }

    pub fn record_assist(&mut self, gold_reward: u32) {
        self.assists += 1;
        self.gold += gold_reward;
    }

    pub fn tick_death_timer(&mut self, seconds: u32) {
        self.death_timer = self.death_timer.saturating_sub(seconds);
    }

    /// KDA ratio: (kills + assists) / max(deaths, 1).
    pub fn kda(&self) -> f64 {
        let ka = (self.kills + self.assists) as f64;
        let d = self.deaths.max(1) as f64;
        if self.kills == 0 && self.assists == 0 {
            return 0.0;
        }
        ka / d
    }

    /// Bounty gold awarded to the killer. Increases with kill streak.
    pub fn bounty(&self) -> u32 {
        BASE_BOUNTY + self.kill_streak * BOUNTY_PER_STREAK
    }
}
