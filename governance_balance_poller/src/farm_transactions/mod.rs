use bigdecimal::{BigDecimal, ToPrimitive};
use std::{str::FromStr};
use chrono::NaiveDateTime;
use diesel::{
    sql_types::*,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use web3::types::U64;
use web3::ethabi::{
    Topic,
    TopicFilter,
};
use diesel::result::Error;
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
use crate::schema::{
    farms_transactions,
    pollers_data,
};

use web3::{
    self,
    types::*,
};

pub fn create_instance(rpc_url: &str) -> Web3Instance {
    let http = web3::transports::Http::new(String::from(rpc_url).as_str())
        .expect("error creating web3 instance");
    web3::Web3::new(http)
}
pub fn prepare_reserve(reserve: U256, dec: i64) -> f64 {
    reserve.to_f64_lossy() / 10_f64.powf(dec as f64)
}

pub fn string_to_h160(string: String) -> ethcontract::H160 {
    ethcontract::H160::from_slice(String::from(string).as_bytes())
}

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

#[derive(Default, Debug, Clone, QueryableByName)]
pub struct Farms {
    #[sql_type = "BigInt"]
    id: i64,
    #[sql_type = "Integer"]
    poller_id: i32,
    #[sql_type = "Text"]
    lock_address: String,
    #[sql_type = "Text"]
    node_url: String,
}

impl Farms {
    fn get_farm_addresses(conn: Arc<Pool<ConnectionManager<PgConnection>>>) -> Vec<Farms> {
        diesel::sql_query("SELECT f.id, f.lock_address as lock_address, c.node_url, f.poller_id
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
        .unwrap()
        .unwrap();
    let stamp_str = block.timestamp.to_string();
    let stamp_big = BigDecimal::from_str(&stamp_str).unwrap();
    let stamp_i64 = stamp_big.to_i64().unwrap();
    NaiveDateTime::from_timestamp(stamp_i64, 0)
}

fn hex_to_string<T: ToHex>(h: T) -> String {
    "0x".to_owned() + &h.encode_hex::<String>()
}

fn parse_block_number(block_number: U64) -> i64 {
    let block_number_str = block_number.to_string();
    let block_number_big = BigDecimal::from_str(&block_number_str).unwrap();
    block_number_big.to_i64().unwrap()
}

#[derive(Insertable)]
#[table_name = "farms_transactions"]
pub struct FarmTxn {
    farm_id: i64,
    amount: BigDecimal,
    tx_type: String,
    tx_hash: String,
    stamp: NaiveDateTime,
    block_number: i64,
    user_address: String,
}

impl FarmTxn {
    pub fn insert(&self, conn:  Arc<Pool<ConnectionManager<PgConnection>>>, ) -> () {
        diesel::insert_into(farms_transactions::table)
        .values(self)
        .execute(&conn.get().unwrap())
        .unwrap();
    }
}

pub fn push_data(
    num: i64,
    poller_id: i32,
    data: Vec<FarmTxn>, 
    conn: &PgConnection
) -> Result<(),Error> {
    conn.build_transaction()
        .read_write()
        .run::<(), diesel::result::Error, _>(|| {
            for d in data {
                diesel::insert_into(farms_transactions::table)
                    .values(d)
                    .execute(conn)?;
            }
            diesel::update(pollers_data::table.filter(pollers_data::id.eq(poller_id)))
                .set(pollers_data::block_id.eq(num))
                .execute(conn)?;
            Ok(())
        })?;
    Ok(())
}

pub struct TxnData {
    amount: BigDecimal,
    tx_hash: String,
    stamp: NaiveDateTime,
    user_address: String,
    block_number: i64
}

pub async fn track_txns(
    web3: Web3Instance,
    prev_block: BlockNumber, 
    current_block: BlockNumber,
    method_topic: H256,
    farm_address: Address,
) -> Vec<TxnData> {
        let mut topics = TopicFilter::default();
        topics.topic0 = Topic::This(method_topic);

        let filter = FilterBuilder::default()
                    .from_block(prev_block) 
                    .to_block(current_block)
                    .address(vec![farm_address])
                    .topic_filter(topics)
                    .build();
        let result= web3.eth().logs(filter).await;
        let mut r: Vec<TxnData> = Vec::new();
        if result.is_err() {
            return r;
        }

        for log in result.unwrap() {
            use std::ops::Index;

            let from = hex::encode(log.topics[2]);
            let from = "0x".to_owned() + &from[from.len()-40..from.len()];

            let tx_hash = hex_to_string(log.transaction_hash.unwrap());
            let amount: U256 = log.data.0.index(0..32).into();
            let stamp: NaiveDateTime = fetch_stamp(web3.clone(), log.block_number.unwrap()).await;
            let block_number: i64 = parse_block_number(log.block_number.unwrap());

            let d = TxnData{
                user_address: from.to_string(),
                tx_hash,
                stamp,
                block_number,
                amount: BigDecimal::from_str(&amount.to_string()).unwrap(),
            };
            r.push(d);
        }
        r
}

pub struct FarmsTransactions {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    postgres: Pool<ConnectionManager<PgConnection>>,
    add_txn_topic: H256,
    remove_txn_topic: H256,
}


impl FarmsTransactions {
    pub fn new() -> Self {
        let manager = ConnectionManager::<PgConnection>::new(
            std::env::var("DATABASE_URL").expect("missing db url"),
        );
        let postgres = Pool::builder().build(manager).expect("pool build");

        let pool = std::sync::Arc::new(postgres.clone());

        let add_txn_topic: H256 = "0xd6aba49fa5adb7dbc18ab12d057e77c75e5d4b345cf473c7514afbbd6f5fc626".parse().unwrap();
        let remove_txn_topic: H256 = "0xd99169b5dcb595fb976fee14578e44584c0ebbbf50cf58d568b3100c59f2f4bb".parse().unwrap(); 

        FarmsTransactions {
            pool,
            postgres,
            add_txn_topic,
            remove_txn_topic
        }
    }
    pub async fn run(&self) -> () {
        loop {
        let pools: Vec<Farms> = Farms::get_farm_addresses(self.pool.clone());
        for pool in pools {
            let web3 = create_instance(&pool.node_url);
            let last_block = PollerState::get(pool.poller_id, self.pool.clone()).await;
            let current_block = web3.eth().block_number().await.unwrap();
            let current_block = current_block - U64::from(10); // delay to keep all blocks mined
            let mut data: Vec<FarmTxn> = Vec::new(); 
            let res = track_txns(web3.clone(), BlockNumber::Number(U64::from(last_block)),
             BlockNumber::Number(current_block), self.add_txn_topic.clone(), 
             pool.lock_address.parse().unwrap()).await;
// TODO Rewrite to one txn
            for item in res {
                let txn = FarmTxn {
                    amount: item.amount,
                    tx_hash: item.tx_hash,
                    stamp: item.stamp,
                    block_number: item.block_number,
                    user_address: item.user_address,
                    farm_id: pool.id,
                    tx_type: "Add".to_string()
                };
                data.push(txn);
            }

            let res = track_txns(web3.clone(), BlockNumber::Number(U64::from(last_block)),
            BlockNumber::Number(current_block), self.remove_txn_topic.clone(), 
            pool.lock_address.parse().unwrap()).await;
            for item in res {
                let txn = FarmTxn {
                    amount: item.amount,
                    tx_hash: item.tx_hash,
                    stamp: item.stamp,
                    block_number: item.block_number,
                    user_address: item.user_address,
                    farm_id: pool.id,
                    tx_type: "Remove".to_string()
                };
                data.push(txn);
            }
            let pushed = push_data(current_block.low_u64() as i64 + 1, pool.poller_id, data, &self.postgres.clone().get().unwrap());

            }
        sleep(Duration::from_secs((60*15) as u64)).await;
        }
    }
}
