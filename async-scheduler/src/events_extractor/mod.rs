use diesel::prelude::*;
use diesel::sql_types::{BigInt, Double, Timestamp};

use crate::DbPool;
pub mod parsers;
pub mod pollers;
pub mod fetchers;
pub mod models;
pub mod constants;
use pollers::poll_events;

pub struct EventsExtractor {
    pool: DbPool,
    ftmscan_api_key: String,
    ethscan_api_key: String,
}

impl EventsExtractor {
    pub fn new(
        pool: DbPool,
    ) -> Self {
        let ftmscan_api_key: String = std::env::var("FTMSCAN_API_KEY")
            .expect("ftmscan api key get");
        let ethscan_api_key: String = std::env::var("ETHSCAN_API_KEY")
            .expect("ethscan api key get");

        EventsExtractor {
            pool,
            ftmscan_api_key,
            ethscan_api_key,
        }
    }

    pub async fn run(&self) {
        poll_events(&self.pool, &self.ftmscan_api_key, &self.ethscan_api_key).await;
    }
}
