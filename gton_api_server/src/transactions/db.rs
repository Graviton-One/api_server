use diesel::prelude::*;
use actix_web_dev::error::{
    Result,
};
use bigdecimal::BigDecimal;
use serde::{
    Serialize,
};
use chrono::NaiveDateTime;

use diesel::sql_types::{
    Varchar,
    Numeric,
    Timestamp,
    Text,
};

#[derive(QueryableByName, Serialize, Queryable, Clone,Debug)]
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
    #[sql_type="Varchar"]
    explorer: String,
    #[sql_type="Varchar"]
    pair_link: String,
    #[sql_type="Timestamp"]
    stamp: NaiveDateTime,
}

impl Transaction {
    pub async fn get_all(limit: i64, offset: i64, conn: &PgConnection) -> Result<Vec<Transaction>> {
        diesel::sql_query("
        select t.amount, t.tx_hash, t.tx_type, t.user_address as address, t.stamp, p.image, p.pair_link, p.name, c.explorer
        from farms_transactions t
        left join gton_farms f on f.id = t.farm_id
        left join pools p on p.id = f.pool_id
        left join dexes d on d.id = p.dex_id
        left join chains c on c.id = d.chain_id
        ORDER BY stamp DESC
        LIMIT ($1) OFFSET ($2);")
        .bind::<diesel::sql_types::BigInt,_>(limit)
        .bind::<diesel::sql_types::BigInt,_>(offset)
        .get_results::<Transaction>(conn)
        .map_err(|e|e.into())

    }
    pub async fn get_remove(
        limit: i64,
        offset: i64,
        conn: &PgConnection,
    ) -> Result<Vec<Transaction>> {
        diesel::sql_query("select t.amount, t.tx_hash, t.tx_type, t.user_address as address, t.stamp, p.image, p.pair_link, p.name, c.explorer
        from farms_transactions t
        left join gton_farms f on f.id = t.farm_id
        left join pools p on p.id = f.pool_id
        left join dexes d on d.id = p.dex_id
        left join chains c on c.id = d.chain_id
        where t.tx_type = 'Remove'
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
    ) -> Result<Vec<Transaction>> {
        diesel::sql_query("select t.amount, t.tx_hash, t.tx_type, t.user_address as address, t.stamp, p.image, p.pair_link, p.name, c.explorer
                    from farms_transactions t
                    left join gton_farms f on f.id = t.farm_id
                    left join pools p on p.id = f.pool_id
                    left join dexes d on d.id = p.dex_id
                    left join chains c on c.id = d.chain_id
                    where t.tx_type = 'Add'
                    ORDER BY stamp DESC
                    LIMIT ($1) OFFSET ($2);")
        .bind::<diesel::sql_types::BigInt,_>(limit)
        .bind::<diesel::sql_types::BigInt,_>(offset)
        .get_results::<Transaction>(conn)
        .map_err(|e|e.into())
    }
}

