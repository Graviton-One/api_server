use crate::DbPool;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Numeric, Text, Timestamp};
use diesel::result::Error::DatabaseError;
use diesel::result::DatabaseErrorKind::UniqueViolation;

use super::constants::C;
use super::fetchers::{fetch_events_erc20_approval, fetch_events_erc20_transfer, fetch_events_anyv4_transfer, fetch_events_anyv4_swapin, fetch_events_anyv4_swapout, fetch_events_univ2_pair_created, fetch_events_univ2_swap, fetch_events_univ2_mint, fetch_events_univ2_burn};
use super::parsers::{parse_events_erc20_approval, parse_events_erc20_transfer, parse_events_anyv4_swapin, parse_events_anyv4_swapout, parse_events_univ2_pair_created, parse_events_univ2_swap, parse_events_univ2_mint, parse_events_univ2_burn};

#[derive(QueryableByName, PartialEq, Debug)]
struct BlockNumber {
    #[sql_type = "BigInt"]
    block_number: i64,
}

#[derive(QueryableByName, PartialEq, Debug)]
struct Pair {
    #[sql_type = "BigInt"]
    id: i64,
    #[sql_type = "Text"]
    address: String,
}

pub async fn poll_events(p: &DbPool, ftmscan_api_key: &str, ethscan_api_key: &str) {

    poll_events_erc20_approval(
        p,
        "events_erc20_approval_ftm",
        "https://api.ftmscan.com",
        ftmscan_api_key,
        C.ftm_gton
    ).await;

    poll_events_erc20_transfer(
        p,
        "events_erc20_transfer_ftm",
        "https://api.ftmscan.com",
        ftmscan_api_key,
        C.ftm_gton
    ).await;

    poll_events_anyv4_transfer(
        p,
        ethscan_api_key
    ).await;

    poll_events_anyv4_swapin(
        p,
        "events_anyv4_swapin_ftm",
        "https://api.ftmscan.com",
        ftmscan_api_key,
        C.ftm_gton
    ).await;

    poll_events_anyv4_swapout(
        p,
        "events_anyv4_swapout_ftm",
        "https://api.ftmscan.com",
        ftmscan_api_key,
        C.ftm_gton
    ).await;

    poll_events_univ2_pair_created(
        p,
        "events_univ2_pair_created_spirit",
        "https://api.ftmscan.com",
        ftmscan_api_key,
        C.ftm_gton,
        C.ftm_spirit_factory
    ).await;

    let pairs = diesel::sql_query(
        format!("SELECT id, address FROM {} ORDER BY block_number DESC;", "events_univ2_pair_created_spirit"),
    )
        .get_results::<Pair>(&p.get().unwrap()).unwrap();

    for pair in pairs {
        poll_events_univ2_swap(
            p,
            "events_univ2_swap_spirit",
            "https://api.ftmscan.com",
            ftmscan_api_key,
            pair.id,
            &pair.address,
        ).await;
        poll_events_univ2_mint(
            p,
            "events_univ2_mint_spirit",
            "https://api.ftmscan.com",
            ftmscan_api_key,
            pair.id,
            &pair.address,
        ).await;
        poll_events_univ2_burn(
            p,
            "events_univ2_burn_spirit",
            "https://api.ftmscan.com",
            ftmscan_api_key,
            pair.id,
            &pair.address,
        ).await;
    }
}

pub async fn poll_events_erc20_approval(
    p: &DbPool,
    table_name: &str,
    api_url: &str,
    api_key: &str,
    gton: &str
) {

    println!("polling events erc20 approval");
    // get latest block from db
    let last_block: String = match diesel::sql_query(
        format!("SELECT block_number FROM {} ORDER BY block_number DESC;", table_name),
    )
        .get_result::<BlockNumber>(&p.get().unwrap()) {
            Err(_) => "earliest".to_string(),
            Ok(e) => e.block_number.to_string()
        };
    println!("starting from block {}", last_block);

    let values_erc20_approval = fetch_events_erc20_approval(
        api_url,
        gton,
        &last_block,
        api_key,
    )
    .await.unwrap();

    let events_erc20_approval = parse_events_erc20_approval(values_erc20_approval)
    .await
    .unwrap();
    for event in events_erc20_approval {
        let result = diesel::sql_query(
            format!("insert into {}(\
                               owner,\
                               spender,\
                               amount,\
                               stamp,\
                               block_number,\
                               tx_hash,
                               log_index) VALUES ($1,$2,$3,$4,$5,$6,$7);",
            table_name),
        )
        .bind::<Text, _>(&event.owner)
        .bind::<Text, _>(&event.spender)
        .bind::<Numeric, _>(&event.amount)
        .bind::<Timestamp, _>(&event.stamp)
        .bind::<BigInt, _>(&event.block_number)
        .bind::<Text, _>(&event.tx_hash)
        .bind::<BigInt, _>(&event.log_index)
        .execute(&p.get().unwrap());
        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => continue,
            Err(DatabaseError(UniqueViolation, _)) => continue,
            Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
        };
    }
}

pub async fn poll_events_erc20_transfer(
    p: &DbPool,
    table_name: &str,
    api_url: &str,
    api_key: &str,
    gton: &str
) {

    println!("polling events erc20 transfer");
    // get latest block from db
    let last_block: String = match diesel::sql_query(
        format!("SELECT block_number FROM {} ORDER BY block_number DESC;", table_name),
    )
        .get_result::<BlockNumber>(&p.get().unwrap()) {
            Err(_) => "earliest".to_string(),
            Ok(e) => e.block_number.to_string()
        };
    println!("starting from block {}", last_block);

    let values_erc20_transfer = fetch_events_erc20_transfer(
        api_url,
        gton,
        &last_block,
        api_key,
    )
    .await.unwrap();

    let events_erc20_transfer = parse_events_erc20_transfer(values_erc20_transfer)
    .await
    .unwrap();
    for event in events_erc20_transfer {
        let result = diesel::sql_query(
            format!("insert into {}(\
                               sender,\
                               receiver,\
                               amount,\
                               stamp,\
                               block_number,\
                               tx_hash,
                               log_index) VALUES ($1,$2,$3,$4,$5,$6,$7);",
            table_name),
        )
        .bind::<Text, _>(&event.sender)
        .bind::<Text, _>(&event.receiver)
        .bind::<Numeric, _>(&event.amount)
        .bind::<Timestamp, _>(&event.stamp)
        .bind::<BigInt, _>(&event.block_number)
        .bind::<Text, _>(&event.tx_hash)
        .bind::<BigInt, _>(&event.log_index)
        .execute(&p.get().unwrap());
        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => continue,
            Err(DatabaseError(UniqueViolation, _)) => continue,
            Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
        };
    }
}

pub async fn poll_events_anyv4_transfer(
    p: &DbPool,
    api_key: &str,
) {
    println!("polling events anyv4 transfer");

    let table_name = "events_anyv4_transfer";

    // get latest block from db
    let last_block: String = match diesel::sql_query(
        format!("SELECT block_number FROM {} ORDER BY block_number DESC;", table_name),
    )
        .get_result::<BlockNumber>(&p.get().unwrap()) {
            Err(_) => "earliest".to_string(),
            Ok(e) => e.block_number.to_string()
        };
    println!("starting from block {}", last_block);

    let values_erc20_transfer = fetch_events_anyv4_transfer(
        &last_block,
        api_key,
    )
    .await.unwrap();

    let events_erc20_transfer = parse_events_erc20_transfer(values_erc20_transfer)
    .await
    .unwrap();
    for event in events_erc20_transfer {
        let result = diesel::sql_query(
            format!("insert into {}(\
                               sender,\
                               receiver,\
                               amount,\
                               stamp,\
                               block_number,\
                               tx_hash,
                               log_index) VALUES ($1,$2,$3,$4,$5,$6,$7);",
            table_name),
        )
        .bind::<Text, _>(&event.sender)
        .bind::<Text, _>(&event.receiver)
        .bind::<Numeric, _>(&event.amount)
        .bind::<Timestamp, _>(&event.stamp)
        .bind::<BigInt, _>(&event.block_number)
        .bind::<Text, _>(&event.tx_hash)
        .bind::<BigInt, _>(&event.log_index)
        .execute(&p.get().unwrap());
        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => continue,
            Err(DatabaseError(UniqueViolation, _)) => continue,
            Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
        };
    }
}

pub async fn poll_events_anyv4_swapin(
    p: &DbPool,
    table_name: &str,
    api_url: &str,
    api_key: &str,
    gton: &str
) {

    println!("polling events anyv4 swapin");
    // get latest block from db
    let last_block: String = match diesel::sql_query(
        format!("SELECT block_number FROM {} ORDER BY block_number DESC;", table_name),
    )
        .get_result::<BlockNumber>(&p.get().unwrap()) {
            Err(_) => "earliest".to_string(),
            Ok(e) => e.block_number.to_string()
        };
    println!("starting from block {}", last_block);

    let values_anyv4_swapin = fetch_events_anyv4_swapin(
        api_url,
        gton,
        &last_block,
        api_key,
    )
    .await.unwrap();

    let events_anyv4_swapin = parse_events_anyv4_swapin(values_anyv4_swapin)
    .await
    .unwrap();
    for event in events_anyv4_swapin {
        let result = diesel::sql_query(
            format!("insert into {}(\
                               account,\
                               amount,\
                               transfer_tx_hash,\
                               stamp,\
                               block_number,\
                               tx_hash,
                               log_index) VALUES ($1,$2,$3,$4,$5,$6,$7);",
            table_name),
        )
        .bind::<Text, _>(&event.account)
        .bind::<Numeric, _>(&event.amount)
        .bind::<Text, _>(&event.transfer_tx_hash)
        .bind::<Timestamp, _>(&event.stamp)
        .bind::<BigInt, _>(&event.block_number)
        .bind::<Text, _>(&event.tx_hash)
        .bind::<BigInt, _>(&event.log_index)
        .execute(&p.get().unwrap());
        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => continue,
            Err(DatabaseError(UniqueViolation, _)) => continue,
            Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
        };
    }
}

pub async fn poll_events_anyv4_swapout(
    p: &DbPool,
    table_name: &str,
    api_url: &str,
    api_key: &str,
    gton: &str
) {

    println!("polling events anyv4 swapout");
    // get latest block from db
    let last_block: String = match diesel::sql_query(
        format!("SELECT block_number FROM {} ORDER BY block_number DESC;", table_name),
    )
        .get_result::<BlockNumber>(&p.get().unwrap()) {
            Err(_) => "earliest".to_string(),
            Ok(e) => e.block_number.to_string()
        };
    println!("starting from block {}", last_block);

    let values_anyv4_swapout = fetch_events_anyv4_swapout(
        api_url,
        gton,
        &last_block,
        api_key,
    )
    .await.unwrap();

    let events_anyv4_swapout = parse_events_anyv4_swapout(values_anyv4_swapout)
    .await
    .unwrap();
    for event in events_anyv4_swapout {
        let result = diesel::sql_query(
            format!("insert into {}(\
                               account,\
                               bindaddr,\
                               amount,\
                               stamp,\
                               block_number,\
                               tx_hash,
                               log_index) VALUES ($1,$2,$3,$4,$5,$6,$7);",
            table_name),
        )
        .bind::<Text, _>(&event.account)
        .bind::<Text, _>(&event.bindaddr)
        .bind::<Numeric, _>(&event.amount)
        .bind::<Timestamp, _>(&event.stamp)
        .bind::<BigInt, _>(&event.block_number)
        .bind::<Text, _>(&event.tx_hash)
        .bind::<BigInt, _>(&event.log_index)
        .execute(&p.get().unwrap());
        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => continue,
            Err(DatabaseError(UniqueViolation, _)) => continue,
            Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
        };
    }
}

pub async fn poll_events_univ2_pair_created(
    p: &DbPool,
    table_name: &str,
    api_url: &str,
    api_key: &str,
    gton: &str,
    factory: &str
) {

    println!("polling events univ2 pair created");
    // get latest block from db
    let last_block: String = match diesel::sql_query(
        format!("SELECT block_number FROM {} ORDER BY block_number DESC;", table_name),
    )
        .get_result::<BlockNumber>(&p.get().unwrap()) {
            Err(_) => "earliest".to_string(),
            Ok(e) => e.block_number.to_string()
        };
    println!("starting from block {}", last_block);

    let values_univ2_pair_created = fetch_events_univ2_pair_created(
        api_url,
        gton,
        factory,
        &last_block,
        api_key
    )
    .await.unwrap();

    let events_univ2_pair_created = parse_events_univ2_pair_created(values_univ2_pair_created)
    .await
    .unwrap();
    for event in events_univ2_pair_created {
        let result = diesel::sql_query(
            format!("insert into {}(\
                               address,\
                               token0,\
                               token1,\
                               index,\
                               stamp,\
                               block_number,\
                               tx_hash,
                               log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8);",
            table_name),
        )
        .bind::<Text, _>(&event.address)
        .bind::<Text, _>(&event.token0)
        .bind::<Text, _>(&event.token1)
        .bind::<Numeric, _>(&event.index)
        .bind::<Timestamp, _>(&event.stamp)
        .bind::<BigInt, _>(&event.block_number)
        .bind::<Text, _>(&event.tx_hash)
        .bind::<BigInt, _>(&event.log_index)
        .execute(&p.get().unwrap());
        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => continue,
            Err(DatabaseError(UniqueViolation, _)) => continue,
            Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
        };
    }
}

pub async fn poll_events_univ2_swap(
    p: &DbPool,
    table_name: &str,
    api_url: &str,
    api_key: &str,
    pair_id: i64,
    pair_address: &str
) {

    println!("polling events univ2 swap");
    // get latest block from db
    let last_block: String = match diesel::sql_query(
        format!("SELECT block_number FROM {} ORDER BY block_number DESC;", table_name),
    )
        .get_result::<BlockNumber>(&p.get().unwrap()) {
            Err(_) => "earliest".to_string(),
            Ok(e) => e.block_number.to_string()
        };
    println!("starting from block {}", last_block);

    let values_univ2_swap = fetch_events_univ2_swap(
        api_url,
        pair_address,
        &last_block,
        api_key
    )
    .await.unwrap();

    let events_univ2_swap = parse_events_univ2_swap(values_univ2_swap)
    .await
    .unwrap();
    for event in events_univ2_swap {
        let result = diesel::sql_query(
            format!("insert into {}(\
                               pair,
                               sender,\
                               receiver,\
                               amount0_in,\
                               amount1_in,\
                               amount0_out,\
                               amount1_out,\
                               stamp,\
                               block_number,\
                               tx_hash,
                               log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11);",
            table_name),
        )
        .bind::<BigInt, _>(pair_id)
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
        .execute(&p.get().unwrap());
        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => continue,
            Err(DatabaseError(UniqueViolation, _)) => continue,
            Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
        };
    }
}

pub async fn poll_events_univ2_mint(
    p: &DbPool,
    table_name: &str,
    api_url: &str,
    api_key: &str,
    pair_id: i64,
    pair_address: &str
) {

    println!("polling events univ2 mint");
    // get latest block from db
    let last_block: String = match diesel::sql_query(
        format!("SELECT block_number FROM {} ORDER BY block_number DESC;", table_name),
    )
        .get_result::<BlockNumber>(&p.get().unwrap()) {
            Err(_) => "earliest".to_string(),
            Ok(e) => e.block_number.to_string()
        };
    println!("starting from block {}", last_block);

    let values_univ2_mint = fetch_events_univ2_mint(
        api_url,
        pair_address,
        &last_block,
        api_key
    )
    .await.unwrap();

    let events_univ2_mint = parse_events_univ2_mint(values_univ2_mint)
    .await
    .unwrap();
    for event in events_univ2_mint {
        let result = diesel::sql_query(
            format!("insert into {}(\
                               pair,
                               sender,\
                               amount0,\
                               amount1,\
                               stamp,\
                               block_number,\
                               tx_hash,
                               log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8);",
            table_name),
        )
        .bind::<BigInt, _>(pair_id)
        .bind::<Text, _>(&event.sender)
        .bind::<Numeric, _>(&event.amount0)
        .bind::<Numeric, _>(&event.amount1)
        .bind::<Timestamp, _>(&event.stamp)
        .bind::<BigInt, _>(&event.block_number)
        .bind::<Text, _>(&event.tx_hash)
        .bind::<BigInt, _>(&event.log_index)
        .execute(&p.get().unwrap());
        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => continue,
            Err(DatabaseError(UniqueViolation, _)) => continue,
            Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
        };
    }
}

pub async fn poll_events_univ2_burn(
    p: &DbPool,
    table_name: &str,
    api_url: &str,
    api_key: &str,
    pair_id: i64,
    pair_address: &str
) {

    println!("polling events univ2 burn");
    // get latest block from db
    let last_block: String = match diesel::sql_query(
        format!("SELECT block_number FROM {} ORDER BY block_number DESC;", table_name),
    )
        .get_result::<BlockNumber>(&p.get().unwrap()) {
            Err(_) => "earliest".to_string(),
            Ok(e) => e.block_number.to_string()
        };
    println!("starting from block {}", last_block);

    let values_univ2_burn = fetch_events_univ2_burn(
        api_url,
        pair_address,
        &last_block,
        api_key
    )
    .await.unwrap();

    let events_univ2_burn = parse_events_univ2_burn(values_univ2_burn)
    .await
    .unwrap();
    for event in events_univ2_burn {
        let result = diesel::sql_query(
            format!("insert into {}(\
                               pair,
                               sender,\
                               receiver,\
                               amount0,\
                               amount1,\
                               stamp,\
                               block_number,\
                               tx_hash,
                               log_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9);",
            table_name),
        )
        .bind::<BigInt, _>(pair_id)
        .bind::<Text, _>(&event.sender)
        .bind::<Text, _>(&event.receiver)
        .bind::<Numeric, _>(&event.amount0)
        .bind::<Numeric, _>(&event.amount1)
        .bind::<Timestamp, _>(&event.stamp)
        .bind::<BigInt, _>(&event.block_number)
        .bind::<Text, _>(&event.tx_hash)
        .bind::<BigInt, _>(&event.log_index)
        .execute(&p.get().unwrap());
        match result {
            // ignore if already processed, panic otherwise
            Ok(_) => continue,
            Err(DatabaseError(UniqueViolation, _)) => continue,
            Err(e) => panic!("write to db: {:#?}, err {}", &event, e),
        };
    }
}
