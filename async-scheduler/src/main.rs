// use tokio_cron_scheduler::{JobScheduler, JobToRun, Job};

use async_scheduler::events_extractor::EventsExtractor;
use async_scheduler::forum_extractor::ForumExtractor;
use async_scheduler::keeper_extractor::KeeperExtractor;
use async_scheduler::price_coingecko::CoingeckoPrice;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

#[tokio::main]
async fn main() {
    // let mut sched = JobScheduler::new();

    let manager = ConnectionManager::<PgConnection>::new(
        std::env::var("DATABASE_URL").expect("missing db url"),
    );
    let pool = Pool::builder().build(manager).expect("pool build");

    let pool = std::sync::Arc::new(pool);

    let p = pool.clone();
    tokio::task::spawn(async move {
        EventsExtractor::new(p.clone()).run().await;
    })
    .await
    .unwrap();

    // let p = pool.clone();
    // tokio::task::spawn(async move {
    //     ForumExtractor::new(p.clone()).run().await;
    // }).await.unwrap();

    // let p = pool.clone();
    // tokio::task::spawn(async move {
    //     KeeperExtractor::new(p.clone()).run().await;
    // });

    // sched.add(Job::new("0 0 * * * *", move |_,_| {
    //     let p = pool.clone();
    //     tokio::task::spawn(async move {
    //         CoingeckoPrice::new(p.clone()).run().await;
    // });}).unwrap()).unwrap();

    // sched.start().await.unwrap();
}
