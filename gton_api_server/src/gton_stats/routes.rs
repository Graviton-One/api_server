use crate::DbPool;
use chrono::NaiveDateTime;
//use super::db::GtonPrice;
use actix_web::{HttpResponse, web::{self, Query}};
use super::db::{
    GtonPrice,
    UsersValues,
    TotalValues,
    ReservesData,
    TvlData
};
use serde::{Serialize,Deserialize};
use actix_web_dev::error::{
    Result,
};

pub fn stats_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/stats")
        .route("/price", web::post().to(gton_cost))
        .route("/users_values", web::get().to(total_by_users))
        .route("/total_values", web::get().to(total_values))
        .route("/tvl_list", web::get().to(tvl_list))
        .route("/reserves_list", web::get().to(reserves_list))

    );
}

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

#[derive(Serialize,Deserialize)]
pub struct UserAddress {
    address: String,
}

pub async fn total_by_users (
    data: Query<UserAddress>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = pool.get()?;
    let r = UsersValues::get(data.address.as_str(), &conn).await?;
    Ok(HttpResponse::Ok().json(r))
}

pub async fn total_values (
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = pool.get()?;
    let r = TotalValues::get(&conn).await?;
    Ok(HttpResponse::Ok().json(r))
}

pub async fn tvl_list (
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = pool.get()?;
    let r = TvlData::get(&conn).await?;
    Ok(HttpResponse::Ok().json(r))
}

pub async fn reserves_list (
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = pool.get()?;
    let r = ReservesData::get(&conn).await?;
    Ok(HttpResponse::Ok().json(r))
}

