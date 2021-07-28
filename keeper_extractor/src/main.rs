#[macro_use]
extern crate diesel;
use diesel::r2d2;
use tokio::time::{
    delay_for, 
      Duration
};
use bigdecimal::BigDecimal;
use diesel_migrations::run_pending_migrations;
use tokio_diesel::*;
use std::str::FromStr;

use web3::transports::Http;
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

pub struct Data {
    pub from: String,
    pub to: String,
    pub amount: BigDecimal,
}

impl Data {
    pub async fn insert(
        data: Vec<Self>,
        conn: &PgConnection,
    ) {
        conn.build_transaction()
            .read_write()
            .run::<_, diesel::result::Error, _>(|| {
                for el in data {
                    diesel::sql_query("select * from add_new_value_func($1,$2,$3)")
                        .bind::<diesel::sql_types::Varchar,_>(el.to.clone())
                        .bind::<diesel::sql_types::Varchar,_>(el.from.clone())
                        .bind::<diesel::sql_types::Numeric,_>(el.amount.clone())
                        .execute(conn)
                        .unwrap();
                }
                Ok(())
            })
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

pub async fn farms_tracker(
    web3: &web3::Web3<Http>,
    prev_block: BlockNumber, 
    current_block: BlockNumber,
    farm_method_topic: H256,
    farm_address: Address,
    pool: r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
) -> Vec<Data> {
        let mut topics = TopicFilter::default();
        topics.topic0 = Topic::This(farm_method_topic);

        let filter = FilterBuilder::default()
                    .from_block(prev_block) 
                    .to_block(current_block)
                    .address(vec![farm_address])
                    .topic_filter(topics)
                    .build();
        let result: Vec<web3::types::Log> = web3.eth().logs(filter).await.unwrap();
        //println!("starting from block {:?} to block {:?} ...",prev_block,current_block);
        let mut r: Vec<Data> = Vec::new();
        for block in result {
            use std::ops::Index;
                let to: U256 = block.topics[3].as_bytes().into();
                let to = to.to_string();

                let from = hex::encode(block.topics[2]);
                let from = &from[from.len()-40..from.len()];
                let from = "0x".to_string() + from;
                let from = from.to_lowercase();

                let amount: U256 = block.data.0.index(32..64).into();
                let amount = BigDecimal::from_str(&amount.to_string()).unwrap();
                println!("TRANSACTION {:?}",block.transaction_hash);
                println!("am: {}",amount);
                let d = Data{
                    from: from,
                    to: to,
                    amount: amount,
                };
            r.push(d);
        }
        r
}

pub async fn plain_tracker(
    web3: &web3::Web3<Http>,
    prev_block: BlockNumber, 
    current_block: BlockNumber,
    method_topic: H256,
    balance_keeper: Address,
    farm_address: Address,
    pool: r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
) -> Vec<Data> {
        let mut topics = TopicFilter::default();
        topics.topic0 = Topic::This(method_topic);

        let filter = FilterBuilder::default()
                    .from_block(prev_block) 
                    .to_block(current_block)
                    .address(vec![balance_keeper])
                    .topic_filter(topics)
                    .build();
        let result: Vec<web3::types::Log> = web3.eth().logs(filter).await.unwrap();
        //println!("starting from block {:?} to block {:?} ...",prev_block,current_block);
        let mut r: Vec<Data> = Vec::new();
        for block in result {
            use std::ops::Index;

            let from = hex::encode(block.topics[1]);
            let from = &from[from.len()-40..from.len()];
            let t: Address = from.parse().unwrap();

            println!("TRANSACTION {:?}",block.transaction_hash);
            println!("from: {:?} == farm: {:?}",t,farm_address);
            if t == farm_address {
                println!("skipping");
                continue;
            }
            let to = block.topics[2].as_bytes();
            let to: U256 = to.into();
            let from = "0x".to_string() + from;
            let from = from.to_lowercase();

            let amount: U256 = block.data.0.index(0..32).into();

            let d = Data{
                from: from,
                to: to.to_string(),
                amount: BigDecimal::from_str(&amount.to_string()).unwrap(),
            };
            r.push(d);
        }
        r
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let manager =
        ConnectionManager::<PgConnection>::new(std::env::var("DATABASE_URL")
        .expect("Add db url"));
    let pool = Pool::builder().build(manager).expect("pool build");

    //match run_pending_migrations(&pool.get().unwrap()) {
    //    Ok(_) => print!("migration success\n"),
    //    Err(e)=> print!("migration error: {}\n",&e),
    //};
    //fuck

    let balance_keeper = std::env::var("BALANCE_KEEPER_ADDRESS")
        .expect("failed to get address");
    let balance_keeper: Address = balance_keeper.parse().unwrap();

    let farmer = std::env::var("FARMER_ADDRESS")
        .expect("failed to get address");
    let farmer: Address = farmer.parse().unwrap();

    let add_method_topic = 
        "0xc264f49177bdbe55a01fae0e77c3fdc75d515d242b32bc4d56c565f5b47865ba";
    let add_method_topic: H256 = add_method_topic.parse().unwrap();

    let farm_method_topic = 
        "0xdb82536d6a90c757b9cecfe267e7dd17bbb96cb1acd169e21771d6b816ab0bc4";
    let farm_method_topic: H256 = farm_method_topic.parse().unwrap();

    let http = web3::transports::Http::new("https://rpc.ftm.tools")
        .expect("err creating http");
    let web3 = web3::Web3::new(http);
    println!("starting");

    loop {
        let num = PollerState::get(1, &pool.get().unwrap()).await;
        let prev_block = BlockNumber::Number(num.into());
        let current_block_num = web3.eth().block_number().await.unwrap();
        let current_block_num = (current_block_num-U64::from(10))
            .min(U64::from(num + 1000));
        let current_block = BlockNumber::Number(current_block_num);

        let mut r = plain_tracker(
            &web3, 
            prev_block, 
            current_block, 
            add_method_topic, 
            balance_keeper, 
            farmer, 
            pool.clone()).await;
        let mut ap = farms_tracker(
            &web3, 
            prev_block, 
            current_block, 
            farm_method_topic, 
            farmer, 
            pool.clone()).await;
        r.append(&mut ap);
        Data::insert(r,&pool.get().unwrap()).await;
        PollerState::save(1, 
                (current_block_num.as_u64()+1) as i64, 
                &pool.get().unwrap())
            .await;

        delay_for(Duration::from_secs((1) as u64)).await;
    }
}
