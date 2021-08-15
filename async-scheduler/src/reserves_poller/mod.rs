use std::error::Error;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use std::sync::Arc;
use serde_json::Value;
use async_trait::async_trait;
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

#[derive(Default, Debug, Clone)]
pub struct PoolData {
    id: i64,
    gton_address: String,
    network_id: i64,
    node_url: String,
    pool_address: String
}

impl PoolData {
    fn get_pools(&conn: Arc<Pool<ConnectionManager<PgConnection>>>) -> Vec<Self> {
        pools::table.inner_join(dexes::table)
            .inner_join(chains::table)
            .select((pools::id, chains::gton_address, chains::network_id, chains::node_url, pools::pool_address))
            .get_result::<PoolData>(&conn.get().unwrap())
            .unwrap()
    }
    fn set_tvl(id: i64, tvl: i64) -> () {
        diesel::update(pools::table)
        .filter(pools::id.eq(id))
        .set(pools::tvl.eq(tvl))
    }
    fn set_gton_reserves(id: i64, reserves: i64) -> () {
        diesel::update(pools::table)
        .filter(pools::id.eq(id))
        .set(pools::gton_reserves.eq(reserves))
    }
}


async fn retrieve_token<T: Transport>(contract: &Contract<T>, property: &str) -> Result<Address, web3::contract::Error> {
    contract
        .query(property, (), None, Options::default(), None).await
}

pub async fn get_token_price(chain: String, address: String) -> f64 {
    let resp: Value = reqwest::get("https://api.coingecko.com/api/v3/simple/token_price/" + chain + "?contract_addresses=" + address + "&vc_currencies=usd")
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

impl PoolsExtractor {
    pub async fn get_gton_reserves(&self) -> i64 {
        let pools: Vec<PoolData> = PoolData::get_pools(self.pool.clone());
        for pool in pools {
            let http = web3::transports::Http::new(pool.node_url)
            .expect("err creating http");
            let web3 = web3::Web3::new(http);
            let contract = Contract::from_json(web3.eth(), pool.gton_address as Address, include_bytes!("abi/erc20.json"))
            .expect("create erc20 contract");

            let reserves: i64 = contract
            .query("balanceOf", pool.pool_address as Address, None, Options::default(), None)
            .await
            .expect("error getting gton reserves");
            PoolData::set_gton_reserves(pool.id, reserves/10**18)
        }
    }
    pub async fn get_pool_tvl(&self) -> i64{
        let pools: Vec<PoolData> = PoolData::get_pools(self.pool.clone());
        for pool in pools {
            let http = web3::transports::Http::new(pool.node_url)
            .expect("err creating http");
            let web3 = web3::Web3::new(http);
            let reserves = get_pool_reserves(pool.pool_address, web3).await.expect("Error getting pool reserves");
            let price_a: i64 = get_token_price(reserves.token_a).await;
            let price_b: i64 = get_token_price(reserves.token_b).await;
            let tvl = price_a * reserves.token_a_reserves as f64 + price_b * reserves.token_b_reserves as f64;
            PoolData::set_tvl(pool.id, tvl)
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    type WrappedResult<T> = Result<T, Box<dyn Error>>;

    fn new_runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Runtime::new().unwrap()
    }

    // #[test]
    // fn test_pool_reserves_retrieval() -> WrappedResult<()> {
    //     let mut rt = self::new_runtime();
    //     rt.block_on(async {
    //         let pancake_pool = list::pancake();
    //         let first_pool = &pancake_pool.pools[0];
    //         let pool_reserves = get_pool_reserves(first_pool.address, pancake_pool.chain_id.web3_rpc()).await?;

    //         println!("retrieved pool reserves successfully");
    //         println!("pool reserves: {:?} \n", pool_reserves);

    //         Ok(())
    //     })
    // }
}