use esm_engine::economy::{Finances, Transaction, TransactionKind};

// ---------------------------------------------------------------------------
// TransactionKind
// ---------------------------------------------------------------------------

#[test]
fn transaction_kind_variants_exist() {
    let _ = TransactionKind::Salary;
    let _ = TransactionKind::TransferFee;
    let _ = TransactionKind::Sponsorship;
    let _ = TransactionKind::TournamentPrize;
    let _ = TransactionKind::Merchandise;
    let _ = TransactionKind::ScoutingFee;
}

// ---------------------------------------------------------------------------
// Transaction
// ---------------------------------------------------------------------------

#[test]
fn transaction_stores_fields() {
    let tx = Transaction::new(
        TransactionKind::Salary,
        -50_000,
        1,
        "Monthly salary".to_string(),
    );
    assert_eq!(tx.kind(), TransactionKind::Salary);
    assert_eq!(tx.amount(), -50_000);
    assert_eq!(tx.day(), 1);
    assert_eq!(tx.description(), "Monthly salary");
}

// ---------------------------------------------------------------------------
// Finances
// ---------------------------------------------------------------------------

#[test]
fn finances_starts_with_given_budget() {
    let fin = Finances::new(1_000_000);
    assert_eq!(fin.balance(), 1_000_000);
}

#[test]
fn finances_apply_income_increases_balance() {
    let mut fin = Finances::new(1_000_000);
    fin.record(Transaction::new(
        TransactionKind::Sponsorship,
        200_000,
        1,
        "Sponsor deal".to_string(),
    ));
    assert_eq!(fin.balance(), 1_200_000);
}

#[test]
fn finances_apply_expense_decreases_balance() {
    let mut fin = Finances::new(1_000_000);
    fin.record(Transaction::new(
        TransactionKind::Salary,
        -50_000,
        1,
        "Player salary".to_string(),
    ));
    assert_eq!(fin.balance(), 950_000);
}

#[test]
fn finances_can_go_negative() {
    let mut fin = Finances::new(10_000);
    fin.record(Transaction::new(
        TransactionKind::TransferFee,
        -500_000,
        1,
        "Big transfer".to_string(),
    ));
    assert_eq!(fin.balance(), -490_000);
}

#[test]
fn finances_tracks_transaction_history() {
    let mut fin = Finances::new(1_000_000);
    fin.record(Transaction::new(
        TransactionKind::Salary,
        -50_000,
        1,
        "Salary".to_string(),
    ));
    fin.record(Transaction::new(
        TransactionKind::Sponsorship,
        100_000,
        2,
        "Sponsor".to_string(),
    ));
    assert_eq!(fin.transactions().len(), 2);
}

#[test]
fn finances_is_in_debt_when_negative() {
    let mut fin = Finances::new(10_000);
    assert!(!fin.is_in_debt());

    fin.record(Transaction::new(
        TransactionKind::TransferFee,
        -20_000,
        1,
        "Transfer".to_string(),
    ));
    assert!(fin.is_in_debt());
}

// ---------------------------------------------------------------------------
// FinancialReport
// ---------------------------------------------------------------------------

#[test]
fn financial_report_computes_totals() {
    let mut fin = Finances::new(1_000_000);
    fin.record(Transaction::new(
        TransactionKind::Sponsorship,
        200_000,
        1,
        "Sponsor".to_string(),
    ));
    fin.record(Transaction::new(
        TransactionKind::TournamentPrize,
        50_000,
        3,
        "Prize".to_string(),
    ));
    fin.record(Transaction::new(
        TransactionKind::Salary,
        -80_000,
        5,
        "Salaries".to_string(),
    ));
    fin.record(Transaction::new(
        TransactionKind::ScoutingFee,
        -10_000,
        6,
        "Scout".to_string(),
    ));

    let report = fin.report();
    assert_eq!(report.total_income, 250_000);
    assert_eq!(report.total_expenses, 90_000);
    assert_eq!(report.net, 160_000);
}

#[test]
fn financial_report_empty_finances() {
    let fin = Finances::new(500_000);
    let report = fin.report();
    assert_eq!(report.total_income, 0);
    assert_eq!(report.total_expenses, 0);
    assert_eq!(report.net, 0);
}

#[test]
fn finances_can_afford_true_when_sufficient() {
    let fin = Finances::new(100_000);
    assert!(fin.can_afford(100_000));
    assert!(fin.can_afford(50_000));
}

#[test]
fn finances_can_afford_false_when_insufficient() {
    let fin = Finances::new(100_000);
    assert!(!fin.can_afford(100_001));
}
