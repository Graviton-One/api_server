#[macro_use]
extern crate diesel;
use tokio::time::{
    delay_for, 
      Duration
};
use crate::schema::{uni_stats, dodo_stats};
use diesel_migrations::run_pending_migrations;
use tokio_diesel::*;
use web3::types::*;
use web3::ethabi::{
    Topic,
    TopicFilter,
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

use diesel::{PgConnection, row};
use diesel::r2d2::ConnectionManager;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

mod constants;
pub mod db;
mod utils;
pub mod schema;
use tokio::prelude::*;

use schema::pollers_data;
use serde;
use serde::{
    Serialize,
    Deserialize,
};

#[derive(Serialize,Deserialize,Queryable)]
pub struct PollerState {
    id: i32,
    block_id: i64,
    poller_id: i32, 
}

impl PollerState {
    pub async fn save(
        id: i32,
        num: i64,
        conn: &PgConnection, 
    ) {
        diesel::update(pollers_data::table)
            .filter(pollers_data::poller_id.eq(id))
            .set(pollers_data::block_id.eq(num))
            .execute(conn)
            .unwrap();
    }
    
    pub async fn get(
        id: i32,
        conn: &PgConnection, 
    ) -> i64 {
        pollers_data::table
            .filter(pollers_data::poller_id.eq(id))
            .select(pollers_data::block_id)
            .get_result::<i64>(conn)
            .unwrap()
    }
}

pub fn get_volume(block: web3::types::Log, gton_first: bool, gton_dig: U256) -> f64 {
        // 4 variables for 32 bytes length
        use std::ops::Index;
        let mut start = 0;
        let mut end = 0;
        if(gton_first){
            start = 0;
            end = 64;
        } else {
            start = 65;
            end = 128;
        }
        let gton_amount = if(gton_first) {
            block.data[0..64]
        } else {
            block.data[65..128]
        };
        let amount: U256 = if(f64::from_be_bytes(gton_amount[start..end-32]) > 0) {
            gton_amount[start..end-32].parse().unwrap();
        } else {
            gton_amount[start+32..end].parse().unwrap();
        };
        utils::prepare_volume(amount, gton_dig)
}

pub async fn get_pool_events(web3: Web3Instance, &pool: DbPool, gton_pool: GtonPool, method_topic: H256) -> Vec<web3::types::Log> {
    let num = PollerState::get(gton_pool.poller_id, pool.get().unwrap()).await;
    let prev_block = BlockNumber::Number(num.into());
    let current_block_num = web3.eth().block_number().await.unwrap();
    let current_block = BlockNumber::Number(current_block_num);
    let filter = build_filter(method_topic, gton_pool.address, prev_block, current_block);
    PollerState::save(gton_pool.poller_id, 
        (current_block_num.as_u64()+1) as i64, 
        pool.get().unwrap())
    .await;
    web3.eth().logs(filter).await.unwrap()
}

pub async fn get_dodo_volume(web3: Web3Instance, &pool: DbPool, gton_address: H256, gton_dig: U256) -> f64 {
    let topic = std::env::var("DODO_TOPIC")
                        .expect("failed to get method hash");
    let topic: H256 = topic.parse().unwrap();
    let data = get_pool_events(web3, &pool, gton_pool, topic).await;
    process_dodo(data, gton_address)
}

pub fn process_dodo(data: Vec<Log>, gton_address: H256) -> f64 {
    use std::ops::Index;
    let first_address: H256 = block.data[0..32].parse().unwrap();
    let mut volume: f64 = 0;
    for block in data {
        let block_vol: U256 = if(first_address == gton_address) {
            // f64::from_be_bytes(block.data[65..96]);
            block.data[65..96].parse().unwrap()
        } else {
            block.data[129..160].parse().unwrap()
        };
        volume += utils::prepare_volume(block_vol, gton_dig);
    }
    return volume;
}

pub async fn get_uni_volume(web3: Web3Instance, &pool: DbPool, gton_dig: U256) -> f64 {
    let topic = std::env::var("UNI_TOPIC")
    .expect("failed to get method hash");
    let topic: H256 = topic.parse().unwrap();
    let data = get_pool_events(web3, &pool, gton_pool, topic).await;
    let mut volume: f64 = 0;
    for block in data {
        let block_vol: U256 = block.data[65..96].parse().unwrap();
        volume += utils::prepare_volume(volume, gton_dig);
    }
    return volume;
}


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let manager =
        ConnectionManager::<PgConnection>::new(std::env::var("DATABASE_URL")
        .expect("Add db url"));
    let pool = Pool::builder().build(manager).expect("pool build");

    match run_pending_migrations(&pool.get().unwrap()) {
        Ok(_) => print!("migration success\n"),
        Err(e)=> print!("migration error: {}\n",&e),
    };
    let pools = constants::getPools();

    let method_topic = std::env::var("METHOD_TOPICV2")
        .expect("failed to get method hash");
    let method_topic: H256 = method_topic.parse().unwrap();

    // ethereum gton address. it is used in dodo volume getters
    let gton_address = std::env::var("GTON_ADDRESS")
        .expect("failed to get method hash");
    let gton_address: H256 = gton_address.parse().unwrap();

    let eth_provider = utils::create_instance("https://mainnet.infura.io/v3/ec6afadb1810471dbb600f24b86391d2");
    let gton_dig = U256::from_dec_str(&std::env::var("DIGITS_DIVISION").unwrap()).unwrap();
    loop {
        let price = db.getLastPrice(&pool);
        for gton_pool in pools {
            let provider = utils::create_instance(gton_pool.url);
            let result = get_pool_events(provider, &pool, gton_pool, method_topic).await;
            let mut volume: f64 = 0;
            for block in result {
                gton_volume = get_volume(block, gton_pool.gton_first, gton_dig);
                volume += gton_volume * price;
            }
            let data = db::PoolData{volume: volume, tvl: 0, addresses_count: 0, apy: 0};
            data.insert(gton_pool.table, &pool);
        }
        let dodo_volume = get_dodo_volume(eth_provider, &pool, gton_address, gton_dig).await;
        let dodo_data = db::PoolData{volume: dodo_volume * price, tvl: 0, addresses_count: 0, apy: 0};
        dodo_data.insert(dodo_stats, &pool);
        let uni_volume = get_uni_volume(eth_provider, &pool);
        let uni_data = db::PoolData{volume: uni_volume * price, tvl: 0, addresses_count: 0, apy: 0};
        uni_data.insert(uni_stats, &pool);
        delay_for(Duration::from_secs((60) as u64)).await;
    }
}
