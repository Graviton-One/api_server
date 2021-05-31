#[macro_use]
extern crate diesel;
use tokio::time::{
    delay_for, 
      Duration
};
use diesel_migrations::run_pending_migrations;
use tokio_diesel::*;

use web3::types::*;
use web3::ethabi::{
    Topic,
    TopicFilter,
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
pub mod schema;
use tokio::prelude::*;

use schema::pollers_data;
use serde;
use serde::{
    Serialize,
    Deserialize,
};

#[derive(Serialize,Deserialize)]
pub struct Data {
    pub from: String,
    pub to: String,
    pub amount: i64,
}

impl Data {
    pub async fn insert(
        &self,
        conn: &PgConnection,
    ) {
        diesel::sql_query("call add_new_value($1,$2,$3)")
            .bind::<diesel::sql_types::Varchar,_>(self.to.clone())
            .bind::<diesel::sql_types::Varchar,_>(self.from.clone())
            .bind::<diesel::sql_types::BigInt,_>(self.amount)
            .execute(conn)
            .unwrap();
    }
}

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
        conn: &PgConnection, 
    ) {
        diesel::update(pollers_data::table)
            .filter(pollers_data::poller_id.eq(id))
            .set(pollers_data::block_id.eq(num))
            .execute(conn)
            .unwrap();
    }

    pub async fn get(
        id: i32,
        conn: &PgConnection, 
    ) -> i64 {
        pollers_data::table
            .filter(pollers_data::poller_id.eq(id))
            .select(pollers_data::block_id)
            .get_result::<i64>(conn)
            .unwrap()
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let manager =
        ConnectionManager::<PgConnection>::new(std::env::var("DATABASE_URL")
        .expect("Add db url"));
    let pool = Pool::builder().build(manager).expect("pool build");

    match run_pending_migrations(&pool.get().unwrap()) {
        Ok(_) => print!("migration success\n"),
        Err(e)=> print!("migration error: {}\n",&e),
    };

    let balance_keeper = std::env::var("BALANCE_KEEPER_ADDRESS")
        .expect("failed to get address");
    let balance_keeper: Address = balance_keeper.parse().unwrap();

    let method_topic = std::env::var("METHOD_TOPIC")
        .expect("failed to get method hash");
    let method_topic: H256 = method_topic.parse().unwrap();

    let http = web3::transports::Http::new("https://rpcapi.fantom.network")
        .expect("err creating http");
    let web3 = web3::Web3::new(http);

    let dig = U256::from_dec_str(&std::env::var("DIGITS_DIVISION").unwrap()).unwrap();
    loop {
        let num = PollerState::get(1, &pool.get().unwrap()).await;
        let prev_block = BlockNumber::Number(num.into());
        let current_block_num = web3.eth().block_number().await.unwrap();
        let current_block = BlockNumber::Number(current_block_num);
        let mut topics = TopicFilter::default();
        topics.topic0 = Topic::This(method_topic);

        let filter = FilterBuilder::default()
                    .from_block(prev_block) 
                    .to_block(current_block)
                    .address(vec![balance_keeper])
                    .topic_filter(topics)
                    .build();
        let result: Vec<web3::types::Log> = web3.eth().logs(filter).await.unwrap();
        println!("starting from block {} to block {} ...",num,current_block_num);
        for block in result {
            use std::ops::Index;
            //println!("transaction id {}", block.transaction_hash.unwrap());
            let to = hex::encode(block.topics[2]);
            let to = &to[to.len()-40..to.len()];
            let to = "0x".to_string() + to;
            let to = to.to_lowercase();
            //println!("to {}", to);

            let from = hex::encode(block.topics[1]);
            let from = &from[from.len()-40..from.len()];
            let from = "0x".to_string() + from;
            let from = from.to_lowercase();
            //println!("from {}",from);

            let mut amount: U256 = block.topics[3].as_bytes().into();
            amount = amount.checked_div(U256::from_dec_str("10")
                .unwrap().pow(dig)).unwrap();
            let amount: i64 = amount.as_u128() as i64;
            //println!("amount {}", amount);

            let d = Data{
                from: from,
                to: to,
                amount: amount,
            };
            d.insert(&pool.get().unwrap()).await;
            //println!("---------------------------------");
        }
        PollerState::save(1, 
                (current_block_num.as_u64()+1) as i64, 
                &pool.get().unwrap())
            .await;

        delay_for(Duration::from_secs((60) as u64)).await;
    }
}
