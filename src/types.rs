use crate::depot;
use crate::operations;

impl From<&operations::Stock> for depot::ShareDetails {
    fn from(stock: &operations::Stock) -> Self {
        depot::ShareDetails {
            symbol: stock.symbol.clone(),
            count: stock.count,
            price_per_share: stock.price_per_share,
        }
    }
}

impl From<&operations::Transaction> for depot::Transaction {
    fn from(tx: &operations::Transaction) -> Self {
        depot::Transaction {
            r#type: match tx.transaction_type {
                operations::TransactionType::Deposit => {
                    depot::transaction::TransactionType::Deposit.into()
                }
                operations::TransactionType::Withdraw => {
                    depot::transaction::TransactionType::Withdraw.into()
                }
                operations::TransactionType::Buy => depot::transaction::TransactionType::Buy.into(),
                operations::TransactionType::Sell => {
                    depot::transaction::TransactionType::Sell.into()
                }
            },
            amount: tx.amount_cash,
            cash_difference: match tx.transaction_type {
                operations::TransactionType::Deposit | operations::TransactionType::Sell => {
                    tx.amount_cash
                }
                operations::TransactionType::Withdraw | operations::TransactionType::Buy => {
                    -tx.amount_cash
                }
            },
            count: tx.amount_shares,
            price_per_share: tx.price_per_share,
            symbol: tx.symbol.clone().unwrap_or_default(),
            timestamp: tx.timestamp.to_rfc3339(),
        }
    }
}
