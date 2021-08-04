use anyhow::{Context, Result, Error};

use diesel::prelude::*;
use diesel::sql_types::{BigInt, Double, Text, Timestamp};

use web3::transports::Http;
use web3::Web3;

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
    eth_web3: web3::Web3<Http>,
    bsc_web3: web3::Web3<Http>,
    plg_web3: web3::Web3<Http>,
    ava_web3: web3::Web3<Http>,
    hec_web3: web3::Web3<Http>,
    dai_web3: web3::Web3<Http>,
}

impl EventsExtractor {
    pub fn new(pool: DbPool) -> Self {
        let ftm_web3 =
            web3::Web3::new(Http::new("https://rpc.ftm.tools").expect("err creating web3 ftm"));
        let eth_rpc: String =
            std::env::var("ETH_RPC").expect("eth rpc get");
        let eth_web3 =
            web3::Web3::new(Http::new(&eth_rpc).expect("err creating web3 eth"));
        let bsc_web3 =
            web3::Web3::new(Http::new("https://bsc-dataseed4.binance.org").expect("err creating web3 ftm"));
        let plg_web3 =
            web3::Web3::new(Http::new("https://matic-mainnet.chainstacklabs.com").expect("err creating web3 ftm"));
        let ava_web3 =
            web3::Web3::new(Http::new("https://api.avax.network/ext/bc/C/rpc").expect("err creating web3 ftm"));
        let hec_web3 =
            web3::Web3::new(Http::new("https://http-mainnet.hecochain.com").expect("err creating web3 ftm"));
        let dai_web3 =
            web3::Web3::new(Http::new("https://rpc.xdaichain.com").expect("err creating web3 ftm"));

        EventsExtractor {
            pool,
            ftm_web3,
            eth_web3,
            bsc_web3,
            plg_web3,
            ava_web3,
            hec_web3,
            dai_web3,
        }
    }

    pub async fn run(&self) {
        &self.poll_ftm().await;
        &self.poll_bsc().await;
        &self.poll_plg().await;
        // &self.poll_eth().await;
    }

    async fn poll_ftm(&self) {
        &self.poll_erc20(
            &self.ftm_web3,
            C.ftm_gton,
            "events_erc20_approval_ftm",
            "events_erc20_transfer_ftm"
        ).await;
        &self.poll_anyv4(
            &self.ftm_web3,
            C.ftm_gton,
            "events_anyv4_swapin_ftm",
            "events_anyv4_swapout_ftm"
        ).await;
        &self.poll_univ2(
            &self.ftm_web3,
            C.ftm_gton,
            C.ftm_spirit_factory,
            "events_univ2_pair_created_ftm_spirit",
            "events_univ2_transfer_ftm_spirit",
            "events_univ2_swap_ftm_spirit",
            "events_univ2_mint_ftm_spirit",
            "events_univ2_burn_ftm_spirit",
            "univ2_buy_ftm_spirit",
            "univ2_sell_ftm_spirit",
            "univ2_lp_add_ftm_spirit",
            "univ2_lp_remove_ftm_spirit",
        ).await;
        &self.poll_univ2(
            &self.ftm_web3,
            C.ftm_gton,
            C.ftm_spooky_factory,
            "events_univ2_pair_created_ftm_spooky",
            "events_univ2_transfer_ftm_spooky",
            "events_univ2_swap_ftm_spooky",
            "events_univ2_mint_ftm_spooky",
            "events_univ2_burn_ftm_spooky",
            "univ2_buy_ftm_spooky",
            "univ2_sell_ftm_spooky",
            "univ2_lp_add_ftm_spooky",
            "univ2_lp_remove_ftm_spooky",
        ).await;
    }
    async fn poll_bsc(&self) {
        &self.poll_erc20(
            &self.bsc_web3,
            C.bsc_gton,
            "events_erc20_approval_bsc",
            "events_erc20_transfer_bsc",
        ).await;
        &self.poll_anyv4(
            &self.bsc_web3,
            C.bsc_gton,
            "events_anyv4_swapin_bsc",
            "events_anyv4_swapout_bsc"
        ).await;
        &self.poll_univ2(
            &self.bsc_web3,
            C.bsc_gton,
            C.bsc_pancake_factory,
            "events_univ2_pair_created_bsc_pancake",
            "events_univ2_transfer_bsc_pancake",
            "events_univ2_swap_bsc_pancake",
            "events_univ2_mint_bsc_pancake",
            "events_univ2_burn_bsc_pancake",
            "univ2_buy_bsc_pancake",
            "univ2_sell_bsc_pancake",
            "univ2_lp_add_bsc_pancake",
            "univ2_lp_remove_bsc_pancake",
        ).await;
    }
    async fn poll_plg(&self) {
        &self.poll_erc20(
            &self.plg_web3,
            C.plg_gton,
            "events_erc20_approval_plg",
            "events_erc20_transfer_plg",
        ).await;
        &self.poll_anyv4(
            &self.plg_web3,
            C.plg_gton,
            "events_anyv4_swapin_plg",
            "events_anyv4_swapout_plg"
        ).await;
        &self.poll_univ2(
            &self.plg_web3,
            C.plg_gton,
            C.plg_sushi_factory,
            "events_univ2_pair_created_plg_sushi",
            "events_univ2_transfer_plg_sushi",
            "events_univ2_swap_plg_sushi",
            "events_univ2_mint_plg_sushi",
            "events_univ2_burn_plg_sushi",
            "univ2_buy_plg_sushi",
            "univ2_sell_plg_sushi",
            "univ2_lp_add_plg_sushi",
            "univ2_lp_remove_plg_sushi",
        ).await;
        &self.poll_univ2(
            &self.plg_web3,
            C.plg_gton,
            C.plg_quick_factory,
            "events_univ2_pair_created_plg_quick",
            "events_univ2_transfer_plg_quick",
            "events_univ2_swap_plg_quick",
            "events_univ2_mint_plg_quick",
            "events_univ2_burn_plg_quick",
            "univ2_buy_plg_quick",
            "univ2_sell_plg_quick",
            "univ2_lp_add_plg_quick",
            "univ2_lp_remove_plg_quick",
        ).await;
    }

    async fn poll_eth(&self) {
        &self.poll_erc20(
            &self.plg_web3,
            C.plg_gton,
            "events_erc20_approval_plg",
            "events_erc20_transfer_plg",
        ).await;

        let result = poll_events_anyv4_transfer(
            &self.pool,
            "events_anyv4_transfer_eth",
            &self.eth_web3,
            C.eth_gton,
        )
        .await;
        match_error(result);

        &self.poll_univ2(
            &self.eth_web3,
            C.eth_gton,
            C.eth_sushi_factory,
            "events_univ2_pair_created_eth_sushi",
            "events_univ2_transfer_eth_sushi",
            "events_univ2_swap_eth_sushi",
            "events_univ2_mint_eth_sushi",
            "events_univ2_burn_eth_sushi",
            "univ2_buy_eth_sushi",
            "univ2_sell_eth_sushi",
            "univ2_lp_add_eth_sushi",
            "univ2_lp_remove_eth_sushi",
        ).await;
    }

    async fn poll_erc20(
        &self,
        web3: &Web3<Http>,
        gton: &str,
        events_erc20_approval_table: &str,
        events_erc20_transfer_table: &str,
    ) {
        let result = poll_events_erc20_approval(
            &self.pool,
            events_erc20_approval_table,
            web3,
            gton,
        )
        .await;
        match_error(result);

        let result = poll_events_erc20_transfer(
            &self.pool,
            events_erc20_transfer_table,
            web3,
            gton,
        )
        .await;
        match_error(result);
    }

    async fn poll_anyv4(
        &self,
        web3: &Web3<Http>,
        gton: &str,
        events_anyv4_swapin_table: &str,
        events_anyv4_swapout_table: &str,
    ) {

        let result = poll_events_anyv4_swapin(
            &self.pool,
            events_anyv4_swapin_table,
            web3,
            gton,
        )
        .await;
        match_error(result);

        let result = poll_events_anyv4_swapout(
            &self.pool,
            events_anyv4_swapout_table,
            web3,
            gton,
        )
        .await;
        match_error(result);
    }

    async fn poll_univ2(
        &self,
        web3: &Web3<Http>,
        gton: &str,
        factory: &str,
        events_univ2_pair_created_table: &str,
        events_univ2_transfer_table: &str,
        events_univ2_swap_table: &str,
        events_univ2_mint_table: &str,
        events_univ2_burn_table: &str,
        univ2_buy_table: &str,
        univ2_sell_table: &str,
        univ2_lp_add_table: &str,
        univ2_lp_remove_table: &str,
    ) {
        let result = poll_events_univ2_pair_created(
            &self.pool,
            events_univ2_pair_created_table,
            web3,
            gton,
            factory,
        )
        .await;
        match_error(result);

        let pairs = diesel::sql_query(format!(
            "SELECT id, address, title \
             FROM {};", events_univ2_pair_created_table
        ))
        .get_results::<Pair>(&self.pool.get().unwrap())
        .unwrap();

        for pair in pairs {
            println!("caching {}", pair.title);
            let result = poll_events_univ2_transfer(
                &self.pool,
                events_univ2_transfer_table,
                web3,
                pair.id,
                &pair.address,
            )
            .await;
            match_error(result);

            let result = poll_events_univ2_swap(
                &self.pool,
                events_univ2_swap_table,
                web3,
                pair.id,
                &pair.address,
            )
            .await;
            match_error(result);

            let result = poll_events_univ2_mint(
                &self.pool,
                events_univ2_mint_table,
                web3,
                pair.id,
                &pair.address,
            )
            .await;
            match_error(result);

            let result = poll_events_univ2_burn(
                &self.pool,
                events_univ2_burn_table,
                web3,
                pair.id,
                &pair.address,
            )
            .await;
            match_error(result);
        }

        let result = report_buy(
            &self.pool,
            events_univ2_pair_created_table,
            events_univ2_swap_table,
            univ2_buy_table,
        )
        .await;
        match_error(result);

        let result = report_sell(
            &self.pool,
            events_univ2_pair_created_table,
            events_univ2_swap_table,
            univ2_sell_table,
        )
        .await;
        match_error(result);

        let result = report_lp_add(
            &self.pool,
            events_univ2_pair_created_table,
            events_univ2_mint_table,
            events_univ2_transfer_table,
            univ2_lp_add_table,
        )
        .await;
        match_error(result);

        let result = report_lp_remove(
            &self.pool,
            events_univ2_pair_created_table,
            events_univ2_burn_table,
            events_univ2_transfer_table,
            univ2_lp_remove_table,
        )
        .await;
        match_error(result);
    }
}

fn match_error<T>(result: Result<T>) {
    // match on this for possible rate limit
    // let internal_rpc_error = anyhow!(jsonrpc_core::types::error::Error {
    //     code: jsonrpc_core::types::error::ErrorCode::InternalError,
    //     message: String::from(Internal error),
    //     data: None
    // });
    match result {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
