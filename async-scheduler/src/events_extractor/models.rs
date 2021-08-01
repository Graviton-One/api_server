use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Blockscan {
    pub base_url: String,
    pub block_from: String,
    pub block_to: String,
    pub address: String,
    pub topic0: String,
    pub is_topic1: bool,
    pub topic1: String,
    pub is_topic2: bool,
    pub topic2: String,
    pub is_topic3: bool,
    pub topic3: String,
    pub apikey: String,
}

pub struct Event {
    // pub address: String,
    pub topics: Vec<String>,
    pub data: String,
    pub block_number: String,
    pub stamp: String,
    // pub gas_price: String,
    // pub gas_used: String,
    pub log_index: String,
    pub transaction_hash: String,
    // pub transaction_index: String,
}

#[derive(Debug)]
pub struct EventERC20Transfer {
    pub sender: String,
    pub receiver: String,
    pub amount: BigDecimal,
    pub stamp: NaiveDateTime,
    pub block_number: i64,
    pub tx_hash: String,
    pub log_index: i64,
}

#[derive(Debug)]
pub struct EventERC20Approval {
    pub owner: String,
    pub spender: String,
    pub amount: BigDecimal,
    pub stamp: NaiveDateTime,
    pub block_number: i64,
    pub tx_hash: String,
    pub log_index: i64,
}

#[derive(Debug)]
pub struct EventAnyV4Swapin {
    pub account: String,
    pub amount: BigDecimal,
    pub transfer_tx_hash: String,
    pub stamp: NaiveDateTime,
    pub block_number: i64,
    pub tx_hash: String,
    pub log_index: i64,
}

#[derive(Debug)]
pub struct EventAnyV4Swapout {
    pub account: String,
    pub bindaddr: String,
    pub amount: BigDecimal,
    pub stamp: NaiveDateTime,
    pub block_number: i64,
    pub tx_hash: String,
    pub log_index: i64,
}

#[derive(Debug)]
pub struct EventUniV2PairCreated {
    pub address: String,
    pub token0: String,
    pub token1: String,
    pub index: BigDecimal,
    pub stamp: NaiveDateTime,
    pub block_number: i64,
    pub tx_hash: String,
    pub log_index: i64,
}

#[derive(Debug)]
pub struct EventUniV2Swap {
    pub sender: String,
    pub receiver: String,
    pub amount0_in: BigDecimal,
    pub amount1_in: BigDecimal,
    pub amount0_out: BigDecimal,
    pub amount1_out: BigDecimal,
    pub stamp: NaiveDateTime,
    pub block_number: i64,
    pub tx_hash: String,
    pub log_index: i64,
}

#[derive(Debug)]
pub struct EventUniV2Mint {
    pub sender: String,
    pub amount0: BigDecimal,
    pub amount1: BigDecimal,
    pub stamp: NaiveDateTime,
    pub block_number: i64,
    pub tx_hash: String,
    pub log_index: i64,
}

#[derive(Debug)]
pub struct EventUniV2Burn {
    pub sender: String,
    pub receiver: String,
    pub amount0: BigDecimal,
    pub amount1: BigDecimal,
    pub stamp: NaiveDateTime,
    pub block_number: i64,
    pub tx_hash: String,
    pub log_index: i64,
}
