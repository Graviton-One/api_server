use diesel::prelude::*;
use diesel::sql_types::{BigInt, Double, Timestamp, Text};

use web3::transports::Http;

use crate::DbPool;
pub mod parsers;
pub mod pollers;
pub mod fetchers;
pub mod pollers_web3;
pub mod models;
pub mod constants;
use constants::C;
use pollers::poll_events_anyv4_transfer;
use pollers_web3::*;

#[derive(QueryableByName, PartialEq, Debug)]
struct Pair {
    #[sql_type = "BigInt"]
    id: i64,
    #[sql_type = "Text"]
    address: String,
}

pub struct EventsExtractor {
    pool: DbPool,
    ftm_web3: web3::Web3<Http>,
    ftmscan_api_key: String,
    ethscan_api_key: String,
}

impl EventsExtractor {
    pub fn new(
        pool: DbPool,
    ) -> Self {
        let ftm_web3 = web3::Web3::new(Http::new("https://rpc.ftm.tools")
            .expect("err creating http"));
        let ftmscan_api_key: String = std::env::var("FTMSCAN_API_KEY")
            .expect("ftmscan api key get");
        let ethscan_api_key: String = std::env::var("ETHSCAN_API_KEY")
            .expect("ethscan api key get");

        EventsExtractor {
            pool,
            ftm_web3,
            ftmscan_api_key,
            ethscan_api_key,
        }
    }

    pub async fn run(&self) {

        &self.poll_ftm().await;

        // get ethereum from etherscan for now
        poll_events_anyv4_transfer(
            &self.pool,
            &self.ethscan_api_key
        ).await;

    }

    async fn poll_ftm(&self) {
        // get ftm from rpc
        poll_events_erc20_approval(
            &self.pool,
            "events_erc20_approval_ftm",
            &self.ftm_web3,
            C.ftm_gton
        ).await;

        poll_events_erc20_transfer(
            &self.pool,
            "events_erc20_transfer_ftm",
            &self.ftm_web3,
            C.ftm_gton
        ).await;

        poll_events_anyv4_swapin(
            &self.pool,
            "events_anyv4_swapin_ftm",
            &self.ftm_web3,
            C.ftm_gton
        ).await;

        poll_events_anyv4_swapout(
            &self.pool,
            "events_anyv4_swapout_ftm",
            &self.ftm_web3,
            C.ftm_gton
        ).await;

        poll_events_univ2_pair_created(
            &self.pool,
            "events_univ2_pair_created_spirit",
            &self.ftm_web3,
            C.ftm_gton,
            C.ftm_spirit_factory
        ).await;

        let pairs = diesel::sql_query(
            "SELECT id, address \
             FROM events_univ2_pair_created_spirit \
             ORDER BY block_number DESC;")
            .get_results::<Pair>(&self.pool.get().unwrap()).unwrap();

        for pair in pairs {
            poll_events_univ2_swap(
                &self.pool,
                "events_univ2_swap_spirit",
                &self.ftm_web3,
                pair.id,
                &pair.address,
            ).await;
            poll_events_univ2_mint(
                &self.pool,
                "events_univ2_mint_spirit",
                &self.ftm_web3,
                pair.id,
                &pair.address,
            ).await;
            poll_events_univ2_burn(
                &self.pool,
                "events_univ2_burn_spirit",
                &self.ftm_web3,
                pair.id,
                &pair.address,
            ).await;
        }

    }
}
