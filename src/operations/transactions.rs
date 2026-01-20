use super::Balance;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
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

impl Balance {
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

    pub fn delete_transaction(&mut self, index: usize) -> Option<Transaction> {
        if index < self.history.len() {
            Some(self.history.remove(index))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operations::Balance;

    #[test]
    fn test_delete_transaction() {
        let mut balance = Balance::new(100.0, 0);

        // Add a few transactions
        balance.add_transaction(TransactionType::Deposit, 100.0, 0, None);
        balance.add_transaction(TransactionType::Buy, 50.0, 1, Some("AAPL".to_string()));

        assert_eq!(balance.history.len(), 2);

        // Delete the first transaction (index 0)
        let removed = balance.delete_transaction(0);

        assert!(removed.is_some());
        assert_eq!(removed.unwrap().transaction_type, TransactionType::Deposit);
        assert_eq!(balance.history.len(), 1);

        // Check that the remaining transaction is the Buy
        assert_eq!(balance.history[0].transaction_type, TransactionType::Buy);

        // Try to delete out of bounds
        let invalid = balance.delete_transaction(10);
        assert!(invalid.is_none());

        // Delete the last remaining transaction
        let removed_last = balance.delete_transaction(0);
        assert!(removed_last.is_some());
        assert_eq!(balance.history.len(), 0);
    }
}
