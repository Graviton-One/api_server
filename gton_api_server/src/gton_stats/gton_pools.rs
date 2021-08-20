use diesel::prelude::*;
use actix_web_dev::error::{
    Result,
};
use bigdecimal::BigDecimal;
use serde::{
    Serialize,
    Deserialize,
};
use diesel::sql_types::{
    Varchar,
    Numeric,
    Float8,
    BigInt
};

#[derive(QueryableByName, Debug, Clone, Serialize)]
pub struct ReservesData {
    #[sql_type="BigInt"]
    pub id: i64,
    #[sql_type="Varchar"]
    pub name: String,
    #[sql_type="Varchar"]
    pub image: String,
    #[sql_type="Float8"]
    pub reserves: f64,
}

#[derive(QueryableByName, Debug, Clone, Serialize)]
pub struct TvlData {
    #[sql_type="BigInt"]
    pub id: i64,
    #[sql_type="Varchar"]
    pub name: String,
    #[sql_type="Varchar"]
    pub swap_link: String,
    #[sql_type="Varchar"]
    pub pair_link: String,
    #[sql_type="Varchar"]
    pub pool_image: String,
    #[sql_type="Varchar"]
    pub amm_image: String,
    #[sql_type="Varchar"]
    pub chain_image: String,
    #[sql_type="Varchar"]
    pub amm_name: String,
    #[sql_type="Float8"]
    pub tvl: f64,
}

impl TvlData {
    pub async fn get( 
        conn: &PgConnection,
    ) -> Result<Vec<TvlData>> {
        let r = diesel::sql_query(
            "SELECT p.id, p.name, p.swap_link, p.pair_link, p.image AS pool_image, p.tvl, d.small_image AS amm_image, d.name AS amm_name, c.chain_icon AS chain_image 
            FROM chains AS c 
            LEFT JOIN dexes AS d ON d.chain_id = c.id 
            LEFT JOIN pools AS p ON d.id = p.dex_id;")
            .get_results::<TvlData>(conn)?;
        Ok(r)
    }
}
impl ReservesData {
    pub async fn get( 
        conn: &PgConnection,
    ) -> Result<Vec<ReservesData>> {
        let r = diesel::sql_query(
            "SELECT d.id, d.image, d.name, SUM(p.gton_reserves) AS reserves FROM dexes AS d
            LEFT JOIN pools AS p ON d.id = p.dex_id
            GROUP BY d.id;")
            .get_results::<ReservesData>(conn)?;
        Ok(r)
    }
}

