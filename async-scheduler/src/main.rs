use tokio_cron_scheduler::{JobScheduler, JobToRun, Job};

use async_sceduler::price_coingeco::CoingecoPrice;
use async_sceduler::keeper_extractor::KeeperExtractor;

use diesel::r2d2::{ConnectionManager,Pool};
use diesel::PgConnection;

#[tokio::main]
async fn main() {
    let mut sched = JobScheduler::new();

    let manager =
        ConnectionManager::<PgConnection>::new(std::env::var("DATABASE_URL")
        .expect("missing db url"));
    let pool = Pool::builder().build(manager).expect("pool build");

    let pool = std::sync::Arc::new(pool);
    let p = pool.clone();
    tokio::task::spawn(async move {
        KeeperExtractor::new(p.clone()).run().await;
    });

    sched.add(Job::new("0 0 * * * *", move |_,_| {
        let p = pool.clone();
        tokio::task::spawn(async move {
            CoingecoPrice::new(p.clone()).run().await;
    });}).unwrap()).unwrap();

    sched.start().await.unwrap();
}
