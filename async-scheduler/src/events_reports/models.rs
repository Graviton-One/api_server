use bigdecimal::{BigDecimal, ToPrimitive};

use chrono::NaiveDateTime;

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
    pub pair_id: i64,
    pub pair_title: String,
    pub tx_origin: String,
    pub amount_gton_in: BigDecimal,
    pub amount_token_out: BigDecimal,
    pub amountLP: BigDecimal,
    pub stamp: NaiveDateTime,
    pub tx_hash: String,
    pub log_index: i64
}

#[derive(Debug)]
pub struct UniV2LPRemove {
    pub pair_id: i64,
    pub pair_title: String,
    pub tx_origin: String,
    pub amount_gton_in: BigDecimal,
    pub amount_token_out: BigDecimal,
    pub stamp: NaiveDateTime,
    pub tx_hash: String,
    pub log_index: i64
}
