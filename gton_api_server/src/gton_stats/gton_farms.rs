
use diesel::prelude::*;
use actix_web_dev::error::{
    Result,
};
use serde::{
    Serialize,
};
use diesel::sql_types::{
    Varchar,
    Bool,
    Float8,
    BigInt,
    Text
};

#[derive(QueryableByName, Debug, Clone, Serialize)]
pub struct FarmsData {
    #[sql_type="BigInt"]
    pub id: i64,
    #[sql_type="Varchar"]
    pub name: String,
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
    #[sql_type="Varchar"]
    pub token_image: String,
    #[sql_type="Float8"]
    pub tvl: f64,
    #[sql_type="Float8"]
    pub apy: f64,
    #[sql_type="Float8"]
    pub farmed: f64,
    #[sql_type="Float8"]
    pub assigned: f64,
    #[sql_type="BigInt"]
    pub allocation: i64,
    #[sql_type="Bool"]
    pub status: bool,
}

impl FarmsData {
    pub async fn get_largest_farm( 
        conn: &PgConnection,
    ) -> Result<Vec<Self>> {
        let r = diesel::sql_query(
            "SELECT f.id, p.name, p.image as pool_image, p.pair_link, d.name as amm_name, d.image as amm_image, p.image AS pool_image, 
            p.tvl, f.active as status, f.apy, f.farmed, f.allocation, f.assigned, c.chain_icon as chain_image, p.token_pair_image as token_image
            FROM gton_farms AS f 
            INNER JOIN pools AS p ON p.id = f.pool_id 
            INNER JOIN dexes AS d ON d.id = p.dex_id 
            LEFT JOIN chains AS c ON c.id = d.chain_id
            ORDER BY f.apy desc
            LIMIT 1;")
            .get_results::<Self>(conn)?;
        Ok(r)
    }
    pub async fn get_one( 
        address: &str,
        conn: &PgConnection,
    ) -> Result<Vec<Self>> {
        let r = diesel::sql_query(
            "SELECT f.id, p.name, p.image as pool_image, p.pair_link, d.name as amm_name, d.image as amm_image, p.image AS pool_image, 
            p.tvl, f.active as status, f.apy, f.farmed, f.allocation, f.assigned, c.chain_icon as chain_image,  p.token_pair_image as token_image
            FROM gton_farms AS f 
            INNER JOIN pools AS p ON p.id = f.pool_id 
            INNER JOIN dexes AS d ON d.id = p.dex_id 
            LEFT JOIN chains AS c ON c.id = d.chain_id
            WHERE f.lock_address = ($1);")
            .bind::<Text, _>(&address)
            .get_results::<Self>(conn)?;
        Ok(r)
    }

    pub async fn get( 
        conn: &PgConnection,
    ) -> Result<Vec<Self>> {
        let r = diesel::sql_query(
            "SELECT f.id, p.name, p.image as pool_image, p.pair_link, d.name as amm_name, d.image as amm_image, p.image AS pool_image, 
            p.tvl, f.active as status, f.apy, f.farmed, f.allocation, f.assigned, c.chain_icon as chain_image,  p.token_pair_image as token_image
            FROM gton_farms AS f 
            INNER JOIN pools AS p ON p.id = f.pool_id 
            INNER JOIN dexes AS d ON d.id = p.dex_id 
            LEFT JOIN chains AS c ON c.id = d.chain_id;")
            .get_results::<Self>(conn)?;
        Ok(r)
    }
}