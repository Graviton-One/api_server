use diesel::r2d2;
#[macro_use]
extern crate diesel;
use tokio::time::{
        sleep,
      Duration,
};
use std::sync::Arc;
use bigdecimal::BigDecimal;
use std::str::FromStr;

use web3::transports::Http;
use web3::{Web3, types::*};
use web3::ethabi::{
    Topic,
    TopicFilter,
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

use serde::{
    Serialize,
    Deserialize,
};

pub mod users_total_balances;
pub mod split_by_sources;
pub mod reserves_poller;
pub mod db_state;
pub mod schema;
pub mod user_address_mapping;

#[tokio::main]
async fn main() {

}