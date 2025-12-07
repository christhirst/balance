use depot::depot_server::{Depot, DepotServer};
use depot::{
    BuyRequest, DepositRequest, Empty, SellRequest, StateResponse, TransactionResponse,
    WithdrawRequest,
};
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status, transport::Server};

pub mod depot {
    tonic::include_proto!("depot");
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("depot_descriptor");
}

mod operations;
use operations::Balance;

#[derive(Debug)]
pub struct MyDepot {
    balance: Arc<Mutex<Balance>>,
}

impl MyDepot {
    fn new() -> Self {
        Self {
            balance: Arc::new(Mutex::new(Balance::new(0.0, 0))),
        }
    }
}

#[tonic::async_trait]
impl Depot for MyDepot {
    async fn deposit(
        &self,
        request: Request<DepositRequest>,
    ) -> Result<Response<TransactionResponse>, Status> {
        let req = request.into_inner();
        let mut balance = self.balance.lock().unwrap();
        balance.add_cash(req.amount);

        Ok(Response::new(TransactionResponse {
            success: true,
            message: "Deposit created successfully".to_string(),
            current_cash: balance.cash,
            current_shares: balance.shares.values().sum(),
        }))
    }

    async fn withdraw(
        &self,
        request: Request<WithdrawRequest>,
    ) -> Result<Response<TransactionResponse>, Status> {
        let req = request.into_inner();
        let mut balance = self.balance.lock().unwrap();
        balance.take_cash(req.amount);

        Ok(Response::new(TransactionResponse {
            success: true,
            message: "Withdraw successful".to_string(),
            current_cash: balance.cash,
            current_shares: balance.shares.values().sum(),
        }))
    }

    async fn buy_shares(
        &self,
        request: Request<BuyRequest>,
    ) -> Result<Response<TransactionResponse>, Status> {
        let req = request.into_inner();
        let mut balance = self.balance.lock().unwrap();

        let initial_shares_count = *balance.shares.get(&req.symbol).unwrap_or(&0);
        balance.buy_shares(req.count, req.price_per_share, req.symbol.clone());
        let new_shares_count = *balance.shares.get(&req.symbol).unwrap_or(&0);

        let success = new_shares_count > initial_shares_count;
        let message = if success {
            "Buy successful"
        } else {
            "Buy failed (insufficient funds)"
        };

        // Total shares count across all symbols
        let total_shares: i32 = balance.shares.values().sum();

        Ok(Response::new(TransactionResponse {
            success,
            message: message.to_string(),
            current_cash: balance.cash,
            current_shares: total_shares,
        }))
    }

    async fn sell_shares(
        &self,
        request: Request<SellRequest>,
    ) -> Result<Response<TransactionResponse>, Status> {
        let req = request.into_inner();
        let mut balance = self.balance.lock().unwrap();

        let initial_shares_count = *balance.shares.get(&req.symbol).unwrap_or(&0);
        balance.sell_shares(req.count, req.price_per_share, req.symbol.clone());
        let new_shares_count = *balance.shares.get(&req.symbol).unwrap_or(&0);

        let success = new_shares_count < initial_shares_count;
        let message = if success {
            "Sell successful"
        } else {
            "Sell failed (insufficient shares)"
        };

        let total_shares: i32 = balance.shares.values().sum();

        Ok(Response::new(TransactionResponse {
            success,
            message: message.to_string(),
            current_cash: balance.cash,
            current_shares: total_shares,
        }))
    }

    async fn get_state(&self, _request: Request<Empty>) -> Result<Response<StateResponse>, Status> {
        let balance = self.balance.lock().unwrap();
        let history = balance
            .history
            .iter()
            .map(|tx| {
                format!(
                    "[{}] {:?}: Cash: {:.2}, Shares: {}, Symbol: {:?}",
                    tx.timestamp, tx.transaction_type, tx.amount_cash, tx.amount_shares, tx.symbol
                )
            })
            .collect();

        let total_shares: i32 = balance.shares.values().sum();

        Ok(Response::new(StateResponse {
            cash: balance.cash,
            shares: total_shares,
            history,
        }))
    }

    async fn get_gain(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<depot::GainResponse>, Status> {
        let balance = self.balance.lock().unwrap();
        let total_deposited = balance.calculate_total_deposited();
        let current_cash = balance.cash;
        // Gain calculation logic: here we treat gain just as Cash - Deposits.
        // Realistically this should include portfolio value, but we don't have prices.
        // User asked for: "money which was initially added and the money which is now in".
        // We will interpret "gain" as the difference.

        Ok(Response::new(depot::GainResponse {
            total_deposited,
            current_cash,
            gain: current_cash - total_deposited,
        }))
    }

    async fn get_share_balance(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<depot::ShareBalanceResponse>, Status> {
        let balance = self.balance.lock().unwrap();
        let shares = balance
            .shares
            .iter()
            .map(|(symbol, count)| depot::ShareDetails {
                symbol: symbol.clone(),
                count: *count,
            })
            .collect();

        Ok(Response::new(depot::ShareBalanceResponse { shares }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr = "[::]:50051".parse()?;
    let depot = MyDepot::new();

    tracing::info!("DepotServer listening on {}", addr);

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(depot::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    Server::builder()
        .add_service(reflection_service)
        .add_service(DepotServer::new(depot))
        .serve(addr)
        .await?;

    Ok(())
}
