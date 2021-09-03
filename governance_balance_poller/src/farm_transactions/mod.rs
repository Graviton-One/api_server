use std::error::Error;
use bigdecimal::BigDecimal;
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


#[derive(Insertable)]
#[table_name = "farm_transactions"]
pub struct FarmTxn {
    id: i64,
    farm_id: i64,
    amount: BigDecimal,
    tx_type: String,
    tx_hash: String,
    stamp: NaiveDateTime,
    user_address: String,
}

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



pub async fn track_txns(
    web3: Web3Instance,
    prev_block: BlockNumber, 
    current_block: BlockNumber,
    method_topic: H256,
    balance_keeper: Address,
    farm_address: Address,
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
) -> Vec<Data> {
        let mut topics = TopicFilter::default();
        topics.topic0 = Topic::This(method_topic);

        let filter = FilterBuilder::default()
                    .from_block(prev_block) 
                    .to_block(current_block)
                    .address(vec![balance_keeper])
                    .topic_filter(topics)
                    .build();
        let result: Vec<web3::types::Log> = web3.eth().logs(filter).await.unwrap();
        //println!("starting from block {:?} to block {:?} ...",prev_block,current_block);
        let mut r: Vec<Data> = Vec::new();
        for block in result {
            use std::ops::Index;

            let from = hex::encode(block.topics[1]);
            let from = &from[from.len()-40..from.len()];
            let t: Address = from.parse().unwrap();

            println!("TRANSACTION {:?}",block.transaction_hash);
            println!("from: {:?} == farm: {:?}",t,farm_address);
            if t == farm_address {
                println!("skipping");
                continue;
            }
            let to = block.topics[2].as_bytes();
            let to: U256 = to.into();
            let from = "0x".to_string() + from;
            let from = from.to_lowercase();

            let amount: U256 = block.data.0.index(0..32).into();

            let d = Data{
                from: from,
                to: to.to_string(),
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

        let add_txn_topic: H256 = "".parse().unwrap();
        let remove_txn_topic: H256 = "".parse().unwrap(); 

        FarmsTransactions {
            pool,
            add_txn_topic,
            remove_txn_topic
        }
    }
    pub async fn run(&self) -> () {
        loop {
        let pools: Vec<PoolData> = Farms::get_pools(self.pool.clone());
        for pool in pools {
            let web3 = create_instance(&pool.node_url);
            let contract_address: Address = pool.gton_address.parse().unwrap();
            let contract = Contract::from_json(web3.eth(), contract_address, include_bytes!("abi/erc20.json"))
            .expect("create erc20 contract");
            let query_address: Address = pool.pool_address.parse().unwrap();
            let reserves: U256 = contract
            .query("balanceOf",  query_address, None, Options::default(), None)
            .await
            .expect("error getting gton reserves");
            // let reserves = BigDecimal::from_str(&reserves.to_string()).unwrap();
            let reserves = prepare_reserve(reserves, 18);
            PoolData::set_gton_reserves(pool.id, reserves, self.pool.clone());
            }
        sleep(Duration::from_secs((15) as u64)).await;
        }
    }
}
