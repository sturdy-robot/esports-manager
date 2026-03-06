use serde::{Deserialize, Serialize};

use crate::activity::DailySchedule;

// ---------------------------------------------------------------------------
// Value Objects
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BoundedAttribute {
    value: u8,
}

impl BoundedAttribute {
    const MIN: u8 = 0;
    const MAX: u8 = 100;

    pub fn new(value: u8) -> Self {
        Self {
            value: value.min(Self::MAX),
        }
    }

    pub fn value(self) -> u8 {
        self.value
    }

    pub fn increase(&mut self, amount: u8) {
        self.value = self.value.saturating_add(amount).min(Self::MAX);
    }

    pub fn decrease(&mut self, amount: u8) {
        self.value = self.value.saturating_sub(amount).max(Self::MIN);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Confidence {
    Slumping,
    Neutral,
    Confident,
    Hyped,
}

impl Default for Confidence {
    fn default() -> Self {
        Self::Neutral
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    Top,
    Jungle,
    Mid,
    Bot,
    Support,
}

// ---------------------------------------------------------------------------
// Attribute Groups
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalAttributes {
    pub endurance: BoundedAttribute,
    pub reaction_time: BoundedAttribute,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentalAttributes {
    pub decision_making: BoundedAttribute,
    pub clutch: BoundedAttribute,
    pub discipline: BoundedAttribute,
    pub tilt_resistance: BoundedAttribute,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalAttributes {
    pub mechanics: BoundedAttribute,
    pub vision_control: BoundedAttribute,
    pub teamfighting: BoundedAttribute,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAttributes {
    pub physical: PhysicalAttributes,
    pub mental: MentalAttributes,
    pub technical: TechnicalAttributes,
}

// ---------------------------------------------------------------------------
// Dynamic State
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub stamina: BoundedAttribute,
    pub morale: BoundedAttribute,
    pub confidence: Confidence,
    pub satisfaction: BoundedAttribute,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            stamina: BoundedAttribute::new(100),
            morale: BoundedAttribute::new(50),
            confidence: Confidence::default(),
            satisfaction: BoundedAttribute::new(50),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerTalk {
    /// "We've got this!" — boosts morale, slight stamina cost
    Motivate,
    /// "Stay calm, play your game" — reduces tilt, restores confidence
    Calm,
    /// "Let's focus on objectives" — boosts satisfaction, slight morale
    Strategize,
    /// "Take a breather" — restores stamina at the cost of morale momentum
    Rest,
}

impl PlayerState {
    /// Apply a manager talk to this player between games.
    /// Each talk type has different effects on morale, stamina, confidence, satisfaction.
    pub fn apply_talk(&mut self, talk: PlayerTalk) {
        match talk {
            PlayerTalk::Motivate => {
                self.morale.increase(8);
                self.stamina.decrease(3);
                if self.confidence == Confidence::Slumping {
                    self.confidence = Confidence::Neutral;
                }
            }
            PlayerTalk::Calm => {
                self.morale.increase(3);
                self.stamina.increase(2);
                // Restore one confidence level if slumping
                if self.confidence == Confidence::Slumping {
                    self.confidence = Confidence::Neutral;
                }
            }
            PlayerTalk::Strategize => {
                self.satisfaction.increase(5);
                self.morale.increase(2);
            }
            PlayerTalk::Rest => {
                self.stamina.increase(8);
                self.morale.decrease(2);
            }
        }
    }

    pub fn apply_match_result(&mut self, is_win: bool) {
        if is_win {
            self.morale.increase(10);
            self.confidence = match self.confidence {
                Confidence::Slumping => Confidence::Neutral,
                Confidence::Neutral => Confidence::Confident,
                Confidence::Confident => Confidence::Hyped,
                Confidence::Hyped => Confidence::Hyped,
            };
        } else {
            self.morale.decrease(10);
            self.confidence = match self.confidence {
                Confidence::Hyped => Confidence::Confident,
                Confidence::Confident => Confidence::Neutral,
                Confidence::Neutral => Confidence::Slumping,
                Confidence::Slumping => Confidence::Slumping,
            };
        }
    }
}

// ---------------------------------------------------------------------------
// Player Entity
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    nickname: String,
    first_name: String,
    last_name: String,
    role: Role,
    attributes: PlayerAttributes,
    state: PlayerState,
    schedule: DailySchedule,
}

impl Player {
    pub fn new(
        nickname: String,
        first_name: String,
        last_name: String,
        role: Role,
        attributes: PlayerAttributes,
    ) -> Self {
        Self {
            nickname,
            first_name,
            last_name,
            role,
            attributes,
            state: PlayerState::default(),
            schedule: DailySchedule::default(),
        }
    }

    pub fn nickname(&self) -> &str {
        &self.nickname
    }

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    pub fn role(&self) -> Role {
        self.role
    }

    pub fn attributes(&self) -> &PlayerAttributes {
        &self.attributes
    }

    pub fn state(&self) -> &PlayerState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut PlayerState {
        &mut self.state
    }

    pub fn schedule(&self) -> &DailySchedule {
        &self.schedule
    }

    pub fn schedule_mut(&mut self) -> &mut DailySchedule {
        &mut self.schedule
    }
}
