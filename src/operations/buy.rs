use super::{Balance, TransactionType};

impl Balance {
    pub fn buy_shares(&mut self, count: i32, price_per_share: f64, symbol: String) {
        let cost = count as f64 * price_per_share;
        if self.cash >= cost {
            self.cash -= cost;
            *self.shares.entry(symbol.clone()).or_insert(0) += count;
            self.add_transaction(TransactionType::Buy, cost, count, Some(symbol));
        } else {
            tracing::error!("Not enough cash to buy shares");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::operations::{Balance, TransactionType};

    #[test]
    fn test_buy_shares() {
        let mut balance = Balance::new(1000.0, 0);
        balance.buy_shares(10, 50.0, "AAPL".to_string());
        assert_eq!(balance.cash, 500.0);
        assert_eq!(*balance.shares.get("AAPL").unwrap(), 10);
        assert_eq!(balance.history.len(), 1);
        match balance.history[0].transaction_type {
            TransactionType::Buy => {}
            _ => panic!("Wrong transaction type"),
        }
    }
}
