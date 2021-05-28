#[macro_use]
extern crate diesel;
use hex_literal::hex;
use tokio::time::{
    delay_for, 
    Duration
};
use ethabi::TopicFilter;
use tokio_diesel::*;
use web3::types::*;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use super::db::{
    Blocks,
    Data,
};

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    let manager =
        ConnectionManager::<PgConnection>::new(std::env::var("DATABASE_URL")
        .expect("Add db url"));
    let pool = Pool::builder().build(manager).expect("pool build");

    let balance_keeper = std::env::var("BALANCE_KEEPER_ADDRESS").expect("failed to get address");
    let balance_keeper: Address = balance_keeper.parse().unwrap();
    let method_hash = std::env::var("METHOD_HASH").expect("failed to get method hash");
    let method_hash: H256 = H256::from_bytes(method_hash.to_bytes());

    let http = web3::transports::Http::new("https://rpcapi.fantom.network").expect("err creating http");
    let web3 = web3::Web3::new(http);
    loop {
        // let block_data: Data =  ..........................
        // let prev_block = block_data.get(&pool);
        let block = 7979676 as u64;
        let prev_block = BlockNumber::Number(block);
        let current_block = web3.block_number().await;
        let topics = TopicFilter::default();
        topics.topic0 = method_hash;

        let filter = FilterBuilder::default()
                    .from_block(prev_block) // missing
                    .to_block(current_block)
                    .address(balance_keeper)
                    .topic_filter(topics)
                    .build();
        let result = web3.logs(filter).await;
        for block in result {
            let bytes = block.data;
            let user_address = bytes.slice(0, 20);
            println("Got address {}", user_address)
            let giver_address = bytes.slice(20, 40);
            println("Got second address {}", giver_address)
            let amount = bytes.slice_from(40);
            println("Got amount {}", amount)
        }

        delay_for(Duration::from_secs((60*60) as u64)).await;
    }


}
