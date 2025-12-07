use super::{Balance, TransactionType};

impl Balance {
    pub fn take_cash(&mut self, amount: f64) {
        self.cash -= amount;
        self.add_transaction(TransactionType::Withdraw, amount, 0, None);
    }
}

#[cfg(test)]
mod tests {
    use crate::operations::{Balance, TransactionType};

    #[test]
    fn test_take_cash() {
        let mut balance = Balance::new(100.0, 0);
        balance.take_cash(30.0);
        assert_eq!(balance.cash, 70.0);
        assert_eq!(balance.history.len(), 1);
        match balance.history[0].transaction_type {
            TransactionType::Withdraw => {}
            _ => panic!("Wrong transaction type"),
        }
    }
}
