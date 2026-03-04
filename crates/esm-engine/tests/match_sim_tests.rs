use esm_core::rng::GameRng;
use esm_engine::match_sim::{
    MatchEvent, MatchEventKind, MatchPhase, MatchSimulator, MatchState, TeamSide,
};

// ---------------------------------------------------------------------------
// MatchPhase
// ---------------------------------------------------------------------------

#[test]
fn match_phase_from_minute_early_game() {
    assert_eq!(MatchPhase::from_minute(0), MatchPhase::Early);
    assert_eq!(MatchPhase::from_minute(14), MatchPhase::Early);
}

#[test]
fn match_phase_from_minute_mid_game() {
    assert_eq!(MatchPhase::from_minute(15), MatchPhase::Mid);
    assert_eq!(MatchPhase::from_minute(24), MatchPhase::Mid);
}

#[test]
fn match_phase_from_minute_late_game() {
    assert_eq!(MatchPhase::from_minute(25), MatchPhase::Late);
    assert_eq!(MatchPhase::from_minute(40), MatchPhase::Late);
}

// ---------------------------------------------------------------------------
// MatchState
// ---------------------------------------------------------------------------

#[test]
fn match_state_starts_at_minute_zero() {
    let state = MatchState::new(5000, 5000);
    assert_eq!(state.minute(), 0);
    assert_eq!(state.phase(), MatchPhase::Early);
}

#[test]
fn match_state_initial_gold() {
    let state = MatchState::new(5000, 4800);
    assert_eq!(state.blue_gold(), 5000);
    assert_eq!(state.red_gold(), 4800);
}

#[test]
fn match_state_gold_delta() {
    let state = MatchState::new(6000, 5000);
    assert_eq!(state.gold_delta(), 1000);
}

#[test]
fn match_state_gold_delta_negative() {
    let state = MatchState::new(4000, 5000);
    assert_eq!(state.gold_delta(), -1000);
}

#[test]
fn match_state_advance_minute() {
    let mut state = MatchState::new(5000, 5000);
    state.advance_minute();
    assert_eq!(state.minute(), 1);
}

#[test]
fn match_state_phase_transitions_with_time() {
    let mut state = MatchState::new(5000, 5000);
    for _ in 0..15 {
        state.advance_minute();
    }
    assert_eq!(state.phase(), MatchPhase::Mid);
}

#[test]
fn match_state_is_over_false_initially() {
    let state = MatchState::new(5000, 5000);
    assert!(!state.is_over());
}

#[test]
fn match_state_set_winner() {
    let mut state = MatchState::new(5000, 5000);
    state.set_winner(TeamSide::Blue);
    assert!(state.is_over());
    assert_eq!(state.winner(), Some(TeamSide::Blue));
}

// ---------------------------------------------------------------------------
// MatchEvent
// ---------------------------------------------------------------------------

#[test]
fn match_event_stores_data() {
    let event = MatchEvent {
        minute: 5,
        kind: MatchEventKind::SoloKill,
        winner: TeamSide::Blue,
        gold_reward: 300,
    };
    assert_eq!(event.minute, 5);
    assert_eq!(event.kind, MatchEventKind::SoloKill);
    assert_eq!(event.winner, TeamSide::Blue);
    assert_eq!(event.gold_reward, 300);
}

#[test]
fn match_event_kinds_exist() {
    let _ = MatchEventKind::SoloKill;
    let _ = MatchEventKind::TowerPlates;
    let _ = MatchEventKind::Dragon;
    let _ = MatchEventKind::RiftHerald;
    let _ = MatchEventKind::Tower;
    let _ = MatchEventKind::Baron;
    let _ = MatchEventKind::Teamfight;
    let _ = MatchEventKind::Inhibitor;
    let _ = MatchEventKind::Elder;
    let _ = MatchEventKind::Nexus;
}

// ---------------------------------------------------------------------------
// MatchSimulator: deterministic simulation
// ---------------------------------------------------------------------------

#[test]
fn match_simulator_produces_a_result() {
    let mut rng = GameRng::from_seed(42);
    let result = MatchSimulator::simulate(&mut rng, 5500, 5000);
    assert!(result.winner == TeamSide::Blue || result.winner == TeamSide::Red);
    assert!(!result.events.is_empty());
    assert!(result.duration_minutes > 0);
}

#[test]
fn match_simulator_is_deterministic() {
    let mut rng1 = GameRng::from_seed(42);
    let mut rng2 = GameRng::from_seed(42);

    let r1 = MatchSimulator::simulate(&mut rng1, 5500, 5000);
    let r2 = MatchSimulator::simulate(&mut rng2, 5500, 5000);

    assert_eq!(r1.winner, r2.winner);
    assert_eq!(r1.duration_minutes, r2.duration_minutes);
    assert_eq!(r1.events.len(), r2.events.len());
    for (e1, e2) in r1.events.iter().zip(r2.events.iter()) {
        assert_eq!(e1.minute, e2.minute);
        assert_eq!(e1.kind, e2.kind);
        assert_eq!(e1.winner, e2.winner);
        assert_eq!(e1.gold_reward, e2.gold_reward);
    }
}

#[test]
fn match_simulator_different_seeds_different_results() {
    let mut rng1 = GameRng::from_seed(42);
    let mut rng2 = GameRng::from_seed(999);

    let r1 = MatchSimulator::simulate(&mut rng1, 5000, 5000);
    let r2 = MatchSimulator::simulate(&mut rng2, 5000, 5000);

    // With different seeds, at least some events should differ
    let events_differ = r1.events.len() != r2.events.len()
        || r1.winner != r2.winner
        || r1
            .events
            .iter()
            .zip(r2.events.iter())
            .any(|(e1, e2)| e1.kind != e2.kind || e1.winner != e2.winner);
    assert!(
        events_differ,
        "Different seeds should produce different match results"
    );
}

#[test]
fn match_simulator_result_has_final_gold() {
    let mut rng = GameRng::from_seed(42);
    let result = MatchSimulator::simulate(&mut rng, 5000, 5000);
    // Gold should have changed from initial values due to events
    assert!(result.blue_gold != 5000 || result.red_gold != 5000);
}

#[test]
fn match_simulator_ends_within_reasonable_time() {
    let mut rng = GameRng::from_seed(42);
    let result = MatchSimulator::simulate(&mut rng, 5000, 5000);
    assert!(
        result.duration_minutes <= 60,
        "Match should end within 60 minutes, got {}",
        result.duration_minutes
    );
}

#[test]
fn match_simulator_last_event_is_nexus() {
    let mut rng = GameRng::from_seed(42);
    let result = MatchSimulator::simulate(&mut rng, 5000, 5000);
    let last = result.events.last().unwrap();
    assert_eq!(last.kind, MatchEventKind::Nexus);
}

#[test]
fn match_simulator_higher_power_team_wins_more_often() {
    let mut blue_wins = 0;
    for seed in 0..200 {
        let mut rng = GameRng::from_seed(seed);
        let result = MatchSimulator::simulate(&mut rng, 7000, 4000);
        if result.winner == TeamSide::Blue {
            blue_wins += 1;
        }
    }
    // With a massive power advantage, blue should win most games
    assert!(
        blue_wins > 120,
        "Blue (7000 power) should win >60% vs Red (4000 power), but won {blue_wins}/200"
    );
}
