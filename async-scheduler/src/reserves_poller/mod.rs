use std::error::Error;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use std::sync::Arc;
use serde_json::Value;
use crate::schema::{
    chains,
    dexes,
    pools,
};

use web3::{
    self,
    Transport,
    contract::{Contract, Options},
};
use ethcontract::prelude::*;

pub type Web3Instance = web3::Web3<ethcontract::Http>;


#[derive(Default, Debug, Clone)]
pub struct PoolStats {
    pub token_a: Address,
    pub token_b: Address,
    pub token_a_reserves: U256,
    pub token_b_reserves: U256,
}

#[derive(Default, Debug, Clone, AsChangeset, Queryable)]
#[table_name = "pools"]

pub struct PoolData {
    id: i64,
    pool_address: String
}

#[derive(Default, Debug, Clone, AsChangeset, Queryable)]
#[table_name = "chains"]
pub struct ChainData {
    id: i64,
    gton_address: String,
    coingecko_id: String,
    network_id: i64,
    node_url: String,
}

pub fn create_instance(rpc_url: &str) -> Web3Instance {
    let http = web3::transports::Http::new(String::from(rpc_url).as_str())
        .expect("error creating web3 instance");
    web3::Web3::new(http)
}

impl PoolData {
    fn get_pools(conn: Arc<Pool<ConnectionManager<PgConnection>>>) -> Vec<ChainData, PoolData> {
        pools::table.inner_join(dexes::table)
            .inner_join(chains::table)
            .select((pools::id, chains::gton_address, chains::network_id, chains::coingecko_id, chains::node_url, pools::pool_address))
            .get_result::<PoolData>(&conn.get().unwrap())
            .unwrap()
    }
    fn set_tvl(id: i64, tvl: f64) -> () {
        diesel::update(pools::table)
        .filter(pools::id.eq(id))
        .set(pools::tvl.eq(tvl));
    }
    fn set_gton_reserves(id: i64, reserves: i64) -> () {
        diesel::update(pools::table)
        .filter(pools::id.eq(id))
        .set(pools::gton_reserves.eq(reserves));
    }
}


async fn retrieve_token<T: Transport>(contract: &Contract<T>, property: &str) -> Result<Address, web3::contract::Error> {
    contract
        .query(property, (), None, Options::default(), None).await
}

pub async fn get_token_price(chain: String, address: String) -> f64 {
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
        // retrieve_token(&contract, "token0").await?,
        // retrieve_token(&contract, "token1").await?,
        contract.query("token0", (), None, Options::default(), None).await?,
        contract.query("token1", (), None, Options::default(), None).await?,

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
    pub async fn get_gton_reserves(&self) -> () {
        let pools: Vec<PoolData> = PoolData::get_pools(self.pool.clone());
        for pool in pools {
            let http = web3::transports::Http::new(&pool.node_url)
            .expect("err creating http");
            let web3 = web3::Web3::new(http);
            let contract_address = string_to_h160(pool.gton_address);
            let contract = Contract::from_json(web3.eth(), contract_address, include_bytes!("abi/erc20.json"))
            .expect("create erc20 contract");
            let query_address = string_to_h160(pool.pool_address);
            let reserves: i64 = contract
            .query("balanceOf",  query_address, None, Options::default(), None)
            .await
            .expect("error getting gton reserves");
            PoolData::set_gton_reserves(pool.id, reserves/10^18);
        }
    }
    pub async fn get_pool_tvl(&self) -> (){
        let pools: Vec<PoolData> = PoolData::get_pools(self.pool.clone());
        for pool in pools {
            let http = web3::transports::Http::new(&pool.node_url)
            .expect("err creating http");
            let web3 = create_instance(&pool.node_url);
            let reserves = get_pool_reserves(&pool.pool_address, web3).await.expect("Error getting pool reserves");
            let price_a: f64 = get_token_price(pool.coingecko_id, reserves.token_a.to_string()).await;
            let price_b: f64 = get_token_price(pool.coingecko_id, reserves.token_b.to_string()).await;
            let tvl = price_a * reserves.token_a_reserves.to_f64_lossy() + price_b * reserves.token_b_reserves.to_f64_lossy();
            PoolData::set_tvl(pool.id, tvl);
        }
    }
}
