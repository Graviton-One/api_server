use crate::DbPool;
use chrono::NaiveDateTime;
//use super::db::GtonPrice;
use actix_web::{
    web,
    HttpResponse,
};
use super::db::GtonPrice;
use serde::{Serialize,Deserialize};
use actix_web_dev::error::{
    Result,
};

#[derive(Serialize,Deserialize)]
pub struct TimeInterval {
    from: NaiveDateTime,
    to: NaiveDateTime,
}


pub async fn gton_cost (
    duration: web::Json<TimeInterval>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = pool.get()?;
    let r = GtonPrice::interval(duration.from, duration.to, &conn).await?;
    Ok(HttpResponse::Ok().json(r))
}
