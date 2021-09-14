use crate::DbPool;
use actix_web::{HttpResponse, web::{self}};
use super::db::{
    Transaction,
};
use serde::{Serialize,Deserialize};
use actix_web_dev::error::{
    Result,
};


pub fn txn_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/transactions")
        .route("/list", web::post().to(get_transactions))
    );
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum TxnType {
    All,
    Swap,
    Remove,
    Add
}

#[derive(Serialize,Deserialize)]
pub struct TxnQuery {
    limit: i64,
    offset: i64,
    txn_type: TxnType
}

pub async fn get_transactions (
    query: web::Json<TxnQuery>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = pool.get()?;
    let res;
    match query.txn_type {
        TxnType::All => res = Transaction::get_all(query.limit, query.offset, &conn).await,
        TxnType::Swap => res = Transaction::get_swap(query.limit, query.offset, &conn).await,
        TxnType::Remove => res = Transaction::get_remove(query.limit, query.offset, &conn).await,
        TxnType::Add => res = Transaction::get_add(query.limit, query.offset, &conn).await,
    }
    Ok(HttpResponse::Ok().json(res))
}
