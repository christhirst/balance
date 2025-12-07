use super::{Balance, TransactionType};

impl Balance {
    pub fn get_state(&self) -> String {
        format!(
            "Cash: {:.2}, Shares: {:?}, Transactions: {}",
            self.cash,
            self.shares,
            self.history.len()
        )
    }

    pub fn print_history(&self) {
        tracing::info!("Transaction History:");
        for tx in &self.history {
            tracing::info!(
                "[{}] {:?}: Cash: {:.2}, Shares: {}, Symbol: {:?}",
                tx.timestamp,
                tx.transaction_type,
                tx.amount_cash,
                tx.amount_shares,
                tx.symbol
            );
        }
    }

    pub fn calculate_total_deposited(&self) -> f64 {
        self.history
            .iter()
            .filter(|tx| matches!(tx.transaction_type, TransactionType::Deposit))
            .map(|tx| tx.amount_cash)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::operations::{Balance, TransactionType};

    #[test]
    fn test_get_state() {
        let balance = Balance::new(100.0, 0);
        let state = balance.get_state();
        assert!(state.contains("Cash: 100.00"));
        assert!(state.contains("Shares: {}"));
    }

    #[test]
    fn test_calculate_total_deposited() {
        let mut balance = Balance::new(0.0, 0);
        balance.add_transaction(TransactionType::Deposit, 100.0, 0, None);
        balance.add_transaction(TransactionType::Deposit, 50.0, 0, None);
        // This fails because Balance::new doesn't take shares anymore, and add_transaction takes 4 args
        assert_eq!(balance.calculate_total_deposited(), 150.0);
    }
}
