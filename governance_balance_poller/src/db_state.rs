use diesel::r2d2;
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

use crate::schema::pollers_data;
use serde::{
    Serialize,
    Deserialize,
};

#[derive(Serialize,Deserialize,Queryable)]
pub struct PollerState {
    id: i32,
    block_id: i64,
    poller_id: i32, 
}

impl PollerState {
    pub fn save(
        id: i32,
        num: i64,
        conn: Arc<Pool<ConnectionManager<PgConnection>>>, 
    ) {
        diesel::update(pollers_data::table)
            .filter(pollers_data::poller_id.eq(id))
            .set(pollers_data::block_id.eq(num))
            .execute(&conn.get().unwrap())
            .unwrap();
    }

    pub fn get(
        id: i32,
        conn: Arc<Pool<ConnectionManager<PgConnection>>>, 
    ) -> i64 {
        pollers_data::table
            .filter(pollers_data::poller_id.eq(id))
            .select(pollers_data::block_id)
            .get_result::<i64>(&conn.get().unwrap())
            .unwrap()
    }
}
