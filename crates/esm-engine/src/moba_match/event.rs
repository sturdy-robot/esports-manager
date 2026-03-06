use serde::{Deserialize, Serialize};

use super::map::Lane;
use super::state::TeamSide;

// ---------------------------------------------------------------------------
// MobaMatchPhase
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MobaMatchPhase {
    Early,
    Mid,
    Late,
}

impl MobaMatchPhase {
    pub fn from_minute(minute: u32) -> Self {
        match minute {
            0..=14 => MobaMatchPhase::Early,
            15..=24 => MobaMatchPhase::Mid,
            _ => MobaMatchPhase::Late,
        }
    }
}

// ---------------------------------------------------------------------------
// MatchEventKind
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchEventKind {
    FarmTick,
    SoloKill {
        killer_idx: usize,
        victim_idx: usize,
        lane: Lane,
    },
    Teamfight {
        winning_side: TeamSide,
        kills_blue: u32,
        kills_red: u32,
    },
    TowerDestroyed {
        attacker: TeamSide,
        lane: Lane,
    },
    DragonKill {
        killer: TeamSide,
    },
    HeraldKill {
        killer: TeamSide,
    },
    BaronKill {
        killer: TeamSide,
    },
    InhibitorDestroyed {
        attacker: TeamSide,
        lane: Lane,
    },
    NexusDestroyed {
        winner: TeamSide,
    },
    MultiKill {
        side: TeamSide,
        player_idx: usize,
        tier: String,
    },
    KillingSpree {
        side: TeamSide,
        player_idx: usize,
        label: String,
    },
}

// ---------------------------------------------------------------------------
// Commentary
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commentary {
    text: String,
}

impl Commentary {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn for_solo_kill(killer: &str, victim: &str, lane: Lane, is_first_blood: bool) -> Self {
        let lane_name = lane_str(lane);
        if is_first_blood {
            Self::new(format!(
                "FIRST BLOOD! {killer} takes down {victim} in the {lane_name} lane!"
            ))
        } else {
            Self::new(format!(
                "{killer} finds the solo kill onto {victim} in {lane_name}!"
            ))
        }
    }

    pub fn for_dragon(team_name: &str, dragon_number: u32) -> Self {
        match dragon_number {
            4 => Self::new(format!(
                "{team_name} secures the Dragon Soul! This could be the turning point!"
            )),
            _ => Self::new(format!(
                "{team_name} slays the dragon! That's dragon number {dragon_number} for them."
            )),
        }
    }

    pub fn for_baron(team_name: &str) -> Self {
        Self::new(format!(
            "BARON NASHOR IS DOWN! {team_name} secures the baron buff — this is huge!"
        ))
    }

    pub fn for_tower_destroy(team_name: &str, lane: Lane) -> Self {
        let lane_name = lane_str(lane);
        Self::new(format!(
            "{team_name} takes down the {lane_name} tower! The map opens up."
        ))
    }

    pub fn for_inhibitor_destroy(team_name: &str, lane: Lane) -> Self {
        let lane_name = lane_str(lane);
        Self::new(format!(
            "{team_name} destroys the {lane_name} inhibitor! Super minions incoming!"
        ))
    }

    pub fn for_nexus_destroy(winner_name: &str) -> Self {
        Self::new(format!(
            "AND THAT'S THE GAME! {winner_name} destroys the Nexus for the victory! GG!"
        ))
    }

    pub fn for_teamfight(winning_team: &str, kills_won: u32, kills_lost: u32) -> Self {
        if kills_won >= 4 {
            Self::new(format!(
                "AN ACE! {winning_team} wipes the floor in that teamfight — {kills_won} for {kills_lost}!"
            ))
        } else {
            Self::new(format!(
                "{winning_team} wins the teamfight {kills_won} to {kills_lost}!"
            ))
        }
    }

    pub fn for_herald(team_name: &str) -> Self {
        Self::new(format!(
            "{team_name} takes down the Rift Herald! Time to crack open a tower."
        ))
    }

    pub fn for_multi_kill(player_name: &str, tier: &str) -> Self {
        Self::new(format!("{tier}! {player_name} is on fire!"))
    }

    pub fn for_killing_spree(player_name: &str, label: &str) -> Self {
        Self::new(format!("{player_name} is on a {label}!"))
    }
}

fn lane_str(lane: Lane) -> &'static str {
    match lane {
        Lane::Top => "top",
        Lane::Mid => "mid",
        Lane::Bot => "bot",
    }
}

// ---------------------------------------------------------------------------
// Snapshots — lightweight state captures attached to each event
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSnapshot {
    pub kills: u32,
    pub deaths: u32,
    pub assists: u32,
    pub cs: u32,
    pub gold: u32,
    pub is_dead: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSnapshot {
    pub blue_players: Vec<PlayerSnapshot>,
    pub red_players: Vec<PlayerSnapshot>,
    pub blue_team_gold: u32,
    pub red_team_gold: u32,
    pub dragons_blue: u32,
    pub dragons_red: u32,
    pub baron_alive: bool,
    pub baron_timer: u32,
    pub dragon_timer: u32,
    pub herald_available: bool,
}

// ---------------------------------------------------------------------------
// MatchEvent
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchEvent {
    minute: u32,
    phase: MobaMatchPhase,
    kind: MatchEventKind,
    commentary: Option<Commentary>,
    snapshot: Option<GameSnapshot>,
}

impl MatchEvent {
    pub fn new(minute: u32, phase: MobaMatchPhase, kind: MatchEventKind) -> Self {
        Self {
            minute,
            phase,
            kind,
            commentary: None,
            snapshot: None,
        }
    }

    pub fn minute(&self) -> u32 {
        self.minute
    }

    pub fn phase(&self) -> MobaMatchPhase {
        self.phase
    }

    pub fn kind(&self) -> &MatchEventKind {
        &self.kind
    }

    pub fn commentary(&self) -> Option<&Commentary> {
        self.commentary.as_ref()
    }

    pub fn set_commentary(&mut self, commentary: Commentary) {
        self.commentary = Some(commentary);
    }

    pub fn snapshot(&self) -> Option<&GameSnapshot> {
        self.snapshot.as_ref()
    }

    pub fn set_snapshot(&mut self, snapshot: GameSnapshot) {
        self.snapshot = Some(snapshot);
    }
}
