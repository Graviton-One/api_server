use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

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
    pub tx_origin: String,
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
    pub tx_origin: String,
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
    pub tx_origin: String,
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
    pub tx_origin: String,
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
    pub tx_origin: String,
    pub address: String,
    pub token0: String,
    pub token1: String,
    pub gtonToken0: bool,
    pub title: String,
    pub index: BigDecimal,
    pub stamp: NaiveDateTime,
    pub block_number: i64,
    pub tx_hash: String,
    pub log_index: i64,
}

#[derive(Debug)]
pub struct EventUniV2Swap {
    pub tx_origin: String,
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
    pub tx_origin: String,
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
    pub tx_origin: String,
    pub sender: String,
    pub receiver: String,
    pub amount0: BigDecimal,
    pub amount1: BigDecimal,
    pub stamp: NaiveDateTime,
    pub block_number: i64,
    pub tx_hash: String,
    pub log_index: i64,
}

#[derive(Debug)]
pub struct UniV2Sell {
    pub swap_id: i64,
    pub pair_id: i64,
    pub pair_title: String,
    pub tx_origin: String,
    pub amount_gton_in: BigDecimal,
    pub amount_token_out: BigDecimal,
    pub stamp: NaiveDateTime,
    pub tx_hash: String,
    pub log_index: i64
}

#[derive(Debug)]
pub struct UniV2Buy {
    pub swap_id: i64,
    pub pair_id: i64,
    pub pair_title: String,
    pub tx_origin: String,
    pub amount_token_in: BigDecimal,
    pub amount_gton_out: BigDecimal,
    pub stamp: NaiveDateTime,
    pub tx_hash: String,
    pub log_index: i64
}

#[derive(Debug)]
pub struct UniV2LPAdd {
    pub mint_id: i64,
    pub pair_id: i64,
    pub pair_title: String,
    pub tx_origin: String,
    pub amount_gton_in: BigDecimal,
    pub amount_token_out: BigDecimal,
    pub amount_lp_out: BigDecimal,
    pub stamp: NaiveDateTime,
    pub tx_hash: String,
    pub log_index: i64
}

#[derive(Debug)]
pub struct UniV2LPRemove {
    pub burn_id: i64,
    pub pair_id: i64,
    pub pair_title: String,
    pub tx_origin: String,
    pub amount_gton_in: BigDecimal,
    pub amount_token_out: BigDecimal,
    pub stamp: NaiveDateTime,
    pub tx_hash: String,
    pub log_index: i64
}
