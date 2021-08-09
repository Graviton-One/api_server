use anyhow::{Result, Context};

use diesel::prelude::*;
use diesel::sql_types::{BigInt, Text};

use web3::transports::Http;
use web3::Web3;
use web3::types::{Block, BlockNumber, H256};

use crate::DbPool;
pub mod constants;
pub mod pollers;
pub mod models;
use constants::C;
use pollers::*;

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

pub struct UsersExtractor {
    pool: DbPool,
    ftm_web3: web3::Web3<Http>,
}

impl UsersExtractor {
    pub fn new(pool: DbPool) -> Self {
        let ftm_web3 =
            web3::Web3::new(Http::new("https://rpc.ftm.tools").expect("err creating web3 ftm"));

        UsersExtractor {
            pool,
            ftm_web3,
        }
    }

    pub async fn run(&self) -> Result<()> {
        println!("polling Fantom");
        let block: Block<H256> = self.ftm_web3
            .eth()
            .block(BlockNumber::Latest.into())
            .await
            .context("fetch block")?
            .context("block option")?;
        let latest_block = block.number.context("block number option")?.as_u64();

        match_error(self.poll_balance_keeper(latest_block).await);
        match_error(self.poll_lp_keeper(latest_block).await);
        Ok(())
    }

    async fn poll_balance_keeper (&self, latest_block: u64) -> Result<()> {
        diesel::sql_query(format!(
            "INSERT INTO blocks(name_table, block_number) VALUES ('{}',{}) \
             ON CONFLICT DO NOTHING",
            "events_balance_keeper_open_user", C.balance_keeper_deploy
        ))
            .execute(&self.pool.get().context("execute sql query")?);

        match_error(poll_events_balance_keeper_open_user(&self.pool, &self.ftm_web3, latest_block).await);

        diesel::sql_query(format!(
            "INSERT INTO blocks(name_table, block_number) VALUES ('{}',{}) \
             ON CONFLICT DO NOTHING",
            "events_balance_keeper_add", C.balance_keeper_deploy
        ))
            .execute(&self.pool.get().context("execute sql query")?);

        match_error(poll_events_balance_keeper_add(&self.pool, &self.ftm_web3, latest_block).await);

        diesel::sql_query(format!(
            "INSERT INTO blocks(name_table, block_number) VALUES ('{}',{}) \
             ON CONFLICT DO NOTHING",
            "events_balance_keeper_subtract", C.balance_keeper_deploy
        ))
            .execute(&self.pool.get().context("execute sql query")?);

        match_error(poll_events_balance_keeper_subtract(&self.pool, &self.ftm_web3, latest_block).await);
        Ok(())
    }

    async fn poll_lp_keeper(&self, latest_block: u64) -> Result<()> {

        diesel::sql_query(format!(
            "INSERT INTO blocks(name_table, block_number) VALUES ('{}',{}) \
             ON CONFLICT DO NOTHING",
            "events_lp_keeper_add", C.lp_keeper_deploy
        ))
            .execute(&self.pool.get().context("execute sql query")?);

        match_error(poll_events_lp_keeper_add(&self.pool, &self.ftm_web3, latest_block).await);

        diesel::sql_query(format!(
            "INSERT INTO blocks(name_table, block_number) VALUES ('{}',{}) \
             ON CONFLICT DO NOTHING",
            "events_lp_keeper_subtract", C.lp_keeper_deploy
        ))
            .execute(&self.pool.get().context("execute sql query")?);

        match_error(poll_events_lp_keeper_subtract(&self.pool, &self.ftm_web3, latest_block).await);
        Ok(())
    }
}
