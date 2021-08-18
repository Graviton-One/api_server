use anyhow::{Result, Context};

use diesel::prelude::*;
use diesel::sql_types::{BigInt, Text};

use web3::transports::Http;
use web3::Web3;
use web3::types::{Block, BlockNumber, H256};

use crate::DbPool;
pub mod constants;
pub mod models;
pub mod pollers;
pub mod views;
pub mod reports;
use constants::C;
use pollers::*;
use views::*;
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

fn match_error<T>(result: Result<T>) {
    // comes up when web3 queries return too many results
    // let internal_rpc_error = anyhow!(jsonrpc_core::types::error::Error {
    //     code: jsonrpc_core::types::error::ErrorCode::InternalError,
    //     message: String::from("Internal error"),
    //     data: None
    // });
    match result {
        Err(e) => {
            println!("error {}", e);
            for cause in e.chain() {
                println!("caused by {}", cause)
            }
        },
        _ => (),
    }
}

pub struct EventsExtractor {
    pool: DbPool,
    ftm_web3: web3::Web3<Http>,
    eth_web3: web3::Web3<Http>,
    bsc_web3: web3::Web3<Http>,
    plg_web3: web3::Web3<Http>,
    // ava_web3: web3::Web3<Http>,
    // hec_web3: web3::Web3<Http>,
    // dai_web3: web3::Web3<Http>,
}

impl EventsExtractor {
    pub fn new(pool: DbPool) -> Self {
        let ftm_web3 =
            web3::Web3::new(Http::new("https://target.nodes.gravityhub.org/ftm").expect("err creating web3 ftm"));
        let eth_rpc: String =
            std::env::var("ETH_RPC").unwrap_or("https://mainnet.infura.io/v3/77f1c5201f43496fb13f1855b97da1dc".to_string());
        let eth_web3 =
            web3::Web3::new(Http::new(&eth_rpc).expect("err creating web3 eth"));
        let bsc_rpc: String =
            std::env::var("BSC_RPC").unwrap_or("https://target.nodes.gravityhub.org/bsc".to_string());
        let bsc_web3 =
            web3::Web3::new(Http::new(&bsc_rpc).expect("err creating web3 ftm"));
        let plg_rpc: String =
            std::env::var("PLG_RPC").unwrap_or("https://target.nodes.gravityhub.org/polygon".to_string());
        let plg_web3 =
            web3::Web3::new(Http::new(&plg_rpc).expect("err creating web3 ftm"));
        // let ava_web3 =
        //     web3::Web3::new(Http::new("https://api.avax.network/ext/bc/C/rpc").expect("err creating web3 ftm"));
        // let hec_web3 =
        //     web3::Web3::new(Http::new("https://http-mainnet.hecochain.com").expect("err creating web3 ftm"));
        // let dai_web3 =
        //     web3::Web3::new(Http::new("https://rpc.xdaichain.com").expect("err creating web3 ftm"));

        EventsExtractor {
            pool,
            ftm_web3,
            eth_web3,
            bsc_web3,
            plg_web3,
            // ava_web3,
            // hec_web3,
            // dai_web3,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let web3 = self.ftm_web3.clone();
        let pool = self.pool.clone();
        let handle_ftm = tokio::spawn(async move {
            poll_ftm(&web3, &pool).await
        });
        let web3 = self.bsc_web3.clone();
        let pool = self.pool.clone();
        let handle_bsc = tokio::spawn(async move {
            poll_bsc(&web3, &pool).await
        });
        let web3 = self.eth_web3.clone();
        let pool = self.pool.clone();
        let handle_eth = tokio::spawn(async move {
            poll_eth(&web3, &pool).await
        });
        let web3 = self.plg_web3.clone();
        let pool = self.pool.clone();
        let handle_plg = tokio::spawn(async move {
            poll_plg(&web3, &pool).await
        });
        match_error(handle_ftm.await.unwrap());
        match_error(handle_bsc.await.unwrap());
        match_error(handle_eth.await.unwrap());
        match_error(handle_plg.await.unwrap());
        let pool = self.pool.clone();
        match_error(build_reports(&pool).await);
        println!("finished");
        Ok(())
    }

}

    async fn poll_ftm(web3: &web3::Web3<Http>, pool: &DbPool) -> Result<()> {
        println!("polling Fantom");
        let block: Block<H256> = web3
            .eth()
            .block(BlockNumber::Latest.into())
            .await
            .context("fetch block")?
            .context("block option")?;
        let latest_block = block.number.context("block number option")?.as_u64();
        poll_erc20(
            pool,
            web3,
            C.ftm_gton,
            "events_erc20_approval_ftm",
            "events_erc20_transfer_ftm",
            C.gton_deploy_ftm,
            100000,
            latest_block,
        ).await;
        poll_anyv4(
            pool,
            web3,
            C.ftm_gton,
            "events_anyv4_swapin_ftm",
            "events_anyv4_swapout_ftm",
            C.gton_deploy_ftm,
            1000000,
            latest_block,
        ).await;
        println!("Polling Spirit");
        poll_univ2(
            pool,
            web3,
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
            C.gton_deploy_ftm,
            1000000,
            latest_block,
        ).await;
        println!("Polling Spooky");
        poll_univ2(
            pool,
            web3,
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
            C.gton_deploy_ftm,
            1000000,
            latest_block,
        ).await;
        Ok(())
    }

    async fn poll_bsc(web3: &web3::Web3<Http>, pool: &DbPool) -> Result<()> {
        println!("polling Binance");
        let block: Block<H256> = web3
            .eth()
            .block(BlockNumber::Latest.into())
            .await
            .context("fetch block")?
            .context("block option")?;
        let latest_block = block.number.context("block number option")?.as_u64();
        poll_erc20(
            pool,
            &web3,
            C.bsc_gton,
            "events_erc20_approval_bsc",
            "events_erc20_transfer_bsc",
            C.gton_deploy_bsc,
            500,
            latest_block,
        ).await;

        poll_anyv4(
            pool,
            &web3,
            C.bsc_gton,
            "events_anyv4_swapin_bsc",
            "events_anyv4_swapout_bsc",
            C.gton_deploy_bsc,
            500,
            latest_block,
        ).await;
        println!("Polling Pancake");
        poll_univ2(
            pool,
            &web3,
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
            C.gton_deploy_bsc,
            500,
            latest_block,
        ).await;
        Ok(())
    }
    async fn poll_plg(web3: &web3::Web3<Http>, pool: &DbPool) -> Result<()> {
        println!("polling Polygon");
        let block: Block<H256> = web3
            .eth()
            .block(BlockNumber::Latest.into())
            .await
            .context("fetch block")?
            .context("block option")?;
        let latest_block = block.number.context("block number option")?.as_u64();
        poll_erc20(
            pool,
            &web3,
            C.plg_gton,
            "events_erc20_approval_plg",
            "events_erc20_transfer_plg",
            C.gton_deploy_plg,
            2000,
            latest_block,
        ).await;
        poll_anyv4(
            pool,
            &web3,
            C.plg_gton,
            "events_anyv4_swapin_plg",
            "events_anyv4_swapout_plg",
            C.gton_deploy_plg,
            2000,
            latest_block,
        ).await;
        println!("Polling Quick");
        poll_univ2(
            pool,
            &web3,
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
            C.gton_deploy_plg,
            2000,
            latest_block,
        ).await;
        Ok(())
    }

    async fn poll_eth(web3: &web3::Web3<Http>, pool: &DbPool) -> Result <()> {
        println!("polling Ethereum");
        let block: Block<H256> = web3
            .eth()
            .block(BlockNumber::Latest.into())
            .await
            .context("fetch block")?
            .context("block option")?;
        let latest_block = block.number.context("block number option")?.as_u64();
        poll_erc20(
            pool,
            &web3,
            C.eth_gton,
            "events_erc20_approval_eth",
            "events_erc20_transfer_eth",
            C.gton_deploy_eth,
            100000,
            latest_block,
        ).await;

        let result = poll_events_anyv4_transfer(
            &pool,
            "events_anyv4_transfer_eth",
            &web3,
            C.eth_gton,
            C.gton_deploy_eth,
            100000,
            latest_block,
        )
        .await;
        match_error(result);

        println!("Polling eth Sushi");
        poll_univ2(
            pool,
            &web3,
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
            C.gton_deploy_eth,
            100000,
            latest_block,
        ).await;
        Ok(())
    }

    async fn poll_erc20(
        pool: &DbPool,
        web3: &Web3<Http>,
        gton: &str,
        events_erc20_approval_table: &str,
        events_erc20_transfer_table: &str,
        default_from_block: u64,
        block_step: u64,
        latest_block: u64,
    ) -> Result<()> {

        diesel::sql_query(format!(
            "INSERT INTO blocks(name_table, block_number) VALUES ('{}',{}) \
             ON CONFLICT DO NOTHING",
            events_erc20_transfer_table, default_from_block
        ))
            .execute(&pool.get().context("execute sql query")?);

        let result = poll_events_erc20_transfer(
            &pool,
            events_erc20_transfer_table,
            web3,
            gton,
            default_from_block,
            block_step,
            latest_block,
        )
        .await;
        match_error(result);

        diesel::sql_query(format!(
            "INSERT INTO blocks(name_table, block_number) VALUES ('{}',{}) \
             ON CONFLICT DO NOTHING",
            events_erc20_approval_table, default_from_block
        ))
            .execute(&pool.get().context("execute sql query")?);

        let result = poll_events_erc20_approval(
            &pool,
            events_erc20_approval_table,
            web3,
            gton,
            default_from_block,
            block_step,
            latest_block,
        )
        .await;
        match_error(result);

        Ok(())
    }

    async fn poll_anyv4(
        pool: &DbPool,
        web3: &Web3<Http>,
        gton: &str,
        events_anyv4_swapin_table: &str,
        events_anyv4_swapout_table: &str,
        default_from_block: u64,
        block_step: u64,
        latest_block: u64,
    ) -> Result<()> {

        diesel::sql_query(format!(
            "INSERT INTO blocks(name_table, block_number) VALUES ('{}',{}) \
             ON CONFLICT DO NOTHING",
            events_anyv4_swapin_table, default_from_block
        ))
            .execute(&pool.get().context("execute sql query")?);

        let result = poll_events_anyv4_swapin(
            &pool,
            events_anyv4_swapin_table,
            web3,
            gton,
            default_from_block,
            block_step,
            latest_block,
        )
        .await;
        match_error(result);

        diesel::sql_query(format!(
            "INSERT INTO blocks(name_table, block_number) VALUES ('{}',{}) \
             ON CONFLICT DO NOTHING",
            events_anyv4_swapout_table, default_from_block
        ))
            .execute(&pool.get().context("execute sql query")?);

        let result = poll_events_anyv4_swapout(
            &pool,
            events_anyv4_swapout_table,
            web3,
            gton,
            default_from_block,
            block_step,
            latest_block,
        )
        .await;
        match_error(result);
        Ok(())
    }

    async fn poll_univ2(
        pool: &DbPool,
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
        default_from_block: u64,
        block_step: u64,
        latest_block: u64,
    ) -> Result<()> {

        diesel::sql_query(format!(
            "INSERT INTO blocks(name_table, block_number) VALUES ('{}',{}) \
             ON CONFLICT DO NOTHING",
            events_univ2_pair_created_table, default_from_block
        ))
            .execute(&pool.get().context("execute sql query")?);

        let result = poll_events_univ2_pair_created(
            &pool,
            events_univ2_pair_created_table,
            web3,
            gton,
            factory,
            default_from_block,
            block_step,
            latest_block,
        )
        .await;
        match_error(result);

        let pairs = diesel::sql_query(format!(
            "SELECT id, address, title \
             FROM {};", events_univ2_pair_created_table
        ))
        .get_results::<Pair>(&pool.get().unwrap())
        .unwrap();

        for pair in pairs {
            println!("caching {}", pair.title);

            diesel::sql_query(format!(
                "INSERT INTO blocks(name_table, block_number) VALUES ('{}-{}',{}) \
                  ON CONFLICT DO NOTHING",
                events_univ2_transfer_table, pair.id, default_from_block
            ))
                .execute(&pool.get().context("execute sql query")?);

            let result = poll_events_univ2_transfer(
                &pool,
                events_univ2_transfer_table,
                web3,
                pair.id,
                &pair.address,
                default_from_block,
                block_step,
                latest_block,
            )
            .await;
            match_error(result);

            diesel::sql_query(format!(
                "INSERT INTO blocks(name_table, block_number) VALUES ('{}-{}',{}) \
                  ON CONFLICT DO NOTHING",
                events_univ2_swap_table, pair.id, default_from_block
            ))
                .execute(&pool.get().context("execute sql query")?);

            let result = poll_events_univ2_swap(
                &pool,
                events_univ2_swap_table,
                web3,
                pair.id,
                &pair.address,
                default_from_block,
                block_step,
                latest_block,
            )
            .await;
            match_error(result);

            diesel::sql_query(format!(
                "INSERT INTO blocks(name_table, block_number) VALUES ('{}-{}',{}) \
                  ON CONFLICT DO NOTHING",
                events_univ2_mint_table, pair.id, default_from_block
            ))
                .execute(&pool.get().context("execute sql query")?);

            let result = poll_events_univ2_mint(
                &pool,
                events_univ2_mint_table,
                web3,
                pair.id,
                &pair.address,
                default_from_block,
                block_step,
                latest_block,
            )
            .await;
            match_error(result);

            diesel::sql_query(format!(
                "INSERT INTO blocks(name_table, block_number) VALUES ('{}-{}',{}) \
                  ON CONFLICT DO NOTHING",
                events_univ2_burn_table, pair.id, default_from_block
            ))
                .execute(&pool.get().context("execute sql query")?);

            let result = poll_events_univ2_burn(
                &pool,
                events_univ2_burn_table,
                web3,
                pair.id,
                &pair.address,
                default_from_block,
                block_step,
                latest_block,
            )
            .await;
            match_error(result);
        }

        let result = view_buy(
            &pool,
            events_univ2_pair_created_table,
            events_univ2_swap_table,
            univ2_buy_table,
        )
        .await;
        match_error(result);

        let result = view_sell(
            &pool,
            events_univ2_pair_created_table,
            events_univ2_swap_table,
            univ2_sell_table,
        )
        .await;
        match_error(result);

        let result = view_lp_add(
            &pool,
            events_univ2_pair_created_table,
            events_univ2_mint_table,
            events_univ2_transfer_table,
            univ2_lp_add_table,
        )
        .await;
        match_error(result);

        let result = view_lp_remove(
            &pool,
            events_univ2_pair_created_table,
            events_univ2_burn_table,
            events_univ2_transfer_table,
            univ2_lp_remove_table,
        )
        .await;
        match_error(result);
        Ok(())
    }

    async fn build_reports(pool: &DbPool) -> Result <()> {
        report_sell_amount_daily_eth(&pool).await?;
        report_sell_amount_daily_other(&pool).await?;
        report_buy_amount_daily_eth(&pool).await?;
        report_buy_amount_daily_other(&pool).await?;
        report_unique_buyers_eth(&pool).await?;
        report_unique_buyers_other(&pool).await?;
        report_unique_sellers_eth(&pool).await?;
        report_unique_sellers_other(&pool).await?;
        Ok(())
    }
