use esm_core::rng::GameRng;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchPhase {
    Early,
    Mid,
    Late,
}

impl MatchPhase {
    pub fn from_minute(minute: u32) -> Self {
        match minute {
            0..=14 => MatchPhase::Early,
            15..=24 => MatchPhase::Mid,
            _ => MatchPhase::Late,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeamSide {
    Blue,
    Red,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchEventKind {
    SoloKill,
    TowerPlates,
    Dragon,
    RiftHerald,
    Tower,
    Baron,
    Teamfight,
    Inhibitor,
    Elder,
    Nexus,
}

// ---------------------------------------------------------------------------
// MatchEvent
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchEvent {
    pub minute: u32,
    pub kind: MatchEventKind,
    pub winner: TeamSide,
    pub gold_reward: u32,
}

// ---------------------------------------------------------------------------
// MatchState
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchState {
    minute: u32,
    blue_gold: i64,
    red_gold: i64,
    winner: Option<TeamSide>,
}

impl MatchState {
    pub fn new(blue_gold: i64, red_gold: i64) -> Self {
        Self {
            minute: 0,
            blue_gold,
            red_gold,
            winner: None,
        }
    }

    pub fn minute(&self) -> u32 {
        self.minute
    }

    pub fn phase(&self) -> MatchPhase {
        MatchPhase::from_minute(self.minute)
    }

    pub fn blue_gold(&self) -> i64 {
        self.blue_gold
    }

    pub fn red_gold(&self) -> i64 {
        self.red_gold
    }

    pub fn gold_delta(&self) -> i64 {
        self.blue_gold - self.red_gold
    }

    pub fn advance_minute(&mut self) {
        self.minute += 1;
    }

    pub fn is_over(&self) -> bool {
        self.winner.is_some()
    }

    pub fn winner(&self) -> Option<TeamSide> {
        self.winner
    }

    pub fn set_winner(&mut self, side: TeamSide) {
        self.winner = Some(side);
    }

    fn award_gold(&mut self, side: TeamSide, amount: u32) {
        match side {
            TeamSide::Blue => self.blue_gold += amount as i64,
            TeamSide::Red => self.red_gold += amount as i64,
        }
    }
}

// ---------------------------------------------------------------------------
// MatchResult
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub winner: TeamSide,
    pub duration_minutes: u32,
    pub events: Vec<MatchEvent>,
    pub blue_gold: i64,
    pub red_gold: i64,
}

// ---------------------------------------------------------------------------
// MatchSimulator
// ---------------------------------------------------------------------------

/// Maximum match length to prevent infinite loops.
const MAX_MATCH_MINUTES: u32 = 55;

/// Event probability per minute — chance that *any* event fires.
const EVENT_PROBABILITY_EARLY: f64 = 0.35;
const EVENT_PROBABILITY_MID: f64 = 0.50;
const EVENT_PROBABILITY_LATE: f64 = 0.65;

/// Nexus probability per minute in late game, scaling with time.
const NEXUS_BASE_PROBABILITY: f64 = 0.04;

pub struct MatchSimulator;

impl MatchSimulator {
    /// Run a full match simulation using the provided RNG and composite team power ratings.
    ///
    /// `blue_power` and `red_power` are abstract composite scores derived from
    /// player attributes, draft quality, synergy, etc. Higher power = better odds.
    pub fn simulate(rng: &mut GameRng, blue_power: u32, red_power: u32) -> MatchResult {
        let mut state = MatchState::new(500, 500); // Starting gold
        let mut events = Vec::new();

        while !state.is_over() && state.minute() < MAX_MATCH_MINUTES {
            state.advance_minute();

            let event_prob = match state.phase() {
                MatchPhase::Early => EVENT_PROBABILITY_EARLY,
                MatchPhase::Mid => EVENT_PROBABILITY_MID,
                MatchPhase::Late => EVENT_PROBABILITY_LATE,
            };

            if rng.check_probability(event_prob) {
                let kind = Self::pick_event_kind(rng, &state);
                let winner = Self::resolve_event_winner(rng, blue_power, red_power, &state);
                let gold_reward = Self::gold_for_event(kind);

                state.award_gold(winner, gold_reward);

                events.push(MatchEvent {
                    minute: state.minute(),
                    kind,
                    winner,
                    gold_reward,
                });

                // Check for Nexus (game-ending)
                if kind == MatchEventKind::Nexus {
                    state.set_winner(winner);
                }
            }

            // In late game, check for Nexus push even without a regular event
            if !state.is_over() && state.phase() == MatchPhase::Late {
                let nexus_prob =
                    NEXUS_BASE_PROBABILITY * (state.minute() as f64 - 24.0) / 10.0;
                let gold_advantage_bonus = (state.gold_delta().abs() as f64) / 20_000.0;
                let total_nexus_prob = (nexus_prob + gold_advantage_bonus).min(0.5);

                if rng.check_probability(total_nexus_prob) {
                    let winner =
                        Self::resolve_event_winner(rng, blue_power, red_power, &state);
                    let gold_reward = Self::gold_for_event(MatchEventKind::Nexus);
                    state.award_gold(winner, gold_reward);
                    state.set_winner(winner);

                    events.push(MatchEvent {
                        minute: state.minute(),
                        kind: MatchEventKind::Nexus,
                        winner,
                        gold_reward,
                    });
                }
            }
        }

        // If match hits time limit, decide winner by gold lead
        if !state.is_over() {
            let winner = if state.gold_delta() >= 0 {
                TeamSide::Blue
            } else {
                TeamSide::Red
            };
            state.set_winner(winner);
            events.push(MatchEvent {
                minute: state.minute(),
                kind: MatchEventKind::Nexus,
                winner,
                gold_reward: 0,
            });
        }

        MatchResult {
            winner: state.winner().unwrap(),
            duration_minutes: state.minute(),
            events,
            blue_gold: state.blue_gold(),
            red_gold: state.red_gold(),
        }
    }

    fn pick_event_kind(rng: &mut GameRng, state: &MatchState) -> MatchEventKind {
        let roll = rng.range_u32(0, 100);
        match state.phase() {
            MatchPhase::Early => match roll {
                0..=29 => MatchEventKind::SoloKill,
                30..=49 => MatchEventKind::TowerPlates,
                50..=69 => MatchEventKind::Dragon,
                70..=84 => MatchEventKind::RiftHerald,
                _ => MatchEventKind::SoloKill,
            },
            MatchPhase::Mid => match roll {
                0..=19 => MatchEventKind::Tower,
                20..=39 => MatchEventKind::Dragon,
                40..=54 => MatchEventKind::Teamfight,
                55..=69 => MatchEventKind::Baron,
                70..=84 => MatchEventKind::SoloKill,
                _ => MatchEventKind::Tower,
            },
            MatchPhase::Late => match roll {
                0..=24 => MatchEventKind::Teamfight,
                25..=39 => MatchEventKind::Baron,
                40..=54 => MatchEventKind::Elder,
                55..=69 => MatchEventKind::Inhibitor,
                70..=79 => MatchEventKind::Tower,
                80..=89 => MatchEventKind::Nexus,
                _ => MatchEventKind::Teamfight,
            },
        }
    }

    fn resolve_event_winner(
        rng: &mut GameRng,
        blue_power: u32,
        red_power: u32,
        state: &MatchState,
    ) -> TeamSide {
        let total = blue_power as f64 + red_power as f64;
        if total == 0.0 {
            return if rng.check_probability(0.5) {
                TeamSide::Blue
            } else {
                TeamSide::Red
            };
        }

        // Base probability from power ratio
        let mut blue_prob = blue_power as f64 / total;

        // Gold delta influence: every 1000 gold shifts probability by ~3%
        let gold_shift = state.gold_delta() as f64 / 1000.0 * 0.03;
        blue_prob = (blue_prob + gold_shift).clamp(0.05, 0.95);

        if rng.check_probability(blue_prob) {
            TeamSide::Blue
        } else {
            TeamSide::Red
        }
    }

    fn gold_for_event(kind: MatchEventKind) -> u32 {
        match kind {
            MatchEventKind::SoloKill => 300,
            MatchEventKind::TowerPlates => 160,
            MatchEventKind::Dragon => 200,
            MatchEventKind::RiftHerald => 200,
            MatchEventKind::Tower => 550,
            MatchEventKind::Baron => 1500,
            MatchEventKind::Teamfight => 800,
            MatchEventKind::Inhibitor => 400,
            MatchEventKind::Elder => 500,
            MatchEventKind::Nexus => 0,
        }
    }
}
