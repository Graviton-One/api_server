#[macro_use]
extern crate diesel;
use tokio::time::{
    delay_for,
    Duration
};
use tokio_diesel::*;
use reqwest;
use serde_json::Value;

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

//aaaaaaaa
#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let manager =
        ConnectionManager::<PgConnection>::new(std::env::var("DATABASE_URL")
        .expect("Add db url"));
    let pool = Pool::builder().build(manager).expect("pool build");

    loop {
        println!("waki waki, it'ss time for work");
        // calling method of instance
        // that should be the price for moment
        let resp: Value = reqwest::get("https://api.coingecko.com/api/v3/coins/graviton")
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();
        let val = resp["market_data"]["current_price"]["usd"].as_f64().unwrap();
        //println!("start push");
        diesel::sql_query("INSERT INTO gton_price (price) VALUES ($1)")
            .bind::<diesel::sql_types::BigInt,_>((val * 100f64) as i64)
            .execute_async(&pool)
            .await
            .expect("exec err");

        println!("value {} going to sleep for 1 hour",val);
        delay_for(Duration::from_secs((60) as u64)).await;
    }
    Ok(())
}
