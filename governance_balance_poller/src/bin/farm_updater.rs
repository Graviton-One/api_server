use governance_poller::farm_updater::FarmUpdater;

#[tokio::main]
async fn main() {
    FarmUpdater::new().update_farms().await;
}
