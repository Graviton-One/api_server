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
use governance_poller::user_address_mapping::Poller;

#[tokio::main]
async fn main() {
    Poller::new().run().await;
}


