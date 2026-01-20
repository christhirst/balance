use depot::depot_client::DepotClient;
use depot::{BuyRequest, DepositRequest, Empty, SellRequest, WithdrawRequest};

pub mod depot {
    tonic::include_proto!("depot");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let _client = DepotClient::connect("http://[::1]:50051").await?;

    /*  tracing::info!("Sending Deposit Request...");
    let response = client.deposit(DepositRequest { amount: 1000.0 }).await?;
    tracing::info!("RESPONSE={:?}", response.into_inner());

    tracing::info!("Sending Buy Request...");
    let response = client
        .buy_shares(BuyRequest {
            count: 10,
            price_per_share: 50.0,
            symbol: "AAPL".to_string(),
        })
        .await?;
    tracing::info!("RESPONSE={:?}", response.into_inner());

    tracing::info!("Sending Sell Request...");
    let response = client
        .sell_shares(SellRequest {
            count: 5,
            price_per_share: 60.0,
            symbol: "AAPL".to_string(),
        })
        .await?;
    tracing::info!("deposit={:?}", response.into_inner());

    tracing::info!("Sending Withdraw Request...");
    let response = client.withdraw(WithdrawRequest { amount: 100.0 }).await?;
    tracing::info!("withdraw={:?}", response.into_inner());

    tracing::info!("Getting Gain...");
    let response = client.get_gain(Empty {}).await?;
    tracing::info!("gain={:?}", response.into_inner());

    tracing::info!("Getting Share Balance...");
    let response = client
        .get_share_balance(StockRequest {
            symbol: "AAPL".to_string(),
        })
        .await?;
    tracing::info!("share_balance={:?}", response.into_inner());

    tracing::info!("Getting State...");
    let response = client.get_state(Empty {}).await?;
    tracing::info!("state={:?}", response.into_inner()); */

    Ok(())
}
