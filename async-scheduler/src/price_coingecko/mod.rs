use tokio::time::{
    delay_for,
    Duration
};
use tokio_diesel::*;
use reqwest;
use serde_json::Value;
use std::sync::Arc;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

pub struct CoingeckoPrice {
    pub pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl CoingeckoPrice {
    pub fn new(
        pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    ) -> Self {
        CoingeckoPrice {
            pool,
        }
    }
    pub async fn run(
        &self
    ) {
        println!("starting price polling");
        let resp: Value = reqwest::get("https://api.coingecko.com/api/v3/coins/graviton")
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();
        let val = resp["market_data"]["current_price"]["usd"].as_f64().unwrap();
        diesel::sql_query("INSERT INTO gton_price (price) VALUES ($1)")
            .bind::<diesel::sql_types::Double,_>(val)
            .execute_async(&self.pool)
            .await
            .expect("exec err");
    }
}
