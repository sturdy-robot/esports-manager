use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractStatus {
    Active,
    Expired,
    Terminated,
}

/// Transfer negotiation pipeline states per DESIGN.md §5.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NegotiationState {
    Scouting,
    BuyoutNegotiation,
    ContractNegotiation,
    Completed,
    Rejected,
}

impl NegotiationState {
    /// Advance to the next state in the negotiation pipeline.
    /// Terminal states (`Completed`, `Rejected`) return `None`.
    pub fn next(self) -> Option<NegotiationState> {
        match self {
            NegotiationState::Scouting => Some(NegotiationState::BuyoutNegotiation),
            NegotiationState::BuyoutNegotiation => Some(NegotiationState::ContractNegotiation),
            NegotiationState::ContractNegotiation => Some(NegotiationState::Completed),
            NegotiationState::Completed | NegotiationState::Rejected => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Contract Entity
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    player_id: String,
    team_id: String,
    salary: u64,
    length_days: u32,
    remaining_days: u32,
    buyout_clause: Option<u64>,
    status: ContractStatus,
}

impl Contract {
    pub fn new(
        player_id: String,
        team_id: String,
        salary: u64,
        length_days: u32,
        buyout_clause: Option<u64>,
    ) -> Self {
        Self {
            player_id,
            team_id,
            salary,
            length_days,
            remaining_days: length_days,
            buyout_clause,
            status: ContractStatus::Active,
        }
    }

    pub fn player_id(&self) -> &str {
        &self.player_id
    }

    pub fn team_id(&self) -> &str {
        &self.team_id
    }

    pub fn salary(&self) -> u64 {
        self.salary
    }

    pub fn length_days(&self) -> u32 {
        self.length_days
    }

    pub fn remaining_days(&self) -> u32 {
        self.remaining_days
    }

    pub fn buyout_clause(&self) -> Option<u64> {
        self.buyout_clause
    }

    pub fn status(&self) -> ContractStatus {
        self.status
    }

    /// Advance the contract by one day. Expires automatically when remaining hits 0.
    pub fn tick_day(&mut self) {
        if self.status != ContractStatus::Active {
            return;
        }
        self.remaining_days = self.remaining_days.saturating_sub(1);
        if self.remaining_days == 0 {
            self.status = ContractStatus::Expired;
        }
    }

    pub fn terminate(&mut self) {
        self.status = ContractStatus::Terminated;
    }

    /// Returns `true` if the contract is active and remaining days ≤ `threshold`.
    pub fn is_expiring_soon(&self, threshold: u32) -> bool {
        self.status == ContractStatus::Active && self.remaining_days <= threshold
    }
}
