use governance_poller::users_total_balances::Poller;

#[tokio::main]
async fn main() {
    Poller::new().run().await;
}


