use governance_poller::farm_transactions::FarmsTransactions;

#[tokio::main]
async fn main() {
    FarmsTransactions::new().run().await;
}