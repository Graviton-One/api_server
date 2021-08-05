use anyhow::{Context, Result};

use diesel::prelude::*;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::{NotFound, DatabaseError};
use diesel::sql_types::{BigInt, Bool, Numeric, Nullable, Text, Timestamp};

use hex::ToHex;

use std::str::FromStr;
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
    pub pair_id: i64,
    #[sql_type = "Text"]
    pub tx_from: String,
    #[sql_type = "Text"]
    pub tx_to: String,
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
    pub pair_id: i64,
    #[sql_type = "Text"]
    pub tx_from: String,
    #[sql_type = "Text"]
    pub tx_to: String,
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

#[derive(QueryableByName, PartialEq, Debug)]
struct Record {
    #[sql_type = "BigInt"]
    id: i64,
}

pub fn debug_limit(
    pool: &DbPool,
    table_name: &str,
    limit: usize,
) -> bool {
    let recs = diesel::sql_query(format!(
        "SELECT id FROM {};",
        table_name
    ))
        .get_results::<Record>(&pool.get().unwrap()).unwrap();
    recs.len() > limit
}

pub async fn view_buy(
    pool: &DbPool,
    pair_table: &str,
    swap_table: &str,
    buy_table: &str,
) -> Result<()> {
    println!("updating {}", buy_table);

    #[cfg(target_os = "macos")]
    if debug_limit(pool, buy_table, 10) { return Ok(()) }

    // from last block in the report table, get swap table,
    let swaps: Vec<Swap> = diesel::sql_query(format!(
        "SELECT id, pair_id, tx_from, tx_to, amount0_in, amount1_in, \
         amount0_out, amount1_out, stamp, block_number, tx_hash, log_index \
         FROM {} \
         ORDER BY block_number ASC;",
        swap_table
    ))
        .get_results::<Swap>(&pool.get().context("execute sql query")?)
        .context("get events from table")?;

    // get gtonToken0 from pair created table
    for (i, swap) in swaps.into_iter().enumerate() {
        let pair = diesel::sql_query(format!(
            "SELECT id, address, gtonToken0, title \
         FROM {} \
         WHERE id = $1;",
            pair_table
        ))
        .bind::<BigInt, _>(swap.pair_id)
        .get_result::<Pair>(&pool.get().context("execute sql query")?)
        .context("get pair from table")?;

        #[cfg(target_os = "macos")]
        if i > 50 {
            return Ok(());
        }

        if (pair.gtonToken0 && swap.amount1_in != 0.into() && swap.amount0_out != 0.into())
            || (!pair.gtonToken0 && swap.amount0_in != 0.into() && swap.amount1_out != 0.into())
        {
            let event = if pair.gtonToken0 {
                UniV2Buy {
                    swap_id: swap.id.clone(),
                    pair_id: swap.pair_id.clone(),
                    pair_title: pair.title.clone(),
                    tx_from: swap.tx_from.clone(),
                    amount_token_in: swap.amount1_in.clone(),
                    amount_gton_out: swap.amount0_out.clone(),
                    stamp: swap.stamp.clone(),
                    tx_hash: swap.tx_hash.clone(),
                    log_index: swap.log_index.clone(),
                }
            } else {
                UniV2Buy {
                    swap_id: swap.id.clone(),
                    pair_id: swap.pair_id.clone(),
                    pair_title: pair.title.clone(),
                    tx_from: swap.tx_from.clone(),
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
                             tx_from,\
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
            .bind::<Text, _>(&event.tx_from)
            .bind::<Numeric, _>(&event.amount_token_in)
            .bind::<Numeric, _>(&event.amount_gton_out)
            .bind::<Timestamp, _>(&event.stamp)
            .bind::<Text, _>(&event.tx_hash)
            .bind::<BigInt, _>(&event.log_index)
            .execute(&pool.get().context("execute sql query")?);
            match result {
                // ignore if already processed, panic otherwise
                Ok(_) => continue,
                Err(DatabaseError(UniqueViolation, _)) => continue,
                Err(e) => bail!(e),
            };
        }
    }
    Ok(())
}

pub async fn view_sell(
    pool: &DbPool,
    pair_table: &str,
    swap_table: &str,
    sell_table: &str,
) -> Result<()> {
    println!("updating {}", sell_table);

    #[cfg(target_os = "macos")]
    if debug_limit(pool, sell_table, 10) { return Ok(()) }

    // from last block in the report table, get swap table,
    let swaps = diesel::sql_query(format!(
        "SELECT id, pair_id, tx_from, tx_to, amount0_in, amount1_in, \
         amount0_out, amount1_out, stamp, block_number, tx_hash, log_index \
         FROM {} \
         ORDER BY block_number ASC;",
        swap_table
    ))
        .get_results::<Swap>(&pool.get().context("execute sql query")?)
        .context("get events from table")?;

    // get gtonToken0 from pair created table
    for (i, swap) in swaps.into_iter().enumerate() {
        let pair = diesel::sql_query(format!(
            "SELECT id, address, gtonToken0, title \
         FROM {} \
         WHERE id=$1;",
            pair_table
        ))
        .bind::<BigInt, _>(swap.pair_id)
        .get_result::<Pair>(&pool.get().context("execute sql query")?)
        .context("get pair from table")?;

        #[cfg(target_os = "macos")]
        if i > 50 {
            return Ok(());
        }

        if (pair.gtonToken0 && swap.amount0_in != 0.into() && swap.amount1_out != 0.into())
            || (!pair.gtonToken0 && swap.amount1_in != 0.into() && swap.amount0_out != 0.into())
        {
            let event = if pair.gtonToken0 {
                UniV2Sell {
                    swap_id: swap.id.clone(),
                    pair_id: swap.pair_id.clone(),
                    pair_title: pair.title.clone(),
                    tx_from: swap.tx_from.clone(),
                    amount_gton_in: swap.amount0_in.clone(),
                    amount_token_out: swap.amount1_out.clone(),
                    stamp: swap.stamp.clone(),
                    tx_hash: swap.tx_hash.clone(),
                    log_index: swap.log_index.clone(),
                }
            } else {
                UniV2Sell {
                    swap_id: swap.id.clone(),
                    pair_id: swap.pair_id.clone(),
                    pair_title: pair.title.clone(),
                    tx_from: swap.tx_from.clone(),
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
                             tx_from,\
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
            .bind::<Text, _>(&event.tx_from)
            .bind::<Numeric, _>(&event.amount_gton_in)
            .bind::<Numeric, _>(&event.amount_token_out)
            .bind::<Timestamp, _>(&event.stamp)
            .bind::<Text, _>(&event.tx_hash)
            .bind::<BigInt, _>(&event.log_index)
            .execute(&pool.get().context("execute sql query")?);
            match result {
                // ignore if already processed, panic otherwise
                Ok(_) => continue,
                Err(DatabaseError(UniqueViolation, _)) => continue,
                Err(e) => bail!(e),
            };
        }
    }
    Ok(())
}

pub async fn view_lp_add(
    pool: &DbPool,
    pair_table: &str,
    mint_table: &str,
    lp_transfer_table: &str,
    lp_add_table: &str,
) -> Result<()> {
    println!("updating {}", lp_add_table);

    #[cfg(target_os = "macos")]
    if debug_limit(pool, lp_add_table, 10) { return Ok(()) }

    // from last block in the report table, get mint table
    let mints = diesel::sql_query(format!(
        "SELECT id, pair_id, tx_from, tx_to, amount0, amount1, \
         stamp, block_number, tx_hash, log_index \
         FROM {} \
         ORDER BY block_number ASC;",
        mint_table
    ))
        .get_results::<MintBurn>(&pool.get().context("execute sql query")?)
        .context("get events from table")?;

    for (i, mint) in mints.into_iter().enumerate() {
        let pair = diesel::sql_query(format!(
            "SELECT id, address, gtonToken0, title \
         FROM {} \
         WHERE id=$1;",
            pair_table
        ))
        .bind::<BigInt, _>(mint.pair_id)
        .get_result::<Pair>(&pool.get().context("execute sql query")?)
        .context("get pair from table")?;

        // in tx_hash find log of erc20 transfer from 0x00 to tx_from
        let transfer_to_origin = diesel::sql_query(format!(
            "SELECT amount \
             FROM {} \
             WHERE pair_id={} AND sender='{}' AND receiver='{}';",
            lp_transfer_table,
            mint.pair_id,
            C.zero_address,
            mint.tx_from
        ))
            .get_result::<Transfer>(&pool.get().context("execute sql query")?);
        let transfer_to_contract = diesel::sql_query(format!(
            "SELECT amount \
             FROM {} \
             WHERE pair_id={} AND sender='{}' AND receiver='{}';",
            lp_transfer_table,
            mint.pair_id,
            C.zero_address,
            mint.tx_to
        ))
            .get_result::<Transfer>(&pool.get().context("execute sql query")?);
        let transfer = transfer_to_origin.or(transfer_to_contract);
        let amount_lp_out = match transfer {
            Ok(transfer) => Some(transfer.amount),
            Err(NotFound) => {
                println!("no lp transfers match remove liquidity, {}", pair.title);
                None
            },
            Err(e) => bail!(e),
        };

        // from table of univ2 lp transfers get one with the same tx_hash 0x00 sender and tx_from receiver
        // if no such log, use sender
        let event = if pair.gtonToken0 {
            UniV2LPAdd {
                mint_id: mint.id.clone(),
                pair_id: mint.pair_id.clone(),
                pair_title: pair.title.clone(),
                tx_from: mint.tx_from.clone(),
                amount_gton_in: mint.amount0.clone(),
                amount_token_in: mint.amount1.clone(),
                amount_lp_out,
                stamp: mint.stamp.clone(),
                tx_hash: mint.tx_hash.clone(),
                log_index: mint.log_index.clone(),
            }
        } else {
            UniV2LPAdd {
                mint_id: mint.id.clone(),
                pair_id: mint.pair_id.clone(),
                pair_title: pair.title.clone(),
                tx_from: mint.tx_from.clone(),
                amount_gton_in: mint.amount1.clone(),
                amount_token_in: mint.amount0.clone(),
                amount_lp_out,
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
                     tx_from,\
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
        .bind::<Text, _>(&event.tx_from)
        .bind::<Numeric, _>(&event.amount_gton_in)
        .bind::<Numeric, _>(&event.amount_token_in)
        .bind::<Nullable<Numeric>, _>(&event.amount_lp_out)
        .bind::<Timestamp, _>(&event.stamp)
        .bind::<Text, _>(&event.tx_hash)
        .bind::<BigInt, _>(&event.log_index)
        .execute(&pool.get().context("execute sql query")?);

        #[cfg(target_os = "macos")]
        if i == 10 {
            return Ok(());
        }

        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => continue,
            Err(DatabaseError(UniqueViolation, _)) => continue,
            Err(e) => bail!(e),
        };
    }
    Ok(())
}

pub async fn view_lp_remove(
    pool: &DbPool,
    pair_table: &str,
    burn_table: &str,
    lp_transfer_table: &str,
    lp_remove_table: &str,
) -> Result<()> {
    println!("updating {}", lp_remove_table);

    #[cfg(target_os = "macos")]
    if debug_limit(pool, lp_remove_table, 10) { return Ok(()) }

    // from last block in the report table, get burn table
    let burns = diesel::sql_query(format!(
        "SELECT id, pair_id, tx_from, tx_to, amount0, amount1, \
         stamp, block_number, tx_hash, log_index \
         FROM {} \
         ORDER BY block_number ASC;",
        burn_table
    ))
        .get_results::<MintBurn>(&pool.get().context("execute sql query")?)
        .context("get events from table")?;

    for (i, burn) in burns.into_iter().enumerate() {
        let pair = diesel::sql_query(format!(
            "SELECT id, address, gtonToken0, title \
             FROM {} \
             WHERE id=$1;",
            pair_table
        ))
        .bind::<BigInt, _>(burn.pair_id)
        .get_result::<Pair>(&pool.get().context("execute sql query")?)
        .context("get pair from table")?;

        // in tx_hash find log of erc20 transfer from pair to 0x00
        let transfer_result = diesel::sql_query(format!(
            "SELECT amount \
             FROM {} \
             WHERE pair_id={} AND sender='{}' AND receiver='{}';",
            lp_transfer_table,
            pair.id,
            pair.address,
            C.zero_address
        ))
        .get_result::<Transfer>(&pool.get().context("execute sql query")?);

        let amount_lp_in = match transfer_result {
            Ok(transfer) => Some(transfer.amount),
            Err(NotFound) => {
                println!("no lp transfers match remove liquidity, {}", pair.title);
                None
            },
            Err(e) => bail!(e),
        };

        // from table of univ2 lp transfers
        // get one with the same tx_hash 0x00 sender and tx_from receiver
        // if no such log, use sender
        let event = if pair.gtonToken0 {
            UniV2LPRemove {
                burn_id: burn.id.clone(),
                pair_id: burn.pair_id.clone(),
                pair_title: pair.title.clone(),
                tx_from: burn.tx_from.clone(),
                amount_gton_out: burn.amount0.clone(),
                amount_token_out: burn.amount1.clone(),
                amount_lp_in,
                stamp: burn.stamp.clone(),
                tx_hash: burn.tx_hash.clone(),
                log_index: burn.log_index.clone(),
            }
        } else {
            UniV2LPRemove {
                burn_id: burn.id.clone(),
                pair_id: burn.pair_id.clone(),
                pair_title: pair.title.clone(),
                tx_from: burn.tx_from.clone(),
                amount_gton_out: burn.amount1.clone(),
                amount_token_out: burn.amount0.clone(),
                amount_lp_in,
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
                     tx_from,\
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
        .bind::<Text, _>(&event.tx_from)
        .bind::<Numeric, _>(&event.amount_gton_out)
        .bind::<Numeric, _>(&event.amount_token_out)
        .bind::<Nullable<Numeric>, _>(&event.amount_lp_in)
        .bind::<Timestamp, _>(&event.stamp)
        .bind::<Text, _>(&event.tx_hash)
        .bind::<BigInt, _>(&event.log_index)
        .execute(&pool.get().context("execute sql query")?);

        #[cfg(target_os = "macos")]
        if i == 10 {
            return Ok(());
        }

        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => continue,
            Err(DatabaseError(UniqueViolation, _)) => continue,
            Err(e) => bail!(e),
        };
    }
    Ok(())
}

