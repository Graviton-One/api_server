
use governance_poller::split_by_sources::KeeperExtractor;

#[tokio::main]
async fn main() {
    KeeperExtractor::new().run().await;
}