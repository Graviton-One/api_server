
use crate::schema::uni_stats;
use diesel::prelude::*;
use actix_web_dev::error::{
    Result,
};
use serde::{
    Serialize,
    Deserialize,
};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Volume {
    id: i32,
    tvl: i64,
    volume: i64,
    addresses_count: i32,
    apy: i32,
    date: NaiveDateTime,
}

impl Volume {
    pub async fn uni_stats( 
        from: NaiveDateTime,
        to: NaiveDateTime,
        conn: &PgConnection,
    ) -> Result<Vec<Self>> {
        uni_stats::table
            .filter(uni_stats::date.ge(from))
            .filter(uni_stats::date.le(to))
            .order_by(uni_stats::date.asc())
            .get_results(conn)
            .map_err(|e|e.into())
    }
}