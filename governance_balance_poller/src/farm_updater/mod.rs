use std::error::Error;
use diesel::{
    sql_types::*,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use hex::ToHex;
use tokio::time::{
    sleep,
  Duration,
};
use std::sync::Arc;
use serde_json::Value;
use crate::schema::{
    pools,
};

use ethcontract::web3::{
    self,
    contract::{Contract, Options},
    types::*,
};

pub type Web3Instance = web3::Web3<ethcontract::Http>;

pub fn prepare_amount(reserve: U256, dec: i64) -> f64 {
    reserve.to_f64_lossy() / 10_f64.powf(dec as f64)
}

pub fn create_instance(rpc_url: &str) -> Web3Instance {
    let http = web3::transports::Http::new(String::from(rpc_url).as_str())
        .expect("error creating web3 instance");
    web3::Web3::new(http)
}

pub async fn get_total_supply(web3: &Web3Instance, pool: Address) -> f64 {
    let contract = Contract::from_json(
        web3.eth(),
        pool,
        include_bytes!("./abi/erc20.json"),
    ).expect("error contract creating");
    let res = contract
        .query("totalSupply", (), None, Options::default(), None).await.unwrap();
    prepare_amount(res, 18)
}

pub async fn count_farm_users(token_id: i32, ftm_web3: &Web3Instance, lp_keeper: Address) -> i32 {
    let contract = Contract::from_json(
        ftm_web3.eth(),
        lp_keeper,
        include_bytes!("./abi/lp_keeper.json"),
    ).expect("error contract creating");
    contract
        .query("totalTokenUsers", token_id, None, Options::default(), None).await.unwrap()
}

pub async fn farmed_from_farm(ftm_web3: &Web3Instance, farm_address: Address) -> f64 {
    let contract = Contract::from_json(
        ftm_web3.eth(),
        farm_address,
        include_bytes!("./abi/linear.json"),
    ).expect("error contract creating");
    let res = contract
        .query("totalUnlocked", (), None, Options::default(), None).await.unwrap();
    prepare_amount(res, 18)
}

pub async fn get_locked_amount(web3: &Web3Instance, pool: Address, lock: Address) -> f64 {
    let contract = Contract::from_json(
        web3.eth(),
        pool,
        include_bytes!("./abi/erc20.json"),
    ).expect("error contract creating");
    let res = contract
        .query("balanceOf", lock, None, Options::default(), None).await.unwrap();
        prepare_amount(res, 18)
}

pub fn calc_lp_price(tvl: f64, total_supply: i64) -> f64 {
    tvl / total_supply as f64
}

pub fn calc_lp_liquidity(lp_price: f64, lp_locked: f64) -> f64 {
    lp_price * lp_locked
}


#[derive(Default, Debug, Clone, QueryableByName)]
pub struct FarmsData {
    #[sql_type = "BigInt"]
    id: i64,
    #[sql_type = "Float8"]
    tvl: f64,
    #[sql_type = "Text"]
    node_url: String,
    #[sql_type = "Text"]
    pool_address: String,
    #[sql_type = "Int4"]
    token_id: i32,
    #[sql_type = "Text"]
    lock_address: String,
    #[sql_type = "Text"]
    farm_linear_address: String,
}

impl FarmsData {
    fn get_pools(conn: Arc<Pool<ConnectionManager<PgConnection>>>) -> Vec<FarmsData> {
        diesel::sql_query("SELECT p.id, c.gton_address, c.coingecko_id, c.node_url, p.pool_address 
        FROM chains AS c 
        LEFT JOIN dexes AS d ON d.chain_id = c.id 
        LEFT JOIN pools AS p ON d.id = p.dex_id;").get_results::<FarmsData>(&conn.get().unwrap())
        .unwrap()
    }
    fn set_tvl(id: i64, tvl: f64, conn: Arc<Pool<ConnectionManager<PgConnection>>>) -> () {
        diesel::update(pools::table)
        .filter(pools::id.eq(id))
        .set(pools::tvl.eq(tvl))
        .execute(&conn.get().unwrap())
        .unwrap();
    }
    fn set_gton_reserves(
        id: i64, 
        reserves: f64, 
        conn: Arc<Pool<ConnectionManager<PgConnection>>>
    ) -> () {
        diesel::update(pools::table)
        .filter(pools::id.eq(id))
        .set(pools::gton_reserves.eq(reserves))
        .execute(&conn.get().unwrap())
        .unwrap();
    }
}

pub struct FarmUpdater {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    lp_keeper: Address
}

pub fn string_to_h160(string: String) -> ethcontract::H160 {
    ethcontract::H160::from_slice(String::from(string).as_bytes())
}

impl FarmUpdater {
    pub fn new() -> Self {
        let manager = ConnectionManager::<PgConnection>::new(
            std::env::var("DATABASE_URL").expect("missing db url"),
        );
        let pool = Pool::builder().build(manager).expect("pool build");

        let pool = std::sync::Arc::new(pool);
        FarmUpdater {
            pool,
            lp_keeper: string_to_h160(String::from("0xA0447eE66E44BF567FF9287107B0c3D2F88efD93"))
        }
    }
    pub async fn update_farms(&self) -> () {
        loop {
        let farms: Vec<FarmsData> = FarmsData::get_farms(self.pool.clone());
        for farm in farms {
            let web3 = create_instance(&farm.node_url);
            }
        sleep(Duration::from_secs((15) as u64)).await;
        }
    }
}
