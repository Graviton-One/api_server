use std::error::Error;
use bigdecimal::{BigDecimal, ToPrimitive};
use std::str::FromStr;
use chrono::NaiveDateTime;
use diesel::{
    sql_types::*,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use web3::ethabi::{
    Topic,
    TopicFilter,
};
use serde::{
    Serialize,
    Deserialize,
};
use hex::ToHex;
use tokio::time::{
    sleep,
  Duration,
};
use std::sync::Arc;
use serde_json::Value;
use crate::schema::{
    farm_transactions,
    pollers_data,
};

use web3::{
    self,
    contract::{Contract, Options},
    types::*,
};

pub type Web3Instance = web3::Web3<ethcontract::Http>;

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
        conn: Arc<Pool<ConnectionManager<PgConnection>>>, 
    ) {
        diesel::update(pollers_data::table)
            .filter(pollers_data::poller_id.eq(id))
            .set(pollers_data::block_id.eq(num))
            .execute(&conn.get().unwrap())
            .unwrap();
    }

    pub async fn get(
        id: i32,
        conn: Arc<Pool<ConnectionManager<PgConnection>>>, 
    ) -> i64 {
        pollers_data::table
            .filter(pollers_data::poller_id.eq(id))
            .select(pollers_data::block_id)
            .get_result::<i64>(&conn.get().unwrap())
            .unwrap()
    }
}
pub fn create_instance(rpc_url: &str) -> Web3Instance {
    let http = web3::transports::Http::new(String::from(rpc_url).as_str())
        .expect("error creating web3 instance");
    web3::Web3::new(http)
}
pub fn prepare_reserve(reserve: U256, dec: i64) -> f64 {
    reserve.to_f64_lossy() / 10_f64.powf(dec as f64)
}

fn hex_to_string<T: ToHex>(h: T) -> String {
    "0x".to_owned() + &h.encode_hex::<String>()
}
pub fn string_to_h160(string: String) -> ethcontract::H160 {
    ethcontract::H160::from_slice(String::from(string).as_bytes())
}

#[derive(Default, Debug, Clone, QueryableByName)]
pub struct Farms {
    #[sql_type = "BigInt"]
    id: i64,
    #[sql_type = "Text"]
    farm_address: String,
    #[sql_type = "Text"]
    node_url: String,
}

impl Farms {
    fn get_farm_addresses(conn: Arc<Pool<ConnectionManager<PgConnection>>>) -> Vec<Farms> {
        diesel::sql_query("SELECT f.id, f.lock_address as farm_address, c.node_url 
        FROM gton_farms AS f 
        LEFT JOIN pools AS p ON f.pool_id = p.id 
        LEFT JOIN dexes AS d ON p.dex_id = d.id 
        LEFT JOIN chains AS c ON c.id = d.chain_id;").get_results::<Farms>(&conn.get().unwrap())
        .unwrap()
    }
}

async fn fetch_stamp(web3: Web3Instance, block_number: U64) -> NaiveDateTime {
    let block: Block<H256> = web3
        .eth()
        .block(BlockNumber::Number(block_number).into())
        .await
        .unwrap().unwrap();
    let stamp_str = block.timestamp.to_string();
    let stamp_big = BigDecimal::from_str(&stamp_str).unwrap();
    let stamp_i64 = stamp_big.to_i64().unwrap();
    NaiveDateTime::from_timestamp(stamp_i64, 0)
}

#[derive(Insertable)]
#[table_name = "farm_transactions"]
pub struct FarmTxn {
    farm_id: i64,
    amount: BigDecimal,
    tx_type: String,
    tx_hash: String,
    stamp: NaiveDateTime,
    user_address: String,
}

pub struct TxnData {
    amount: BigDecimal,
    tx_hash: String,
    stamp: NaiveDateTime,
    user_address: String,
}

pub async fn track_txns(
    web3: Web3Instance,
    prev_block: BlockNumber, 
    current_block: BlockNumber,
    method_topic: H256,
    farm_address: Address,
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
) -> Vec<TxnData> {
        let mut topics = TopicFilter::default();
        topics.topic0 = Topic::This(method_topic);

        let filter = FilterBuilder::default()
                    .from_block(prev_block) 
                    .to_block(current_block)
                    .address(vec![farm_address])
                    .topic_filter(topics)
                    .build();
        let result: Vec<web3::types::Log> = web3.eth().logs(filter).await.unwrap();
        //println!("starting from block {:?} to block {:?} ...",prev_block,current_block);
        let mut r: Vec<TxnData> = Vec::new();
        for log in result {
            use std::ops::Index;

            let from = hex::encode(log.topics[2]);
            let from = &from[from.len()-40..from.len()];

            let to = hex::encode(log.topics[3]);
            let to = &to[to.len()-40..to.len()];

            let tx_hash = log.transaction_hash.unwrap().to_string();
            let amount: U256 = log.data.0.index(0..32).into();
            let stamp: NaiveDateTime = fetch_stamp(web3.clone(), log.block_number.unwrap()).await;

            let d = TxnData{
                user_address: from.to_string(),
                tx_hash,
                stamp,
                amount: BigDecimal::from_str(&amount.to_string()).unwrap(),
            };
            r.push(d);
        }
        r
}

pub struct FarmsTransactions {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    add_txn_topic: H256,
    remove_txn_topic: H256,
}


impl FarmsTransactions {
    pub fn new() -> Self {
        let manager = ConnectionManager::<PgConnection>::new(
            std::env::var("DATABASE_URL").expect("missing db url"),
        );
        let pool = Pool::builder().build(manager).expect("pool build");

        let pool = std::sync::Arc::new(pool);

        let add_txn_topic: H256 = "0xd6aba49fa5adb7dbc18ab12d057e77c75e5d4b345cf473c7514afbbd6f5fc626".parse().unwrap();
        let remove_txn_topic: H256 = "0xd99169b5dcb595fb976fee14578e44584c0ebbbf50cf58d568b3100c59f2f4bb".parse().unwrap(); 

        FarmsTransactions {
            pool,
            add_txn_topic,
            remove_txn_topic
        }
    }
    pub async fn run(&self) -> () {
        loop {
        let pools: Vec<Farms> = Farms::get_farm_addresses(self.pool.clone());
        for pool in pools {
            let web3 = create_instance(&pool.node_url);
            }
        sleep(Duration::from_secs((15) as u64)).await;
        }
    }
}
