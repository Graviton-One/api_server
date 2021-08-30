use diesel::prelude::*;
use actix_web_dev::error::{
    Result,
};
use serde::{
    Serialize,
    Deserialize,
};
use chrono::NaiveDateTime;

#[derive(Serialize,Deserialize,Queryable,Clone,Debug)]
pub struct Users {
    id: i32,
    address: String,
    twitter_account: Option<String>, 
}
use diesel::sql_types::{
    Integer,
    Varchar,
};

struct Config {
    table: String,
    pair: String, 
    param: String
}

impl Config {
    pub fn new_add() -> Vec<Config> {
        vec![
            Config{
                table: "univ2_lp_add_bsc_pancake",
                pair: "events_univ2_pair_created_bsc_pancake",
                param: "lp_amount_in"
            },
            Config{
                table: "univ2_lp_add_eth_psushi",
                pair: "events_univ2_pair_created_eth_sushi",
                param: "lp_amount_in"
            },
            Config{
                table: "univ2_lp_add_ftm_spirit",
                pair: "events_univ2_pair_created_ftm_spirit",
                param: "lp_amount_in"
            },
            Config{
                table: "univ2_lp_add_ftm_spooky",
                pair: "events_univ2_pair_created_ftm_spooky",
                param: "lp_amount_in"
            },
            Config{
                table: "univ2_lp_add_plg_quick",
                pair: "events_univ2_pair_created_plg_quick",
                param: "lp_amount_in"
            },
        ]
    }
    pub fn new_remove() -> Vec<Config> {
        vec![
            Config{
                table: "univ2_lp_remove_bsc_pancake",
                pair: "events_univ2_pair_created_bsc_pancake",
                param: "lp_amount_out"
            },
            Config{
                table: "univ2_lp_remove_eth_psushi",
                pair: "events_univ2_pair_created_eth_sushi",
                param: "lp_amount_out"
            },
            Config{
                table: "univ2_lp_remove_ftm_spirit",
                pair: "events_univ2_pair_created_ftm_spirit",
                param: "lp_amount_out"
            },
            Config{
                table: "univ2_lp_remove_ftm_spooky",
                pair: "events_univ2_pair_created_ftm_spooky",
                param: "lp_amount_out"
            },
            Config{
                table: "univ2_lp_remove_plg_quick",
                pair: "events_univ2_pair_created_plg_quick",
                param: "lp_amount_out"
            },
        ]
    }
}

pub fn get_inout_txns(conn: &PgConnection, table: String, pair_table: String, amount_param: String) -> Result<Vec<Transaction>> {
    diesel::sql_query("SELECT t.tx_hash, t.stamp, t.($1) as amount, p.address as pool_address FROM ($2) as t
    LEFT JOIN ($3) AS p ON t.pair_id = p.id;")
    .bind::<diesel::sql_types::Varchar,_>(amount_param)
    .bind::<diesel::sql_types::Varchar,_>(table)
    .bind::<diesel::sql_types::Varchar,_>(pair_table)
    .get_results::<Transaction>(conn)
    .map_err(|e|e.into())
}

#[derive(Serialize,Deserialize,QueryableByName,Clone,Debug)]
pub struct Transaction {
    #[sql_type="Integer"]
    id: i32,
    #[sql_type="BigInt"]
    amount: i64,
    #[sql_type="Varchar"]
    tx_hash: String,
    #[sql_type="Varchar"]
    address: String,
    #[sql_type="Varchar"]
    stamp: NaiveDateTime,
}

#[derive(Serialize,Deserialize,QueryableByName,Clone,Debug)]
pub struct SwapTransaction {
    #[sql_type="Integer"]
    user_id: i32,
    #[sql_type="Varchar"]
    address: String,
    #[sql_type="Integer"]
    id: i32,
    #[sql_type="Varchar"]
    name: String,
    #[sql_type="Varchar"]
    description: String,
    #[sql_type="Varchar"]
    icon: String,
    #[sql_type="Varchar"]
    external_address: String,
    #[sql_type="Varchar"]
    chain_type: String,
}

pub struct FrontendTnx {
    txn_type: String,
    reward_icon: String,
    reward: String,
    amount: f64,
    value: f64,
    tx_hash: String,
    time: NaiveDateTime
}

// shortcode example

// let array: Vec<FrontendTnx> = vec![];
// let res = diesel::sql_query("SELECT tx_hash, stamp, amount_lp_in FROM ;")
//     .get_results::<Transaction>(conn)
//     .map_err(|e|e.into());
// for item in res {
//     array.push(FrontendTnx{
//         txn_type: "Add",
//         reward,
//         reward_icon,
        
//     })
// }

impl Transaction {
    pub async fn get_all(conn: &PgConnection) -> Result<Vec<Self>> {
        let res = diesel::sql_query("SELECT * FROM user_achievements WHERE 
        external_address=lower($1) and chain_type=upper($2);")
        .get_results::<Transaction>(conn)
        .map_err(|e|e.into());
    }
    //Ok(NaiveDateTime::from_timestamp(stamp_i64, 0))
    pub async fn get_remove(
        offset: i32,
        limit: i32,
        conn: &PgConnection,
    ) -> Result<Vec<Self>> {
        diesel::sql_query("SELECT t1.tx_hash, t1.stamp, t1.amount_lp_in as amount, p1.address FROM univ2_lp_remove_bsc_pancake AS t1 
        LEFT JOIN events_univ2_pair_created_bsc_pancake AS p1 on t1.pair_id = p1.id
         UNION SELECT t2.tx_hash, t2.stamp, t2.amount_lp_in as amount, p2.address FROM univ2_lp_remove_eth_sushi AS t2
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p2 on t2.pair_id = p2.id
        UNION SELECT t3.tx_hash, t3.stamp, t3.amount_lp_in as amount, p3.address FROM univ2_lp_remove_ftm_spirit AS t3
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p3 on t3.pair_id = p3.id
        UNION SELECT t4.tx_hash, t4.stamp, t4.amount_lp_in as amount, p4.address FROM univ2_lp_remove_ftm_spooky AS t4
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p4 on t4.pair_id = p4.id
        UNION SELECT t5.tx_hash, t5.stamp, t5.amount_lp_in as amount, p5.address FROM univ2_lp_remove_plg_quick AS t5
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p5 on t5.pair_id = p5.id
        ORDER BY stamp
        LIMIT ($1) OFFSET ($2);")
        .bind::<diesel::sql_types::Varchar,_>(limit)
        .bind::<diesel::sql_types::Varchar,_>(offset)
        .get_results::<Transaction>(conn)
        .map_err(|e|e.into())
    }

    pub async fn get_add(
        conn: &PgConnection,
    ) -> Result<Vec<Self>> {
        diesel::sql_query("SELECT t1.tx_hash, t1.stamp, t1.amount_lp_out as amount, p1.address FROM univ2_lp_add_bsc_pancake AS t1 
        LEFT JOIN events_univ2_pair_created_bsc_pancake AS p1 on t1.pair_id = p1.id
         UNION SELECT t2.tx_hash, t2.stamp, t2.amount_lp_out as amount, p2.address FROM univ2_lp_add_eth_sushi AS t2
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p2 on t2.pair_id = p2.id
        UNION SELECT t3.tx_hash, t3.stamp, t3.amount_lp_out as amount, p3.address FROM univ2_lp_add_ftm_spirit AS t3
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p3 on t3.pair_id = p3.id
        UNION SELECT t4.tx_hash, t4.stamp, t4.amount_lp_out as amount, p4.address FROM univ2_lp_add_ftm_spooky AS t4
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p4 on t4.pair_id = p4.id
        UNION SELECT t5.tx_hash, t5.stamp, t5.amount_lp_out as amount, p5.address FROM univ2_lp_add_plg_quick AS t5
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p5 on t5.pair_id = p5.id
        ORDER BY stamp
        LIMIT ($1) OFFSET ($2);")
        .bind::<diesel::sql_types::Varchar,_>(limit)
        .bind::<diesel::sql_types::Varchar,_>(offset)
        .get_results::<Transaction>(conn)
        .map_err(|e|e.into())
    }

//   CASE 
// WHEN @SelectField1 = 1 THEN Field1
// WHEN @SelectField2 = 1 THEN Field2
// ELSE NULL
// END AS NewField

    pub async fn get_swap(
        conn: &PgConnection,
    ) -> Result<Vec<Self>> {
        let res = diesel::sql_query("SELECT t1.tx_hash, t1.stamp, t1.amount_lp_out as amount, p1.address FROM univ2_lp_add_bsc_pancake AS t1 
        LEFT JOIN events_univ2_pair_created_bsc_pancake AS p1 on t1.pair_id = p1.id")
            .get_results::<SwapTransaction>(conn)
            .map_err(|e|e.into());
    }
}

