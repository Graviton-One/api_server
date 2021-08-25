use std::sync::Arc;
use diesel::{
    sql_types::*,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use chrono::NaiveDateTime;

#[derive(Default, Debug, Clone, QueryableByName)]
pub struct SinglePool {
    id: i64,
    pool_address: String,
}

#[derive(Default, Debug, Clone, QueryableByName)]
pub struct PoolAddressess {
    node_url: String,
    pools: Vec<SinglePool>
}

impl PoolAddresses {
    fn get_pool_addresses(conn: Arc<Pool<ConnectionManager<PgConnection>>>) -> Vec<PoolAddressess> {
        diesel::sql_query("SELECT c.node_url, ARRAY_AGG(p.id, p.pool_address ORDER BY p.id) pools
        FROM chains AS c 
        LEFT JOIN dexes AS d ON d.chain_id = c.id 
        LEFT JOIN pools AS p ON d.id = p.dex_id;").get_results::<PoolAddressess>(&conn.get().unwrap())
        .unwrap()
    }
}

pub struct PoolTransaction {
    id: i64,
    pool_id: i64,
    txn_type: String,
    txn_hash: String,
    amount: f64,
    value: f64,
    timestamp: NaiveDateTime
    // picture and pool name we will get from pools table
}

impl PoolTransaction {
    fn save_txn(&self) -> () {
        diesel::insert_into(pool_transactions::table)
        .values(self)
        .execute(conn)
        .map_err(|e|e.into());
    }
}