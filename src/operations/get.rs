use super::{Balance, TransactionType};

impl Balance {
    #[allow(dead_code)]
    pub fn get_state(&self) -> String {
        format!(
            "Cash: {:.2}, Shares: {:?}, Transactions: {}",
            self.cash,
            self.shares,
            self.history.len()
        )
    }

    #[allow(dead_code)]
    pub fn print_history(&self) {
        tracing::info!("Transaction History:");
        for tx in &self.history {
            tracing::info!(
                "[{}] {:?}: Cash: {:.2}, Shares: {}, Price: {:.2}, Symbol: {:?}",
                tx.timestamp,
                tx.transaction_type,
                tx.amount_cash,
                tx.amount_shares,
                tx.price_per_share,
                tx.symbol
            );
        }
    }

    #[allow(dead_code)]
    pub fn calculate_total_deposited(&self) -> f64 {
        self.history
            .iter()
            .filter(|tx| matches!(tx.transaction_type, TransactionType::Deposit))
            .map(|tx| tx.amount_cash)
            .sum()
    }

    #[allow(dead_code)]
    pub fn get_share_balance(&self, symbol: &str) -> i32 {
        self.shares.get(symbol).map(|s| s.count).unwrap_or(0)
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
        balance.add_transaction(TransactionType::Deposit, 100.0, 0, 0.0, None);
        balance.add_transaction(TransactionType::Deposit, 50.0, 0, 0.0, None);
        // This fails because Balance::new doesn't take shares anymore, and add_transaction takes 4 args
        assert_eq!(balance.calculate_total_deposited(), 150.0);
    }

    #[test]
    fn test_get_share_balance() {
        let mut balance = Balance::new(0.0, 0);

        // Buy shares of AAPL
        balance.buy_shares(10, 0.0, "AAPL".to_string());

        // Buy shares of GOOG
        balance.buy_shares(5, 0.0, "GOOG".to_string());

        // Check share balances using the get_share_balance method
        assert_eq!(balance.get_share_balance("AAPL"), 10);
        assert_eq!(balance.get_share_balance("GOOG"), 5);
        assert_eq!(balance.get_share_balance("MSFT"), 0);

        // Verify transactions were recorded
        assert_eq!(balance.history.len(), 2);
        assert_eq!(balance.history[0].symbol.as_ref().unwrap(), "AAPL");
        assert_eq!(balance.history[0].transaction_type, TransactionType::Buy);
        assert_eq!(balance.history[1].symbol.as_ref().unwrap(), "GOOG");
        assert_eq!(balance.history[1].transaction_type, TransactionType::Buy);
    }
}
