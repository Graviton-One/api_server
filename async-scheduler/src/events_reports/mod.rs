use diesel::prelude::*;
use diesel::sql_types::{BigInt, Bool, Numeric, Text, Timestamp};
use diesel::result::Error::DatabaseError;
use diesel::result::DatabaseErrorKind::UniqueViolation;

use hex::ToHex;

use std::str::FromStr;
use web3::transports::Http;
use web3::{Web3,types::TransactionId};

use crate::DbPool;
pub mod models;
use models::*;

use bigdecimal::{BigDecimal, ToPrimitive};

use chrono::NaiveDateTime;

use crate::schema::{events_univ2_swap_spirit, events_univ2_pair_created_spirit};

#[derive(QueryableByName, PartialEq, Debug)]
struct Pair {
    #[sql_type = "BigInt"]
    pub id: i64,
    #[sql_type = "Text"]
    pub address: String,
    #[sql_type = "Bool"]
    pub gtonToken0: bool,
    #[sql_type = "Text"]
    pub title: String,
}

#[derive(QueryableByName, PartialEq, Debug)]
struct Swap {
    #[sql_type = "BigInt"]
    pub id: i64,
    #[sql_type = "BigInt"]
    pub pair: i64,
    #[sql_type = "Numeric"]
    pub amount0_in: BigDecimal,
    #[sql_type = "Numeric"]
    pub amount1_in: BigDecimal,
    #[sql_type = "Numeric"]
    pub amount0_out: BigDecimal,
    #[sql_type = "Numeric"]
    pub amount1_out: BigDecimal,
    #[sql_type = "Timestamp"]
    pub stamp: NaiveDateTime,
    #[sql_type = "BigInt"]
    pub block_number: i64,
    #[sql_type = "Text"]
    pub tx_hash: String,
    #[sql_type = "BigInt"]
    pub log_index: i64,
}

fn hex_to_string<T: ToHex>(h: T) -> String {
    "0x".to_owned() + &h.encode_hex::<String>()
}

pub struct ReportsExtractor {
    pool: DbPool,
    ftm_web3: web3::Web3<Http>,
}

impl ReportsExtractor {
    pub fn new(
        pool: DbPool,
    ) -> Self {
        let ftm_web3 = web3::Web3::new(Http::new("https://rpc.ftm.tools")
            .expect("err creating http"));

        ReportsExtractor {
            pool,
            ftm_web3
        }
    }

    pub async fn run(&self) {

        &self.report_buy(
            &self.ftm_web3,
            "events_univ2_swap_spirit",
            "events_univ2_pair_created_spirit",
            "univ2_buy_spirit"
        ).await;
        &self.report_sell(
            &self.ftm_web3,
            "events_univ2_swap_spirit",
            "events_univ2_pair_created_spirit",
            "univ2_sell_spirit"
        ).await;

    }

    async fn report_buy(&self, web3: &Web3<Http>, swap_table: &str, pair_table: &str, buy_table: &str) {

        // from last block in the report table, get swap table,
        let swaps = diesel::sql_query(format!(
            "SELECT id, pair, amount0_in, amount1_in, \
             amount0_out, amount1_out, stamp, block_number, tx_hash, log_index \
             FROM {} \
             ORDER BY block_number ASC;", swap_table))
            .get_results::<Swap>(&self.pool.get().unwrap()).unwrap();

        // get gtonToken0 from pair created table
        for swap in swaps {
            let pair = diesel::sql_query(format!(
            "SELECT id, address, gtonToken0, title \
             FROM {} \
             WHERE id = $1;", pair_table))
            .bind::<BigInt,_>(swap.pair)
            .get_result::<Pair>(&self.pool.get().unwrap()).unwrap();


            if (pair.gtonToken0 &&
                swap.amount1_in != 0.into() &&
                swap.amount0_out != 0.into()) ||
                (!pair.gtonToken0 &&
                 swap.amount0_in != 0.into() &&
                 swap.amount1_out != 0.into()) {

                    // get transaction origin
                    let tx = &web3.eth().transaction(TransactionId::Hash(swap.tx_hash.parse().unwrap())).await.unwrap().unwrap();
                    let tx_origin = hex_to_string(tx.from);

                    let event = if pair.gtonToken0 {
                        UniV2Buy {
                            swap_id: swap.id.clone(),
                            pair_id: swap.pair.clone(),
                            pair_title: pair.title.clone(),
                            tx_origin: tx_origin.clone(),
                            amount_token_in: swap.amount1_in.clone(),
                            amount_gton_out: swap.amount0_out.clone(),
                            stamp: swap.stamp.clone(),
                            tx_hash: swap.tx_hash.clone(),
                            log_index: swap.log_index.clone(),
                        }
                    } else {
                        UniV2Buy {
                            swap_id: swap.id.clone(),
                            pair_id: swap.pair.clone(),
                            pair_title: pair.title.clone(),
                            tx_origin: tx_origin.clone(),
                            amount_token_in: swap.amount0_in.clone(),
                            amount_gton_out: swap.amount1_out.clone(),
                            stamp: swap.stamp.clone(),
                            tx_hash: swap.tx_hash.clone(),
                            log_index: swap.log_index.clone(),
                        }
                    };

                    let result = diesel::sql_query(
                        format!("insert into {}(\
                                 swap_id,\
                                 pair_id,\
                                 pair_title,\
                                 tx_origin,\
                                 amount_token_in,\
                                 amount_gton_out,\
                                 stamp,\
                                 tx_hash,
                                 log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9);",
                                buy_table),
                    )
                        .bind::<BigInt, _>(&event.swap_id)
                        .bind::<BigInt, _>(&event.pair_id)
                        .bind::<Text, _>(&event.pair_title)
                        .bind::<Text, _>(&event.tx_origin)
                        .bind::<Numeric, _>(&event.amount_token_in)
                        .bind::<Numeric, _>(&event.amount_gton_out)
                        .bind::<Timestamp, _>(&event.stamp)
                        .bind::<Text, _>(&event.tx_hash)
                        .bind::<BigInt, _>(&event.log_index)
                        .execute(&self.pool.get().unwrap());
                    match result {
                        // ignore if already processed, panic otherwise
                        Ok(_) => (),
                        Err(DatabaseError(UniqueViolation, _)) => (),
                        Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
                    };
                }
        }
    }

    async fn report_sell(&self, web3: &Web3<Http>, swap_table: &str, pair_table: &str, sell_table: &str) {

        // from last block in the report table, get swap table,
        let swaps = diesel::sql_query(format!(
            "SELECT id, pair, amount0_in, amount1_in, \
             amount0_out, amount1_out, stamp, block_number, tx_hash, log_index \
             FROM {} \
             ORDER BY block_number ASC;", swap_table))
            .get_results::<Swap>(&self.pool.get().unwrap()).unwrap();

        // get gtonToken0 from pair created table
        for swap in swaps {
            let pair = diesel::sql_query(format!(
            "SELECT id, address, gtonToken0, title \
             FROM {} \
             WHERE id=$1;", pair_table))
            .bind::<BigInt,_>(swap.pair)
            .get_result::<Pair>(&self.pool.get().unwrap()).unwrap();

            if (pair.gtonToken0 &&
                swap.amount0_in != 0.into() &&
                swap.amount1_out != 0.into()) ||
                (!pair.gtonToken0 &&
                 swap.amount1_in != 0.into() &&
                 swap.amount0_out != 0.into()) {

                    // get transaction origin
                    let tx = &web3.eth().transaction(TransactionId::Hash(swap.tx_hash.parse().unwrap())).await.unwrap().unwrap();
                    let tx_origin = hex_to_string(tx.from);

                    let event = if pair.gtonToken0 {
                        UniV2Sell {
                            swap_id: swap.id.clone(),
                            pair_id: swap.pair.clone(),
                            pair_title: pair.title.clone(),
                            tx_origin: tx_origin.clone(),
                            amount_gton_in: swap.amount0_in.clone(),
                            amount_token_out: swap.amount1_out.clone(),
                            stamp: swap.stamp.clone(),
                            tx_hash: swap.tx_hash.clone(),
                            log_index: swap.log_index.clone(),
                        }
                    } else {
                        UniV2Sell {
                            swap_id: swap.id.clone(),
                            pair_id: swap.pair.clone(),
                            pair_title: pair.title.clone(),
                            tx_origin: tx_origin.clone(),
                            amount_gton_in: swap.amount1_in.clone(),
                            amount_token_out: swap.amount0_out.clone(),
                            stamp: swap.stamp.clone(),
                            tx_hash: swap.tx_hash.clone(),
                            log_index: swap.log_index.clone(),
                        }
                    };

                    let result = diesel::sql_query(
                        format!("insert into {}(\
                                 swap_id,\
                                 pair_id,\
                                 pair_title,\
                                 tx_origin,\
                                 amount_gton_in,\
                                 amount_token_out,\
                                 stamp,\
                                 tx_hash,
                                       log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9);",
                                sell_table),
                    )
                        .bind::<BigInt, _>(&event.swap_id)
                        .bind::<BigInt, _>(&event.pair_id)
                        .bind::<Text, _>(&event.pair_title)
                        .bind::<Text, _>(&event.tx_origin)
                        .bind::<Numeric, _>(&event.amount_gton_in)
                        .bind::<Numeric, _>(&event.amount_token_out)
                        .bind::<Timestamp, _>(&event.stamp)
                        .bind::<Text, _>(&event.tx_hash)
                        .bind::<BigInt, _>(&event.log_index)
                        .execute(&self.pool.get().unwrap());
                    match result {
                        // ignore if already processed, panic otherwise
                        Ok(_) => (),
                        Err(DatabaseError(UniqueViolation, _)) => (),
                        Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
                    };
                }
        }
    }
}
