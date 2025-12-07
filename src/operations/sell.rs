use super::{Balance, TransactionType};

impl Balance {
    pub fn sell_shares(&mut self, count: i32, price_per_share: f64, symbol: String) {
        let current_shares = *self.shares.get(&symbol).unwrap_or(&0);
        if current_shares >= count {
            let earnings = count as f64 * price_per_share;
            self.cash += earnings;
            *self.shares.entry(symbol.clone()).or_insert(0) -= count;
            self.add_transaction(TransactionType::Sell, earnings, count, Some(symbol));
        } else {
            tracing::error!("Not enough shares to sell");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::operations::{Balance, TransactionType};

    #[test]
    fn test_sell_shares() {
        let mut balance = Balance::new(0.0, 0);
        // seed some shares
        balance.shares.insert("AAPL".to_string(), 20);

        balance.sell_shares(10, 50.0, "AAPL".to_string());
        assert_eq!(balance.cash, 500.0);
        assert_eq!(*balance.shares.get("AAPL").unwrap(), 10);
        assert_eq!(balance.history.len(), 1);
        match balance.history[0].transaction_type {
            TransactionType::Sell => {}
            _ => panic!("Wrong transaction type"),
        }
    }
}
