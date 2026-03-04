use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// TransactionKind
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionKind {
    Salary,
    TransferFee,
    Sponsorship,
    TournamentPrize,
    Merchandise,
    ScoutingFee,
}

// ---------------------------------------------------------------------------
// Transaction
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    kind: TransactionKind,
    amount: i64,
    day: u32,
    description: String,
}

impl Transaction {
    pub fn new(kind: TransactionKind, amount: i64, day: u32, description: String) -> Self {
        Self {
            kind,
            amount,
            day,
            description,
        }
    }

    pub fn kind(&self) -> TransactionKind {
        self.kind
    }

    pub fn amount(&self) -> i64 {
        self.amount
    }

    pub fn day(&self) -> u32 {
        self.day
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

// ---------------------------------------------------------------------------
// FinancialReport
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialReport {
    pub total_income: i64,
    pub total_expenses: i64,
    pub net: i64,
}

// ---------------------------------------------------------------------------
// Finances
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finances {
    balance: i64,
    transactions: Vec<Transaction>,
}

impl Finances {
    pub fn new(starting_balance: i64) -> Self {
        Self {
            balance: starting_balance,
            transactions: Vec::new(),
        }
    }

    pub fn balance(&self) -> i64 {
        self.balance
    }

    pub fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }

    pub fn record(&mut self, transaction: Transaction) {
        self.balance += transaction.amount();
        self.transactions.push(transaction);
    }

    pub fn is_in_debt(&self) -> bool {
        self.balance < 0
    }

    pub fn can_afford(&self, cost: i64) -> bool {
        self.balance >= cost
    }

    pub fn report(&self) -> FinancialReport {
        let mut total_income: i64 = 0;
        let mut total_expenses: i64 = 0;

        for tx in &self.transactions {
            if tx.amount() >= 0 {
                total_income += tx.amount();
            } else {
                total_expenses += -tx.amount();
            }
        }

        FinancialReport {
            total_income,
            total_expenses,
            net: total_income - total_expenses,
        }
    }
}
