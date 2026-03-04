use esm_core::rng::GameRng;
use esm_models::contract::NegotiationState;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NegotiationOutcome {
    Accepted,
    Rejected,
}

/// Parameters for a contract offer from the manager to a player's agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfferParams {
    pub bid_amount: u64,
    pub salary_offered: u64,
    pub contract_length_days: u32,
    pub manager_reputation: u8,
    pub team_tournament_status: u8,
}

// ---------------------------------------------------------------------------
// Negotiation state machine wrapper
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Negotiation {
    player_id: String,
    source_team_id: String,
    destination_team_id: String,
    state: NegotiationState,
}

impl Negotiation {
    pub fn new(player_id: String, source_team_id: String, destination_team_id: String) -> Self {
        Self {
            player_id,
            source_team_id,
            destination_team_id,
            state: NegotiationState::Scouting,
        }
    }

    pub fn player_id(&self) -> &str {
        &self.player_id
    }

    pub fn source_team_id(&self) -> &str {
        &self.source_team_id
    }

    pub fn destination_team_id(&self) -> &str {
        &self.destination_team_id
    }

    pub fn state(&self) -> NegotiationState {
        self.state
    }

    pub fn is_terminal(&self) -> bool {
        self.state.next().is_none()
    }

    /// Advance to the next pipeline state. No-op if already terminal.
    pub fn advance(&mut self) {
        if let Some(next) = self.state.next() {
            self.state = next;
        }
    }

    /// Reject the negotiation outright.
    pub fn reject(&mut self) {
        self.state = NegotiationState::Rejected;
    }
}

// ---------------------------------------------------------------------------
// TransferEngine
// ---------------------------------------------------------------------------

pub struct TransferEngine;

impl TransferEngine {
    /// Evaluate whether the selling team accepts the buyout bid.
    ///
    /// The AI evaluates based on bid vs player value, remaining contract
    /// length, and the team's financial health.
    pub fn evaluate_buyout(
        rng: &mut GameRng,
        bid: u64,
        player_value: u64,
        remaining_contract_days: u32,
        selling_team_healthy: bool,
    ) -> bool {
        // Base acceptance: bid must cover at least a fraction of value
        let value_ratio = bid as f64 / player_value.max(1) as f64;

        // Longer contracts make teams demand more
        let contract_factor = 1.0 + (remaining_contract_days as f64 / 365.0) * 0.3;

        // Financially struggling teams accept lower bids
        let health_factor = if selling_team_healthy { 1.0 } else { 0.7 };

        let threshold = contract_factor * health_factor;

        // Add slight randomization
        let noise = rng.range_f64(-0.1, 0.1);
        let effective_ratio = value_ratio + noise;

        effective_ratio >= threshold
    }

    /// Evaluate whether the player's agent accepts the contract offer.
    ///
    /// Per DESIGN.md §5: the agent calculates an "Offer Score" based on
    /// salary, contract length, manager reputation, and team tournament status.
    /// If the score beats the player's minimum threshold, the contract is signed.
    pub fn evaluate_contract_offer(
        rng: &mut GameRng,
        offer: &OfferParams,
        player_ambition: u8,
        player_min_salary: u64,
    ) -> NegotiationOutcome {
        // Hard reject if salary is below player's minimum
        if offer.salary_offered < player_min_salary {
            return NegotiationOutcome::Rejected;
        }

        // Offer score components (0-100 scale each)
        let salary_score = ((offer.salary_offered as f64 / player_min_salary.max(1) as f64) * 50.0)
            .min(100.0);
        let length_score = (offer.contract_length_days as f64 / 365.0 * 40.0).min(80.0);
        let reputation_score = offer.manager_reputation as f64;
        let tournament_score = offer.team_tournament_status as f64;

        // Weighted composite
        let offer_score =
            salary_score * 0.35 + length_score * 0.15 + reputation_score * 0.25 + tournament_score * 0.25;

        // Player ambition sets the acceptance threshold
        let threshold = player_ambition as f64 * 0.8;

        // Add slight randomization
        let noise = rng.range_f64(-5.0, 5.0);
        let effective_score = offer_score + noise;

        if effective_score >= threshold {
            NegotiationOutcome::Accepted
        } else {
            NegotiationOutcome::Rejected
        }
    }
}
