use crate::DbPool;
//use super::db::GtonPrice;
use diesel::prelude;
use actix_web::{
    web,
    Responder,
    HttpResponse,
};


async fn gton_cost (pool: web::Data<DbPool>) -> impl Responder {
    //let mut d: Vec<GtonPrice> = 
     //   diesel::sql_query("SELECT y_value, x_date FROM gton_value")
      //  .load(&pool.get().unwrap()).unwrap();
    HttpResponse::Ok().json("")
}
