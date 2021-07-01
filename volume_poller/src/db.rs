use diesel::prelude::*;
use serde::{
    Serialize,
    Deserialize,
};
use crate::schema::gton_price;

#[derive(Insertable,Serialize,Deserialize,Queryable,Clone,Debug)]
pub struct PoolData {
    tvl: Option<i64>,
    volume: Option<f64>,
    addresses_count: Option<i32>,
    apy: Option<i32>,
}

pub fn getLastPrice(&pool:&PgConnection) -> f64 {
    diesel::sql_query("SELECT price FROM gton_price ORDER BY ID DESC LIMIT 1")
    .load(pool)
}

impl PoolData {
    pub async fn insert(
        &self,
        table: diesel::table,
        conn: &PgConnection, 
    ) -> () {
        diesel::insert_into(table)
            .values(self)
            .get_results(conn)
            .map_err(|e|e.into());
    }
}