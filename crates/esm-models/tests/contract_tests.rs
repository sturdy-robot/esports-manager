use esm_models::contract::{Contract, ContractStatus, NegotiationState};

// ---------------------------------------------------------------------------
// ContractStatus enum
// ---------------------------------------------------------------------------

#[test]
fn contract_status_variants_exist() {
    let _ = ContractStatus::Active;
    let _ = ContractStatus::Expired;
    let _ = ContractStatus::Terminated;
}

// ---------------------------------------------------------------------------
// NegotiationState enum (transfer pipeline per DESIGN.md §5)
// ---------------------------------------------------------------------------

#[test]
fn negotiation_state_variants_exist() {
    let _ = NegotiationState::Scouting;
    let _ = NegotiationState::BuyoutNegotiation;
    let _ = NegotiationState::ContractNegotiation;
    let _ = NegotiationState::Completed;
    let _ = NegotiationState::Rejected;
}

// ---------------------------------------------------------------------------
// Contract entity
// ---------------------------------------------------------------------------

fn make_test_contract() -> Contract {
    Contract::new(
        "player-1".to_string(),
        "team-1".to_string(),
        50_000,
        365,
        Some(200_000),
    )
}

#[test]
fn contract_stores_parties() {
    let c = make_test_contract();
    assert_eq!(c.player_id(), "player-1");
    assert_eq!(c.team_id(), "team-1");
}

#[test]
fn contract_stores_financials() {
    let c = make_test_contract();
    assert_eq!(c.salary(), 50_000);
    assert_eq!(c.buyout_clause(), Some(200_000));
}

#[test]
fn contract_stores_length_in_days() {
    let c = make_test_contract();
    assert_eq!(c.length_days(), 365);
    assert_eq!(c.remaining_days(), 365);
}

#[test]
fn contract_starts_active() {
    let c = make_test_contract();
    assert_eq!(c.status(), ContractStatus::Active);
}

#[test]
fn contract_without_buyout_clause() {
    let c = Contract::new(
        "player-2".to_string(),
        "team-2".to_string(),
        30_000,
        180,
        None,
    );
    assert_eq!(c.buyout_clause(), None);
}

#[test]
fn contract_remaining_days_decreases_on_tick() {
    let mut c = make_test_contract();
    c.tick_day();
    assert_eq!(c.remaining_days(), 364);
}

#[test]
fn contract_expires_when_remaining_days_hits_zero() {
    let mut c = Contract::new(
        "p".to_string(),
        "t".to_string(),
        10_000,
        2,
        None,
    );
    assert_eq!(c.status(), ContractStatus::Active);

    c.tick_day();
    assert_eq!(c.remaining_days(), 1);
    assert_eq!(c.status(), ContractStatus::Active);

    c.tick_day();
    assert_eq!(c.remaining_days(), 0);
    assert_eq!(c.status(), ContractStatus::Expired);
}

#[test]
fn contract_can_be_terminated() {
    let mut c = make_test_contract();
    c.terminate();
    assert_eq!(c.status(), ContractStatus::Terminated);
}

#[test]
fn contract_tick_on_expired_does_not_underflow() {
    let mut c = Contract::new(
        "p".to_string(),
        "t".to_string(),
        10_000,
        1,
        None,
    );
    c.tick_day(); // remaining = 0, expired
    c.tick_day(); // should not underflow
    assert_eq!(c.remaining_days(), 0);
    assert_eq!(c.status(), ContractStatus::Expired);
}

#[test]
fn contract_is_expiring_soon_within_threshold() {
    let mut c = Contract::new(
        "p".to_string(),
        "t".to_string(),
        10_000,
        35,
        None,
    );
    // Tick 5 days, leaving 30 remaining
    for _ in 0..5 {
        c.tick_day();
    }
    assert!(c.is_expiring_soon(30));
    assert!(!c.is_expiring_soon(29));
}

// ---------------------------------------------------------------------------
// NegotiationState transitions
// ---------------------------------------------------------------------------

#[test]
fn negotiation_scouting_can_advance_to_buyout() {
    let state = NegotiationState::Scouting;
    assert_eq!(state.next(), Some(NegotiationState::BuyoutNegotiation));
}

#[test]
fn negotiation_buyout_can_advance_to_contract() {
    let state = NegotiationState::BuyoutNegotiation;
    assert_eq!(state.next(), Some(NegotiationState::ContractNegotiation));
}

#[test]
fn negotiation_contract_can_advance_to_completed() {
    let state = NegotiationState::ContractNegotiation;
    assert_eq!(state.next(), Some(NegotiationState::Completed));
}

#[test]
fn negotiation_completed_has_no_next() {
    let state = NegotiationState::Completed;
    assert_eq!(state.next(), None);
}

#[test]
fn negotiation_rejected_has_no_next() {
    let state = NegotiationState::Rejected;
    assert_eq!(state.next(), None);
}
