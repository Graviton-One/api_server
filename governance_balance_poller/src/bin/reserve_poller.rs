use governance_poller::reserves_poller::PoolsExtractor;

#[tokio::main]
async fn main() {
    PoolsExtractor::new().get_gton_reserves().await;
}