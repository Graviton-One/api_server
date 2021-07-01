use crate::DbPool;
use actix_web::{HttpResponse, web::{self}};
use super::db::{
    Volume,
};
use serde::{Serialize,Deserialize};
use actix_web_dev::error::{
    Result,
};
use chrono::NaiveDateTime;


pub fn pool_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/pools")
        .route("/list", web::get().to(get_pools))
    );
}


#[derive(Serialize,Deserialize)]
pub struct TimeInterval {
    from: NaiveDateTime,
    to: NaiveDateTime,
}

pub async fn get_pools (
    duration: web::Json<TimeInterval>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = pool.get()?;
    let r = Volume::uni_stats(duration.from, duration.to, &conn).await?;
    Ok(HttpResponse::Ok().json(r))
}
