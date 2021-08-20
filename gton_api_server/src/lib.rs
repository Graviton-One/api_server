#[macro_use]
extern crate diesel;
#[macro_use]
extern crate r2d2;
#[macro_use]
extern crate serde_json;

use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub mod schema;
pub mod fee_giver;
pub mod gton_stats;
pub mod users;
pub mod voting;
//pub mod pool_stats;
pub mod chain;

use ethcontract::prelude::*;
#[derive(Clone)]
pub struct ChainConfig {
    pub balance_keeper: Address,
    pub governance_vote: Address,
    pub fee_giver_key: PrivateKey,
}

impl ChainConfig {
    pub fn from_env() -> Self {
        Self {
            balance_keeper: std::env::var("BALANCE_KEEPER_ADDRESS")
                .expect("balance keeper get")
                .parse()
                .expect("balance keeper parse"),
            governance_vote: std::env::var("GOVERNANCE_VOTE_ADDRESS")
                .expect("governance vote keeper")
                .parse()
                .expect("governance vote parse"),
            fee_giver_key: std::env::var("FEE_GIVER_PRIVATE_KEY")
                .expect("fee giver get")
                .parse()
                .expect("fee giver parse"),
        }
    }
}
