use esm_core::board::{BoardEvaluation, ManagerSatisfaction};

// ---------------------------------------------------------------------------
// ManagerSatisfaction
// ---------------------------------------------------------------------------

#[test]
fn manager_satisfaction_starts_at_50() {
    let sat = ManagerSatisfaction::new();
    assert_eq!(sat.score().value(), 50);
}

#[test]
fn manager_satisfaction_increase() {
    let mut sat = ManagerSatisfaction::new();
    sat.score_mut().increase(10);
    assert_eq!(sat.score().value(), 60);
}

#[test]
fn manager_satisfaction_decrease() {
    let mut sat = ManagerSatisfaction::new();
    sat.score_mut().decrease(20);
    assert_eq!(sat.score().value(), 30);
}

#[test]
fn manager_satisfaction_clamps_at_0() {
    let mut sat = ManagerSatisfaction::new();
    sat.score_mut().decrease(100);
    assert_eq!(sat.score().value(), 0);
}

#[test]
fn manager_satisfaction_clamps_at_100() {
    let mut sat = ManagerSatisfaction::new();
    sat.score_mut().increase(100);
    assert_eq!(sat.score().value(), 100);
}

// ---------------------------------------------------------------------------
// Board warning / termination thresholds
// ---------------------------------------------------------------------------

#[test]
fn is_warning_true_at_30() {
    let mut sat = ManagerSatisfaction::new();
    sat.score_mut().decrease(20); // 50 -> 30
    assert!(sat.is_warning());
}

#[test]
fn is_warning_true_below_30() {
    let mut sat = ManagerSatisfaction::new();
    sat.score_mut().decrease(25); // 50 -> 25
    assert!(sat.is_warning());
}

#[test]
fn is_warning_false_above_30() {
    let mut sat = ManagerSatisfaction::new();
    sat.score_mut().decrease(19); // 50 -> 31
    assert!(!sat.is_warning());
}

#[test]
fn is_terminated_true_at_0() {
    let mut sat = ManagerSatisfaction::new();
    sat.score_mut().decrease(100);
    assert!(sat.is_terminated());
}

#[test]
fn is_terminated_false_above_0() {
    let mut sat = ManagerSatisfaction::new();
    sat.score_mut().decrease(49); // 50 -> 1
    assert!(!sat.is_terminated());
}

// ---------------------------------------------------------------------------
// BoardEvaluation
// ---------------------------------------------------------------------------

#[test]
fn board_evaluation_positive_increases_satisfaction() {
    let mut sat = ManagerSatisfaction::new();
    let eval = BoardEvaluation {
        tournament_delta: 5,
        roster_delta: 3,
        financial_delta: 2,
    };
    sat.apply_evaluation(&eval);
    // Primary (tournament) weighted highest, secondary (roster) next, tertiary (financial) least
    assert!(sat.score().value() > 50);
}

#[test]
fn board_evaluation_negative_decreases_satisfaction() {
    let mut sat = ManagerSatisfaction::new();
    let eval = BoardEvaluation {
        tournament_delta: -10,
        roster_delta: -5,
        financial_delta: -3,
    };
    sat.apply_evaluation(&eval);
    assert!(sat.score().value() < 50);
}

#[test]
fn board_evaluation_mixed_applies_net_effect() {
    let mut sat = ManagerSatisfaction::new();
    let eval = BoardEvaluation {
        tournament_delta: 5,
        roster_delta: -3,
        financial_delta: -2,
    };
    sat.apply_evaluation(&eval);
    // Net should still be positive since tournament is weighted heaviest
    // 5*3 + (-3)*2 + (-2)*1 = 15 - 6 - 2 = 7 -> score = 57
    assert_eq!(sat.score().value(), 57);
}

#[test]
fn board_evaluation_all_zero_no_change() {
    let mut sat = ManagerSatisfaction::new();
    let eval = BoardEvaluation {
        tournament_delta: 0,
        roster_delta: 0,
        financial_delta: 0,
    };
    sat.apply_evaluation(&eval);
    assert_eq!(sat.score().value(), 50);
}
