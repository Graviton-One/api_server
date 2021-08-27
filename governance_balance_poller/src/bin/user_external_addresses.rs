use governance_poller::user_address_mapping::Poller;

#[tokio::main]
async fn main() {
    println!("starting");
    Poller::new().run().await;
}


