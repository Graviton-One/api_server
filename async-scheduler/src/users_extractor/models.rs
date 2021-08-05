use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct OpenUser {
    pub tx_from: String,
    pub tx_to: String,
    pub opener: String,
    pub user_id: BigDecimal,
    pub user_address: String,
    pub user_chain: String,
    pub stamp: NaiveDateTime,
    pub block_number: i64,
    pub tx_hash: String,
    pub log_index: i64,
}
