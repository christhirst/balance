use depot::depot_client::DepotClient;
use depot::{BuyRequest, DepositRequest, Empty, SellRequest, WithdrawRequest};

pub mod depot {
    tonic::include_proto!("depot");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let mut client = DepotClient::connect("http://[::1]:50051").await?;

    tracing::info!("Sending Deposit Request...");
    let response = client.deposit(DepositRequest { amount: 1000.0 }).await?;
    tracing::info!("RESPONSE={:?}", response.into_inner());

    tracing::info!("Sending Buy Request...");
    let response = client
        .buy_shares(BuyRequest {
            count: 10,
            price_per_share: 50.0,
        })
        .await?;
    tracing::info!("RESPONSE={:?}", response.into_inner());

    tracing::info!("Sending Sell Request...");
    let response = client
        .sell_shares(SellRequest {
            count: 5,
            price_per_share: 60.0,
        })
        .await?;
    tracing::info!("RESPONSE={:?}", response.into_inner());

    tracing::info!("Sending Withdraw Request...");
    let response = client.withdraw(WithdrawRequest { amount: 100.0 }).await?;
    tracing::info!("RESPONSE={:?}", response.into_inner());

    tracing::info!("Getting State...");
    let response = client.get_state(Empty {}).await?;
    tracing::info!("RESPONSE={:?}", response.into_inner());

    Ok(())
}
