use anyhow::{Context, Result};

use web3::transports::Http;
use web3::contract::{Contract, Options};
use web3::{
    Web3,
    types::{
        Block,
        Address,
        Log,
        FilterBuilder,
        BlockNumber,
        U256,
        H256,
        U64,
        TransactionId
    }
};
use web3::ethabi::{
    Topic,
    TopicFilter,
};
use std::convert::TryFrom;
use std::str::FromStr;
use std::ops::Index;
use hex::ToHex;

use crate::DbPool;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Bool, Numeric, Text, Timestamp};
use diesel::result::Error::DatabaseError;
use diesel::result::DatabaseErrorKind::UniqueViolation;

use bigdecimal::{BigDecimal, ToPrimitive};

use chrono::NaiveDateTime;

use super::constants::C;
use super::models::*;

#[derive(QueryableByName, PartialEq, Debug)]
struct LastBlock {
    #[sql_type = "BigInt"]
    block_number: i64,
}

async fn fetch_stamp(web3: &web3::Web3<Http>, block_number: &Option<U64>) -> NaiveDateTime {
   let block: Block<H256> = web3.eth().block(BlockNumber::Number(block_number.unwrap()).into()).await.unwrap().unwrap();
   let stamp_str = block.timestamp.to_string();
   let stamp_big = BigDecimal::from_str(&stamp_str).unwrap();
   let stamp_i64 = stamp_big.to_i64().unwrap();
   NaiveDateTime::from_timestamp(stamp_i64,0)
}

fn parse_block_number(block_number: &Option<U64>) -> i64 {
   let block_number_str = block_number.unwrap().to_string();
   let block_number_big = BigDecimal::from_str(&block_number_str).unwrap();
   block_number_big.to_i64().unwrap()
}

fn hex_to_string<T: ToHex>(h: T) -> String {
    "0x".to_owned() + &h.encode_hex::<String>()
}

pub async fn poll_events_erc20_approval(
    pool: &DbPool,
    table_name: &str,
    web3: &web3::Web3<Http>,
    token: &str,
) {

    println!("polling events erc20 approval");
    // get latest block from db
    let last_block: BlockNumber = match diesel::sql_query(
        format!("SELECT block_number FROM {} ORDER BY block_number DESC;", table_name),
    )
        .get_result::<LastBlock>(&pool.get().unwrap()) {
            Err(_) => BlockNumber::Earliest,
            Ok(e) => BlockNumber::Number(e.block_number.into())
        };
    println!("starting from block {:#?}", last_block);

    let mut topics = TopicFilter::default();
    topics.topic0 = Topic::This(C.topic0_erc20_approve.parse().unwrap());

    let filter = FilterBuilder::default()
                .from_block(last_block)
                .to_block(BlockNumber::Latest)
                .address(vec![token.parse().unwrap()])
                .topic_filter(topics)
                .build();
    let result: Vec<web3::types::Log> = web3.eth().logs(filter).await.unwrap();


    for e in result.into_iter() {
        let owner: String = hex_to_string(Address::from(e.topics[1]));
        let spender: String = hex_to_string(Address::from(e.topics[2]));
        let amount: BigDecimal = BigDecimal::from_str(&U256::from_big_endian(&e.data.0).to_string()).unwrap();
        let block_number = parse_block_number(&e.block_number);
        let stamp = fetch_stamp(&web3, &e.block_number).await;
        let tx_hash = hex_to_string(e.transaction_hash.unwrap());
        let log_index = i64::try_from(e.log_index.unwrap().as_u64()).unwrap();
        // get transaction origin
        let tx = &web3.eth().transaction(TransactionId::Hash(tx_hash.parse().unwrap())).await.unwrap().unwrap();
        let tx_origin = hex_to_string(tx.from);
        let event = EventERC20Approval {
            tx_origin,
            owner,
            spender,
            amount,
            stamp,
            block_number,
            tx_hash,
            log_index
        };

        let result = diesel::sql_query(
            format!("insert into {}(\
                               tx_origin,\
                               owner,\
                               spender,\
                               amount,\
                               stamp,\
                               block_number,\
                               tx_hash,
                               log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8);",
            table_name),
        )
        .bind::<Text, _>(&event.tx_origin)
        .bind::<Text, _>(&event.owner)
        .bind::<Text, _>(&event.spender)
        .bind::<Numeric, _>(&event.amount)
        .bind::<Timestamp, _>(&event.stamp)
        .bind::<BigInt, _>(&event.block_number)
        .bind::<Text, _>(&event.tx_hash)
        .bind::<BigInt, _>(&event.log_index)
        .execute(&pool.get().unwrap());
        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => continue,
            Err(DatabaseError(UniqueViolation, _)) => continue,
            Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
        };
    }
}

pub async fn poll_events_erc20_transfer(
    pool: &DbPool,
    table_name: &str,
    web3: &web3::Web3<Http>,
    token: &str,
) {

    println!("polling events erc20 transfer");
    // get latest block from db
    let last_block: BlockNumber = match diesel::sql_query(
        format!("SELECT block_number FROM {} ORDER BY block_number DESC;", table_name),
    )
        .get_result::<LastBlock>(&pool.get().unwrap()) {
            Err(_) => BlockNumber::Earliest,
            Ok(e) => BlockNumber::Number(e.block_number.into())
        };
    println!("starting from block {:#?}", last_block);

    let mut topics = TopicFilter::default();
    topics.topic0 = Topic::This(C.topic0_erc20_transfer.parse().unwrap());

    let filter = FilterBuilder::default()
                .from_block(last_block)
                .to_block(BlockNumber::Latest)
                .address(vec![token.parse().unwrap()])
                .topic_filter(topics)
                .build();
    let result: Vec<web3::types::Log> = web3.eth().logs(filter).await.unwrap();

    for e in result.into_iter() {
        let sender: String = hex_to_string(Address::from(e.topics[1]));
        let receiver: String = hex_to_string(Address::from(e.topics[2]));
        let amount: BigDecimal = BigDecimal::from_str(&U256::from_big_endian(&e.data.0).to_string()).unwrap();
        let block_number = parse_block_number(&e.block_number);
        let stamp = fetch_stamp(&web3, &e.block_number).await;
        let tx_hash = hex_to_string(e.transaction_hash.unwrap());
        let log_index = i64::try_from(e.log_index.unwrap().as_u64()).unwrap();
        // get transaction origin
        let tx = &web3.eth().transaction(TransactionId::Hash(tx_hash.parse().unwrap())).await.unwrap().unwrap();
        let tx_origin = hex_to_string(tx.from);
        let event = EventERC20Transfer {
            tx_origin,
            sender,
            receiver,
            amount,
            stamp,
            block_number,
            tx_hash,
            log_index
        };

        let result = diesel::sql_query(
            format!("insert into {}(\
                               tx_origin,\
                               sender,\
                               receiver,\
                               amount,\
                               stamp,\
                               block_number,\
                               tx_hash,
                               log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8);",
            table_name),
        )
        .bind::<Text, _>(&event.tx_origin)
        .bind::<Text, _>(&event.sender)
        .bind::<Text, _>(&event.receiver)
        .bind::<Numeric, _>(&event.amount)
        .bind::<Timestamp, _>(&event.stamp)
        .bind::<BigInt, _>(&event.block_number)
        .bind::<Text, _>(&event.tx_hash)
        .bind::<BigInt, _>(&event.log_index)
        .execute(&pool.get().unwrap());
        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => continue,
            Err(DatabaseError(UniqueViolation, _)) => continue,
            Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
        };
    }
}

pub async fn poll_events_anyv4_transfer(
    pool: &DbPool,
    table_name: &str,
    web3: &web3::Web3<Http>,
    token: &str,
) {

    println!("polling events anyv4 transfer");
    // get latest block from db
    let last_block: BlockNumber = match diesel::sql_query(
        format!("SELECT block_number FROM {} ORDER BY block_number DESC;", table_name),
    )
        .get_result::<LastBlock>(&pool.get().unwrap()) {
            Err(_) => BlockNumber::Earliest,
            Ok(e) => BlockNumber::Number(e.block_number.into())
        };
    println!("starting from block {:#?}", last_block);

    let mut topics = TopicFilter::default();
    topics.topic0 = Topic::This(C.topic0_erc20_transfer.parse().unwrap());
    topics.topic2 = Topic::This(C.eth_anyv4_vault.parse().unwrap());

    let filter = FilterBuilder::default()
                .from_block(last_block)
                .to_block(BlockNumber::Latest)
                .address(vec![token.parse().unwrap()])
                .topic_filter(topics)
                .build();
    let result: Vec<web3::types::Log> = web3.eth().logs(filter).await.unwrap();

    for e in result.into_iter() {
        let sender: String = hex_to_string(Address::from(e.topics[1]));
        let receiver: String = hex_to_string(Address::from(e.topics[2]));
        let amount: BigDecimal = BigDecimal::from_str(&U256::from_big_endian(&e.data.0).to_string()).unwrap();
        let block_number = parse_block_number(&e.block_number);
        let stamp = fetch_stamp(&web3, &e.block_number).await;
        let tx_hash = hex_to_string(e.transaction_hash.unwrap());
        let log_index = i64::try_from(e.log_index.unwrap().as_u64()).unwrap();
        // get transaction origin
        let tx = &web3.eth().transaction(TransactionId::Hash(tx_hash.parse().unwrap())).await.unwrap().unwrap();
        let tx_origin = hex_to_string(tx.from);
        let event = EventERC20Transfer {
            tx_origin,
            sender,
            receiver,
            amount,
            stamp,
            block_number,
            tx_hash,
            log_index
        };

        let result = diesel::sql_query(
            format!("insert into {}(\
                               tx_origin,\
                               sender,\
                               receiver,\
                               amount,\
                               stamp,\
                               block_number,\
                               tx_hash,
                               log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8);",
            table_name),
        )
        .bind::<Text, _>(&event.tx_origin)
        .bind::<Text, _>(&event.sender)
        .bind::<Text, _>(&event.receiver)
        .bind::<Numeric, _>(&event.amount)
        .bind::<Timestamp, _>(&event.stamp)
        .bind::<BigInt, _>(&event.block_number)
        .bind::<Text, _>(&event.tx_hash)
        .bind::<BigInt, _>(&event.log_index)
        .execute(&pool.get().unwrap());
        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => continue,
            Err(DatabaseError(UniqueViolation, _)) => continue,
            Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
        };
    }
}

pub async fn poll_events_anyv4_swapin(
    pool: &DbPool,
    table_name: &str,
    web3: &web3::Web3<Http>,
    token: &str,
) {

    println!("polling events anyv4 swapin");
    // get latest block from db
    let last_block: BlockNumber = match diesel::sql_query(
        format!("SELECT block_number FROM {} ORDER BY block_number DESC;", table_name),
    )
        .get_result::<LastBlock>(&pool.get().unwrap()) {
            Err(_) => BlockNumber::Earliest,
            Ok(e) => BlockNumber::Number(e.block_number.into())
        };
    println!("starting from block {:#?}", last_block);

    let mut topics = TopicFilter::default();
    topics.topic0 = Topic::This(C.topic0_anyv4_swapin.parse().unwrap());

    let filter = FilterBuilder::default()
                .from_block(last_block)
                .to_block(BlockNumber::Latest)
                .address(vec![token.parse().unwrap()])
                .topic_filter(topics)
                .build();
    let result: Vec<web3::types::Log> = web3.eth().logs(filter).await.unwrap();

    let mut events: Vec<EventAnyV4Swapin> = vec![];
    for e in result.into_iter() {
        let transfer_tx_hash: String = hex_to_string(e.topics[1]);
        let account: String = hex_to_string(Address::from(e.topics[2]));
        let amount: BigDecimal = BigDecimal::from_str(&U256::from_big_endian(&e.data.0).to_string()).unwrap();
        let block_number = parse_block_number(&e.block_number);
        let stamp = fetch_stamp(&web3, &e.block_number).await;
        let tx_hash = hex_to_string(e.transaction_hash.unwrap());
        let log_index = i64::try_from(e.log_index.unwrap().as_u64()).unwrap();
        // get transaction origin
        let tx = &web3.eth().transaction(TransactionId::Hash(tx_hash.parse().unwrap())).await.unwrap().unwrap();
        let tx_origin = hex_to_string(tx.from);
        let event = EventAnyV4Swapin {
            tx_origin,
            transfer_tx_hash,
            account,
            amount,
            stamp,
            block_number,
            tx_hash,
            log_index
        };

        let result = diesel::sql_query(
            format!("insert into {}(\
                               tx_origin,\
                               account,\
                               amount,\
                               transfer_tx_hash,\
                               stamp,\
                               block_number,\
                               tx_hash,
                               log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8);",
            table_name),
        )
        .bind::<Text, _>(&event.tx_origin)
        .bind::<Text, _>(&event.account)
        .bind::<Numeric, _>(&event.amount)
        .bind::<Text, _>(&event.transfer_tx_hash)
        .bind::<Timestamp, _>(&event.stamp)
        .bind::<BigInt, _>(&event.block_number)
        .bind::<Text, _>(&event.tx_hash)
        .bind::<BigInt, _>(&event.log_index)
        .execute(&pool.get().unwrap());
        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => continue,
            Err(DatabaseError(UniqueViolation, _)) => continue,
            Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
        };
    }
}

pub async fn poll_events_anyv4_swapout(
    pool: &DbPool,
    table_name: &str,
    web3: &web3::Web3<Http>,
    token: &str,
) {

    println!("polling events anyv4 swapout");
    // get latest block from db
    let last_block: BlockNumber = match diesel::sql_query(
        format!("SELECT block_number FROM {} ORDER BY block_number DESC;", table_name),
    )
        .get_result::<LastBlock>(&pool.get().unwrap()) {
            Err(_) => BlockNumber::Earliest,
            Ok(e) => BlockNumber::Number(e.block_number.into())
        };
    println!("starting from block {:#?}", last_block);

    let mut topics = TopicFilter::default();
    topics.topic0 = Topic::This(C.topic0_anyv4_swapout.parse().unwrap());

    let filter = FilterBuilder::default()
                .from_block(last_block)
                .to_block(BlockNumber::Latest)
                .address(vec![token.parse().unwrap()])
                .topic_filter(topics)
                .build();
    let result: Vec<web3::types::Log> = web3.eth().logs(filter).await.unwrap();

    let mut events: Vec<EventAnyV4Swapout> = vec![];
    for e in result.into_iter() {
        let account: String = hex_to_string(Address::from(e.topics[1]));
        let bindaddr: String = hex_to_string(Address::from(e.topics[2]));
        let amount: BigDecimal = BigDecimal::from_str(&U256::from_big_endian(&e.data.0).to_string()).unwrap();
        let block_number = parse_block_number(&e.block_number);
        let stamp = fetch_stamp(&web3, &e.block_number).await;
        let tx_hash = hex_to_string(e.transaction_hash.unwrap());
        let log_index = i64::try_from(e.log_index.unwrap().as_u64()).unwrap();
        // get transaction origin
        let tx = &web3.eth().transaction(TransactionId::Hash(tx_hash.parse().unwrap())).await.unwrap().unwrap();
        let tx_origin = hex_to_string(tx.from);
        let event = EventAnyV4Swapout {
            tx_origin,
            account,
            bindaddr,
            amount,
            stamp,
            block_number,
            tx_hash,
            log_index
        };

        let result = diesel::sql_query(
            format!("insert into {}(\
                               tx_origin,\
                               account,\
                               bindaddr,\
                               amount,\
                               stamp,\
                               block_number,\
                               tx_hash,
                               log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8);",
            table_name),
        )
        .bind::<Text, _>(&event.tx_origin)
        .bind::<Text, _>(&event.account)
        .bind::<Text, _>(&event.bindaddr)
        .bind::<Numeric, _>(&event.amount)
        .bind::<Timestamp, _>(&event.stamp)
        .bind::<BigInt, _>(&event.block_number)
        .bind::<Text, _>(&event.tx_hash)
        .bind::<BigInt, _>(&event.log_index)
        .execute(&pool.get().unwrap());
        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => continue,
            Err(DatabaseError(UniqueViolation, _)) => continue,
            Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
        };
    }
}

async fn get_token_name (web3: &Web3<Http>, token: Address) -> String {
    let contract = Contract::from_json(
        web3.eth(),
        token,
        include_bytes!("abi/erc20.json"),
    )
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
    factory: &str
) {

    println!("polling events univ2 pair created");
    // get latest block from db
    let last_block: BlockNumber = match diesel::sql_query(
        format!("SELECT block_number FROM {} ORDER BY block_number DESC;", table_name),
    )
        .get_result::<LastBlock>(&pool.get().unwrap()) {
            Err(_) => BlockNumber::Earliest,
            Ok(e) => BlockNumber::Number(e.block_number.into())
        };
    println!("starting from block {:#?}", last_block);

    let token_addr: Address = token.parse().unwrap();
    let token_h256 = H256::from(token_addr);

    // check pairs where gton is token0
    let mut topics1 = TopicFilter::default();
    topics1.topic0 = Topic::This(C.topic0_univ2_pair_created.parse().unwrap());
    topics1.topic1 = Topic::This(token_h256);

    let filter1 = FilterBuilder::default()
                .from_block(last_block)
                .to_block(BlockNumber::Latest)
                .address(vec![factory.parse().unwrap()])
                .topic_filter(topics1)
                .build();
    let result1: Vec<web3::types::Log> = web3.eth().logs(filter1).await.unwrap();

    // check pairs where gton is token1
    let mut topics2 = TopicFilter::default();
    topics2.topic0 = Topic::This(C.topic0_univ2_pair_created.parse().unwrap());
    topics2.topic2 = Topic::This(token_h256);

    let filter2 = FilterBuilder::default()
                .from_block(last_block)
                .to_block(BlockNumber::Latest)
                .address(vec![factory.parse().unwrap()])
                .topic_filter(topics2)
                .build();
    let result2: Vec<web3::types::Log> = web3.eth().logs(filter2).await.unwrap();

    let result = [result1, result2].concat();

    for e in result.into_iter() {
        let token0: String = hex_to_string(Address::from(e.topics[1]));
        let token1: String = hex_to_string(Address::from(e.topics[2]));
        let mut address_bytes: [u8; 20] = [0;20];
        address_bytes.copy_from_slice(&e.data.0[12..32]);
        let address = hex_to_string(Address::from(&address_bytes));
        let gtonToken0: bool = token0 == token;
        let title0 = get_token_name(&web3, Address::from(e.topics[1])).await;
        let title1 = get_token_name(&web3, Address::from(e.topics[2])).await;
        let title = title0 + "-" + &title1;
        let index_str = U256::from(&e.data.0[32..64]).to_string();
        let index = BigDecimal::from_str(&index_str).unwrap();
        let block_number = parse_block_number(&e.block_number);
        let stamp = fetch_stamp(&web3, &e.block_number).await;
        let tx_hash = hex_to_string(e.transaction_hash.unwrap());
        let log_index = i64::try_from(e.log_index.unwrap().as_u64()).unwrap();
        // get transaction origin
        let tx = &web3.eth().transaction(TransactionId::Hash(tx_hash.parse().unwrap())).await.unwrap().unwrap();
        let tx_origin = hex_to_string(tx.from);
        let event = EventUniV2PairCreated {
            tx_origin,
            address,
            token0,
            token1,
            index,
            gtonToken0,
            title,
            stamp,
            block_number,
            tx_hash,
            log_index
        };

        let result = diesel::sql_query(
            format!("insert into {}(\
                               tx_origin,\
                               address,\
                               token0,\
                               token1,\
                               gtonToken0,\
                               title,\
                               index,\
                               stamp,\
                               block_number,\
                               tx_hash,
                               log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11);",
            table_name),
        )
        .bind::<Text, _>(&event.tx_origin)
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
        .execute(&pool.get().unwrap());
        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => continue,
            Err(DatabaseError(UniqueViolation, _)) => continue,
            Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
        };
    }
}

pub async fn poll_events_univ2_transfer(
    pool: &DbPool,
    table_name: &str,
    web3: &web3::Web3<Http>,
    pair_id: i64,
    pair_address: &str,
) {

    println!("polling events lp transfer");
    // get latest block from db
    let last_block: BlockNumber = match diesel::sql_query(
        format!("SELECT block_number FROM {} ORDER BY block_number DESC;", table_name),
    )
        .get_result::<LastBlock>(&pool.get().unwrap()) {
            Err(_) => BlockNumber::Earliest,
            Ok(e) => BlockNumber::Number(e.block_number.into())
        };
    println!("starting from block {:#?}", last_block);

    let mut topics = TopicFilter::default();
    topics.topic0 = Topic::This(C.topic0_erc20_transfer.parse().unwrap());

    let filter = FilterBuilder::default()
                .from_block(last_block)
                .to_block(BlockNumber::Latest)
                .address(vec![pair_address.parse().unwrap()])
                .topic_filter(topics)
                .build();
    let result: Vec<web3::types::Log> = web3.eth().logs(filter).await.unwrap();

    for e in result.into_iter() {
        let sender: String = hex_to_string(Address::from(e.topics[1]));
        let receiver: String = hex_to_string(Address::from(e.topics[2]));
        let amount: BigDecimal = BigDecimal::from_str(&U256::from_big_endian(&e.data.0).to_string()).unwrap();
        let block_number = parse_block_number(&e.block_number);
        let stamp = fetch_stamp(&web3, &e.block_number).await;
        let tx_hash = hex_to_string(e.transaction_hash.unwrap());
        let log_index = i64::try_from(e.log_index.unwrap().as_u64()).unwrap();
        // get transaction origin
        let tx = &web3.eth().transaction(TransactionId::Hash(tx_hash.parse().unwrap())).await.unwrap().unwrap();
        let tx_origin = hex_to_string(tx.from);
        let event = EventERC20Transfer {
            tx_origin,
            sender,
            receiver,
            amount,
            stamp,
            block_number,
            tx_hash,
            log_index
        };

        let result = diesel::sql_query(
            format!("insert into {}(\
                               pair_id,\
                               tx_origin,\
                               sender,\
                               receiver,\
                               amount,\
                               stamp,\
                               block_number,\
                               tx_hash,
                               log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9);",
            table_name),
        )
        .bind::<BigInt, _>(&pair_id)
        .bind::<Text, _>(&event.tx_origin)
        .bind::<Text, _>(&event.sender)
        .bind::<Text, _>(&event.receiver)
        .bind::<Numeric, _>(&event.amount)
        .bind::<Timestamp, _>(&event.stamp)
        .bind::<BigInt, _>(&event.block_number)
        .bind::<Text, _>(&event.tx_hash)
        .bind::<BigInt, _>(&event.log_index)
        .execute(&pool.get().unwrap());
        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => continue,
            Err(DatabaseError(UniqueViolation, _)) => continue,
            Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
        };
    }
}
pub async fn poll_events_univ2_swap(
    pool: &DbPool,
    table_name: &str,
    web3: &web3::Web3<Http>,
    pair_id: i64,
    pair_address: &str
) {

    println!("polling events univ2 swap");
    // get latest block from db
    let last_block: BlockNumber = match diesel::sql_query(
        format!("SELECT block_number FROM {} \
                 WHERE pair = {} \
                 ORDER BY block_number DESC;",
                table_name,
                pair_id),
    )
        .get_result::<LastBlock>(&pool.get().unwrap()) {
            Err(_) => BlockNumber::Earliest,
            Ok(e) => BlockNumber::Number(e.block_number.into())
        };
    println!("starting from block {:#?}", last_block);

    let mut topics = TopicFilter::default();
    topics.topic0 = Topic::This(C.topic0_univ2_swap.parse().unwrap());

    let filter = FilterBuilder::default()
                .from_block(last_block)
                .to_block(BlockNumber::Latest)
                .address(vec![pair_address.parse().unwrap()])
                .topic_filter(topics)
                .build();
    let result: Vec<web3::types::Log> = web3.eth().logs(filter).await.unwrap();

    for e in result.into_iter() {
        let sender: String = hex_to_string(Address::from(e.topics[1]));
        let receiver: String = hex_to_string(Address::from(e.topics[2]));

        let amount0_in_str = U256::from(&e.data.0[..32]).to_string();
        let amount0_in = BigDecimal::from_str(&amount0_in_str).unwrap();
        let amount1_in_str = U256::from(&e.data.0[32..64]).to_string();
        let amount1_in = BigDecimal::from_str(&amount1_in_str).unwrap();
        let amount0_out_str = U256::from(&e.data.0[64..96]).to_string();
        let amount0_out = BigDecimal::from_str(&amount0_out_str).unwrap();
        let amount1_out_str = U256::from(&e.data.0[96..128]).to_string();
        let amount1_out = BigDecimal::from_str(&amount1_out_str).unwrap();

        let block_number = parse_block_number(&e.block_number);
        let stamp = fetch_stamp(&web3, &e.block_number).await;
        let tx_hash = hex_to_string(e.transaction_hash.unwrap());
        let log_index = i64::try_from(e.log_index.unwrap().as_u64()).unwrap();
        // get transaction origin
        let tx = &web3.eth().transaction(TransactionId::Hash(tx_hash.parse().unwrap())).await.unwrap().unwrap();
        let tx_origin = hex_to_string(tx.from);
        let event = EventUniV2Swap {
            tx_origin,
            sender,
            receiver,
            amount0_in,
            amount1_in,
            amount0_out,
            amount1_out,
            stamp,
            block_number,
            tx_hash,
            log_index
        };

        let result = diesel::sql_query(
            format!("insert into {}(\
                               pair_id,\
                               tx_origin,\
                               sender,\
                               receiver,\
                               amount0_in,\
                               amount1_in,\
                               amount0_out,\
                               amount1_out,\
                               stamp,\
                               block_number,\
                               tx_hash,
                               log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12);",
            table_name),
        )
        .bind::<BigInt, _>(pair_id)
        .bind::<Text, _>(&event.tx_origin)
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
        .execute(&pool.get().unwrap());
        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => continue,
            Err(DatabaseError(UniqueViolation, _)) => continue,
            Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
        };
    }
}

pub async fn poll_events_univ2_mint(
    pool: &DbPool,
    table_name: &str,
    web3: &web3::Web3<Http>,
    pair_id: i64,
    pair_address: &str
) {

    println!("polling events univ2 mint");
    // get latest block from db
    let last_block: BlockNumber = match diesel::sql_query(
        format!("SELECT block_number FROM {} \
                 WHERE pair = {} \
                 ORDER BY block_number DESC;", table_name, pair_id),
    )
        .get_result::<LastBlock>(&pool.get().unwrap()) {
            Err(_) => BlockNumber::Earliest,
            Ok(e) => BlockNumber::Number(e.block_number.into())
        };
    println!("starting from block {:#?}", last_block);

    let mut topics = TopicFilter::default();
    topics.topic0 = Topic::This(C.topic0_univ2_mint.parse().unwrap());

    let filter = FilterBuilder::default()
                .from_block(last_block)
                .to_block(BlockNumber::Latest)
                .address(vec![pair_address.parse().unwrap()])
                .topic_filter(topics)
                .build();
    let result: Vec<web3::types::Log> = web3.eth().logs(filter).await.unwrap();

    for e in result.into_iter() {
        let sender: String = hex_to_string(Address::from(e.topics[1]));

        let amount0_str = U256::from(&e.data.0[..32]).to_string();
        let amount0 = BigDecimal::from_str(&amount0_str).unwrap();
        let amount1_str = U256::from(&e.data.0[32..64]).to_string();
        let amount1 = BigDecimal::from_str(&amount1_str).unwrap();

        let block_number = parse_block_number(&e.block_number);
        let stamp = fetch_stamp(&web3, &e.block_number).await;
        let tx_hash = hex_to_string(e.transaction_hash.unwrap());
        let log_index = i64::try_from(e.log_index.unwrap().as_u64()).unwrap();
        // get transaction origin
        let tx = &web3.eth().transaction(TransactionId::Hash(tx_hash.parse().unwrap())).await.unwrap().unwrap();
        let tx_origin = hex_to_string(tx.from);
        let event = EventUniV2Mint {
            tx_origin,
            sender,
            amount0,
            amount1,
            stamp,
            block_number,
            tx_hash,
            log_index
        };

        let result = diesel::sql_query(
            format!("insert into {}(\
                               pair_id,\
                               tx_origin,\
                               sender,\
                               amount0,\
                               amount1,\
                               stamp,\
                               block_number,\
                               tx_hash,
                               log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9);",
            table_name),
        )
        .bind::<BigInt, _>(pair_id)
        .bind::<Text, _>(&event.tx_origin)
        .bind::<Text, _>(&event.sender)
        .bind::<Numeric, _>(&event.amount0)
        .bind::<Numeric, _>(&event.amount1)
        .bind::<Timestamp, _>(&event.stamp)
        .bind::<BigInt, _>(&event.block_number)
        .bind::<Text, _>(&event.tx_hash)
        .bind::<BigInt, _>(&event.log_index)
        .execute(&pool.get().unwrap());
        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => continue,
            Err(DatabaseError(UniqueViolation, _)) => continue,
            Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
        };
    }
}

pub async fn poll_events_univ2_burn(
    pool: &DbPool,
    table_name: &str,
    web3: &web3::Web3<Http>,
    pair_id: i64,
    pair_address: &str
) {

    println!("polling events univ2 burn");
    // get latest block from db
    let last_block: BlockNumber = match diesel::sql_query(
        format!("SELECT block_number FROM {} \
                 WHERE pair = {} \
                 ORDER BY block_number DESC;", table_name, pair_id),
    )
        .get_result::<LastBlock>(&pool.get().unwrap()) {
            Err(_) => BlockNumber::Earliest,
            Ok(e) => BlockNumber::Number(e.block_number.into())
        };
    println!("starting from block {:#?}", last_block);

    let mut topics = TopicFilter::default();
    topics.topic0 = Topic::This(C.topic0_univ2_burn.parse().unwrap());

    let filter = FilterBuilder::default()
                .from_block(last_block)
                .to_block(BlockNumber::Latest)
                .address(vec![pair_address.parse().unwrap()])
                .topic_filter(topics)
                .build();
    let result: Vec<web3::types::Log> = web3.eth().logs(filter).await.unwrap();

    for e in result.into_iter() {
        let sender: String = hex_to_string(Address::from(e.topics[1]));
        let receiver: String = hex_to_string(Address::from(e.topics[2]));

        let amount0_str = U256::from(&e.data.0[..32]).to_string();
        let amount0 = BigDecimal::from_str(&amount0_str).unwrap();
        let amount1_str = U256::from(&e.data.0[32..64]).to_string();
        let amount1 = BigDecimal::from_str(&amount1_str).unwrap();

        let block_number = parse_block_number(&e.block_number);
        let stamp = fetch_stamp(&web3, &e.block_number).await;
        let tx_hash = hex_to_string(e.transaction_hash.unwrap());
        let log_index = i64::try_from(e.log_index.unwrap().as_u64()).unwrap();
        // get transaction origin
        let tx = &web3.eth().transaction(TransactionId::Hash(tx_hash.parse().unwrap())).await.unwrap().unwrap();
        let tx_origin = hex_to_string(tx.from);
        let event = EventUniV2Burn {
            tx_origin,
            sender,
            receiver,
            amount0,
            amount1,
            stamp,
            block_number,
            tx_hash,
            log_index
        };

        let result = diesel::sql_query(
            format!("insert into {}(\
                               pair_id,\
                               tx_origin,\
                               sender,\
                               receiver,\
                               amount0,\
                               amount1,\
                               stamp,\
                               block_number,\
                               tx_hash,
                               log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10);",
            table_name),
        )
        .bind::<BigInt, _>(pair_id)
        .bind::<Text, _>(&event.tx_origin)
        .bind::<Text, _>(&event.sender)
        .bind::<Text, _>(&event.receiver)
        .bind::<Numeric, _>(&event.amount0)
        .bind::<Numeric, _>(&event.amount1)
        .bind::<Timestamp, _>(&event.stamp)
        .bind::<BigInt, _>(&event.block_number)
        .bind::<Text, _>(&event.tx_hash)
        .bind::<BigInt, _>(&event.log_index)
        .execute(&pool.get().unwrap());
        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => continue,
            Err(DatabaseError(UniqueViolation, _)) => continue,
            Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
        };
    }
}
