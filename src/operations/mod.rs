pub mod add;
pub mod buy;
pub mod get;
pub mod sell;
pub mod take;
pub mod transactions;

pub use transactions::{Transaction, TransactionType};

#[derive(Debug, Clone)]
pub struct Stock {
    pub symbol: String,
    pub count: i32,
    pub price_per_share: f64,
}

#[derive(Debug)]
pub struct Balance {
    pub cash: f64,
    pub shares: std::collections::HashMap<String, Stock>,
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
}
