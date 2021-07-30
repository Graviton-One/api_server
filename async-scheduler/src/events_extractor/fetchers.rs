use anyhow::{Context, Result};
use serde_json::Value;
use super::models::Blockscan;
use super::constants::C;

fn address_to_bytes32(s: &str) -> String {
     format!("0x{:0>64}", s.trim_start_matches("0x"))
}

async fn fetch_events(c: &Blockscan) -> Result<Vec<Value>> {
    let topic1 = if c.is_topic1 {
        format!("&topic1={}", c.topic1)
    } else {
        String::new()
    };
    let topic2 = if c.is_topic2 {
        format!("&topic2={}", c.topic2)
    } else {
        String::new()
    };
    let topic3 = if c.is_topic3 {
        format!("&topic3={}", c.topic3)
    } else {
        String::new()
    };
    let topics = String::new() + &topic1 + &topic2 + &topic3;
    let url = format!(
        "{}/api?module=logs&action=getLogs\
                       &fromBlock={}\
                       &toBlock={}\
                       &address={}\
                       &topic0={}\
                       {}\
                       &apikey={}",
        c.base_url, c.block_from, c.block_to, c.address, c.topic0, topics, c.apikey
    );
    // println!("{}", url);
    let body: reqwest::Response = reqwest::get(&url).await.context("get url")?;
    let body_text: String = body.text().await.context("parse response into string")?;
    let v: Value = serde_json::from_str(&body_text).context("parse response into json")?;

    if v["status"].as_str().context("parse status to string")? == "1" {
        Ok(v["result"]
            .as_array()
            .context("parse result to array")?
            .clone())
    } else {
        Ok(vec![])
    }
}

pub async fn fetch_all_events(c: Blockscan) -> Result<Vec<Value>> {
    let mut config = c;
    let mut events = vec![];
    loop {
        let es = fetch_events(&config).await?;
        events.append(&mut es.clone());
        if es.len() < 1000 {
            break;
        };
        let last_event = es.last().context("get last event")?;
        let last_block_hex = last_event["blockNumber"]
            .as_str()
            .context("parse last block as string")?;
        let last_block_i64 = i64::from_str_radix(last_block_hex.trim_start_matches("0x"), 16)
            .context("parse last block as i64")?;
        config.block_from = format!("{}", last_block_i64);
    }
    println!("{} events", events.len());
    Ok(events)
}

pub async fn fetch_events_erc20_approval(
    base_url: &str,
    token: &str,
    block_from: &str,
    apikey: &str,
) -> Result<Vec<Value>> {

    println!("fetching events erc20 approval");
    let c = Blockscan {
        base_url: String::from(base_url),
        block_from: String::from(block_from),
        block_to: String::from("latest"),
        address: String::from(token),
        topic0: String::from(C.topic0_erc20_approve),
        is_topic1: false,
        topic1: String::from(""),
        is_topic2: false,
        topic2: String::from(""),
        is_topic3: false,
        topic3: String::from(""),
        apikey: String::from(apikey),
    };
    fetch_all_events(c).await
}

pub async fn fetch_events_erc20_transfer(
    base_url: &str,
    token: &str,
    block_from: &str,
    apikey: &str,
) -> Result<Vec<Value>> {

    println!("fetching events erc20 transfer");
    let c = Blockscan {
        base_url: String::from(base_url),
        block_from: String::from(block_from),
        block_to: String::from("latest"),
        address: String::from(token),
        topic0: String::from(C.topic0_erc20_transfer),
        is_topic1: false,
        topic1: String::from(""),
        is_topic2: false,
        topic2: String::from(""),
        is_topic3: false,
        topic3: String::from(""),
        apikey: String::from(apikey),
    };
    fetch_all_events(c).await
}

pub async fn fetch_events_anyv4_transfer(
    block_from: &str,
    apikey: &str,
) -> Result<Vec<Value>> {

    println!("fetching events anyv4 transfer");
    let c = Blockscan {
        base_url: String::from("https://api.etherscan.io"),
        block_from: String::from(block_from),
        block_to: String::from("latest"),
        address: String::from(C.eth_gton),
        topic0: String::from(C.topic0_erc20_transfer),
        is_topic1: false,
        topic1: String::from(""),
        is_topic2: true,
        topic2: address_to_bytes32(C.eth_anyv4_vault),
        is_topic3: false,
        topic3: String::from(""),
        apikey: String::from(apikey),
    };
    fetch_all_events(c).await
}

pub async fn fetch_events_anyv4_swapin(
    base_url: &str,
    token: &str,
    block_from: &str,
    apikey: &str,
) -> Result<Vec<Value>> {

    println!("fetching events anyv4 swapin");
    let c = Blockscan {
        base_url: String::from(base_url),
        block_from: String::from(block_from),
        block_to: String::from("latest"),
        address: String::from(token),
        topic0: String::from(C.topic0_anyv4_swapin),
        is_topic1: false,
        topic1: String::from(""),
        is_topic2: false,
        topic2: String::from(""),
        is_topic3: false,
        topic3: String::from(""),
        apikey: String::from(apikey),
    };
    fetch_all_events(c).await
}

pub async fn fetch_events_anyv4_swapout(
    base_url: &str,
    token: &str,
    block_from: &str,
    apikey: &str,
) -> Result<Vec<Value>> {

    println!("fetching events anyv4 swapout");
    let c = Blockscan {
        base_url: String::from(base_url),
        block_from: String::from(block_from),
        block_to: String::from("latest"),
        address: String::from(token),
        topic0: String::from(C.topic0_anyv4_swapout),
        is_topic1: false,
        topic1: String::from(""),
        is_topic2: false,
        topic2: String::from(""),
        is_topic3: false,
        topic3: String::from(""),
        apikey: String::from(apikey),
    };
    fetch_all_events(c).await
}

pub async fn fetch_events_univ2_pair_created(
    base_url: &str,
    token: &str,
    factory: &str,
    block_from: &str,
    apikey: &str,
) -> Result<Vec<Value>> {

    println!("fetching events univ2 pools token0");
    let c = Blockscan {
        base_url: String::from(base_url),
        block_from: String::from(block_from),
        block_to: String::from("latest"),
        address: String::from(factory),
        topic0: String::from(C.topic0_univ2_pair_created),
        is_topic1: true,
        topic1: address_to_bytes32(token),
        is_topic2: false,
        topic2: String::from(""),
        is_topic3: false,
        topic3: String::from(""),
        apikey: String::from(apikey),
    };
    let token0 = fetch_all_events(c).await?;

    println!("fetching events univ2 pools token1");
    let c = Blockscan {
        base_url: String::from(base_url),
        block_from: String::from(block_from),
        block_to: String::from("latest"),
        address: String::from(factory),
        topic0: String::from(C.topic0_univ2_pair_created),
        is_topic1: false,
        topic1: String::from(""),
        is_topic2: true,
        topic2: address_to_bytes32(token),
        is_topic3: false,
        topic3: String::from(""),
        apikey: String::from(apikey),
    };
    let token1 = fetch_all_events(c).await?;

    Ok([token0, token1].concat())
}

pub async fn fetch_events_univ2_swap(
    base_url: &str,
    pair: &str,
    block_from: &str,
    apikey: &str,
) -> Result<Vec<Value>> {

    println!("fetching events univ2 pools token0");
    let c = Blockscan {
        base_url: String::from(base_url),
        block_from: String::from(block_from),
        block_to: String::from("latest"),
        address: String::from(pair),
        topic0: String::from(C.topic0_univ2_swap),
        is_topic1: false,
        topic1: String::from(""),
        is_topic2: false,
        topic2: String::from(""),
        is_topic3: false,
        topic3: String::from(""),
        apikey: String::from(apikey),
    };
    fetch_all_events(c).await
}

pub async fn fetch_events_univ2_mint(
    base_url: &str,
    pair: &str,
    block_from: &str,
    apikey: &str,
) -> Result<Vec<Value>> {

    println!("fetching events univ2 pools token0");
    let c = Blockscan {
        base_url: String::from(base_url),
        block_from: String::from(block_from),
        block_to: String::from("latest"),
        address: String::from(pair),
        topic0: String::from(C.topic0_univ2_mint),
        is_topic1: false,
        topic1: String::from(""),
        is_topic2: false,
        topic2: String::from(""),
        is_topic3: false,
        topic3: String::from(""),
        apikey: String::from(apikey),
    };
    fetch_all_events(c).await
}

pub async fn fetch_events_univ2_burn(
    base_url: &str,
    pair: &str,
    block_from: &str,
    apikey: &str,
) -> Result<Vec<Value>> {

    println!("fetching events univ2 pools token0");
    let c = Blockscan {
        base_url: String::from(base_url),
        block_from: String::from(block_from),
        block_to: String::from("latest"),
        address: String::from(pair),
        topic0: String::from(C.topic0_univ2_burn),
        is_topic1: false,
        topic1: String::from(""),
        is_topic2: false,
        topic2: String::from(""),
        is_topic3: false,
        topic3: String::from(""),
        apikey: String::from(apikey),
    };
    fetch_all_events(c).await
}
