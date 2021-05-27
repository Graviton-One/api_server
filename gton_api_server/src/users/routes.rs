use crate::DbPool;
use chrono::NaiveDateTime;
//use super::db::GtonPrice;
use actix_web::{HttpResponse, web::{self, Query}};
use super::db::{
    Achievements,
};
use serde::{Serialize,Deserialize};
use actix_web_dev::error::{
    Result,
};

pub fn users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users")
        .route("/achievements", web::get().to(get_achievements))
    );
}

#[derive(Serialize,Deserialize)]
pub struct UserAddress {
    address: String,
}

pub async fn get_achievements (
    data: Query<UserAddress>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = pool.get()?;
    let r = Achievements::get(data.address.as_str(), &conn).await?;
    Ok(HttpResponse::Ok().json(r))
}


