pub mod logic;
use tokio::time::{
        sleep,
      Duration,
};
use std::sync::Arc;

use web3::transports::Http;
use web3::{Web3, types::*};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};


use crate::db_state::PollerState;
use self::logic::{
    BalanceInOutValues,
};

pub struct Poller {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    web3: Web3<Http>,
    balance_keeper: Address,
    add_method_topic: H256,
    sub_method_topic: H256,
    poller_id: i32,
    delay: u64,
}

impl Poller {
    pub fn new(
    ) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(
            std::env::var("DATABASE_URL").expect("missing db url"),
        );
        let pool = Pool::builder().build(manager).expect("pool build");

        let pool = std::sync::Arc::new(pool);
        let http = web3::transports::Http::new("https://rpc.ftm.tools")
            .expect("err creating http");
        let web3 = web3::Web3::new(http);
        let balance_keeper = std::env::var("BALANCE_KEEPER_ADDRESS")
            .expect("failed to get address");
        let balance_keeper: Address = balance_keeper.parse().unwrap();

        let poller_id: i32 = std::env::var("POLLER_ID")
            .expect("failed to get address").parse::<i32>().unwrap();

        let delay: u64 = std::env::var("DELAY")
            .expect("failed to get address").parse::<u64>().unwrap();

        let sub_method_topic = 
            "0x47dd4a08dedb9e7afcf164b736d2a3fcaed9f9d56ec8d5e38aaafde8c22a58a7";
        let sub_method_topic: H256 = sub_method_topic.parse().unwrap();

        let add_method_topic = 
            "0xc264f49177bdbe55a01fae0e77c3fdc75d515d242b32bc4d56c565f5b47865ba";
        let add_method_topic: H256 = add_method_topic.parse().unwrap();

        Poller {
            pool,
            web3,
            balance_keeper,
            add_method_topic,
            sub_method_topic,
            poller_id,
            delay,
        }
    }

    pub async fn run(&self) {
        println!("starting runtime");
        loop {
            let num = PollerState::get(self.poller_id, self.pool.clone());
            let prev_block = BlockNumber::Number(num.into());
            let current_block_num = self.web3.eth().block_number().await.unwrap();
            let current_block_num = (current_block_num-U64::from(10))
                .min(U64::from(num + 1000));
            let current_block = BlockNumber::Number(current_block_num);

            println!("getting events");
            let add = BalanceInOutValues::new(
                &self.web3, 
                prev_block, 
                current_block, 
                self.add_method_topic, 
                self.balance_keeper, 
            ).await;

            let sub = BalanceInOutValues::new(
                &self.web3, 
                prev_block, 
                current_block, 
                self.sub_method_topic, 
                self.balance_keeper, 
            ).await;

            let conn = self.pool.get().unwrap();
            conn.build_transaction()
                .read_write()
                .run::<_, diesel::result::Error, _>(|| {
                    add.add_to_db(self.pool.clone());
                    sub.sub_to_db(self.pool.clone());
                    PollerState::save(self.poller_id, 
                        (current_block_num.as_u64()+1) as i64, 
                    self.pool.clone());
                    Ok(())
                })
                .unwrap();
            println!("going to sleep");
            sleep(Duration::from_secs((self.delay) as u64)).await;
        }
    }
}
