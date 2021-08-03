use diesel::prelude::*;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::DatabaseError;
use diesel::sql_types::{BigInt, Bool, Numeric, Text, Timestamp};

use hex::ToHex;

use std::str::FromStr;
use web3::transports::Http;
use web3::{types::TransactionId, Web3};

use super::models::*;
use crate::DbPool;

use bigdecimal::{BigDecimal, ToPrimitive};

use chrono::NaiveDateTime;

use super::constants::C;

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
    #[sql_type = "Text"]
    pub tx_origin: String,
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

#[derive(QueryableByName, PartialEq, Debug)]
struct MintBurn {
    #[sql_type = "BigInt"]
    pub id: i64,
    #[sql_type = "BigInt"]
    pub pair: i64,
    #[sql_type = "Text"]
    pub tx_origin: String,
    #[sql_type = "Numeric"]
    pub amount0: BigDecimal,
    #[sql_type = "Numeric"]
    pub amount1: BigDecimal,
    #[sql_type = "Timestamp"]
    pub stamp: NaiveDateTime,
    #[sql_type = "BigInt"]
    pub block_number: i64,
    #[sql_type = "Text"]
    pub tx_hash: String,
    #[sql_type = "BigInt"]
    pub log_index: i64,
}

#[derive(QueryableByName, PartialEq, Debug)]
struct Transfer {
    #[sql_type = "Numeric"]
    pub amount: BigDecimal,
}

fn hex_to_string<T: ToHex>(h: T) -> String {
    "0x".to_owned() + &h.encode_hex::<String>()
}

pub async fn report_buy(
    pool: &DbPool,
    pair_table: &str,
    swap_table: &str,
    buy_table: &str,
    web3: &Web3<Http>,
) {
    // from last block in the report table, get swap table,
    let swaps = diesel::sql_query(format!(
        "SELECT id, pair_id, tx_origin, amount0_in, amount1_in, \
         amount0_out, amount1_out, stamp, block_number, tx_hash, log_index \
         FROM {} \
         ORDER BY block_number ASC;",
        swap_table
    ))
    .get_results::<Swap>(&pool.get().unwrap())
    .unwrap();

    // get gtonToken0 from pair created table
    for swap in swaps {
        let pair = diesel::sql_query(format!(
            "SELECT id, address, gtonToken0, title \
         FROM {} \
         WHERE id = $1;",
            pair_table
        ))
        .bind::<BigInt, _>(swap.pair)
        .get_result::<Pair>(&pool.get().unwrap())
        .unwrap();

        if (pair.gtonToken0 && swap.amount1_in != 0.into() && swap.amount0_out != 0.into())
            || (!pair.gtonToken0 && swap.amount0_in != 0.into() && swap.amount1_out != 0.into())
        {
            let event = if pair.gtonToken0 {
                UniV2Buy {
                    swap_id: swap.id.clone(),
                    pair_id: swap.pair.clone(),
                    pair_title: pair.title.clone(),
                    tx_origin: swap.tx_origin.clone(),
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
                    tx_origin: swap.tx_origin.clone(),
                    amount_token_in: swap.amount0_in.clone(),
                    amount_gton_out: swap.amount1_out.clone(),
                    stamp: swap.stamp.clone(),
                    tx_hash: swap.tx_hash.clone(),
                    log_index: swap.log_index.clone(),
                }
            };

            let result = diesel::sql_query(format!(
                "insert into {}(\
                             swap_id,\
                             pair_id,\
                             pair_title,\
                             tx_origin,\
                             amount_token_in,\
                             amount_gton_out,\
                             stamp,\
                             tx_hash,
                             log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9);",
                buy_table
            ))
            .bind::<BigInt, _>(&event.swap_id)
            .bind::<BigInt, _>(&event.pair_id)
            .bind::<Text, _>(&event.pair_title)
            .bind::<Text, _>(&event.tx_origin)
            .bind::<Numeric, _>(&event.amount_token_in)
            .bind::<Numeric, _>(&event.amount_gton_out)
            .bind::<Timestamp, _>(&event.stamp)
            .bind::<Text, _>(&event.tx_hash)
            .bind::<BigInt, _>(&event.log_index)
            .execute(&pool.get().unwrap());
            match result {
                // ignore if already processed, panic otherwise
                Ok(_) => (),
                Err(DatabaseError(UniqueViolation, _)) => (),
                Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
            };
        }
    }
}

pub async fn report_sell(
    pool: &DbPool,
    pair_table: &str,
    swap_table: &str,
    sell_table: &str,
    web3: &Web3<Http>,
) {
    // from last block in the report table, get swap table,
    let swaps = diesel::sql_query(format!(
        "SELECT id, pair_id, tx_origin, amount0_in, amount1_in, \
         amount0_out, amount1_out, stamp, block_number, tx_hash, log_index \
         FROM {} \
         ORDER BY block_number ASC;",
        swap_table
    ))
    .get_results::<Swap>(&pool.get().unwrap())
    .unwrap();

    // get gtonToken0 from pair created table
    for swap in swaps {
        let pair = diesel::sql_query(format!(
            "SELECT id, address, gtonToken0, title \
         FROM {} \
         WHERE id=$1;",
            pair_table
        ))
        .bind::<BigInt, _>(swap.pair)
        .get_result::<Pair>(&pool.get().unwrap())
        .unwrap();

        if (pair.gtonToken0 && swap.amount0_in != 0.into() && swap.amount1_out != 0.into())
            || (!pair.gtonToken0 && swap.amount1_in != 0.into() && swap.amount0_out != 0.into())
        {
            let event = if pair.gtonToken0 {
                UniV2Sell {
                    swap_id: swap.id.clone(),
                    pair_id: swap.pair.clone(),
                    pair_title: pair.title.clone(),
                    tx_origin: swap.tx_origin.clone(),
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
                    tx_origin: swap.tx_origin.clone(),
                    amount_gton_in: swap.amount1_in.clone(),
                    amount_token_out: swap.amount0_out.clone(),
                    stamp: swap.stamp.clone(),
                    tx_hash: swap.tx_hash.clone(),
                    log_index: swap.log_index.clone(),
                }
            };

            let result = diesel::sql_query(format!(
                "insert into {}(\
                             swap_id,\
                             pair_id,\
                             pair_title,\
                             tx_origin,\
                             amount_gton_in,\
                             amount_token_out,\
                             stamp,\
                             tx_hash,
                                   log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9);",
                sell_table
            ))
            .bind::<BigInt, _>(&event.swap_id)
            .bind::<BigInt, _>(&event.pair_id)
            .bind::<Text, _>(&event.pair_title)
            .bind::<Text, _>(&event.tx_origin)
            .bind::<Numeric, _>(&event.amount_gton_in)
            .bind::<Numeric, _>(&event.amount_token_out)
            .bind::<Timestamp, _>(&event.stamp)
            .bind::<Text, _>(&event.tx_hash)
            .bind::<BigInt, _>(&event.log_index)
            .execute(&pool.get().unwrap());
            match result {
                // ignore if already processed, panic otherwise
                Ok(_) => (),
                Err(DatabaseError(UniqueViolation, _)) => (),
                Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
            };
        }
    }
}

pub async fn report_lp_add(
    pool: &DbPool,
    pair_table: &str,
    mint_table: &str,
    lp_transfer_table: &str,
    lp_add_table: &str,
    web3: &Web3<Http>,
) {
    // from last block in the report table, get mint table
    let mints = diesel::sql_query(format!(
        "SELECT id, pair_id, amount0, amount1, \
         stamp, block_number, tx_hash, log_index \
         FROM {} \
         ORDER BY block_number ASC;",
        mint_table
    ))
    .get_results::<MintBurn>(&pool.get().unwrap())
    .unwrap();

    for mint in mints {
        let pair = diesel::sql_query(format!(
            "SELECT id, address, gtonToken0, title \
         FROM {} \
         WHERE id=$1;",
            pair_table
        ))
        .bind::<BigInt, _>(mint.pair)
        .get_result::<Pair>(&pool.get().unwrap())
        .unwrap();

        // in tx_hash find log of erc20 transfer from 0x00 to tx_origin
        let transfer = diesel::sql_query(format!(
            "SELECT amount \
             FROM {} \
             WHERE id={}, sender={}, receiver={};",
            lp_transfer_table, mint.pair, C.zero_address, mint.tx_origin
        ))
        .get_result::<Transfer>(&pool.get().unwrap())
        .unwrap();

        // from table of univ2 lp transfers get one with the same tx_hash 0x00 sender and tx_origin receiver
        // if no such log, use sender
        let event = if pair.gtonToken0 {
            UniV2LPAdd {
                mint_id: mint.id.clone(),
                pair_id: mint.pair.clone(),
                pair_title: pair.title.clone(),
                tx_origin: mint.tx_origin.clone(),
                amount_gton_in: mint.amount0.clone(),
                amount_token_in: mint.amount1.clone(),
                amount_lp_out: transfer.amount.clone(),
                stamp: mint.stamp.clone(),
                tx_hash: mint.tx_hash.clone(),
                log_index: mint.log_index.clone(),
            }
        } else {
            UniV2LPAdd {
                mint_id: mint.id.clone(),
                pair_id: mint.pair.clone(),
                pair_title: pair.title.clone(),
                tx_origin: mint.tx_origin.clone(),
                amount_gton_in: mint.amount1.clone(),
                amount_token_in: mint.amount0.clone(),
                amount_lp_out: transfer.amount.clone(),
                stamp: mint.stamp.clone(),
                tx_hash: mint.tx_hash.clone(),
                log_index: mint.log_index.clone(),
            }
        };
        let result = diesel::sql_query(format!(
            "insert into {}(\
                     mint_id,\
                     pair_id,\
                     pair_title,\
                     tx_origin,\
                     amount_gton_in,\
                     amount_token_in,\
                     amount_lp_out,\
                     stamp,\
                     tx_hash,
                     log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10);",
            lp_add_table
        ))
        .bind::<BigInt, _>(&event.mint_id)
        .bind::<BigInt, _>(&event.pair_id)
        .bind::<Text, _>(&event.pair_title)
        .bind::<Text, _>(&event.tx_origin)
        .bind::<Numeric, _>(&event.amount_gton_in)
        .bind::<Numeric, _>(&event.amount_token_in)
        .bind::<Numeric, _>(&event.amount_lp_out)
        .bind::<Timestamp, _>(&event.stamp)
        .bind::<Text, _>(&event.tx_hash)
        .bind::<BigInt, _>(&event.log_index)
        .execute(&pool.get().unwrap());
        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => (),
            Err(DatabaseError(UniqueViolation, _)) => (),
            Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
        };
    }
}

pub async fn report_lp_remove(
    pool: &DbPool,
    pair_table: &str,
    burn_table: &str,
    lp_transfer_table: &str,
    lp_remove_table: &str,
    web3: &Web3<Http>,
) {
    // from last block in the report table, get burn table
    let burns = diesel::sql_query(format!(
        "SELECT id, pair_id, amount0, amount1, \
         stamp, block_number, tx_hash, log_index \
         FROM {} \
         ORDER BY block_number ASC;",
        burn_table
    ))
    .get_results::<MintBurn>(&pool.get().unwrap())
    .unwrap();

    for burn in burns {
        let pair = diesel::sql_query(format!(
            "SELECT id, address, gtonToken0, title \
         FROM {} \
         WHERE id=$1;",
            pair_table
        ))
        .bind::<BigInt, _>(burn.pair)
        .get_result::<Pair>(&pool.get().unwrap())
        .unwrap();

        // in tx_hash find log of erc20 transfer from 0x00 to tx_origin
        let transfer = diesel::sql_query(format!(
            "SELECT amount \
             FROM {} \
             WHERE id={}, sender={}, receiver={};",
            lp_transfer_table, burn.pair, pair.address, C.zero_address
        ))
        .get_result::<Transfer>(&pool.get().unwrap())
        .unwrap();

        // from table of univ2 lp transfers get one with the same tx_hash 0x00 sender and tx_origin receiver
        // if no such log, use sender
        let event = if pair.gtonToken0 {
            UniV2LPRemove {
                burn_id: burn.id.clone(),
                pair_id: burn.pair.clone(),
                pair_title: pair.title.clone(),
                tx_origin: burn.tx_origin.clone(),
                amount_gton_out: burn.amount0.clone(),
                amount_token_out: burn.amount1.clone(),
                amount_lp_in: transfer.amount.clone(),
                stamp: burn.stamp.clone(),
                tx_hash: burn.tx_hash.clone(),
                log_index: burn.log_index.clone(),
            }
        } else {
            UniV2LPRemove {
                burn_id: burn.id.clone(),
                pair_id: burn.pair.clone(),
                pair_title: pair.title.clone(),
                tx_origin: burn.tx_origin.clone(),
                amount_gton_out: burn.amount1.clone(),
                amount_token_out: burn.amount0.clone(),
                amount_lp_in: transfer.amount.clone(),
                stamp: burn.stamp.clone(),
                tx_hash: burn.tx_hash.clone(),
                log_index: burn.log_index.clone(),
            }
        };
        let result = diesel::sql_query(format!(
            "insert into {}(\
                     burn_id,\
                     pair_id,\
                     pair_title,\
                     tx_origin,\
                     amount_gton_out,\
                     amount_token_out,\
                     amount_lp_in,
                     stamp,\
                     tx_hash,
                     log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10);",
            lp_remove_table
        ))
        .bind::<BigInt, _>(&event.burn_id)
        .bind::<BigInt, _>(&event.pair_id)
        .bind::<Text, _>(&event.pair_title)
        .bind::<Text, _>(&event.tx_origin)
        .bind::<Numeric, _>(&event.amount_gton_out)
        .bind::<Numeric, _>(&event.amount_token_out)
        .bind::<Numeric, _>(&event.amount_lp_in)
        .bind::<Timestamp, _>(&event.stamp)
        .bind::<Text, _>(&event.tx_hash)
        .bind::<BigInt, _>(&event.log_index)
        .execute(&pool.get().unwrap());
        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => (),
            Err(DatabaseError(UniqueViolation, _)) => (),
            Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
        };
    }
}
