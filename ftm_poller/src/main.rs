#[macro_use]
extern crate diesel;
use hex_literal::hex;
use web3::{
    contract::{Contract, Options},
    types::U256,
};
use tokio::time::{
    delay_for, 
    Duration
};
use tokio_diesel::*;

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

#[tokio::main]
async fn main() -> web3::contract::Result<()> {

    let manager =
        ConnectionManager::<PgConnection>::new(std::env::var("DATABASE_URL")
        .expect("Add db url"));
    let pool = Pool::builder().build(manager).expect("pool build");

    let http = web3::transports::Http::new("https://rpcapi.fantom.network").expect("err creating http");
    // creating web3 object with provider
    let web3 = web3::Web3::new(http);
    let contract_address = hex!("0xd3360862277ba00bac19192240046f086629a6cd").into();
    let query_address = hex!("0x7877ece589eF760F411f6C7655fe5A9786a580C4").into(); // doesn't matter
    let query_balance:U256 = 1000000000000000000
    // creating instance of contract to interact with
    let contract = Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("../src/contracts/dodo.json"),
    ).expect("error contract createing");
    println!("comin in loop");
    loop {
        println!("waki waki, it'ss time for work");
        // calling method of instance
        let result = contract.query("querySellBase", (query_address, query_balance,), None, Options::default(), None);
        // storage contains the total amount
        let storage: U256 = result.await.expect("error getting result");
        println!("got res {}",storage);
        let storage = storage.low_u64() as i128;
        // that should be the price for moment
        let price: i128 = storage/ 10.pow(18);
        let val = price as i64;
        println!("formatted {}",val);

        println!("start push");
        diesel::sql_query("INSERT INTO gton_value (y_value) VALUES ($1)")
            .bind::<diesel::sql_types::BigInt,_>(val)
            .execute_async(&pool)
            .await
            .expect("exec err");

        println!("value {} going to sleep for 1 hour",val);
        delay_for(Duration::from_secs((60*60) as u64)).await;
    }
    Ok(())
}
