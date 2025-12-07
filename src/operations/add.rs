use super::{Balance, TransactionType};

impl Balance {
    pub fn add_cash(&mut self, amount: f64) {
        self.cash += amount;
        self.add_transaction(TransactionType::Deposit, amount, 0, None);
    }
}

#[cfg(test)]
mod tests {
    use crate::operations::{Balance, TransactionType};

    #[test]
    fn test_add_cash() {
        let mut balance = Balance::new(100.0, 0);
        balance.add_cash(50.0);
        assert_eq!(balance.cash, 150.0);
        assert_eq!(balance.history.len(), 1);
        match balance.history[0].transaction_type {
            TransactionType::Deposit => {}
            _ => panic!("Wrong transaction type"),
        }
    }
}
