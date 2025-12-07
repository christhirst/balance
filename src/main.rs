mod operations;

use operations::Balance;

fn main() {
    tracing_subscriber::fmt::init();
    let mut depot = Balance::new(1000.0, 0);
    tracing::info!("Initial state: {}", depot.get_state());

    depot.add_cash(500.0);
    tracing::info!("After deposit: {}", depot.get_state());

    depot.buy_shares(10, 100.0);
    tracing::info!("After buying shares: {}", depot.get_state());

    depot.sell_shares(5, 120.0);
    tracing::info!("After selling shares: {}", depot.get_state());

    depot.take_cash(200.0);
    tracing::info!("After withdraw: {}", depot.get_state());

    depot.print_history();
}
