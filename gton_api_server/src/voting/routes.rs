use crate::DbPool;
use actix_web::{HttpResponse, web::{self}};
use super::db::{
    VotingInstance,
    UpdateVoting,
};
use serde::{Serialize,Deserialize};
use actix_web_dev::error::{
    Result,
};

pub fn voting_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/votings")
        .route("/list", web::post().to(get_votings))
        .route("/save", web::post().to(insert_voting))
        .route("/update", web::post().to(update_voting))
    );
}

#[derive(Serialize,Deserialize)]
pub struct ID {
    id: Option<i32>,
    active: Option<bool>
}

pub async fn get_votings (
    data: web::Json<ID>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = pool.get()?;
    let r = if data.id.is_some() {
        VotingInstance::get(data.id.unwrap(), &conn).await?
    } else if data.active.is_some() {
        VotingInstance::get_active(data.active.unwrap(), &conn).await?
    } else {
        VotingInstance::get_all(&conn).await?
    };

    Ok(HttpResponse::Ok().json(r))
}

pub async fn update_voting (
    data: web::Json<UpdateVoting>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = pool.get()?;
    let r = VotingInstance::update(data.into_inner(), &conn).await?;
    Ok(HttpResponse::Ok().json(r))
}
pub async fn insert_voting (
    data: web::Json<VotingInstance>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = pool.get()?;
    let r = data.insert(&conn).await?;
    Ok(HttpResponse::Ok().json(r))
}