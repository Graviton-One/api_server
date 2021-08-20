use actix_web::{App, HttpRequest, HttpServer, Responder, web, HttpResponse};

use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use gton_api_server::{
    ChainConfig,
};

use serde::{Serialize,Deserialize};
use diesel_migrations::run_pending_migrations;
use gton_api_server::fee_giver::routes::{
    check_vote,
    get_vote_count,
};
use gton_api_server::gton_stats::routes::{
    gton_cost,
};

use dotenv;
use actix_cors::Cors;

use gton_api_server::{
    users::routes::users_routes,
    gton_stats::routes::stats_routes,
    voting::routes::voting_routes,
};

#[derive(Debug, Serialize, Deserialize)]
struct UserVote {
    id: u64,
    address: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    // Create connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let config = ChainConfig::from_env();

    // match run_pending_migrations(&pool.get().unwrap()) {
    //     Ok(_) => print!("migration success\n"),
    //     Err(e)=> print!("migration error: {}\n",&e),
    // };

    // Start HTTP server
    use std::sync::Arc;
    
    let config = Arc::new(config);

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .data(pool.clone())
            .data(config.clone())
            .service(
                web::scope("/api")
                    .configure(users_routes)
                    .configure(stats_routes)
                    .configure(voting_routes)
                    .route("/check_vote", web::post().to(check_vote))
                    .route("/check_vote", web::get().to(get_vote_count))
                    .route("/gton_cost", web::post().to(gton_cost))
            )
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}
