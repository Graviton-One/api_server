use governance_poller::user_address_mapping::Poller;

#[tokio::main]
async fn main() {
    Poller::new().run().await;
}


