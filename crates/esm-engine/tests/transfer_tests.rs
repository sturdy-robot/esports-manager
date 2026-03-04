use esm_core::rng::GameRng;
use esm_engine::transfer::{
    Negotiation, NegotiationOutcome, OfferParams, TransferEngine,
};
use esm_models::contract::NegotiationState;

// ---------------------------------------------------------------------------
// OfferParams
// ---------------------------------------------------------------------------

#[test]
fn offer_params_stores_fields() {
    let offer = OfferParams {
        bid_amount: 500_000,
        salary_offered: 60_000,
        contract_length_days: 365,
        manager_reputation: 70,
        team_tournament_status: 80,
    };
    assert_eq!(offer.bid_amount, 500_000);
    assert_eq!(offer.salary_offered, 60_000);
}

// ---------------------------------------------------------------------------
// Negotiation state machine
// ---------------------------------------------------------------------------

#[test]
fn negotiation_starts_in_scouting() {
    let neg = Negotiation::new("player-1".to_string(), "team-src".to_string(), "team-dst".to_string());
    assert_eq!(neg.state(), NegotiationState::Scouting);
    assert_eq!(neg.player_id(), "player-1");
}

#[test]
fn negotiation_advance_from_scouting() {
    let mut neg = Negotiation::new("p".to_string(), "src".to_string(), "dst".to_string());
    neg.advance();
    assert_eq!(neg.state(), NegotiationState::BuyoutNegotiation);
}

#[test]
fn negotiation_advance_full_pipeline() {
    let mut neg = Negotiation::new("p".to_string(), "src".to_string(), "dst".to_string());
    neg.advance(); // Scouting → BuyoutNegotiation
    neg.advance(); // BuyoutNegotiation → ContractNegotiation
    neg.advance(); // ContractNegotiation → Completed
    assert_eq!(neg.state(), NegotiationState::Completed);
    assert!(neg.is_terminal());
}

#[test]
fn negotiation_reject_sets_rejected() {
    let mut neg = Negotiation::new("p".to_string(), "src".to_string(), "dst".to_string());
    neg.reject();
    assert_eq!(neg.state(), NegotiationState::Rejected);
    assert!(neg.is_terminal());
}

#[test]
fn negotiation_cannot_advance_past_terminal() {
    let mut neg = Negotiation::new("p".to_string(), "src".to_string(), "dst".to_string());
    neg.advance();
    neg.advance();
    neg.advance(); // Completed
    neg.advance(); // Should stay Completed
    assert_eq!(neg.state(), NegotiationState::Completed);
}

// ---------------------------------------------------------------------------
// TransferEngine: buyout evaluation
// ---------------------------------------------------------------------------

#[test]
fn buyout_accepted_when_bid_exceeds_player_value() {
    let mut rng = GameRng::from_seed(42);
    let accepted = TransferEngine::evaluate_buyout(
        &mut rng,
        500_000, // bid
        300_000, // player value
        180,     // remaining contract days
        true,    // selling team financially healthy
    );
    assert!(accepted);
}

#[test]
fn buyout_rejected_when_bid_too_low() {
    let mut rng = GameRng::from_seed(42);
    let accepted = TransferEngine::evaluate_buyout(
        &mut rng,
        50_000,    // bid much lower than value
        500_000,   // player value
        365,       // long contract
        true,
    );
    assert!(!accepted);
}

// ---------------------------------------------------------------------------
// TransferEngine: contract offer evaluation (agent logic)
// ---------------------------------------------------------------------------

#[test]
fn agent_accepts_strong_offer() {
    let mut rng = GameRng::from_seed(42);
    let offer = OfferParams {
        bid_amount: 0,
        salary_offered: 80_000,
        contract_length_days: 365,
        manager_reputation: 85,
        team_tournament_status: 90,
    };
    let result = TransferEngine::evaluate_contract_offer(
        &mut rng,
        &offer,
        50,  // player ambition (0-100)
        40_000, // player minimum salary threshold
    );
    assert_eq!(result, NegotiationOutcome::Accepted);
}

#[test]
fn agent_rejects_low_salary() {
    let mut rng = GameRng::from_seed(42);
    let offer = OfferParams {
        bid_amount: 0,
        salary_offered: 10_000,
        contract_length_days: 365,
        manager_reputation: 30,
        team_tournament_status: 20,
    };
    let result = TransferEngine::evaluate_contract_offer(
        &mut rng,
        &offer,
        90, // high ambition
        80_000, // high minimum salary
    );
    assert_eq!(result, NegotiationOutcome::Rejected);
}

#[test]
fn agent_evaluation_is_deterministic() {
    let offer = OfferParams {
        bid_amount: 0,
        salary_offered: 60_000,
        contract_length_days: 300,
        manager_reputation: 60,
        team_tournament_status: 70,
    };

    let mut rng1 = GameRng::from_seed(42);
    let mut rng2 = GameRng::from_seed(42);

    let r1 = TransferEngine::evaluate_contract_offer(&mut rng1, &offer, 50, 40_000);
    let r2 = TransferEngine::evaluate_contract_offer(&mut rng2, &offer, 50, 40_000);
    assert_eq!(r1, r2);
}
