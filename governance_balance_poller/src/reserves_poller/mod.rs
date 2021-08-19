use std::error::Error;
use diesel::{
    sql_types::*,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use tokio::time::{
    sleep,
  Duration,
};
use std::sync::Arc;
use serde_json::Value;
use crate::schema::{
    pools,
};

use web3::{
    self,
    contract::{Contract, Options},
    types::*,
};

pub type Web3Instance = web3::Web3<ethcontract::Http>;


#[derive(Default, Debug, Clone)]
pub struct PoolStats {
    pub token_a: Address,
    pub token_b: Address,
    pub token_a_reserves: U256,
    pub token_b_reserves: U256,
}

#[derive(Default, Debug, Clone, QueryableByName)]
pub struct PoolData {
    #[sql_type = "BigInt"]
    id: i64,
    #[sql_type = "Text"]
    gton_address: String,
    #[sql_type = "Text"]
    coingecko_id: String,
    #[sql_type = "Text"]
    node_url: String,
    #[sql_type = "Text"]
    pool_address: String
}

pub fn create_instance(rpc_url: &str) -> Web3Instance {
    let http = web3::transports::Http::new(String::from(rpc_url).as_str())
        .expect("error creating web3 instance");
    web3::Web3::new(http)
}

impl PoolData {
    fn get_pools(conn: Arc<Pool<ConnectionManager<PgConnection>>>) -> Vec<PoolData> {
        diesel::sql_query("SELECT c.id, c.gton_address, c.coingecko_id, c.node_url, p.pool_address 
        FROM chains AS c 
        LEFT JOIN dexes AS d ON d.chain_id = c.id 
        LEFT JOIN pools AS p ON d.id = p.dex_id;").get_results::<PoolData>(&conn.get().unwrap())
        .unwrap()
    }
    fn set_tvl(id: i64, tvl: f64, conn: Arc<Pool<ConnectionManager<PgConnection>>>) -> () {
        diesel::update(pools::table)
        .filter(pools::id.eq(id))
        .set(pools::tvl.eq(tvl))
        .execute(&conn.get().unwrap())
        .unwrap();
    }
    fn set_gton_reserves(id: i64, reserves: f64, conn: Arc<Pool<ConnectionManager<PgConnection>>>) -> () {
        diesel::update(pools::table)
        .filter(pools::id.eq(id))
        .set(pools::gton_reserves.eq(reserves))
        .execute(&conn.get().unwrap())
        .unwrap();
    }
}


async fn retrieve_token<T: web3::Transport>(
    contract: &Contract<T>, 
    property: &str
) -> Result<Address, web3::contract::Error> {
    contract
        .query(property, (), None, Options::default(), None).await
}

pub async fn get_token_price(chain: &String, address: &String) -> f64 {
    let resp: Value = reqwest::get(String::from("https://api.coingecko.com/api/v3/simple/token_price/") + &chain + "?contract_addresses=" + &address + "&vc_currencies=usd")
    .await
    .unwrap()
    .json::<Value>()
    .await
    .unwrap();
    resp[address]["usd"].as_f64().unwrap()
}

pub async fn get_pool_reserves(
    pool_address: &str,
    web3: Web3Instance,
) -> Result<PoolStats, Box<dyn Error>> {
    let contract = Contract::from_json(
        web3.eth(),
        pool_address.parse().unwrap(),
        include_bytes!("./abi/pancakeV2pair.json"),
    ).expect("error contract creating");
    let (token_a_reserves, token_b_reserves, _): (U256, U256, U256) = contract
        .query("getReserves", (), None, Options::default(), None).await?;
    let (token_a, token_b) = (
        retrieve_token(&contract, "token0").await?,
        retrieve_token(&contract, "token1").await?,
    );
    
    // PoolStats::default()
    let pool_stats = PoolStats {
        token_a,
        token_b,
        token_a_reserves,
        token_b_reserves,
    };

    Ok(pool_stats)
}

pub struct PoolsExtractor {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

pub fn string_to_h160(string: String) -> ethcontract::H160 {
    ethcontract::H160::from_slice(String::from(string).as_bytes())
}

impl PoolsExtractor {
    pub fn new() -> Self {
        let manager = ConnectionManager::<PgConnection>::new(
            std::env::var("DATABASE_URL").expect("missing db url"),
        );
        let pool = Pool::builder().build(manager).expect("pool build");

        let pool = std::sync::Arc::new(pool);
        PoolsExtractor {
            pool,
        }
    }
    pub async fn get_gton_reserves(&self) -> () {
        loop {
        let pools: Vec<PoolData> = PoolData::get_pools(self.pool.clone());
        for pool in pools {
            let web3 = create_instance(&pool.node_url);
            let contract_address: Address = pool.gton_address.parse().unwrap();
            let contract = Contract::from_json(web3.eth(), contract_address, include_bytes!("abi/erc20.json"))
            .expect("create erc20 contract");
            let query_address: Address = pool.pool_address.parse().unwrap();
            let reserves: i64 = contract
            .query("balanceOf",  query_address, None, Options::default(), None)
            .await
            .expect("error getting gton reserves");
            PoolData::set_gton_reserves(pool.id, (reserves/10^18) as f64, self.pool.clone());
            }
        sleep(Duration::from_secs((15) as u64)).await;
        }
    }
    pub async fn get_pool_tvl(&self) -> (){
        loop {
        let pools: Vec<PoolData> = PoolData::get_pools(self.pool.clone());
        for pool in pools {
            let web3 = create_instance(&pool.node_url);
            let reserves = get_pool_reserves(&pool.pool_address, web3).await.expect("Error getting pool reserves");
            let price_a: f64 = get_token_price(&pool.coingecko_id, &reserves.token_a.to_string()).await;
            let price_b: f64 = get_token_price(&pool.coingecko_id, &reserves.token_b.to_string()).await;
            let tvl = price_a * reserves.token_a_reserves.to_f64_lossy() + price_b * reserves.token_b_reserves.to_f64_lossy();
            PoolData::set_tvl(pool.id, tvl,self.pool.clone());
        }
        sleep(Duration::from_secs((15) as u64)).await;
    }
    }
}
