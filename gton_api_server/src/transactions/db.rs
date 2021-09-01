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

#[derive(QueryableByName, Queryable, Clone,Debug)]
pub struct Transaction {
    #[sql_type="Numeric"]
    amount: BigDecimal,
    #[sql_type="Text"]
    tx_hash: String,
    #[sql_type="Text"]
    tx_type: String,
    #[sql_type="Varchar"]
    address: String,
    #[sql_type="Varchar"]
    name: String,
    #[sql_type="Varchar"]
    image: String,
    #[sql_type="Timestamp"]
    stamp: NaiveDateTime,
}
#[derive(Serialize, Deserialize,Clone,Debug)]

pub struct TransactionSerde {
    amount: String,
    tx_hash: String,
    address: String,
    stamp: NaiveDateTime,
}

// shortcode example

// let array: Vec<FrontendTnx> = vec![];
// let res = diesel::sql_query("SELECT tx_hash, stamp, amount_lp_in FROM ;")
//     .get_results::<Transaction>(conn)
//     .map_err(|e|e.into());
// for item in res {
//     array.push(FrontendTnx{
//         tx_type: "Add",
//         reward,
//         reward_icon,
        
//     })
// }

impl Transaction {
    pub async fn get_all(limit: i64, offset: i64, conn: &PgConnection) -> Result<Vec<TransactionSerde>> {
        let res = diesel::sql_query("SELECT 'Remove' AS tx_type, tr1.tx_hash, tr1.stamp, tr1.amount_lp_in as amount, pr1.address, ptr1.image, ptr1.name FROM univ2_lp_remove_bsc_pancake AS tr1 
        LEFT JOIN events_univ2_pair_created_bsc_pancake AS pr1 on tr1.pair_id = pr1.id
        LEFT JOIN pools AS ptr1 ON LOWER(ptr1.pool_address) = LOWER(pr1.address)
        UNION SELECT 'Remove' AS tx_type, tr2.tx_hash, tr2.stamp, tr2.amount_lp_in as amount, pr2.address, ptr2.image, ptr2.name FROM univ2_lp_remove_eth_sushi AS tr2
        LEFT JOIN events_univ2_pair_created_eth_sushi AS pr2 on tr2.pair_id = pr2.id
        LEFT JOIN pools AS ptr2 ON LOWER(ptr2.pool_address) = LOWER(pr2.address)
        UNION SELECT 'Remove' AS tx_type, tr3.tx_hash, tr3.stamp, tr3.amount_lp_in as amount, pr3.address, ptr3.image, ptr3.name FROM univ2_lp_remove_ftm_spirit AS tr3
        LEFT JOIN events_univ2_pair_created_ftm_spirit AS pr3 on tr3.pair_id = pr3.id
        LEFT JOIN pools AS ptr3 ON LOWER(ptr3.pool_address) = LOWER(pr3.address)
        UNION SELECT 'Remove' AS tx_type, tr4.tx_hash, tr4.stamp, tr4.amount_lp_in as amount, pr4.address, ptr4.image, ptr4.name FROM univ2_lp_remove_ftm_spooky AS tr4
        LEFT JOIN events_univ2_pair_created_ftm_spooky AS pr4 on tr4.pair_id = pr4.id
        LEFT JOIN pools AS ptr4 ON LOWER(ptr4.pool_address) = LOWER(pr4.address)
        UNION SELECT 'Remove' AS tx_type, tr5.tx_hash, tr5.stamp, tr5.amount_lp_in as amount, pr5.address, ptr5.image, ptr5.name FROM univ2_lp_remove_plg_quick AS tr5
        LEFT JOIN events_univ2_pair_created_plg_quick AS pr5 on tr5.pair_id = pr5.id
        LEFT JOIN pools AS ptr5 ON LOWER(ptr5.pool_address) = LOWER(pr5.address)
        UNION SELECT 'Add' AS tx_type, ta1.tx_hash, ta1.stamp, ta1.amount_lp_out as amount, pa1.address, pta1.image, pta1.name FROM univ2_lp_add_bsc_pancake AS ta1 
        LEFT JOIN events_univ2_pair_created_bsc_pancake AS pa1 on ta1.pair_id = pa1.id
        LEFT JOIN pools AS pta1 ON LOWER(pta1.pool_address) = LOWER(pa1.address)
        UNION SELECT 'Add' AS tx_type, ta2.tx_hash, ta2.stamp, ta2.amount_lp_out as amount, pa2.address, pta2.image, pta2.name FROM univ2_lp_add_eth_sushi AS ta2
        LEFT JOIN events_univ2_pair_created_eth_sushi AS pa2 on ta2.pair_id = pa2.id
        LEFT JOIN pools AS pta2 ON LOWER(pta2.pool_address) = LOWER(pa2.address)
        UNION SELECT 'Add' AS tx_type, ta3.tx_hash, ta3.stamp, ta3.amount_lp_out as amount, pa3.address, pta3.image, pta3.name FROM univ2_lp_add_ftm_spirit AS ta3
        LEFT JOIN events_univ2_pair_created_ftm_spirit AS pa3 on ta3.pair_id = pa3.id
        LEFT JOIN pools AS pta3 ON LOWER(pta3.pool_address) = LOWER(pa3.address)
        UNION SELECT 'Add' AS tx_type, ta4.tx_hash, ta4.stamp, ta4.amount_lp_out as amount, pa4.address, pta4.image, pta4.name FROM univ2_lp_add_ftm_spooky AS ta4
        LEFT JOIN events_univ2_pair_created_ftm_spooky AS pa4 on ta4.pair_id = pa4.id
        LEFT JOIN pools AS pta4 ON LOWER(pta4.pool_address) = LOWER(pa4.address)
        UNION SELECT 'Add' AS tx_type, ta5.tx_hash, ta5.stamp, ta5.amount_lp_out as amount, pa5.address, pta5.image, pta5.name FROM univ2_lp_add_plg_quick AS ta5
        LEFT JOIN events_univ2_pair_created_plg_quick AS pa5 on ta5.pair_id = pa5.id
        LEFT JOIN pools AS pta5 ON LOWER(pta5.pool_address) = LOWER(pa5.address)
        UNION SELECT 'Swap' AS tx_type, t1.tx_hash, t1.stamp, t1.amount_gton_out as amount, p1.address, pt1.image, pt1.name FROM univ2_buy_bsc_pancake AS t1 
        LEFT JOIN events_univ2_pair_created_bsc_pancake AS p1 on t1.pair_id = p1.id
        LEFT JOIN pools AS pt1 ON LOWER(pt1.pool_address) = LOWER(p1.address)
        UNION SELECT 'Swap' AS tx_type, t2.tx_hash, t2.stamp, t2.amount_gton_out as amount, p2.address, pt2.image, pt2.name FROM univ2_buy_eth_sushi AS t2
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p2 on t2.pair_id = p2.id
        LEFT JOIN pools AS pt2 ON LOWER(pt2.pool_address) = LOWER(p2.address)
        UNION SELECT 'Swap' AS tx_type, t3.tx_hash, t3.stamp, t3.amount_gton_out as amount, p3.address, pt3.image, pt3.name FROM univ2_buy_ftm_spirit AS t3
        LEFT JOIN events_univ2_pair_created_ftm_spirit AS p3 on t3.pair_id = p3.id
        LEFT JOIN pools AS pt3 ON LOWER(pt3.pool_address) = LOWER(p3.address)
        UNION SELECT 'Swap' AS tx_type, t4.tx_hash, t4.stamp, t4.amount_gton_out as amount, p4.address, pt4.image, pt4.name FROM univ2_buy_ftm_spooky AS t4
        LEFT JOIN events_univ2_pair_created_ftm_spooky AS p4 on t4.pair_id = p4.id
        LEFT JOIN pools AS pt4 ON LOWER(pt4.pool_address) = LOWER(p4.address)
        UNION SELECT 'Swap' AS tx_type, t5.tx_hash, t5.stamp, t5.amount_gton_out as amount, p5.address, pt5.image, pt5.name FROM univ2_buy_plg_quick AS t5
        LEFT JOIN events_univ2_pair_created_plg_quick AS p5 on t5.pair_id = p5.id
        LEFT JOIN pools AS pt5 ON LOWER(pt5.pool_address) = LOWER(p5.address)
        UNION SELECT 'Swap' AS tx_type, t6.tx_hash, t6.stamp, t6.amount_gton_in as amount, p6.address, pt6.image, pt6.name FROM univ2_sell_bsc_pancake AS t6 
        LEFT JOIN events_univ2_pair_created_bsc_pancake AS p6 on t6.pair_id = p6.id
        LEFT JOIN pools AS pt6 ON LOWER(pt6.pool_address) = LOWER(p6.address)
        UNION SELECT 'Swap' AS tx_type, t7.tx_hash, t7.stamp, t7.amount_gton_in as amount, p7.address, pt7.image, pt7.name FROM univ2_sell_eth_sushi AS t7
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p7 on t7.pair_id = p7.id
        LEFT JOIN pools AS pt7 ON LOWER(pt7.pool_address) = LOWER(p7.address)
        UNION SELECT 'Swap' AS tx_type, t8.tx_hash, t8.stamp, t8.amount_gton_in as amount, p8.address, pt8.image, pt8.name FROM univ2_sell_ftm_spirit AS t8
        LEFT JOIN events_univ2_pair_created_ftm_spirit AS p8 on t8.pair_id = p8.id
        LEFT JOIN pools AS pt8 ON LOWER(pt8.pool_address) = LOWER(p8.address)
        UNION SELECT 'Swap' AS tx_type, t9.tx_hash, t9.stamp, t9.amount_gton_in as amount, p9.address, pt9.image, pt9.name FROM univ2_sell_ftm_spooky AS t9
        LEFT JOIN events_univ2_pair_created_ftm_spooky AS p9 on t9.pair_id = p9.id
        LEFT JOIN pools AS pt9 ON LOWER(pt9.pool_address) = LOWER(p9.address)
        UNION SELECT 'Swap' AS tx_type, t10.tx_hash, t10.stamp, t10.amount_gton_in as amount, p10.address, pt10.image, pt10.name FROM univ2_sell_plg_quick AS t10
        LEFT JOIN events_univ2_pair_created_plg_quick AS p10 on t10.pair_id = p10.id
        LEFT JOIN pools AS pt10 ON LOWER(pt10.pool_address) = LOWER(p10.address)
        ORDER BY stamp DESC
        LIMIT ($1) OFFSET ($2);")
        .bind::<diesel::sql_types::BigInt,_>(limit)
        .bind::<diesel::sql_types::BigInt,_>(offset)
        .get_results::<Transaction>(conn)?;
        Ok(res
            .into_iter()
        .map(|el|{
            TransactionSerde {
                tx_hash: el.tx_hash,
                address: el.address,
                stamp: el.stamp,
                amount: el.amount.to_string()
            }
        })
        .collect())
    }
    //Ok(NaiveDateTime::from_timestamp(stamp_i64, 0))
    pub async fn get_remove(
        limit: i64,
        offset: i64,
        conn: &PgConnection,
    ) -> Result<Vec<TransactionSerde>> {
        let res = diesel::sql_query("SELECT 'Remove' AS tx_type, t1.tx_hash, t1.stamp, t1.amount_lp_in as amount, p1.address, pt1.image, pt1.name FROM univ2_lp_remove_bsc_pancake AS t1 
        LEFT JOIN events_univ2_pair_created_bsc_pancake AS p1 on t1.pair_id = p1.id
        LEFT JOIN pools AS pt1 ON LOWER(pt1.pool_address) = LOWER(p1.address)
        UNION SELECT 'Remove' AS tx_type, t2.tx_hash, t2.stamp, t2.amount_lp_in as amount, p2.address, pt2.image, pt2.name FROM univ2_lp_remove_eth_sushi AS t2
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p2 on t2.pair_id = p2.id
        LEFT JOIN pools AS pt2 ON LOWER(pt2.pool_address) = LOWER(p2.address)
        UNION SELECT 'Remove' AS tx_type, t3.tx_hash, t3.stamp, t3.amount_lp_in as amount, p3.address, pt3.image, pt3.name FROM univ2_lp_remove_ftm_spirit AS t3
        LEFT JOIN events_univ2_pair_created_ftm_spirit AS p3 on t3.pair_id = p3.id
        LEFT JOIN pools AS pt3 ON LOWER(pt3.pool_address) = LOWER(p3.address)
        UNION SELECT 'Remove' AS tx_type, t4.tx_hash, t4.stamp, t4.amount_lp_in as amount, p4.address, pt4.image, pt4.name FROM univ2_lp_remove_ftm_spooky AS t4
        LEFT JOIN events_univ2_pair_created_ftm_spooky AS p4 on t4.pair_id = p4.id
        LEFT JOIN pools AS pt4 ON LOWER(pt4.pool_address) = LOWER(p4.address)
        UNION SELECT 'Remove' AS tx_type, t5.tx_hash, t5.stamp, t5.amount_lp_in as amount, p5.address, pt5.image, pt5.name FROM univ2_lp_remove_plg_quick AS t5
        LEFT JOIN events_univ2_pair_created_plg_quick AS p5 on t5.pair_id = p5.id
        LEFT JOIN pools AS pt5 ON LOWER(pt5.pool_address) = LOWER(p5.address)
        ORDER BY stamp DESC
        LIMIT ($1) OFFSET ($2);")
        .bind::<diesel::sql_types::BigInt,_>(limit)
        .bind::<diesel::sql_types::BigInt,_>(offset)
        .get_results::<Transaction>(conn)?;
        Ok(res
            .into_iter()
        .map(|el|{
            TransactionSerde {
                tx_hash: el.tx_hash,
                address: el.address,
                stamp: el.stamp,
                amount: el.amount.to_string()
            }
        })
        .collect())
    }

    pub async fn get_add(
        limit: i64,
        offset: i64,
        conn: &PgConnection,
    ) -> Result<Vec<TransactionSerde>> {
        let res = diesel::sql_query("SELECT 'Add' AS tx_type, t1.tx_hash, t1.stamp, t1.amount_lp_out as amount, p1.address, pt1.image, pt1.name FROM univ2_lp_add_bsc_pancake AS t1 
        LEFT JOIN events_univ2_pair_created_bsc_pancake AS p1 on t1.pair_id = p1.id
        LEFT JOIN pools AS pt1 ON LOWER(pt1.pool_address) = LOWER(p1.address)
        UNION SELECT 'Add' AS tx_type, t2.tx_hash, t2.stamp, t2.amount_lp_out as amount, p2.address, pt2.image, pt2.name FROM univ2_lp_add_eth_sushi AS t2
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p2 on t2.pair_id = p2.id
        LEFT JOIN pools AS pt2 ON LOWER(pt2.pool_address) = LOWER(p2.address)
        UNION SELECT 'Add' AS tx_type, t3.tx_hash, t3.stamp, t3.amount_lp_out as amount, p3.address, pt3.image, pt3.name FROM univ2_lp_add_ftm_spirit AS t3
        LEFT JOIN events_univ2_pair_created_ftm_spirit AS p3 on t3.pair_id = p3.id
        LEFT JOIN pools AS pt3 ON LOWER(pt3.pool_address) = LOWER(p3.address)
        UNION SELECT 'Add' AS tx_type, t4.tx_hash, t4.stamp, t4.amount_lp_out as amount, p4.address, pt4.image, pt4.name FROM univ2_lp_add_ftm_spooky AS t4
        LEFT JOIN events_univ2_pair_created_ftm_spooky AS p4 on t4.pair_id = p4.id
        LEFT JOIN pools AS pt4 ON LOWER(pt4.pool_address) = LOWER(p4.address)
        UNION SELECT 'Add' AS tx_type, t5.tx_hash, t5.stamp, t5.amount_lp_out as amount, p5.address, pt5.image, pt5.name FROM univ2_lp_add_plg_quick AS t5
        LEFT JOIN events_univ2_pair_created_plg_quick AS p5 on t5.pair_id = p5.id
        LEFT JOIN pools AS pt5 ON LOWER(pt5.pool_address) = LOWER(p5.address)
        ORDER BY stamp DESC
        LIMIT ($1) OFFSET ($2);")
        .bind::<diesel::sql_types::BigInt,_>(limit)
        .bind::<diesel::sql_types::BigInt,_>(offset)
        .get_results::<Transaction>(conn)?;
        Ok(res
            .into_iter()
        .map(|el|{
            TransactionSerde {
                tx_hash: el.tx_hash,
                address: el.address,
                stamp: el.stamp,
                amount: el.amount.to_string()
            }
        })
        .collect())
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
    ) -> Result<Vec<TransactionSerde>> {
        let res = diesel::sql_query("SELECT 'Swap' AS tx_type, t1.tx_hash, t1.stamp, t1.amount_gton_out as amount, p1.address, pt1.image, pt1.name FROM univ2_buy_bsc_pancake AS t1 
        LEFT JOIN events_univ2_pair_created_bsc_pancake AS p1 on t1.pair_id = p1.id
        LEFT JOIN pools AS pt1 ON LOWER(pt1.pool_address) = LOWER(p1.address)
        UNION SELECT 'Swap' AS tx_type, t2.tx_hash, t2.stamp, t2.amount_gton_out as amount, p2.address, pt2.image, pt2.name FROM univ2_buy_eth_sushi AS t2
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p2 on t2.pair_id = p2.id
        LEFT JOIN pools AS pt2 ON LOWER(pt2.pool_address) = LOWER(p2.address)
        UNION SELECT 'Swap' AS tx_type, t3.tx_hash, t3.stamp, t3.amount_gton_out as amount, p3.address, pt3.image, pt3.name FROM univ2_buy_ftm_spirit AS t3
        LEFT JOIN events_univ2_pair_created_ftm_spirit AS p3 on t3.pair_id = p3.id
        LEFT JOIN pools AS pt3 ON LOWER(pt3.pool_address) = LOWER(p3.address)
        UNION SELECT 'Swap' AS tx_type, t4.tx_hash, t4.stamp, t4.amount_gton_out as amount, p4.address, pt4.image, pt4.name FROM univ2_buy_ftm_spooky AS t4
        LEFT JOIN events_univ2_pair_created_ftm_spooky AS p4 on t4.pair_id = p4.id
        LEFT JOIN pools AS pt4 ON LOWER(pt4.pool_address) = LOWER(p4.address)
        UNION SELECT 'Swap' AS tx_type, t5.tx_hash, t5.stamp, t5.amount_gton_out as amount, p5.address, pt5.image, pt5.name FROM univ2_buy_plg_quick AS t5
        LEFT JOIN events_univ2_pair_created_plg_quick AS p5 on t5.pair_id = p5.id
        LEFT JOIN pools AS pt5 ON LOWER(pt5.pool_address) = LOWER(p5.address)
        UNION SELECT 'Swap' AS tx_type, t6.tx_hash, t6.stamp, t6.amount_gton_in as amount, p6.address, pt6.image, pt6.name FROM univ2_sell_bsc_pancake AS t6 
        LEFT JOIN events_univ2_pair_created_bsc_pancake AS p6 on t6.pair_id = p6.id
        LEFT JOIN pools AS pt6 ON LOWER(pt6.pool_address) = LOWER(p6.address)
        UNION SELECT 'Swap' AS tx_type, t7.tx_hash, t7.stamp, t7.amount_gton_in as amount, p7.address, pt7.image, pt7.name FROM univ2_sell_eth_sushi AS t7
        LEFT JOIN events_univ2_pair_created_eth_sushi AS p7 on t7.pair_id = p7.id
        LEFT JOIN pools AS pt7 ON LOWER(pt7.pool_address) = LOWER(p7.address)
        UNION SELECT 'Swap' AS tx_type, t8.tx_hash, t8.stamp, t8.amount_gton_in as amount, p8.address, pt8.image, pt8.name FROM univ2_sell_ftm_spirit AS t8
        LEFT JOIN events_univ2_pair_created_ftm_spirit AS p8 on t8.pair_id = p8.id
        LEFT JOIN pools AS pt8 ON LOWER(pt8.pool_address) = LOWER(p8.address)
        UNION SELECT 'Swap' AS tx_type, t9.tx_hash, t9.stamp, t9.amount_gton_in as amount, p9.address, pt9.image, pt9.name FROM univ2_sell_ftm_spooky AS t9
        LEFT JOIN events_univ2_pair_created_ftm_spooky AS p9 on t9.pair_id = p9.id
        LEFT JOIN pools AS pt9 ON LOWER(pt9.pool_address) = LOWER(p9.address)
        UNION SELECT 'Swap' AS tx_type, t10.tx_hash, t10.stamp, t10.amount_gton_in as amount, p10.address, pt10.image, pt10.name FROM univ2_sell_plg_quick AS t10
        LEFT JOIN events_univ2_pair_created_plg_quick AS p10 on t10.pair_id = p10.id
        LEFT JOIN pools AS pt10 ON LOWER(pt10.pool_address) = LOWER(p10.address)
        ORDER BY stamp DESC
        LIMIT ($1) OFFSET ($2);")
        .bind::<diesel::sql_types::BigInt,_>(limit)
        .bind::<diesel::sql_types::BigInt,_>(offset)
        .get_results::<Transaction>(conn)?;
        Ok(res
            .into_iter()
        .map(|el|{
            TransactionSerde {
                tx_hash: el.tx_hash,
                address: el.address,
                stamp: el.stamp,
                amount: el.amount.to_string()
            }
        })
        .collect())
    }
}

