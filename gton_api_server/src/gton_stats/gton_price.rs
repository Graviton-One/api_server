use crate::schema::gton_price;
use diesel::prelude::*;
use actix_web_dev::error::{
    Result,
};
use serde::{
    Serialize,
    Deserialize,
};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Queryable, Debug, Clone)]
pub struct GtonPrice {
    pub id: i32,
    pub price: f64,
    pub market_time: NaiveDateTime,
}

impl GtonPrice {
    pub async fn interval( 
        from: NaiveDateTime,
        to: NaiveDateTime,
        conn: &PgConnection,
    ) -> Result<Vec<Self>> {
        gton_price::table
            .filter(gton_price::market_time.ge(from))
            .filter(gton_price::market_time.le(to))
            .order_by(gton_price::market_time.asc())
            .get_results(conn)
            .map_err(|e|e.into())
    }
}
