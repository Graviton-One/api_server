use pool_transaction_poller::TransactionExtractor;

#[tokio::main]
async fn main() {
    TransactionExtractor::new().get_gton_reserves().await;
}