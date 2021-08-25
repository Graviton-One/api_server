
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

pub mod db;
use self::db::{
    PoolAddressess, SinglePool
};

pub type Web3Instance = web3::Web3<ethcontract::Http>;


pub fn create_instance(rpc_url: &str) -> Web3Instance {
    let http = web3::transports::Http::new(String::from(rpc_url).as_str())
        .expect("error creating web3 instance");
    web3::Web3::new(http)
}

pub struct TransactionExtractor {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

pub async fn extract_pool_transactions(web3: &Web3Instance, pool: SinglePool) -> () {
    
}

impl TransactionExtractor {
    pub fn new() -> Self {
        let manager = ConnectionManager::<PgConnection>::new(
            std::env::var("DATABASE_URL").expect("missing db url"),
        );
        let pool = Pool::builder().build(manager).expect("pool build");

        let pool = std::sync::Arc::new(pool);
        TransactionExtractor {
            pool,
        }
    }
    pub fn process_pools(&self) -> {
        let data = PoolAddressess::get_pool_addresses(self.pool.clone());
        for chain in data {
            let web3 = create_instance(chain.node_url);
            for pool in chain.pools {
                extract_pool_transactions(&web3, pool);
            }
        }
    }
}