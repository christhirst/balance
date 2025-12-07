use depot::depot_server::{Depot, DepotServer};
use depot::{
    BuyRequest, DepositRequest, Empty, SellRequest, StateResponse, TransactionResponse,
    WithdrawRequest,
};
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status, transport::Server};

pub mod depot {
    tonic::include_proto!("depot");
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
            message: "Deposit successful".to_string(),
            current_cash: balance.cash,
            current_shares: balance.shares,
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
            current_shares: balance.shares,
        }))
    }

    async fn buy_shares(
        &self,
        request: Request<BuyRequest>,
    ) -> Result<Response<TransactionResponse>, Status> {
        let req = request.into_inner();
        let mut balance = self.balance.lock().unwrap();
        // Note: buy_shares in operations prints to stdout on failure, we might want to change that later to return Result
        // For now, we check if cash changed to determine success roughly, or just assume success if logic handles it.
        // Better: update operations to return Result. But for now let's just call it.
        let initial_shares = balance.shares;
        balance.buy_shares(req.count, req.price_per_share);

        let success = balance.shares > initial_shares;
        let message = if success {
            "Buy successful"
        } else {
            "Buy failed (insufficient funds)"
        };

        Ok(Response::new(TransactionResponse {
            success,
            message: message.to_string(),
            current_cash: balance.cash,
            current_shares: balance.shares,
        }))
    }

    async fn sell_shares(
        &self,
        request: Request<SellRequest>,
    ) -> Result<Response<TransactionResponse>, Status> {
        let req = request.into_inner();
        let mut balance = self.balance.lock().unwrap();
        let initial_shares = balance.shares;
        balance.sell_shares(req.count, req.price_per_share);

        let success = balance.shares < initial_shares;
        let message = if success {
            "Sell successful"
        } else {
            "Sell failed (insufficient shares)"
        };

        Ok(Response::new(TransactionResponse {
            success,
            message: message.to_string(),
            current_cash: balance.cash,
            current_shares: balance.shares,
        }))
    }

    async fn get_state(&self, _request: Request<Empty>) -> Result<Response<StateResponse>, Status> {
        let balance = self.balance.lock().unwrap();
        let history = balance
            .history
            .iter()
            .map(|tx| {
                format!(
                    "[{}] {:?}: Cash: {:.2}, Shares: {}",
                    tx.timestamp, tx.transaction_type, tx.amount_cash, tx.amount_shares
                )
            })
            .collect();

        Ok(Response::new(StateResponse {
            cash: balance.cash,
            shares: balance.shares,
            history,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr = "[::1]:50051".parse()?;
    let depot = MyDepot::new();

    tracing::info!("DepotServer listening on {}", addr);

    Server::builder()
        .add_service(DepotServer::new(depot))
        .serve(addr)
        .await?;

    Ok(())
}
