use anyhow::{Context, Result};

use hex::ToHex;
use std::convert::{TryFrom, TryInto};
use std::ops::Index;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::ethabi::{Topic, TopicFilter};
use web3::transports::Http;
use web3::{
    types::{Address, Block, BlockNumber, FilterBuilder, TransactionId, H256, U256, U64},
    Web3,
};

use crate::DbPool;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::DatabaseError;
use diesel::sql_types::{BigInt, Bool, Numeric, Text, Timestamp};

use bigdecimal::{BigDecimal, ToPrimitive};

use chrono::NaiveDateTime;

use super::constants::C;
use super::models::*;

#[derive(QueryableByName, PartialEq, Debug)]
struct LastBlock {
    #[sql_type = "BigInt"]
    block_number: i64,
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

async fn fetch_stamp(web3: &web3::Web3<Http>, block_number: U64) -> Result<NaiveDateTime> {
    let block: Block<H256> = web3
        .eth()
        .block(BlockNumber::Number(block_number).into())
        .await
        .context("fetch block")?
        .context("block option")?;
    let stamp_str = block.timestamp.to_string();
    let stamp_big = BigDecimal::from_str(&stamp_str).context("stamp to bigdecimal")?;
    let stamp_i64 = stamp_big.to_i64().context("stamp to i64")?;
    Ok(NaiveDateTime::from_timestamp(stamp_i64, 0))
}

fn parse_block_number(block_number: U64) -> Result<i64> {
    let block_number_str = block_number.to_string();
    let block_number_big = BigDecimal::from_str(&block_number_str).context("block number to bigdecimal")?;
    Ok(block_number_big.to_i64().context("block_number to i64")?)
}

fn hex_to_string<T: ToHex>(h: T) -> String {
    "0x".to_owned() + &h.encode_hex::<String>()
}

pub async fn poll_events_erc20_approval(
    pool: &DbPool,
    table_name: &str,
    web3: &web3::Web3<Http>,
    token: &str,
    default_from_block: u64,
    block_step: u64,
    latest_block: u64,
) -> Result<()> {
    println!("polling events erc20 approval");
    // get latest block from db
    let last_block = match diesel::sql_query(format!(
        "SELECT block_number FROM blocks WHERE name_table='{}';",
        table_name
    ))
    .get_result::<LastBlock>(&pool.get().context("get last block from approval table")?)
    {
        Err(e) => {
            println!("failed to fetch last block: {}", e);
            default_from_block
        },
        Ok(e) => e.block_number.try_into().unwrap(),
    };
    println!("starting from block {:#?}", last_block);

    // get logs in batches of 5k blocks
    // because binance rpc rate limits to 5k
    // because rpc breaks on getting 10k+ logs at once
    for x in (last_block..latest_block).step_by(block_step as usize) {

        let mut topics = TopicFilter::default();
        topics.topic0 = Topic::This(C.topic0_erc20_approve.parse().unwrap());
        let filter = FilterBuilder::default()
            .from_block(BlockNumber::Number(x.into()))
            .to_block(BlockNumber::Number((x + block_step - 1).into()))
            .address(vec![token.parse().unwrap()])
            .topic_filter(topics)
            .build();
        let logs: Vec<web3::types::Log> =
            web3.eth().logs(filter).await.context("get logs erc20 approval")?;
        println!("requested {} logs, block {}-{}", logs.len(), x, x + block_step - 1);

        for (i, e) in logs.into_iter().enumerate() {

            #[cfg(target_os = "macos")]
            if debug_limit(pool, table_name, 100) { return Ok(()) }

            let owner: String = hex_to_string(Address::from(e.topics[1]));
            let spender: String = hex_to_string(Address::from(e.topics[2]));
            let amount: BigDecimal =
                BigDecimal::from_str(&U256::from_big_endian(&e.data.0).to_string()).context("amount to bigdecimal")?;
            let block_number = parse_block_number(e.block_number.context("block number option")?)?;
            let stamp = fetch_stamp(&web3, e.block_number.context("block number option")?).await?;
            let tx_hash = hex_to_string(e.transaction_hash.context("transaction hash option")?);
            let log_index = i64::try_from(e.log_index.context("log index option")?.as_u64()).context("log index to u64")?;

            // get transaction origin
            let tx = &web3
                .eth()
                .transaction(TransactionId::Hash(tx_hash.parse().unwrap()))
                .await
                .context("get transaction from rpc")?
                .context("transaction option")?;
            let tx_from = hex_to_string(tx.from.unwrap());
            let tx_to = hex_to_string(tx.to.context("tx_to option")?);
            let event = EventERC20Approval {
                tx_from,
                tx_to,
                owner,
                spender,
                amount,
                stamp,
                block_number,
                tx_hash,
                log_index,
            };

            let result = diesel::sql_query(format!(
                "insert into {}(\
                                   tx_from,\
                                   tx_to,\
                                   owner,\
                                   spender,\
                                   amount,\
                                   stamp,\
                                   block_number,\
                                   tx_hash,
                                   log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9);",
                table_name
            ))
            .bind::<Text, _>(&event.tx_from)
            .bind::<Text, _>(&event.tx_to)
            .bind::<Text, _>(&event.owner)
            .bind::<Text, _>(&event.spender)
            .bind::<Numeric, _>(&event.amount)
            .bind::<Timestamp, _>(&event.stamp)
            .bind::<BigInt, _>(&event.block_number)
            .bind::<Text, _>(&event.tx_hash)
            .bind::<BigInt, _>(&event.log_index)
            .execute(&pool.get().context("execute sql query")?);
            match result {
                // ignore if already processed, panic otherwise
                Ok(_) => (),
                Err(DatabaseError(UniqueViolation, _)) => (),
                Err(e) => bail!(e)
            };

            diesel::sql_query(format!(
                "UPDATE blocks SET block_number={} WHERE name_table='{}'",
                block_number, table_name
            ))
                .execute(&pool.get().context("execute sql query")?);
        }
        diesel::sql_query(format!(
            "UPDATE blocks SET block_number={} WHERE name_table='{}'",
            x, table_name
        ))
            .execute(&pool.get().context("execute sql query")?);
    }

    Ok(())
}

pub async fn poll_events_erc20_transfer(
    pool: &DbPool,
    table_name: &str,
    web3: &web3::Web3<Http>,
    token: &str,
    default_from_block: u64,
    block_step: u64,
    latest_block: u64,
) -> Result<()> {
    println!("polling events erc20 transfer");
    // get latest block from db
    let last_block = match diesel::sql_query(format!(
        "SELECT block_number FROM blocks WHERE name_table='{}';",
        table_name
    ))
    .get_result::<LastBlock>(&pool.get().context("get last block from erc20 transfer table")?)
    {
        Err(e) => {
            println!("failed to fetch last block: {}", e);
            default_from_block
        },
        Ok(e) => e.block_number.try_into().unwrap(),
    };
    println!("starting from block {:#?}", last_block);

    // get logs in batches of 5k blocks
    // because binance rpc rate limits to 5k
    // because rpc breaks on getting 10k+ logs at once
    for x in (last_block..latest_block).step_by(block_step as usize) {

        let mut topics = TopicFilter::default();
        topics.topic0 = Topic::This(C.topic0_erc20_transfer.parse().unwrap());
        let filter = FilterBuilder::default()
            .from_block(BlockNumber::Number(x.into()))
            .to_block(BlockNumber::Number((x + block_step - 1).into()))
            .address(vec![token.parse().unwrap()])
            .topic_filter(topics)
            .build();
        let logs: Vec<web3::types::Log> =
            web3.eth().logs(filter).await.context("get logs erc20 transfer")?;
        println!("requested {} logs, block {}-{}", logs.len(), x, x + block_step - 1);

        for (i, e) in logs.into_iter().enumerate() {

        #[cfg(target_os = "macos")]
        if debug_limit(pool, table_name, 100) { return Ok(()) }

            let sender: String = hex_to_string(Address::from(e.topics[1]));
            let receiver: String = hex_to_string(Address::from(e.topics[2]));
            let amount: BigDecimal =
                BigDecimal::from_str(&U256::from_big_endian(&e.data.0).to_string()).context("amount to bigdecimal")?;
            let block_number = parse_block_number(e.block_number.context("block number option")?)?;
            let stamp = fetch_stamp(&web3, e.block_number.context("block number option")?).await?;
            let tx_hash = hex_to_string(e.transaction_hash.context("transaction hash option")?);
            let log_index = i64::try_from(e.log_index.context("log index option")?.as_u64()).context("log index to u64")?;
            let tx = &web3
                .eth()
                .transaction(TransactionId::Hash(tx_hash.parse().unwrap()))
                .await
                .context("get transaction from rpc")?
                .context("transaction option")?;
            let tx_from = hex_to_string(tx.from.unwrap());
            // first transfer is contract creation and has no tx.to
            // insert empty string to avoid null
            let tx_to = match tx.to {
                None => String::from(""),
                Some(to) => hex_to_string(to),
            };
            let event = EventERC20Transfer {
                tx_from,
                tx_to,
                sender,
                receiver,
                amount,
                stamp,
                block_number,
                tx_hash,
                log_index,
            };

            let result = diesel::sql_query(format!(
                "insert into {}(\
                                   tx_from,\
                                   tx_to,\
                                   sender,\
                                   receiver,\
                                   amount,\
                                   stamp,\
                                   block_number,\
                                   tx_hash,
                                   log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9);",
                table_name
            ))
            .bind::<Text, _>(&event.tx_from)
            .bind::<Text, _>(&event.tx_to)
            .bind::<Text, _>(&event.sender)
            .bind::<Text, _>(&event.receiver)
            .bind::<Numeric, _>(&event.amount)
            .bind::<Timestamp, _>(&event.stamp)
            .bind::<BigInt, _>(&event.block_number)
            .bind::<Text, _>(&event.tx_hash)
            .bind::<BigInt, _>(&event.log_index)
            .execute(&pool.get().context("execute sql query")?);
            match result {
                Ok(_) => (),
                Err(DatabaseError(UniqueViolation, _)) => (),
                Err(e) => bail!(e)
            };

            diesel::sql_query(format!(
                "UPDATE blocks SET block_number={} WHERE name_table='{}'",
                block_number, table_name
            ))
                .execute(&pool.get().context("execute sql query")?);
        }
        diesel::sql_query(format!(
            "UPDATE blocks SET block_number={} WHERE name_table='{}'",
            x, table_name
        ))
            .execute(&pool.get().context("execute sql query")?);
    }
    Ok(())
}

pub async fn poll_events_anyv4_transfer(
    pool: &DbPool,
    table_name: &str,
    web3: &web3::Web3<Http>,
    token: &str,
    default_from_block: u64,
    block_step: u64,
    latest_block: u64,
) -> Result<()> {
    println!("polling events anyv4 transfer");
    // get latest block from db
    let last_block = match diesel::sql_query(format!(
        "SELECT block_number FROM blocks WHERE name_table='{}';",
        table_name
    ))
    .get_result::<LastBlock>(&pool.get().context("get last block from table")?)
    {
        Err(e) => {
            println!("failed to fetch last block: {}", e);
            default_from_block
        },
        Ok(e) => e.block_number.try_into().unwrap(),
    };
    println!("starting from block {:#?}", last_block);

    // get logs in batches of 5k blocks
    // because binance rpc rate limits to 5k
    // because rpc breaks on getting 10k+ logs at once
    for x in (last_block..latest_block).step_by(block_step as usize) {

        let vault_addr: Address = C.eth_anyv4_vault.parse().unwrap();
        let vault_h256 = H256::from(vault_addr);
        let mut topics = TopicFilter::default();
        topics.topic0 = Topic::This(C.topic0_erc20_transfer.parse().unwrap());
        topics.topic2 = Topic::This(vault_h256);
        let filter = FilterBuilder::default()
            .from_block(BlockNumber::Number(x.into()))
            .to_block(BlockNumber::Number((x + block_step - 1).into()))
            .address(vec![token.parse().unwrap()])
            .topic_filter(topics)
            .build();
        let logs: Vec<web3::types::Log> =
            web3.eth().logs(filter).await.context("get logs anyv4 transfer")?;
        println!("requested {} logs, block {}-{}", logs.len(), x, x + block_step - 1);

        for (i, e) in logs.into_iter().enumerate() {

    #[cfg(target_os = "macos")]
    if debug_limit(pool, table_name, 100) { return Ok(()) }

            let sender: String = hex_to_string(Address::from(e.topics[1]));
            let receiver: String = hex_to_string(Address::from(e.topics[2]));
            let amount: BigDecimal =
                BigDecimal::from_str(&U256::from_big_endian(&e.data.0).to_string()).context("amount to bigdecimal")?;
            let block_number = parse_block_number(e.block_number.context("block number option")?)?;
            let stamp = fetch_stamp(&web3, e.block_number.context("block number option")?).await?;
            let tx_hash = hex_to_string(e.transaction_hash.context("transaction hash option")?);
            let log_index = i64::try_from(e.log_index.context("log index option")?.as_u64()).context("log index to u64")?;
            // get transaction origin
            let tx = &web3
                .eth()
                .transaction(TransactionId::Hash(tx_hash.parse().unwrap()))
                .await
                .context("get transaction from rpc")?
                .context("transaction option")?;
            let tx_from = hex_to_string(tx.from.unwrap());
            let tx_to = hex_to_string(tx.to.context("tx_to option")?);
            let event = EventERC20Transfer {
                tx_from,
                tx_to,
                sender,
                receiver,
                amount,
                stamp,
                block_number,
                tx_hash,
                log_index,
            };

            let result = diesel::sql_query(format!(
                "insert into {}(\
                                   tx_from,\
                                   tx_to,\
                                   sender,\
                                   receiver,\
                                   amount,\
                                   stamp,\
                                   block_number,\
                                   tx_hash,
                                   log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9);",
                table_name
            ))
            .bind::<Text, _>(&event.tx_from)
            .bind::<Text, _>(&event.tx_to)
            .bind::<Text, _>(&event.sender)
            .bind::<Text, _>(&event.receiver)
            .bind::<Numeric, _>(&event.amount)
            .bind::<Timestamp, _>(&event.stamp)
            .bind::<BigInt, _>(&event.block_number)
            .bind::<Text, _>(&event.tx_hash)
            .bind::<BigInt, _>(&event.log_index)
            .execute(&pool.get().context("execute sql query")?);
            match result {
                // ignore if already processed, panic otherwise
                Ok(_) => (),
                Err(DatabaseError(UniqueViolation, _)) => (),
                Err(e) => bail!(e)
            };

            diesel::sql_query(format!(
                "UPDATE blocks SET block_number={} WHERE name_table='{}'",
                block_number, table_name
            ))
                .execute(&pool.get().context("execute sql query")?);
        }
        diesel::sql_query(format!(
            "UPDATE blocks SET block_number={} WHERE name_table='{}'",
            x, table_name
        ))
            .execute(&pool.get().context("execute sql query")?);
    }
    Ok(())
}

pub async fn poll_events_anyv4_swapin(
    pool: &DbPool,
    table_name: &str,
    web3: &web3::Web3<Http>,
    token: &str,
    default_from_block: u64,
    block_step: u64,
    latest_block: u64,
) -> Result<()> {
    println!("polling events anyv4 swapin");
    // get latest block from db
    let last_block = match diesel::sql_query(format!(
        "SELECT block_number FROM blocks WHERE name_table='{}';",
        table_name
    ))
    .get_result::<LastBlock>(&pool.get().context("get last block from table")?)
    {
        Err(e) => {
            println!("failed to fetch last block: {}", e);
            default_from_block
        },
        Ok(e) => e.block_number.try_into().unwrap(),
    };
    println!("starting from block {:#?}", last_block);

    // get logs in batches of blocks
    // because binance rpc rate limits to 5k
    // because fantom rpc breaks on getting 10k+ logs at once
    for x in (last_block..latest_block).step_by(block_step as usize) {

        let mut topics = TopicFilter::default();
        topics.topic0 = Topic::This(C.topic0_anyv4_swapin.parse().unwrap());
        let filter = FilterBuilder::default()
            .from_block(BlockNumber::Number(x.into()))
            .to_block(BlockNumber::Number((x + block_step - 1).into()))
            .address(vec![token.parse().unwrap()])
            .topic_filter(topics)
            .build();
        let logs: Vec<web3::types::Log> =
            web3.eth().logs(filter).await.context("get logs anyv4 swapin")?;
        println!("requested {} logs, block {}-{}", logs.len(), x, x + block_step - 1);

        for (i, e) in logs.into_iter().enumerate() {

    #[cfg(target_os = "macos")]
    if debug_limit(pool, table_name, 100) { return Ok(()) }

            let transfer_tx_hash: String = hex_to_string(e.topics[1]);
            let account: String = hex_to_string(Address::from(e.topics[2]));
            let amount: BigDecimal =
                BigDecimal::from_str(&U256::from_big_endian(&e.data.0).to_string()).context("amount to bigdecimal")?;
            let block_number = parse_block_number(e.block_number.context("block number option")?)?;
            let stamp = fetch_stamp(&web3, e.block_number.context("block number option")?).await?;
            let tx_hash = hex_to_string(e.transaction_hash.context("transaction hash option")?);
            let log_index = i64::try_from(e.log_index.context("log index option")?.as_u64()).context("log index to u64")?;
            // get transaction origin
            let tx = &web3
                .eth()
                .transaction(TransactionId::Hash(tx_hash.parse().unwrap()))
                .await
                .context("get transaction from rpc")?
                .context("transaction option")?;
            let tx_from = hex_to_string(tx.from.unwrap());
            let tx_to = hex_to_string(tx.to.context("tx_to option")?);
            let event = EventAnyV4Swapin {
                tx_from,
                tx_to,
                transfer_tx_hash,
                account,
                amount,
                stamp,
                block_number,
                tx_hash,
                log_index,
            };

            let result = diesel::sql_query(format!(
                "insert into {}(\
                                   tx_from,\
                                   tx_to,\
                                   account,\
                                   amount,\
                                   transfer_tx_hash,\
                                   stamp,\
                                   block_number,\
                                   tx_hash,
                                   log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9);",
                table_name
            ))
            .bind::<Text, _>(&event.tx_from)
            .bind::<Text, _>(&event.tx_to)
            .bind::<Text, _>(&event.account)
            .bind::<Numeric, _>(&event.amount)
            .bind::<Text, _>(&event.transfer_tx_hash)
            .bind::<Timestamp, _>(&event.stamp)
            .bind::<BigInt, _>(&event.block_number)
            .bind::<Text, _>(&event.tx_hash)
            .bind::<BigInt, _>(&event.log_index)
            .execute(&pool.get().context("execute sql query")?);
            match result {
                // ignore if already processed, panic otherwise
                Ok(_) => (),
                Err(DatabaseError(UniqueViolation, _)) => (),
                Err(e) => bail!(e)
            };

            diesel::sql_query(format!(
                "UPDATE blocks SET block_number={} WHERE name_table='{}'",
                block_number, table_name
            ))
                .execute(&pool.get().context("execute sql query")?);
        }
        diesel::sql_query(format!(
            "UPDATE blocks SET block_number={} WHERE name_table='{}'",
            x, table_name
        ))
            .execute(&pool.get().context("execute sql query")?);
    }
    Ok(())
}

pub async fn poll_events_anyv4_swapout(
    pool: &DbPool,
    table_name: &str,
    web3: &web3::Web3<Http>,
    token: &str,
    default_from_block: u64,
    block_step: u64,
    latest_block: u64,
) -> Result<()> {
    println!("polling events anyv4 swapout");
    // get latest block from db
    let last_block = match diesel::sql_query(format!(
        "SELECT block_number FROM blocks WHERE name_table='{}';",
        table_name
    ))
    .get_result::<LastBlock>(&pool.get().context("get last block from table")?)
    {
        Err(e) => {
            println!("failed to fetch last block: {}", e);
            default_from_block
        },
        Ok(e) => e.block_number.try_into().unwrap(),
    };
    println!("starting from block {:#?}", last_block);

    // get logs in batches of blocks
    // because binance rpc rate limits to 5k
    // because fantom rpc breaks on getting 10k+ logs at once
    for x in (last_block..latest_block).step_by(block_step as usize) {

        let mut topics = TopicFilter::default();
        topics.topic0 = Topic::This(C.topic0_anyv4_swapout.parse().unwrap());
        let filter = FilterBuilder::default()
            .from_block(BlockNumber::Number(x.into()))
            .to_block(BlockNumber::Number((x + block_step - 1).into()))
            .address(vec![token.parse().unwrap()])
            .topic_filter(topics)
            .build();
        let logs: Vec<web3::types::Log> =
            web3.eth().logs(filter).await.context("get logs anyv4 swapout")?;
        println!("requested {} logs, block {}-{}", logs.len(), x, x + block_step - 1);

        for (i, e) in logs.into_iter().enumerate() {

    #[cfg(target_os = "macos")]
    if debug_limit(pool, table_name, 100) { return Ok(()) }

            let account: String = hex_to_string(Address::from(e.topics[1]));
            let bindaddr: String = hex_to_string(Address::from(e.topics[2]));
            let amount: BigDecimal =
                BigDecimal::from_str(&U256::from_big_endian(&e.data.0).to_string()).context("amount to bigdecimal")?;
            let block_number = parse_block_number(e.block_number.context("block number option")?)?;
            let stamp = fetch_stamp(&web3, e.block_number.context("block number option")?).await?;
            let tx_hash = hex_to_string(e.transaction_hash.context("transaction hash option")?);
            let log_index = i64::try_from(e.log_index.context("log index option")?.as_u64()).context("log index to u64")?;
            // get transaction origin
            let tx = &web3
                .eth()
                .transaction(TransactionId::Hash(tx_hash.parse().unwrap()))
                .await
                .context("get transaction from rpc")?
                .context("transaction option")?;
            let tx_from = hex_to_string(tx.from.unwrap());
            let tx_to = hex_to_string(tx.to.context("tx_to option")?);
            let event = EventAnyV4Swapout {
                tx_from,
                tx_to,
                account,
                bindaddr,
                amount,
                stamp,
                block_number,
                tx_hash,
                log_index,
            };

            let result = diesel::sql_query(format!(
                "insert into {}(\
                                   tx_from,\
                                   tx_to,\
                                   account,\
                                   bindaddr,\
                                   amount,\
                                   stamp,\
                                   block_number,\
                                   tx_hash,
                                   log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9);",
                table_name
            ))
            .bind::<Text, _>(&event.tx_from)
            .bind::<Text, _>(&event.tx_to)
            .bind::<Text, _>(&event.account)
            .bind::<Text, _>(&event.bindaddr)
            .bind::<Numeric, _>(&event.amount)
            .bind::<Timestamp, _>(&event.stamp)
            .bind::<BigInt, _>(&event.block_number)
            .bind::<Text, _>(&event.tx_hash)
            .bind::<BigInt, _>(&event.log_index)
            .execute(&pool.get().context("execute sql query")?);
            match result {
                // ignore if already processed, panic otherwise
                Ok(_) => (),
                Err(DatabaseError(UniqueViolation, _)) => (),
                Err(e) => bail!(e)
            };

            diesel::sql_query(format!(
                "UPDATE blocks SET block_number={} WHERE name_table='{}'",
                block_number, table_name
            ))
                .execute(&pool.get().context("execute sql query")?);
        }
        diesel::sql_query(format!(
            "UPDATE blocks SET block_number={} WHERE name_table='{}'",
            x, table_name
        ))
            .execute(&pool.get().context("execute sql query")?);
    }
    Ok(())
}

async fn get_token_name(web3: &Web3<Http>, token: Address) -> String {
    let contract = Contract::from_json(web3.eth(), token, include_bytes!("abi/erc20.json"))
        .expect("create erc20 contract");

    let res: String = contract
        .query("name", (), None, Options::default(), None)
        .await
        .expect("get token name");
    res.to_string()
}

pub async fn poll_events_univ2_pair_created(
    pool: &DbPool,
    table_name: &str,
    web3: &web3::Web3<Http>,
    token: &str,
    factory: &str,
    default_from_block: u64,
    block_step: u64,
    latest_block: u64,
) -> Result<()> {
    println!("polling events univ2 pair created");
    // get latest block from db
    let last_block = match diesel::sql_query(format!(
        "SELECT block_number FROM blocks WHERE name_table='{}';",
        table_name
    ))
    .get_result::<LastBlock>(&pool.get().context("get last block from table")?)
    {
        Err(e) => {
            println!("failed to fetch last block: {}", e);
            default_from_block
        },
        Ok(e) => e.block_number.try_into().unwrap(),
    };
    println!("starting from block {:#?}", last_block);

    // get logs in batches of blocks
    // because binance rpc rate limits to 5k
    // because fantom rpc breaks on getting 10k+ logs at once
    for x in (last_block..latest_block).step_by(block_step as usize) {

        let token_addr: Address = token.parse().unwrap();
        let token_h256 = H256::from(token_addr);
        // check pairs where gton is token0
        let mut topics1 = TopicFilter::default();
        topics1.topic0 = Topic::This(C.topic0_univ2_pair_created.parse().unwrap());
        topics1.topic1 = Topic::This(token_h256);
        let filter1 = FilterBuilder::default()
            .from_block(BlockNumber::Number(x.into()))
            .to_block(BlockNumber::Number((x + block_step - 1).into()))
            .address(vec![factory.parse().unwrap()])
            .topic_filter(topics1)
            .build();
        let logs1: Vec<web3::types::Log> =
            web3.eth().logs(filter1).await.context("get logs univ2 pair created")?;
        // check pairs where gton is token1
        let mut topics2 = TopicFilter::default();
        topics2.topic0 = Topic::This(C.topic0_univ2_pair_created.parse().unwrap());
        topics2.topic2 = Topic::This(token_h256);
        let filter2 = FilterBuilder::default()
            .from_block(BlockNumber::Number(x.into()))
            .to_block(BlockNumber::Number((x + block_step - 1).into()))
            .address(vec![factory.parse().unwrap()])
            .topic_filter(topics2)
            .build();
        let logs2: Vec<web3::types::Log> =
            web3.eth().logs(filter2).await.context("get logs univ2 pair created")?;
        let logs = [logs1, logs2].concat();
        println!("requested {} logs, block {}-{}", logs.len(), x, x + block_step - 1);

        for (i, e) in logs.into_iter().enumerate() {


            let token0: String = hex_to_string(Address::from(e.topics[1]));
            let token1: String = hex_to_string(Address::from(e.topics[2]));
            let mut address_bytes: [u8; 20] = [0; 20];
            address_bytes.copy_from_slice(&e.data.0[12..32]);
            let address = hex_to_string(Address::from(&address_bytes));
            let gtonToken0: bool = token0 == token.to_lowercase();
            let title0 = get_token_name(&web3, Address::from(e.topics[1])).await;
            let title1 = get_token_name(&web3, Address::from(e.topics[2])).await;
            let title = title0 + "-" + &title1;
            let index_str = U256::from(&e.data.0[32..64]).to_string();
            let index = BigDecimal::from_str(&index_str).context("index to bigdecimal")?;
            let block_number = parse_block_number(e.block_number.context("block number option")?)?;
            let stamp = fetch_stamp(&web3, e.block_number.context("block number option")?).await?;
            let tx_hash = hex_to_string(e.transaction_hash.context("transaction hash option")?);
            let log_index = i64::try_from(e.log_index.context("log index option")?.as_u64()).context("log index to u64")?;
            // get transaction origin
            let tx = &web3
                .eth()
                .transaction(TransactionId::Hash(tx_hash.parse().unwrap()))
                .await
                .context("get transaction from rpc")?
                .context("transaction option")?;
            let tx_from = hex_to_string(tx.from.unwrap());
            let tx_to = hex_to_string(tx.to.context("tx_to option")?);
            let event = EventUniV2PairCreated {
                tx_from,
                tx_to,
                address,
                token0,
                token1,
                gtonToken0,
                title,
                index,
                stamp,
                block_number,
                tx_hash,
                log_index,
            };

            let result = diesel::sql_query(format!(
                "insert into {}(\
                                   tx_from,\
                                   tx_to,\
                                   address,\
                                   token0,\
                                   token1,\
                                   gtonToken0,\
                                   title,\
                                   index,\
                                   stamp,\
                                   block_number,\
                                   tx_hash,
                                   log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12);",
                table_name
            ))
            .bind::<Text, _>(&event.tx_from)
            .bind::<Text, _>(&event.tx_to)
            .bind::<Text, _>(&event.address)
            .bind::<Text, _>(&event.token0)
            .bind::<Text, _>(&event.token1)
            .bind::<Bool, _>(&event.gtonToken0)
            .bind::<Text, _>(&event.title)
            .bind::<Numeric, _>(&event.index)
            .bind::<Timestamp, _>(&event.stamp)
            .bind::<BigInt, _>(&event.block_number)
            .bind::<Text, _>(&event.tx_hash)
            .bind::<BigInt, _>(&event.log_index)
            .execute(&pool.get().context("execute sql query")?);
            match result {
                // ignore if already processed, panic otherwise
                Ok(_) => (),
                Err(DatabaseError(UniqueViolation, _)) => (),
                Err(e) => bail!(e)
            };

            diesel::sql_query(format!(
                "UPDATE blocks SET block_number={} WHERE name_table='{}'",
                block_number, table_name
            ))
                .execute(&pool.get().context("execute sql query")?);
        }
        diesel::sql_query(format!(
            "UPDATE blocks SET block_number={} WHERE name_table='{}'",
            x, table_name
        ))
            .execute(&pool.get().context("execute sql query")?);
    }
    Ok(())
}

pub async fn poll_events_univ2_transfer(
    pool: &DbPool,
    table_name: &str,
    web3: &web3::Web3<Http>,
    pair_id: i64,
    pair_address: &str,
    default_from_block: u64,
    block_step: u64,
    latest_block: u64,
) -> Result<()> {
    println!("polling events lp transfer");
    // get latest block from db
    let last_block = match diesel::sql_query(format!(
        "SELECT block_number \
         FROM blocks WHERE name_table='{}-{}';",
        table_name, pair_id
    ))
    .get_result::<LastBlock>(&pool.get().context("get last block from table")?)
    {
        Err(e) => {
            println!("failed to fetch last block: {}", e);
            default_from_block
        },
        Ok(e) => e.block_number.try_into().unwrap(),
    };
    println!("starting from block {:#?}", last_block);

    // get logs in batches of blocks
    // because binance rpc rate limits to 5k
    // because fantom rpc breaks on getting 10k+ logs at once
    for x in (last_block..latest_block).step_by(block_step as usize) {

        let mut topics = TopicFilter::default();
        topics.topic0 = Topic::This(C.topic0_erc20_transfer.parse().unwrap());
        let filter = FilterBuilder::default()
            .from_block(BlockNumber::Number(x.into()))
            .to_block(BlockNumber::Number((x + block_step - 1).into()))
            .address(vec![pair_address.parse().unwrap()])
            .topic_filter(topics)
            .build();
        let logs: Vec<web3::types::Log> =
            web3.eth().logs(filter).await.context("get logs univ2 transfer")?;
        println!("requested {} logs, block {}-{}", logs.len(), x, x + block_step - 1);

        for (i, e) in logs.into_iter().enumerate() {

            #[cfg(target_os = "macos")]
            if debug_limit(pool, table_name, 10) { return Ok(()) }

            let sender: String = hex_to_string(Address::from(e.topics[1]));
            let receiver: String = hex_to_string(Address::from(e.topics[2]));
            let amount: BigDecimal =
                BigDecimal::from_str(&U256::from_big_endian(&e.data.0).to_string()).context("amount to bigdecimal")?;
            let block_number = parse_block_number(e.block_number.context("block number option")?)?;
            let stamp = fetch_stamp(&web3, e.block_number.context("block number option")?).await?;
            let tx_hash = hex_to_string(e.transaction_hash.context("transaction hash option")?);
            let log_index = i64::try_from(e.log_index.context("log index option")?.as_u64()).context("log index to u64")?;
            // get transaction origin
            let tx = &web3
                .eth()
                .transaction(TransactionId::Hash(tx_hash.parse().unwrap()))
                .await
                .context("get transaction from rpc")?
                .context("transaction option")?;
            let tx_from = hex_to_string(tx.from.unwrap());
            let tx_to = hex_to_string(tx.to.context("tx_to option")?);
            let event = EventERC20Transfer {
                tx_from,
                tx_to,
                sender,
                receiver,
                amount,
                stamp,
                block_number,
                tx_hash,
                log_index,
            };

            let result = diesel::sql_query(format!(
                "insert into {}(\
                                   pair_id,\
                                   tx_from,\
                                   tx_to,\
                                   sender,\
                                   receiver,\
                                   amount,\
                                   stamp,\
                                   block_number,\
                                   tx_hash,
                                   log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10);",
                table_name
            ))
            .bind::<BigInt, _>(&pair_id)
            .bind::<Text, _>(&event.tx_from)
            .bind::<Text, _>(&event.tx_to)
            .bind::<Text, _>(&event.sender)
            .bind::<Text, _>(&event.receiver)
            .bind::<Numeric, _>(&event.amount)
            .bind::<Timestamp, _>(&event.stamp)
            .bind::<BigInt, _>(&event.block_number)
            .bind::<Text, _>(&event.tx_hash)
            .bind::<BigInt, _>(&event.log_index)
            .execute(&pool.get().context("execute sql query")?);
            match result {
                // ignore if already processed, panic otherwise
                Ok(_) => (),
                Err(DatabaseError(UniqueViolation, _)) => (),
                Err(e) => bail!(e)
            };

            diesel::sql_query(format!(
                "UPDATE blocks SET block_number={} WHERE name_table='{}-{}'",
                block_number, table_name, pair_id
            ))
                .execute(&pool.get().context("execute sql query")?);
        }
        diesel::sql_query(format!(
            "UPDATE blocks SET block_number={} WHERE name_table='{}-{}'",
            x, table_name, pair_id
        ))
            .execute(&pool.get().context("execute sql query")?);
    }
    Ok(())
}
pub async fn poll_events_univ2_swap(
    pool: &DbPool,
    table_name: &str,
    web3: &web3::Web3<Http>,
    pair_id: i64,
    pair_address: &str,
    default_from_block: u64,
    block_step: u64,
    latest_block: u64,
) -> Result<()> {
    println!("polling events univ2 swap");
    // get latest block from db
    let last_block = match diesel::sql_query(format!(
        "SELECT block_number \
         FROM blocks WHERE name_table='{}-{}';",
        table_name, pair_id
    ))
    .get_result::<LastBlock>(&pool.get().context("get last block from table")?)
    {
        Err(e) => {
            println!("failed to fetch last block: {}", e);
            default_from_block}
        ,
        Ok(e) => e.block_number.try_into().unwrap(),
    };
    println!("starting from block {:#?}", last_block);

    // get logs in batches of blocks
    // because binance rpc rate limits to 5k
    // because fantom rpc breaks on getting 10k+ logs at once
    for x in (last_block..latest_block).step_by(block_step as usize) {

        let mut topics = TopicFilter::default();
        topics.topic0 = Topic::This(C.topic0_univ2_swap.parse().unwrap());
        let filter = FilterBuilder::default()
            .from_block(BlockNumber::Number(x.into()))
            .to_block(BlockNumber::Number((x + block_step - 1).into()))
            .address(vec![pair_address.parse().unwrap()])
            .topic_filter(topics)
            .build();
        let logs: Vec<web3::types::Log> =
            web3.eth().logs(filter).await.context("get logs univ2 swap")?;
        println!("requested {} logs, block {}-{}", logs.len(), x, x + block_step - 1);

        for (i, e) in logs.into_iter().enumerate() {

            #[cfg(target_os = "macos")]
            if debug_limit(pool, table_name, 10) { return Ok(()) }

            let sender: String = hex_to_string(Address::from(e.topics[1]));
            let receiver: String = hex_to_string(Address::from(e.topics[2]));

            let amount0_in_str = U256::from(&e.data.0[..32]).to_string();
            let amount0_in = BigDecimal::from_str(&amount0_in_str).context("amount to bigdecimal")?;
            let amount1_in_str = U256::from(&e.data.0[32..64]).to_string();
            let amount1_in = BigDecimal::from_str(&amount1_in_str).context("amount to bigdecimal")?;
            let amount0_out_str = U256::from(&e.data.0[64..96]).to_string();
            let amount0_out = BigDecimal::from_str(&amount0_out_str).context("amount to bigdecimal")?;
            let amount1_out_str = U256::from(&e.data.0[96..128]).to_string();
            let amount1_out = BigDecimal::from_str(&amount1_out_str).context("amount to bigdecimal")?;

            let block_number = parse_block_number(e.block_number.context("block number option")?)?;
            let stamp = fetch_stamp(&web3, e.block_number.context("block number option")?).await?;
            let tx_hash = hex_to_string(e.transaction_hash.context("transaction hash option")?);
            let log_index = i64::try_from(e.log_index.context("log index option")?.as_u64()).context("log index to u64")?;
            // get transaction origin
            let tx = &web3
                .eth()
                .transaction(TransactionId::Hash(tx_hash.parse().unwrap()))
                .await
                .context("get transaction from rpc")?
                .context("transaction option")?;
            let tx_from = hex_to_string(tx.from.unwrap());
            let tx_to = hex_to_string(tx.to.context("tx_to option")?);
            let event = EventUniV2Swap {
                tx_from,
                tx_to,
                sender,
                receiver,
                amount0_in,
                amount1_in,
                amount0_out,
                amount1_out,
                stamp,
                block_number,
                tx_hash,
                log_index,
            };

            let result = diesel::sql_query(format!(
                "insert into {}(\
                                   pair_id,\
                                   tx_from,\
                                   tx_to,\
                                   sender,\
                                   receiver,\
                                   amount0_in,\
                                   amount1_in,\
                                   amount0_out,\
                                   amount1_out,\
                                   stamp,\
                                   block_number,\
                                   tx_hash,
                                   log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13);",
                table_name
            ))
            .bind::<BigInt, _>(pair_id)
            .bind::<Text, _>(&event.tx_from)
            .bind::<Text, _>(&event.tx_to)
            .bind::<Text, _>(&event.sender)
            .bind::<Text, _>(&event.receiver)
            .bind::<Numeric, _>(&event.amount0_in)
            .bind::<Numeric, _>(&event.amount1_in)
            .bind::<Numeric, _>(&event.amount0_out)
            .bind::<Numeric, _>(&event.amount1_out)
            .bind::<Timestamp, _>(&event.stamp)
            .bind::<BigInt, _>(&event.block_number)
            .bind::<Text, _>(&event.tx_hash)
            .bind::<BigInt, _>(&event.log_index)
            .execute(&pool.get().context("execute sql query")?);
            match result {
                // ignore if already processed, panic otherwise
                Ok(_) => (),
                Err(DatabaseError(UniqueViolation, _)) => (),
                Err(e) => bail!(e)
            };

            diesel::sql_query(format!(
                "UPDATE blocks SET block_number={} WHERE name_table='{}-{}'",
                block_number, table_name, pair_id
            ))
                .execute(&pool.get().context("execute sql query")?);
        }
        diesel::sql_query(format!(
            "UPDATE blocks SET block_number={} WHERE name_table='{}-{}'",
            x, table_name, pair_id
        ))
            .execute(&pool.get().context("execute sql query")?);
    }
    Ok(())
}

pub async fn poll_events_univ2_mint(
    pool: &DbPool,
    table_name: &str,
    web3: &web3::Web3<Http>,
    pair_id: i64,
    pair_address: &str,
    default_from_block: u64,
    block_step: u64,
    latest_block: u64,
) -> Result<()> {
    println!("polling events univ2 mint");
    // get latest block from db
    let last_block = match diesel::sql_query(format!(
        "SELECT block_number \
         FROM blocks WHERE name_table='{}-{}';",
        table_name, pair_id
    ))
    .get_result::<LastBlock>(&pool.get().context("get last block from table")?)
    {
        Err(_) => default_from_block,
        Ok(e) => e.block_number.try_into().unwrap(),
    };
    println!("starting from block {:#?}", last_block);

    // get logs in batches of blocks
    // because binance rpc rate limits to 5k
    // because fantom rpc breaks on getting 10k+ logs at once
    for x in (last_block..latest_block).step_by(block_step as usize) {

        let mut topics = TopicFilter::default();
        topics.topic0 = Topic::This(C.topic0_univ2_mint.parse().unwrap());
        let filter = FilterBuilder::default()
            .from_block(BlockNumber::Number(x.into()))
            .to_block(BlockNumber::Number((x + block_step - 1).into()))
            .address(vec![pair_address.parse().unwrap()])
            .topic_filter(topics)
            .build();
        let logs: Vec<web3::types::Log> =
            web3.eth().logs(filter).await.context("get logs univ2 mint")?;
        println!("requested {} logs, block {}-{}", logs.len(), x, x + block_step - 1);

        for (i, e) in logs.into_iter().enumerate() {

            #[cfg(target_os = "macos")]
            if debug_limit(pool, table_name, 10) { return Ok(()) }

            let sender: String = hex_to_string(Address::from(e.topics[1]));

            let amount0_str = U256::from(&e.data.0[..32]).to_string();
            let amount0 = BigDecimal::from_str(&amount0_str).context("amount to bigdecimal")?;
            let amount1_str = U256::from(&e.data.0[32..64]).to_string();
            let amount1 = BigDecimal::from_str(&amount1_str).context("amount to bigdecimal")?;

            let block_number = parse_block_number(e.block_number.context("block number option")?)?;
            let stamp = fetch_stamp(&web3, e.block_number.context("block number option")?).await?;
            let tx_hash = hex_to_string(e.transaction_hash.context("transaction hash option")?);
            let log_index = i64::try_from(e.log_index.context("log index option")?.as_u64()).context("log index to u64")?;
            // get transaction origin
            let tx = &web3
                .eth()
                .transaction(TransactionId::Hash(tx_hash.parse().unwrap()))
                .await
                .context("get transaction from rpc")?
                .context("transaction option")?;
            let tx_from = hex_to_string(tx.from.unwrap());
            let tx_to = hex_to_string(tx.to.context("tx_to option")?);
            let event = EventUniV2Mint {
                tx_from,
                tx_to,
                sender,
                amount0,
                amount1,
                stamp,
                block_number,
                tx_hash,
                log_index,
            };

            let result = diesel::sql_query(format!(
                "insert into {}(\
                                   pair_id,\
                                   tx_from,\
                                   tx_to,\
                                   sender,\
                                   amount0,\
                                   amount1,\
                                   stamp,\
                                   block_number,\
                                   tx_hash,
                                   log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10);",
                table_name
            ))
            .bind::<BigInt, _>(pair_id)
            .bind::<Text, _>(&event.tx_from)
            .bind::<Text, _>(&event.tx_to)
            .bind::<Text, _>(&event.sender)
            .bind::<Numeric, _>(&event.amount0)
            .bind::<Numeric, _>(&event.amount1)
            .bind::<Timestamp, _>(&event.stamp)
            .bind::<BigInt, _>(&event.block_number)
            .bind::<Text, _>(&event.tx_hash)
            .bind::<BigInt, _>(&event.log_index)
            .execute(&pool.get().context("execute sql query")?);
            match result {
                // ignore if already processed, panic otherwise
                Ok(_) => (),
                Err(DatabaseError(UniqueViolation, _)) => (),
                Err(e) => bail!(e)
            };

            diesel::sql_query(format!(
                "UPDATE blocks SET block_number={} WHERE name_table='{}-{}'",
                block_number, table_name, pair_id
            ))
                .execute(&pool.get().context("execute sql query")?);
        }
        diesel::sql_query(format!(
            "UPDATE blocks SET block_number={} WHERE name_table='{}-{}'",
            x, table_name, pair_id
        ))
            .execute(&pool.get().context("execute sql query")?);
    }
    Ok(())
}

pub async fn poll_events_univ2_burn(
    pool: &DbPool,
    table_name: &str,
    web3: &web3::Web3<Http>,
    pair_id: i64,
    pair_address: &str,
    default_from_block: u64,
    block_step: u64,
    latest_block: u64,
) -> Result<()> {
    println!("polling events univ2 burn");
    // get latest block from db
    let last_block = match diesel::sql_query(format!(
        "SELECT block_number \
         FROM blocks WHERE name_table='{}-{}';",
        table_name, pair_id
    ))
    .get_result::<LastBlock>(&pool.get().context("get last block from table")?)
    {
        Err(_) => default_from_block,
        Ok(e) => e.block_number.try_into().unwrap(),
    };
    println!("starting from block {:#?}", last_block);

    // get logs in batches of blocks
    // because binance rpc rate limits to 5k
    // because fantom rpc breaks on getting 10k+ logs at once
    for x in (last_block..latest_block).step_by(block_step as usize) {

        let mut topics = TopicFilter::default();
        topics.topic0 = Topic::This(C.topic0_univ2_burn.parse().unwrap());
        let filter = FilterBuilder::default()
            .from_block(BlockNumber::Number(x.into()))
            .to_block(BlockNumber::Number((x + block_step - 1).into()))
            .address(vec![pair_address.parse().unwrap()])
            .topic_filter(topics)
            .build();
        let logs: Vec<web3::types::Log> =
            web3.eth().logs(filter).await.context("get logs univ2 burn")?;
        println!("requested {} logs, block {}-{}", logs.len(), x, x + block_step - 1);

        for (i, e) in logs.into_iter().enumerate() {

            #[cfg(target_os = "macos")]
            if debug_limit(pool, table_name, 10) { return Ok(()) }

            let sender: String = hex_to_string(Address::from(e.topics[1]));
            let receiver: String = hex_to_string(Address::from(e.topics[2]));

            let amount0_str = U256::from(&e.data.0[..32]).to_string();
            let amount0 = BigDecimal::from_str(&amount0_str).context("amount to bigdecimal")?;
            let amount1_str = U256::from(&e.data.0[32..64]).to_string();
            let amount1 = BigDecimal::from_str(&amount1_str).context("amount to bigdecimal")?;

            let block_number = parse_block_number(e.block_number.context("block number option")?)?;
            let stamp = fetch_stamp(&web3, e.block_number.context("block number option")?).await?;
            let tx_hash = hex_to_string(e.transaction_hash.context("transaction hash option")?);
            let log_index = i64::try_from(e.log_index.context("log index option")?.as_u64()).context("log index to u64")?;
            // get transaction origin
            let tx = &web3
                .eth()
                .transaction(TransactionId::Hash(tx_hash.parse().unwrap()))
                .await
                .context("get transaction from rpc")?
                .context("transaction option")?;
            let tx_from = hex_to_string(tx.from.unwrap());
            let tx_to = hex_to_string(tx.to.context("tx_to option")?);
            let event = EventUniV2Burn {
                tx_from,
                tx_to,
                sender,
                receiver,
                amount0,
                amount1,
                stamp,
                block_number,
                tx_hash,
                log_index,
            };

            let result = diesel::sql_query(format!(
                "insert into {}(\
                                   pair_id,\
                                   tx_from,\
                                   tx_to,\
                                   sender,\
                                   receiver,\
                                   amount0,\
                                   amount1,\
                                   stamp,\
                                   block_number,\
                                   tx_hash,
                                   log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11);",
                table_name
            ))
            .bind::<BigInt, _>(pair_id)
            .bind::<Text, _>(&event.tx_from)
            .bind::<Text, _>(&event.tx_to)
            .bind::<Text, _>(&event.sender)
            .bind::<Text, _>(&event.receiver)
            .bind::<Numeric, _>(&event.amount0)
            .bind::<Numeric, _>(&event.amount1)
            .bind::<Timestamp, _>(&event.stamp)
            .bind::<BigInt, _>(&event.block_number)
            .bind::<Text, _>(&event.tx_hash)
            .bind::<BigInt, _>(&event.log_index)
            .execute(&pool.get().context("execute sql query")?);

            match result {
                // ignore if already processed, panic otherwise
                Ok(_) => (),
                Err(DatabaseError(UniqueViolation, _)) => (),
                Err(e) => bail!(e)
            };

            diesel::sql_query(format!(
                "UPDATE blocks SET block_number={} WHERE name_table='{}-{}'",
                block_number, table_name, pair_id
            ))
                .execute(&pool.get().context("execute sql query")?);
        }
        diesel::sql_query(format!(
            "UPDATE blocks SET block_number={} WHERE name_table='{}-{}'",
            x, table_name, pair_id
        ))
            .execute(&pool.get().context("execute sql query")?);
    }
    Ok(())
}
