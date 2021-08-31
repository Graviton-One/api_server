use diesel::prelude::*;
use actix_web_dev::error::{
    Result,
};
use bigdecimal::BigDecimal;
use serde::{
    Serialize,
    Deserialize,
};
use chrono::NaiveDateTime;

use diesel::sql_types::{
    Varchar,
    Numeric,
    Timestamp,
    Text,
};

// struct Config {
//     table: String,
//     pair: String, 
//     param: String
// }

// impl Config {
//     pub fn new_add() -> Vec<Config> {
//         vec![
//             Config{
//                 table: "univ2_lp_add_bsc_pancake",
//                 pair: "events_univ2_pair_created_bsc_pancake",
//                 param: "lp_amount_in"
//             },
//             Config{
//                 table: "univ2_lp_add_eth_psushi",
//                 pair: "events_univ2_pair_created_eth_sushi",
//                 param: "lp_amount_in"
//             },
//             Config{
//                 table: "univ2_lp_add_ftm_spirit",
//                 pair: "events_univ2_pair_created_ftm_spirit",
//                 param: "lp_amount_in"
//             },
//             Config{
//                 table: "univ2_lp_add_ftm_spooky",
//                 pair: "events_univ2_pair_created_ftm_spooky",
//                 param: "lp_amount_in"
//             },
//             Config{
//                 table: "univ2_lp_add_plg_quick",
//                 pair: "events_univ2_pair_created_plg_quick",
//                 param: "lp_amount_in"
//             },
//         ]
//     }
//     pub fn new_remove() -> Vec<Config> {
//         vec![
//             Config{
//                 table: "univ2_lp_remove_bsc_pancake",
//                 pair: "events_univ2_pair_created_bsc_pancake",
//                 param: "lp_amount_out"
//             },
//             Config{
//                 table: "univ2_lp_remove_eth_psushi",
//                 pair: "events_univ2_pair_created_eth_sushi",
//                 param: "lp_amount_out"
//             },
//             Config{
//                 table: "univ2_lp_remove_ftm_spirit",
//                 pair: "events_univ2_pair_created_ftm_spirit",
//                 param: "lp_amount_out"
//             },
//             Config{
//                 table: "univ2_lp_remove_ftm_spooky",
//                 pair: "events_univ2_pair_created_ftm_spooky",
//                 param: "lp_amount_out"
//             },
//             Config{
//                 table: "univ2_lp_remove_plg_quick",
//                 pair: "events_univ2_pair_created_plg_quick",
//                 param: "lp_amount_out"
//             },
//         ]
//     }
// }

// pub fn get_inout_txns(conn: &PgConnection, table: String, pair_table: String, amount_param: String) -> Result<Vec<Transaction>> {
//     diesel::sql_query("SELECT t.tx_hash, t.stamp, t.($1) as amount, p.address as pool_address FROM ($2) as t
//     LEFT JOIN ($3) AS p ON t.pair_id = p.id;")
//     .bind::<diesel::sql_types::Varchar,_>(amount_param)
//     .bind::<diesel::sql_types::Varchar,_>(table)
//     .bind::<diesel::sql_types::Varchar,_>(pair_table)
//     .get_results::<Transaction>(conn)
//     .map_err(|e|e.into())
// }

#[derive(Serialize,Deserialize,QueryableByName, Queryable, Clone,Debug)]
pub struct Transaction {
    #[sql_type="Numeric"]
    amount: BigDecimal,
    #[sql_type="Text"]
    tx_hash: String,
    #[sql_type="Varchar"]
    address: String,
    #[sql_type="Timestamp"]
    stamp: NaiveDateTime,
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
    pub async fn get_all(limit: i64, offset: i64, conn: &PgConnection) -> Result<Vec<Self>> {
        diesel::sql_query("SELECT tr1.tx_hash, tr1.stamp, tr1.amount_lp_in as amount, pr1.address FROM univ2_lp_remove_bsc_pancake AS tr1 
        LEFT JOIN events_univ2_pair_created_bsc_pancake AS pr1 on tr1.pair_id = pr1.id
         UNION SELECT tr2.tx_hash, tr2.stamp, tr2.amount_lp_in as amount, pr2.address FROM univ2_lp_remove_eth_sushi AS tr2
        LEFT JOIN events_univ2_pair_created_eth_sushi AS pr2 on tr2.pair_id = pr2.id
        UNION SELECT tr3.tx_hash, tr3.stamp, tr3.amount_lp_in as amount, pr3.address FROM univ2_lp_remove_ftm_spirit AS tr3
        LEFT JOIN events_univ2_pair_created_eth_sushi AS pr3 on tr3.pair_id = pr3.id
        UNION SELECT tr4.tx_hash, tr4.stamp, tr4.amount_lp_in as amount, pr4.address FROM univ2_lp_remove_ftm_spooky AS tr4
        LEFT JOIN events_univ2_pair_created_eth_sushi AS pr4 on tr4.pair_id = pr4.id
        UNION SELECT tr5.tx_hash, tr5.stamp, tr5.amount_lp_in as amount, pr5.address FROM univ2_lp_remove_plg_quick AS tr5
        LEFT JOIN events_univ2_pair_created_eth_sushi AS pr5 on tr5.pair_id = pr5.id
        UNION SELECT ta1.tx_hash, ta1.stamp, ta1.amount_lp_out as amount, pa1.address FROM univ2_lp_add_bsc_pancake AS ta1 
        LEFT JOIN events_univ2_pair_created_bsc_pancake AS pa1 on ta1.pair_id = pa1.id
         UNION SELECT ta2.tx_hash, ta2.stamp, ta2.amount_lp_out as amount, pa2.address FROM univ2_lp_add_eth_sushi AS ta2
        LEFT JOIN events_univ2_pair_created_eth_sushi AS pa2 on ta2.pair_id = pa2.id
        UNION SELECT ta3.tx_hash, ta3.stamp, ta3.amount_lp_out as amount, pa3.address FROM univ2_lp_add_ftm_spirit AS ta3
        LEFT JOIN events_univ2_pair_created_eth_sushi AS pa3 on ta3.pair_id = pa3.id
        UNION SELECT ta4.tx_hash, ta4.stamp, ta4.amount_lp_out as amount, pa4.address FROM univ2_lp_add_ftm_spooky AS ta4
        LEFT JOIN events_univ2_pair_created_eth_sushi AS pa4 on ta4.pair_id = pa4.id
        UNION SELECT ta5.tx_hash, ta5.stamp, ta5.amount_lp_out as amount, pa5.address FROM univ2_lp_add_plg_quick AS ta5
        LEFT JOIN events_univ2_pair_created_eth_sushi AS pa5 on ta5.pair_id = pa5.id
        UNION SELECT t1.tx_hash, t1.stamp, t1.amount_gton_out as amount, p1.address FROM univ2_buy_bsc_pancake AS t1 
        LEFT JOIN events_univ2_pair_created_bsc_pancake AS p1 on t1.pair_id = p1.id
         UNION SELECT t2.tx_hash, t2.stamp, t2.amount_gton_out as amount, p2.address FROM univ2_buy_eth_sushi AS t2
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p2 on t2.pair_id = p2.id
        UNION SELECT t3.tx_hash, t3.stamp, t3.amount_gton_out as amount, p3.address FROM univ2_buy_ftm_spirit AS t3
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p3 on t3.pair_id = p3.id
        UNION SELECT t4.tx_hash, t4.stamp, t4.amount_gton_out as amount, p4.address FROM univ2_buy_ftm_spooky AS t4
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p4 on t4.pair_id = p4.id
        UNION SELECT t5.tx_hash, t5.stamp, t5.amount_gton_out as amount, p5.address FROM univ2_buy_plg_quick AS t5
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p5 on t5.pair_id = p5.id
        UNION SELECT t6.tx_hash, t6.stamp, t6.amount_gton_in as amount, p6.address FROM univ2_sell_bsc_pancake AS t6 
        LEFT JOIN events_univ2_pair_created_bsc_pancake AS p6 on t6.pair_id = p6.id
        UNION SELECT t7.tx_hash, t7.stamp, t7.amount_gton_in as amount, p7.address FROM univ2_sell_eth_sushi AS t7
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p7 on t7.pair_id = p7.id
        UNION SELECT t8.tx_hash, t8.stamp, t8.amount_gton_in as amount, p8.address FROM univ2_sell_ftm_spirit AS t8
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p8 on t8.pair_id = p8.id
        UNION SELECT t9.tx_hash, t9.stamp, t9.amount_gton_in as amount, p9.address FROM univ2_sell_ftm_spooky AS t9
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p9 on t9.pair_id = p9.id
        UNION SELECT t10.tx_hash, t10.stamp, t10.amount_gton_in as amount, p10.address FROM univ2_sell_plg_quick AS t10
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p10 on t10.pair_id = p10.id
        ORDER BY stamp DESC
        LIMIT ($1) OFFSET ($2);")
        .bind::<diesel::sql_types::BigInt,_>(limit)
        .bind::<diesel::sql_types::BigInt,_>(offset)
        .get_results::<Transaction>(conn)
        .map_err(|e|e.into())
    }
    //Ok(NaiveDateTime::from_timestamp(stamp_i64, 0))
    pub async fn get_remove(
        limit: i64,
        offset: i64,
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
        ORDER BY stamp DESC
        LIMIT ($1) OFFSET ($2);")
        .bind::<diesel::sql_types::BigInt,_>(limit)
        .bind::<diesel::sql_types::BigInt,_>(offset)
        .get_results::<Transaction>(conn)
        .map_err(|e|e.into())
    }

    pub async fn get_add(
        limit: i64,
        offset: i64,
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
        ORDER BY stamp DESC
        LIMIT ($1) OFFSET ($2);")
        .bind::<diesel::sql_types::BigInt,_>(limit)
        .bind::<diesel::sql_types::BigInt,_>(offset)
        .get_results::<Transaction>(conn)
        .map_err(|e|e.into())
    }

//   CASE 
// WHEN @SelectField1 = 1 THEN Field1
// WHEN @SelectField2 = 1 THEN Field2
// ELSE NULL
// END AS NewField

    pub async fn get_swap(
        limit: i64,
        offset: i64,
        conn: &PgConnection,
    ) -> Result<Vec<Self>> {
        diesel::sql_query("SELECT t1.tx_hash, t1.stamp, t1.amount_gton_out as amount, p1.address FROM univ2_buy_bsc_pancake AS t1 
        LEFT JOIN events_univ2_pair_created_bsc_pancake AS p1 on t1.pair_id = p1.id
         UNION SELECT t2.tx_hash, t2.stamp, t2.amount_gton_out as amount, p2.address FROM univ2_buy_eth_sushi AS t2
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p2 on t2.pair_id = p2.id
        UNION SELECT t3.tx_hash, t3.stamp, t3.amount_gton_out as amount, p3.address FROM univ2_buy_ftm_spirit AS t3
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p3 on t3.pair_id = p3.id
        UNION SELECT t4.tx_hash, t4.stamp, t4.amount_gton_out as amount, p4.address FROM univ2_buy_ftm_spooky AS t4
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p4 on t4.pair_id = p4.id
        UNION SELECT t5.tx_hash, t5.stamp, t5.amount_gton_out as amount, p5.address FROM univ2_buy_plg_quick AS t5
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p5 on t5.pair_id = p5.id
        UNION SELECT t6.tx_hash, t6.stamp, t6.amount_gton_in as amount, p6.address FROM univ2_sell_bsc_pancake AS t6 
        LEFT JOIN events_univ2_pair_created_bsc_pancake AS p6 on t6.pair_id = p6.id
        UNION SELECT t7.tx_hash, t7.stamp, t7.amount_gton_in as amount, p7.address FROM univ2_sell_eth_sushi AS t7
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p7 on t7.pair_id = p7.id
        UNION SELECT t8.tx_hash, t8.stamp, t8.amount_gton_in as amount, p8.address FROM univ2_sell_ftm_spirit AS t8
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p8 on t8.pair_id = p8.id
        UNION SELECT t9.tx_hash, t9.stamp, t9.amount_gton_in as amount, p9.address FROM univ2_sell_ftm_spooky AS t9
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p9 on t9.pair_id = p9.id
        UNION SELECT t10.tx_hash, t10.stamp, t10.amount_gton_in as amount, p10.address FROM univ2_sell_plg_quick AS t10
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p10 on t10.pair_id = p10.id
        ORDER BY stamp DESC
        LIMIT ($1) OFFSET ($2);")
        .bind::<diesel::sql_types::BigInt,_>(limit)
        .bind::<diesel::sql_types::BigInt,_>(offset)
        .get_results::<Transaction>(conn)
        .map_err(|e|e.into())
    }
}

