use super::Balance;

impl Balance {
    pub fn get_state(&self) -> String {
        format!(
            "Cash: {:.2}, Shares: {}, Transactions: {}",
            self.cash,
            self.shares,
            self.history.len()
        )
    }

    pub fn print_history(&self) {
        tracing::info!("Transaction History:");
        for tx in &self.history {
            tracing::info!(
                "[{}] {:?}: Cash: {:.2}, Shares: {}",
                tx.timestamp,
                tx.transaction_type,
                tx.amount_cash,
                tx.amount_shares
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::operations::Balance;

    #[test]
    fn test_get_state() {
        let balance = Balance::new(100.0, 10);
        let state = balance.get_state();
        assert!(state.contains("Cash: 100.00"));
        assert!(state.contains("Shares: 10"));
    }
}
