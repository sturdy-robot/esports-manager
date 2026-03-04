use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct DataPackError {
    pub message: String,
}

impl fmt::Display for DataPackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DataPackError: {}", self.message)
    }
}

impl std::error::Error for DataPackError {}

pub type ValidationResult = Result<(), DataPackError>;

// ---------------------------------------------------------------------------
// Valid values
// ---------------------------------------------------------------------------

const VALID_ROLES: &[&str] = &["Top", "Jungle", "Mid", "Bot", "Support"];
const VALID_CLASSES: &[&str] = &["Tank", "Fighter", "Assassin", "Mage", "Marksman", "Support"];
const VALID_SCALINGS: &[&str] = &["Early", "Mid", "Late"];
const VALID_TAGS: &[&str] = &[
    "Knockup", "Engage", "Poke", "Splitpush", "Waveclear", "Peel", "Burst", "Sustain",
];

// ---------------------------------------------------------------------------
// Data structures
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamData {
    pub name: String,
    pub tag: String,
    pub budget: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub nickname: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub team: String,
    pub endurance: u8,
    pub reaction_time: u8,
    pub decision_making: u8,
    pub clutch: u8,
    pub discipline: u8,
    pub tilt_resistance: u8,
    pub mechanics: u8,
    pub vision_control: u8,
    pub teamfighting: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChampionData {
    pub name: String,
    pub class: String,
    pub scaling: String,
    pub tags: Vec<String>,
}

// ---------------------------------------------------------------------------
// DataPack
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPack {
    pub teams: Vec<TeamData>,
    pub players: Vec<PlayerData>,
    pub champions: Vec<ChampionData>,
}

impl DataPack {
    /// Parse a DataPack from a JSON string.
    pub fn from_json(json: &str) -> Result<Self, DataPackError> {
        serde_json::from_str(json).map_err(|e| DataPackError {
            message: format!("JSON parse error: {e}"),
        })
    }

    /// Run the full validation pipeline on this data pack.
    ///
    /// Checks:
    /// - Attribute bounds (0-100)
    /// - Valid enum values (role, class, scaling, tags)
    /// - Referential integrity (player.team must exist in teams)
    /// - No duplicate team names
    pub fn validate(&self) -> ValidationResult {
        self.validate_teams()?;
        self.validate_players()?;
        self.validate_champions()?;
        Ok(())
    }

    fn validate_teams(&self) -> ValidationResult {
        let mut names = HashSet::new();
        for team in &self.teams {
            if !names.insert(&team.name) {
                return Err(DataPackError {
                    message: format!("Duplicate team name: '{}'", team.name),
                });
            }
        }
        Ok(())
    }

    fn validate_players(&self) -> ValidationResult {
        let team_names: HashSet<&str> = self.teams.iter().map(|t| t.name.as_str()).collect();

        for player in &self.players {
            if !VALID_ROLES.contains(&player.role.as_str()) {
                return Err(DataPackError {
                    message: format!(
                        "Player '{}' has invalid role: '{}'",
                        player.nickname, player.role
                    ),
                });
            }

            if !team_names.contains(player.team.as_str()) {
                return Err(DataPackError {
                    message: format!(
                        "Player '{}' references unknown team: '{}'",
                        player.nickname, player.team
                    ),
                });
            }

            self.validate_attribute_bounds(&player.nickname, &[
                ("endurance", player.endurance),
                ("reaction_time", player.reaction_time),
                ("decision_making", player.decision_making),
                ("clutch", player.clutch),
                ("discipline", player.discipline),
                ("tilt_resistance", player.tilt_resistance),
                ("mechanics", player.mechanics),
                ("vision_control", player.vision_control),
                ("teamfighting", player.teamfighting),
            ])?;
        }
        Ok(())
    }

    fn validate_champions(&self) -> ValidationResult {
        for champ in &self.champions {
            if !VALID_CLASSES.contains(&champ.class.as_str()) {
                return Err(DataPackError {
                    message: format!(
                        "Champion '{}' has invalid class: '{}'",
                        champ.name, champ.class
                    ),
                });
            }
            if !VALID_SCALINGS.contains(&champ.scaling.as_str()) {
                return Err(DataPackError {
                    message: format!(
                        "Champion '{}' has invalid scaling: '{}'",
                        champ.name, champ.scaling
                    ),
                });
            }
            for tag in &champ.tags {
                if !VALID_TAGS.contains(&tag.as_str()) {
                    return Err(DataPackError {
                        message: format!(
                            "Champion '{}' has invalid tag: '{}'",
                            champ.name, tag
                        ),
                    });
                }
            }
        }
        Ok(())
    }

    fn validate_attribute_bounds(
        &self,
        entity_name: &str,
        attrs: &[(&str, u8)],
    ) -> ValidationResult {
        for (name, value) in attrs {
            if *value > 100 {
                return Err(DataPackError {
                    message: format!(
                        "'{entity_name}' has {name} = {value}, must be 0-100"
                    ),
                });
            }
        }
        Ok(())
    }
}
