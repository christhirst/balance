use super::{Balance, TransactionType};

impl Balance {
    pub fn sell_shares(&mut self, count: i32, price_per_share: f64, symbol: String) {
        if let Some(stock) = self.shares.get_mut(&symbol) {
            if stock.count >= count {
                let earnings = count as f64 * price_per_share;
                self.cash += earnings;
                stock.count -= count;
                if stock.count == 0 {
                    self.shares.remove(&symbol);
                }
                self.add_transaction(TransactionType::Sell, earnings, count, Some(symbol));
            } else {
                tracing::error!("Not enough shares to sell");
            }
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
        balance.shares.insert(
            "AAPL".to_string(),
            crate::operations::Stock {
                symbol: "AAPL".to_string(),
                count: 20,
                price_per_share: 50.0, // Assuming initial price
            },
        );

        balance.sell_shares(10, 50.0, "AAPL".to_string());
        assert_eq!(balance.cash, 500.0);
        assert_eq!(balance.shares.get("AAPL").unwrap().count, 10);
        assert_eq!(balance.history.len(), 1);
        match balance.history[0].transaction_type {
            TransactionType::Sell => {}
            _ => panic!("Wrong transaction type"),
        }
    }
}
