use anyhow::{Context, Result};

use hex::ToHex;
use std::convert::{TryFrom, TryInto};
use std::ops::Index;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::ethabi::{Topic, TopicFilter};
use web3::ethabi::ethereum_types::BigEndianHash;
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

pub async fn poll_events_open_user(
    pool: &DbPool,
    web3: &web3::Web3<Http>,
    latest_block: u64,
) -> Result<()> {
    println!("polling events open user");
    let table_name = "events_open_user";
    let block_step = 2000;
    // get latest block from db
    let last_block = match diesel::sql_query(format!(
        "SELECT block_number FROM {} ORDER BY block_number DESC;",
        table_name
    ))
    .get_result::<LastBlock>(&pool.get().context("get last block from open user table")?)
    {
        Err(e) => {
            println!("failed to fetch last block: {}", e);
            C.balance_keeper_deploy
        },
        Ok(e) => e.block_number.try_into().unwrap(),
    };
    println!("starting from block {:#?}", last_block);

    for x in (last_block..latest_block).step_by(block_step as usize) {

        let mut topics = TopicFilter::default();
        topics.topic0 = Topic::This(C.topic0_balance_keeper_open.parse().unwrap());
        let filter = FilterBuilder::default()
            .from_block(BlockNumber::Number(x.into()))
            .to_block(BlockNumber::Number((x + block_step - 1).into()))
            .address(vec![C.balance_keeper.parse().unwrap()])
            .topic_filter(topics)
            .build();
        let logs: Vec<web3::types::Log> =
            web3.eth().logs(filter).await.context("get logs erc20 approval")?;
        println!("requested {} logs, block {}-{}", logs.len(), x, x + block_step - 1);

        for (i, e) in logs.into_iter().enumerate() {

            #[cfg(target_os = "macos")]
            if debug_limit(pool, table_name, 100) { return Ok(()) }

            let opener: String = hex_to_string(Address::from(e.topics[1]));
            let user_id: BigDecimal =
                BigDecimal::from_str(&e.topics[2].into_uint().to_string()).context("amount to bigdecimal")?;
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

            // get user address from the contract
            let contract = Contract::from_json(
                web3.eth(),
                C.balance_keeper.parse().unwrap(),
                include_bytes!("abi/balance_keeper.json")
            )
                .expect("create balance keeper contract");
            let (user_chain, user_address_bytes): (String, Vec<u8>) = contract
                .query("userChainAddressById", (e.topics[2].into_uint()), None, Options::default(), None)
                .await
                .expect("get user chain address");
            let user_address = hex_to_string(user_address_bytes);

            let event = OpenUser {
                tx_from,
                tx_to,
                opener,
                user_id,
                user_chain,
                user_address,
                stamp,
                block_number,
                tx_hash,
                log_index,
            };

            let result = diesel::sql_query(format!(
                "insert into {}(\
                                   tx_from,\
                                   tx_to,\
                                   opener,\
                                   user_id,\
                                   user_chain,\
                                   user_address,\
                                   stamp,\
                                   block_number,\
                                   tx_hash,
                                   log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10);",
                table_name
            ))
            .bind::<Text, _>(&event.tx_from)
            .bind::<Text, _>(&event.tx_to)
            .bind::<Text, _>(&event.opener)
            .bind::<Numeric, _>(&event.user_id)
            .bind::<Text, _>(&event.user_chain)
            .bind::<Text, _>(&event.user_address)
            .bind::<Timestamp, _>(&event.stamp)
            .bind::<BigInt, _>(&event.block_number)
            .bind::<Text, _>(&event.tx_hash)
            .bind::<BigInt, _>(&event.log_index)
            .execute(&pool.get().context("execute sql query")?);
            match result {
                // ignore if already processed, panic otherwise
                Ok(_) => continue,
                Err(DatabaseError(UniqueViolation, _)) => continue,
                Err(e) => bail!(e)
            };
        }

    }
    Ok(())
}
