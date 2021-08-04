use anyhow::{Context, Result, Error};

use diesel::prelude::*;
use diesel::sql_types::{BigInt, Double, Text, Timestamp};

use web3::transports::Http;

use crate::DbPool;
pub mod constants;
pub mod models;
pub mod pollers;
pub mod reports;
use constants::C;
use models::*;
use pollers::poll_events_anyv4_transfer;
use pollers::*;
use reports::*;

#[derive(QueryableByName, PartialEq, Debug)]
struct Pair {
    #[sql_type = "BigInt"]
    id: i64,
    #[sql_type = "Text"]
    address: String,
    #[sql_type = "Text"]
    title: String,
}

pub struct EventsExtractor {
    pool: DbPool,
    ftm_web3: web3::Web3<Http>,
    ftmscan_api_key: String,
    eth_web3: web3::Web3<Http>,
    ethscan_api_key: String,
}

impl EventsExtractor {
    pub fn new(pool: DbPool) -> Self {
        let ftm_web3 =
            web3::Web3::new(Http::new("https://rpc.ftm.tools").expect("err creating web3 ftm"));
        let eth_rpc: String =
            std::env::var("ETH_RPC").expect("eth rpc get");
        let eth_web3 =
            web3::Web3::new(Http::new(&eth_rpc).expect("err creating web3 eth"));
        let ftmscan_api_key: String =
            std::env::var("FTMSCAN_API_KEY").expect("ftmscan api key get");
        let ethscan_api_key: String =
            std::env::var("ETHSCAN_API_KEY").expect("ethscan api key get");

        EventsExtractor {
            pool,
            ftm_web3,
            ftmscan_api_key,
            eth_web3,
            ethscan_api_key,
        }
    }

    pub async fn run(&self) {

        &self.poll_ftm().await;

        // get ethereum from etherscan for now
        // let result = poll_events_anyv4_transfer(
        //     &self.pool,
        //     "events_anyv4_transfer",
        //     &self.eth_web3,
        //     C.eth_gton,
        // )
        // .await.unwrap();

    }

    async fn poll_ftm(&self) {
        let result = poll_events_erc20_approval(
            &self.pool,
            "events_erc20_approval_ftm",
            &self.ftm_web3,
            C.ftm_gton,
        )
        .await;
        match_error(result);

        let result = poll_events_erc20_transfer(
            &self.pool,
            "events_erc20_transfer_ftm",
            &self.ftm_web3,
            C.ftm_gton,
        )
        .await;
        match_error(result);

        let result = poll_events_anyv4_swapin(
            &self.pool,
            "events_anyv4_swapin_ftm",
            &self.ftm_web3,
            C.ftm_gton,
        )
        .await;
        match_error(result);

        let result = poll_events_anyv4_swapout(
            &self.pool,
            "events_anyv4_swapout_ftm",
            &self.ftm_web3,
            C.ftm_gton,
        )
        .await;
        match_error(result);

        let result = poll_events_univ2_pair_created(
            &self.pool,
            "events_univ2_pair_created_spirit",
            &self.ftm_web3,
            C.ftm_gton,
            C.ftm_spirit_factory,
        )
        .await;
        match_error(result);

        let pairs = diesel::sql_query(
            "SELECT id, address, title \
             FROM events_univ2_pair_created_spirit;",
        )
        .get_results::<Pair>(&self.pool.get().unwrap())
        .unwrap();

        for pair in pairs {
            println!("caching {}", pair.title);
            let result = poll_events_univ2_transfer(
                &self.pool,
                "events_univ2_transfer_spirit",
                &self.ftm_web3,
                pair.id,
                &pair.address,
            )
            .await;
            match_error(result);

            let result = poll_events_univ2_swap(
                &self.pool,
                "events_univ2_swap_spirit",
                &self.ftm_web3,
                pair.id,
                &pair.address,
            )
            .await;
            match_error(result);

            let result = poll_events_univ2_mint(
                &self.pool,
                "events_univ2_mint_spirit",
                &self.ftm_web3,
                pair.id,
                &pair.address,
            )
            .await;
            match_error(result);

            let result = poll_events_univ2_burn(
                &self.pool,
                "events_univ2_burn_spirit",
                &self.ftm_web3,
                pair.id,
                &pair.address,
            )
            .await;
            match_error(result);
        }

        let result = report_buy(
            &self.pool,
            "events_univ2_pair_created_spirit",
            "events_univ2_swap_spirit",
            "univ2_buy_spirit",
        )
        .await;
        match_error(result);

        let result = report_sell(
            &self.pool,
            "events_univ2_pair_created_spirit",
            "events_univ2_swap_spirit",
            "univ2_sell_spirit",
        )
        .await;
        match_error(result);

        let result = report_lp_add(
            &self.pool,
            "events_univ2_pair_created_spirit",
            "events_univ2_mint_spirit",
            "events_univ2_transfer_spirit",
            "univ2_lp_add_spirit",
        )
        .await;
        match_error(result);

        let result = report_lp_remove(
            &self.pool,
            "events_univ2_pair_created_spirit",
            "events_univ2_burn_spirit",
            "events_univ2_transfer_spirit",
            "univ2_lp_remove_spirit",
        )
        .await;
        match_error(result);
    }
}

fn match_error<T>(result: Result<T>) {
    // match on this for possible rate limit
    // let internal_rpc_error = anyhow!(jsonrpc_core::types::error::Error {
    //     code: jsonrpc_core::types::error::ErrorCode::InternalError,
    //     message: String::from("Internal error"),
    //     data: None
    // });
    match result {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
