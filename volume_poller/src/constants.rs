use crate::schema::*;
use diesel;
pub struct GtonPool {
    address: Address,
    poller_id: i32,
    id: i32,
    volume: Option<i32>,
    table: diesel::table,
    url: String,
    gton_first: bool
}

pub fn getPools() -> Vec<GtonPool> {
    [Pool{address: std::env::var("SUSHI_POOL_WETH")
                            .expect("balance keeper get")
                            .parse()
                            .expect("balance keeper parse"),
              id: 1,
              poller_id: 2,
              table: uni_stats,
              gton_first: true,
              url: "https://mainnet.infura.io/v3/ec6afadb1810471dbb600f24b86391d2"},
        Pool{address: std::env::var("SPOOKY_POOL_FTM")
                            .expect("balance keeper get")
                            .parse()
                            .expect("balance keeper parse"),
              id: 2,
              poller_id: 3,
              table: spooky_ftm_stats,
              gton_first: false,
              url: "https://rpcapi.fantom.network",},
              Pool{address: std::env::var("SPOOKY_POOL_USDC")
                            .expect("balance keeper get")
                            .parse()
                            .expect("balance keeper parse"),
              id: 3,
              poller_id: 4,
              table: spooky_usdc_stats,
              gton_first: false,
              url: "https://rpcapi.fantom.network"},
              Pool{address: std::env::var("SPIRIT_POOL_FTM")
                            .expect("balance keeper get")
                            .parse()
                            .expect("balance keeper parse"),
              id: 4,
              poller_id: 5,
              table: spirit_ftm_stats,
              gton_first: false,
              url: "https://rpcapi.fantom.network"},
              Pool{address: std::env::var("SPIRIT_POOL_USDC")
                            .expect("balance keeper get")
                            .parse()
                            .expect("balance keeper parse"),
              id: 5,
              poller_id: 6,
              table: spirit_usdc_stats,
              gton_first: false,
              url: "https://rpcapi.fantom.network"},
              Pool{address: std::env::var("SPIRIT_POOL_FUSDT")
                            .expect("balance keeper get")
                            .parse()
                            .expect("balance keeper parse"),
              id: 6,
              poller_id: 7,
              table: spirit_fusdt_stats,
              gton_first: false,
              url: "https://rpcapi.fantom.network"},
              Pool{address: std::env::var("PANCAKE_POOL_BUSD")
                            .expect("balance keeper get")
                            .parse()
                            .expect("balance keeper parse"),
              id: 7,
              poller_id: 8,
              table: pakcake_busd_stats,
              gton_first: true,
              url: "https://bsc-dataseed.binance.org"},
              Pool{address: std::env::var("PANCAKE_POOL_BNB")
                            .expect("balance keeper get")
                            .parse()
                            .expect("balance keeper parse"),
              id: 8,
              poller_id: 9,
              table: spirit_fusdt_stats,
              gton_first: true,
              url: "https://bsc-dataseed.binance.org"},
              ]
}