use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct BalanceKeeperOpenUser {
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

#[derive(Debug)]
pub struct BalanceKeeperAdd {
    pub tx_from: String,
    pub tx_to: String,
    pub adder: String,
    pub user_id: BigDecimal,
    pub amount: BigDecimal,
    pub stamp: NaiveDateTime,
    pub block_number: i64,
    pub tx_hash: String,
    pub log_index: i64,
}

#[derive(Debug)]
pub struct BalanceKeeperSubtract {
    pub tx_from: String,
    pub tx_to: String,
    pub subtractor: String,
    pub user_id: BigDecimal,
    pub amount: BigDecimal,
    pub stamp: NaiveDateTime,
    pub block_number: i64,
    pub tx_hash: String,
    pub log_index: i64,
}

#[derive(Debug)]
pub struct VoterStartRound {
    pub tx_from: String,
    pub tx_to: String,
    pub owner: String,
    pub total_rounds: BigDecimal,
    pub round_name: String,
    pub option_names: Vec<String>,
    pub stamp: NaiveDateTime,
    pub block_number: i64,
    pub tx_hash: String,
    pub log_index: i64,
}

#[derive(Debug)]
pub struct VoterFinalizeRound {
    pub tx_from: String,
    pub tx_to: String,
    pub owner: String,
    pub round_id: BigDecimal,
    pub stamp: NaiveDateTime,
    pub block_number: i64,
    pub tx_hash: String,
    pub log_index: i64,
}

#[derive(Debug)]
pub struct VoterCastVotes {
    pub tx_from: String,
    pub tx_to: String,
    pub caster: String,
    pub round_id: BigDecimal,
    pub user_id: BigDecimal,
    pub votes: Vec<BigDecimal>,
    pub stamp: NaiveDateTime,
    pub block_number: i64,
    pub tx_hash: String,
    pub log_index: i64,
}

#[derive(Debug)]
pub struct VoterCheckVoteBalance {
    pub tx_from: String,
    pub tx_to: String,
    pub checker: String,
    pub user_id: BigDecimal,
    pub new_balance: BigDecimal,
    pub stamp: NaiveDateTime,
    pub block_number: i64,
    pub tx_hash: String,
    pub log_index: i64,
}

#[derive(Debug)]
pub struct LPKeeperAdd {
    pub tx_from: String,
    pub tx_to: String,
    pub adder: String,
    pub token_id: BigDecimal,
    pub user_id: BigDecimal,
    pub amount: BigDecimal,
    pub stamp: NaiveDateTime,
    pub block_number: i64,
    pub tx_hash: String,
    pub log_index: i64,
}

#[derive(Debug)]
pub struct LPKeeperSubtract {
    pub tx_from: String,
    pub tx_to: String,
    pub subtractor: String,
    pub token_id: BigDecimal,
    pub user_id: BigDecimal,
    pub amount: BigDecimal,
    pub stamp: NaiveDateTime,
    pub block_number: i64,
    pub tx_hash: String,
    pub log_index: i64,
}
