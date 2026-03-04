use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TeamSide {
    Blue,
    Red,
}

impl TeamSide {
    pub fn opposite(self) -> Self {
        match self {
            TeamSide::Blue => TeamSide::Red,
            TeamSide::Red => TeamSide::Blue,
        }
    }
}
