use chrono::{DateTime, Utc};

pub mod add;
pub mod buy;
pub mod get;
pub mod sell;
pub mod take;

#[derive(Debug, Clone)]
pub enum TransactionType {
    Deposit,
    Withdraw,
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub amount_cash: f64,
    pub amount_shares: i32,
    pub symbol: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug)]
pub struct Balance {
    pub cash: f64,
    pub shares: std::collections::HashMap<String, i32>,
    pub history: Vec<Transaction>,
}

impl Balance {
    pub fn new(cash: f64, _initial_shares_ignored: i32) -> Self {
        Self {
            cash,
            shares: std::collections::HashMap::new(),
            history: Vec::new(),
        }
    }

    pub fn add_transaction(
        &mut self,
        transaction_type: TransactionType,
        amount_cash: f64,
        amount_shares: i32,
        symbol: Option<String>,
    ) {
        self.history.push(Transaction {
            transaction_type,
            amount_cash,
            amount_shares,
            symbol,
            timestamp: Utc::now(),
        });
    }
}
