use tokio::time::{
        sleep,
      Duration,
};
use std::sync::Arc;

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

//users table
table! {
    users (id) {
        id -> Int4,
        address -> Varchar,
        twitter_account -> Nullable<Varchar>,
        external_address -> Nullable<Varchar>,
        chain_type -> Nullable<Varchar>,
    }
}

#[derive(Queryable,Debug)]
pub struct Users {
    id: i32,
    address: String,
    twitter_account: Option<String>,
    external_address: Option<String>,
    chain_type: Option<String>,
}

use ethcontract::prelude::*;
pub type Web3Instance = web3::Web3<ethcontract::Http>;

pub struct Poller {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    web3: Web3Instance,
    balance_keeper: Address,
}

impl Poller {
    pub fn new(
    ) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(
            std::env::var("DATABASE_URL").expect("missing db url"),
        );
        let pool = Pool::builder().build(manager).expect("pool build");

        let pool = std::sync::Arc::new(pool);
        let rpc_url = std::env::var("RPC_URL").expect("missing rpc url");
        let http = web3::transports::Http::new(&rpc_url)
            .expect("err creating http");
        let web3 = web3::Web3::new(http);
        let balance_keeper = std::env::var("BALANCE_KEEPER_ADDRESS")
            .expect("failed to get address");
        let balance_keeper: Address = balance_keeper.parse().unwrap();

        Poller {
            pool,
            web3,
            balance_keeper,
        }
    }

    pub async fn run(&self) {
            let contract = web3::contract::Contract::from_json(
                self.web3.eth(),
                self.balance_keeper,
                include_bytes!("../abi/balance_keeper.json"),
            ).expect("error contract creating");



        loop {
            let new_users = users::table
                .filter(users::external_address.is_null())
                .get_results::<Users>(&self.pool.get().unwrap())
                .unwrap();
            println!("new users: {:?}", new_users);
            for instance in new_users {
                let internal_id: U256 = U256::from_dec_str(&instance.address).unwrap();
                let data: (String,Vec<u8>) = contract
                    .query("userChainAddressById", internal_id, None, 
                        web3::contract::Options::default(), None)
                    .await
                    .expect("error getting active rounds");
                diesel::update(users::table)
                    .filter(users::id.eq(instance.id))
                    .set((
                        users::external_address
                        .eq(format!("0x{}",hex::encode(&data.1.clone()))),
                        users::chain_type.eq(data.0.clone()),
                    ))
                    .execute(&self.pool.get().unwrap())
                    .unwrap();
                println!("inserted for id: {} data: {}",instance.id,
                    format!("0x{}",hex::encode(&data.1.clone())));
            }
            sleep(Duration::from_secs((15) as u64)).await;
        }
    }
}

