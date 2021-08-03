use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use serde_json::Value;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;

use super::models::{
    Event, EventAnyV4Swapin, EventAnyV4Swapout, EventERC20Approval, EventERC20Transfer,
    EventUniV2Burn, EventUniV2Mint, EventUniV2PairCreated, EventUniV2Swap,
};

pub fn parse_bytes32_to_decimal(s: &str) -> Result<BigDecimal> {
    let s16 = s.trim_start_matches("0x");
    // first convert to BigInt and decimal string because BigDecimal parses only radix 10
    let n16 = BigInt::parse_bytes(&s16.as_bytes(), 16).context("parse bytes32 as bytes")?;
    let s10 = n16.to_str_radix(10);
    let n10 = BigDecimal::parse_bytes(&s10.as_bytes(), 10).context("parse bytes as decimal")?;
    Ok(n10)
}

pub fn parse_bytes32_to_address(s: &str) -> Result<String> {
    // not using trim_start_matches('0') because addresses with leading zero would break
    Ok(format!("0x{}", s.strip_prefix("0x000000000000000000000000")
               .context("parse byte32 to address")?
               .to_string()))
}

pub fn parse_value_to_event(v: Value) -> Result<Event> {
    let ts: &Vec<Value> = match v["topics"].as_array() {
        None => return Err(anyhow!("parse topics as array")),
        Some(arr) => arr,
    };
    let mut topics: Vec<String> = vec![];
    for t in ts.iter() {
        let topic = match t.as_str() {
            None => return Err(anyhow!("parse topic as string {}", t)),
            Some(s) => s,
        };
        topics.push(topic.to_string().clone());
    }
    let data = match v["data"].as_str() {
        None => return Err(anyhow!("parse data as string {}", v["data"])),
        Some(s) => s.to_string(),
    };
    let stamp = match v["timeStamp"].as_str() {
        None => return Err(anyhow!("parse stamp as string {}", v["timeStamp"])),
        Some(s) => s.to_string(),
    };
    let block_number = match v["blockNumber"].as_str() {
        None => return Err(anyhow!("parse block as string {}", v["blockNumber"])),
        Some(s) => s.to_string(),
    };
    let transaction_hash = match v["transactionHash"].as_str() {
        None => return Err(anyhow!("parse tx hash as string {}", v["transactionHash"])),
        Some(s) => s.to_string(),
    };
    let log_index = match v["logIndex"].as_str() {
        None => return Err(anyhow!("parse log index as string {}", v["logIndex"])),
        Some(s) => s.to_string(),
    };
    Ok(Event {
        topics,
        data,
        block_number,
        stamp,
        transaction_hash,
        log_index
    })
}

pub fn parse_event_erc20_approval(e: Event) -> Result<EventERC20Approval> {
    let owner = parse_bytes32_to_address(&e.topics[1])?;
    let spender = parse_bytes32_to_address(&e.topics[2])?;
    let amount = parse_bytes32_to_decimal(&e.data).context("parse data as bigdecimal")?;
    let stamp = NaiveDateTime::from_timestamp(
        i64::from_str_radix(&e.stamp.trim_start_matches("0x"), 16)
            .context("parse stamp as timestamp")?,
        0,
    );
    let block_number = i64::from_str_radix(&e.block_number.trim_start_matches("0x"), 16)
        .context("parse block as i64")?;
    let tx_hash = e.transaction_hash;
    // zero logIndex comes as empty bytes, 0x, parse it as 0
    let log_index = match e.log_index.trim_start_matches("0x") {
        "" => 0,
        s => i64::from_str_radix(&s, 16).context("parse log index as i64")?
    };

    Ok(EventERC20Approval {
        owner,
        spender,
        amount,
        stamp,
        block_number,
        tx_hash,
        log_index
    })
}

pub async fn parse_events_erc20_approval(vs: Vec<Value>) -> Result<Vec<EventERC20Approval>> {

    println!("parsing events erc20 approval");
    let mut events: Vec<EventERC20Approval> = vec![];
    for v in vs.into_iter() {
        let e = parse_value_to_event(v)?;
        let event = parse_event_erc20_approval(e)?;
        events.push(event);
    }

    Ok(events)
}

pub fn parse_event_erc20_transfer(e: Event) -> Result<EventERC20Transfer> {
    let sender = parse_bytes32_to_address(&e.topics[1])?;
    let receiver = parse_bytes32_to_address(&e.topics[2])?;
    let amount = parse_bytes32_to_decimal(&e.data).context("parse data as bigdecimal")?;
    let stamp = NaiveDateTime::from_timestamp(
        i64::from_str_radix(&e.stamp.trim_start_matches("0x"), 16)
            .context("parse stamp as timestamp")?,
        0,
    );
    let block_number = i64::from_str_radix(&e.block_number.trim_start_matches("0x"), 16)
        .context("parse block as i64")?;
    let tx_hash = e.transaction_hash;
    // zero logIndex comes as empty bytes, 0x, parse it as 0
    let log_index = match e.log_index.trim_start_matches("0x") {
        "" => 0,
        s => i64::from_str_radix(&s, 16).context("parse log index as i64")?
    };

    Ok(EventERC20Transfer {
        sender,
        receiver,
        amount,
        stamp,
        block_number,
        tx_hash,
        log_index
    })
}

pub async fn parse_events_erc20_transfer(vs: Vec<Value>) -> Result<Vec<EventERC20Transfer>> {

    println!("parsing events erc20 transfer");
    let mut events: Vec<EventERC20Transfer> = vec![];
    for v in vs.into_iter() {
        let e = parse_value_to_event(v)?;
        let event = parse_event_erc20_transfer(e)?;
        events.push(event);
    }

    Ok(events)
}

pub fn parse_event_anyv4_swapin(e: Event) -> Result<EventAnyV4Swapin> {
    let transfer_tx_hash = String::from(&e.topics[1]);
    let account = parse_bytes32_to_address(&e.topics[2])?;
    let amount = parse_bytes32_to_decimal(&e.data).context("parse data as bigdecimal")?;
    let stamp = NaiveDateTime::from_timestamp(
        i64::from_str_radix(&e.stamp.trim_start_matches("0x"), 16)
            .context("parse stamp as timestamp")?,
        0,
    );
    let block_number = i64::from_str_radix(&e.block_number.trim_start_matches("0x"), 16)
        .context("parse block as i64")?;
    let tx_hash = e.transaction_hash;
    // zero logIndex comes as empty bytes, 0x, parse it as 0
    let log_index = match e.log_index.trim_start_matches("0x") {
        "" => 0,
        s => i64::from_str_radix(&s, 16).context("parse log index as i64")?
    };

    Ok(EventAnyV4Swapin {
        account,
        amount,
        transfer_tx_hash,
        stamp,
        block_number,
        tx_hash,
        log_index
    })
}

pub async fn parse_events_anyv4_swapin(vs: Vec<Value>) -> Result<Vec<EventAnyV4Swapin>> {

    println!("parsing events anyv4 swapin");
    let mut events: Vec<EventAnyV4Swapin> = vec![];
    for v in vs.into_iter() {
        let e = parse_value_to_event(v)?;
        let event = parse_event_anyv4_swapin(e)?;
        events.push(event);
    }

    Ok(events)
}

pub fn parse_event_anyv4_swapout(e: Event) -> Result<EventAnyV4Swapout> {
    let account = parse_bytes32_to_address(&e.topics[1])?;
    let bindaddr = parse_bytes32_to_address(&e.topics[2])?;
    let amount = parse_bytes32_to_decimal(&e.data).context("parse data as bigdecimal")?;
    let stamp = NaiveDateTime::from_timestamp(
        i64::from_str_radix(&e.stamp.trim_start_matches("0x"), 16)
            .context("parse stamp as timestamp")?,
        0,
    );
    let block_number = i64::from_str_radix(&e.block_number.trim_start_matches("0x"), 16)
        .context("parse block as i64")?;
    let tx_hash = e.transaction_hash;
    // zero logIndex comes as empty bytes, 0x, parse it as 0
    let log_index = match e.log_index.trim_start_matches("0x") {
        "" => 0,
        s => i64::from_str_radix(&s, 16).context("parse log index as i64")?
    };

    Ok(EventAnyV4Swapout {
        account,
        bindaddr,
        amount,
        stamp,
        block_number,
        tx_hash,
        log_index
    })
}

pub async fn parse_events_anyv4_swapout(vs: Vec<Value>) -> Result<Vec<EventAnyV4Swapout>> {

    println!("parsing events anyv4 swapout");
    let mut events: Vec<EventAnyV4Swapout> = vec![];
    for v in vs.into_iter() {
        let e = parse_value_to_event(v)?;
        let event = parse_event_anyv4_swapout(e)?;
        events.push(event);
    }

    Ok(events)
}

pub fn parse_event_univ2_pair_created(e: Event, gton: &str) -> Result<EventUniV2PairCreated> {
    let token0 = parse_bytes32_to_address(&e.topics[1])?;
    let token1 = parse_bytes32_to_address(&e.topics[2])?;
    let data = &e.data.trim_start_matches("0x");
    let data1 = data.chars().take(64).collect::<String>();
    let address = parse_bytes32_to_address(&format!("0x{}", data1))?;
    let data2 = data.chars().skip(64).collect::<String>();
    let index = parse_bytes32_to_decimal(&format!("0x{}", data2))?;
    let gtonToken0 = token0 == gton;
    let title = token0.clone() + &token1;
    let stamp = NaiveDateTime::from_timestamp(
        i64::from_str_radix(&e.stamp.trim_start_matches("0x"), 16)
            .context("parse stamp as timestamp")?,
        0,
    );
    let block_number = i64::from_str_radix(&e.block_number.trim_start_matches("0x"), 16)
        .context("parse block as i64")?;
    let tx_hash = e.transaction_hash;
    // zero logIndex comes as empty bytes, 0x, parse it as 0
    let log_index = match e.log_index.trim_start_matches("0x") {
        "" => 0,
        s => i64::from_str_radix(&s, 16).context("parse log index as i64")?
    };

    Ok(EventUniV2PairCreated {
        address,
        token0,
        token1,
        gtonToken0,
        title,
        index,
        stamp,
        block_number,
        tx_hash,
        log_index
    })
}

pub async fn parse_events_univ2_pair_created(vs: Vec<Value>, gton: &str) -> Result<Vec<EventUniV2PairCreated>> {

    println!("parsing events univ2 pair created");
    let mut events: Vec<EventUniV2PairCreated> = vec![];
    for v in vs.into_iter() {
        let e = parse_value_to_event(v)?;
        let event = parse_event_univ2_pair_created(e, gton)?;
        events.push(event);
    }

    Ok(events)
}

pub fn parse_event_univ2_swap(e: Event) -> Result<EventUniV2Swap> {
    let sender = parse_bytes32_to_address(&e.topics[1])?;
    let receiver = parse_bytes32_to_address(&e.topics[2])?;
    let data = &e.data.trim_start_matches("0x");
    let data1 = data.chars().take(64).collect::<String>();
    let amount0_in = parse_bytes32_to_decimal(&format!("0x{}", data1))?;
    let data2 = data.chars().skip(64).take(64).collect::<String>();
    let amount1_in = parse_bytes32_to_decimal(&format!("0x{}", data2))?;
    let data3 = data.chars().skip(124).take(64).collect::<String>();
    let amount0_out = parse_bytes32_to_decimal(&format!("0x{}", data3))?;
    let data4 = data.chars().skip(192).collect::<String>();
    let amount1_out = parse_bytes32_to_decimal(&format!("0x{}", data4))?;
    let stamp = NaiveDateTime::from_timestamp(
        i64::from_str_radix(&e.stamp.trim_start_matches("0x"), 16)
            .context("parse stamp as timestamp")?,
        0,
    );
    let block_number = i64::from_str_radix(&e.block_number.trim_start_matches("0x"), 16)
        .context("parse block as i64")?;
    let tx_hash = e.transaction_hash;
    // zero logIndex comes as empty bytes, 0x, parse it as 0
    let log_index = match e.log_index.trim_start_matches("0x") {
        "" => 0,
        s => i64::from_str_radix(&s, 16).context("parse log index as i64")?
    };

    Ok(EventUniV2Swap {
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
    })
}

pub async fn parse_events_univ2_swap(vs: Vec<Value>) -> Result<Vec<EventUniV2Swap>> {

    println!("parsing events univ2 swap");
    let mut events: Vec<EventUniV2Swap> = vec![];
    for v in vs.into_iter() {
        let e = parse_value_to_event(v)?;
        let event = parse_event_univ2_swap(e)?;
        events.push(event);
    }

    Ok(events)
}

pub fn parse_event_univ2_mint(e: Event) -> Result<EventUniV2Mint> {
    let sender = parse_bytes32_to_address(&e.topics[1])?;
    let data = &e.data.trim_start_matches("0x");
    let data1 = data.chars().take(64).collect::<String>();
    let amount0 = parse_bytes32_to_decimal(&format!("0x{}", data1))?;
    let data2 = data.chars().skip(64).take(64).collect::<String>();
    let amount1 = parse_bytes32_to_decimal(&format!("0x{}", data2))?;
    let stamp = NaiveDateTime::from_timestamp(
        i64::from_str_radix(&e.stamp.trim_start_matches("0x"), 16)
            .context("parse stamp as timestamp")?,
        0,
    );
    let block_number = i64::from_str_radix(&e.block_number.trim_start_matches("0x"), 16)
        .context("parse block as i64")?;
    let tx_hash = e.transaction_hash;
    // zero logIndex comes as empty bytes, 0x, parse it as 0
    let log_index = match e.log_index.trim_start_matches("0x") {
        "" => 0,
        s => i64::from_str_radix(&s, 16).context("parse log index as i64")?
    };

    Ok(EventUniV2Mint {
        sender,
        amount0,
        amount1,
        stamp,
        block_number,
        tx_hash,
        log_index
    })
}

pub async fn parse_events_univ2_mint(vs: Vec<Value>) -> Result<Vec<EventUniV2Mint>> {

    println!("parsing events univ2 mint");
    let mut events: Vec<EventUniV2Mint> = vec![];
    for v in vs.into_iter() {
        let e = parse_value_to_event(v)?;
        let event = parse_event_univ2_mint(e)?;
        events.push(event);
    }

    Ok(events)
}

pub fn parse_event_univ2_burn(e: Event) -> Result<EventUniV2Burn> {
    let sender = parse_bytes32_to_address(&e.topics[1])?;
    let receiver = parse_bytes32_to_address(&e.topics[2])?;
    let data = &e.data.trim_start_matches("0x");
    let data1 = data.chars().take(64).collect::<String>();
    let amount0 = parse_bytes32_to_decimal(&format!("0x{}", data1))?;
    let data2 = data.chars().skip(64).take(64).collect::<String>();
    let amount1 = parse_bytes32_to_decimal(&format!("0x{}", data2))?;
    let stamp = NaiveDateTime::from_timestamp(
        i64::from_str_radix(&e.stamp.trim_start_matches("0x"), 16)
            .context("parse stamp as timestamp")?,
        0,
    );
    let block_number = i64::from_str_radix(&e.block_number.trim_start_matches("0x"), 16)
        .context("parse block as i64")?;
    let tx_hash = e.transaction_hash;
    // zero logIndex comes as empty bytes, 0x, parse it as 0
    let log_index = match e.log_index.trim_start_matches("0x") {
        "" => 0,
        s => i64::from_str_radix(&s, 16).context("parse log index as i64")?
    };

    Ok(EventUniV2Burn {
        sender,
        receiver,
        amount0,
        amount1,
        stamp,
        block_number,
        tx_hash,
        log_index
    })
}

pub async fn parse_events_univ2_burn(vs: Vec<Value>) -> Result<Vec<EventUniV2Burn>> {

    println!("parsing events univ2 burn");
    let mut events: Vec<EventUniV2Burn> = vec![];
    for v in vs.into_iter() {
        let e = parse_value_to_event(v)?;
        let event = parse_event_univ2_burn(e)?;
        events.push(event);
    }

    Ok(events)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn bytes32_to_decimal_0_works() {
        assert_eq!(parse_bytes32_to_decimal("0x00").unwrap(), BigDecimal::from(0));
    }

    #[tokio::test]
    async fn bytes32_to_decimal_255_works() {
        assert_eq!(parse_bytes32_to_decimal("0xff").unwrap(), BigDecimal::from(255));
    }
}
