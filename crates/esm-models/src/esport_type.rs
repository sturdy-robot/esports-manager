use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// The type of eSport for a game session.
///
/// Currently only MOBA is fully implemented. RTS and FPS are architectural
/// placeholders for future expansion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EsportType {
    Moba,
    Rts,
    Fps,
}

impl EsportType {
    /// Returns the canonical string representation used for DB storage.
    pub fn as_str(&self) -> &'static str {
        match self {
            EsportType::Moba => "Moba",
            EsportType::Rts => "Rts",
            EsportType::Fps => "Fps",
        }
    }
}

impl fmt::Display for EsportType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Error returned when parsing an invalid eSport type string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseEsportTypeError(pub String);

impl fmt::Display for ParseEsportTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown eSport type: '{}'", self.0)
    }
}

impl std::error::Error for ParseEsportTypeError {}

impl FromStr for EsportType {
    type Err = ParseEsportTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Moba" => Ok(EsportType::Moba),
            "Rts" => Ok(EsportType::Rts),
            "Fps" => Ok(EsportType::Fps),
            other => Err(ParseEsportTypeError(other.to_string())),
        }
    }
}
