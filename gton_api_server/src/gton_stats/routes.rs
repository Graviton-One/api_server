use crate::DbPool;
use chrono::NaiveDateTime;
//use super::db::GtonPrice;
use actix_web::{HttpResponse, web::{self, Query}};
use super::db::{
    GtonPrice,
    ReservesData,
    TvlData,
    FarmsData
};
use serde::{Serialize,Deserialize};
use actix_web_dev::error::{
    Result,
};

pub fn stats_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/stats")
        .route("/price", web::post().to(gton_cost))
        .route("/tvl_list", web::get().to(tvl_list))
        .route("/pool", web::get().to(pool_data))
        .route("/reserves_list", web::get().to(reserves_list))
        .route("/farms_list", web::get().to(farms_list))
        .route("/farm", web::get().to(farm_data))
        .route("/superb_farm", web::get().to(get_largest))

    );
}

#[derive(Serialize,Deserialize)]
pub struct TimeInterval {
    from: NaiveDateTime,
    to: NaiveDateTime,
}

#[derive(Serialize,Deserialize)]
pub struct Pool {
    address: String,
}

pub async fn gton_cost (
    duration: web::Json<TimeInterval>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = pool.get()?;
    let r = GtonPrice::interval(duration.from, duration.to, &conn).await?;
    Ok(HttpResponse::Ok().json(r))
}

pub async fn tvl_list (
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = pool.get()?;
    let r = TvlData::get(&conn).await?;
    Ok(HttpResponse::Ok().json(r))
}

pub async fn pool_data (
    query: Query<Pool>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = pool.get()?;
    let r = TvlData::get_one(query.address.as_str(), &conn).await?;
    Ok(HttpResponse::Ok().json(r))
}

pub async fn farms_list (
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = pool.get()?;
    let r = FarmsData::get(&conn).await?;
    Ok(HttpResponse::Ok().json(r))
}

pub async fn get_largest (
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = pool.get()?;
    let r = FarmsData::get_largest_farm(&conn).await?;
    Ok(HttpResponse::Ok().json(r))
}
pub async fn farm_data (
    query: Query<Pool>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = pool.get()?;
    let r = FarmsData::get_one(query.address.as_str() ,&conn).await?;
    Ok(HttpResponse::Ok().json(r))
}

pub async fn reserves_list (
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = pool.get()?;
    let r = ReservesData::get(&conn).await?;
    Ok(HttpResponse::Ok().json(r))
}

