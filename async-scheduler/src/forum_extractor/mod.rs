use chrono::{NaiveDate, Datelike, Duration, Utc};

use crate::DbPool;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Double, Timestamp, Date};
use diesel::result::Error::DatabaseError;
use diesel::result::DatabaseErrorKind::UniqueViolation;

mod fetchers;
use fetchers::{
    fetch_forum_active_users, fetch_forum_topics, fetch_forum_total_users, fetch_report,
};
mod parsers;
use parsers::{
    parse_forum_active_users, parse_forum_total_posts, parse_forum_total_topics,
    parse_forum_total_users, parse_report_f64, parse_report_i64,
};

#[derive(QueryableByName, PartialEq, Debug)]
struct Stamp {
    #[sql_type = "Date"]
    stamp: NaiveDate,
}

pub struct ForumExtractor {
    pool: DbPool,
    forum_api_key: String,
}

impl ForumExtractor {
    pub fn new(
        pool: DbPool,
    ) -> Self {
        let forum_api_key: String = std::env::var("FORUM_API_KEY")
            .expect("forum api key get");

        ForumExtractor {
            pool,
            forum_api_key
        }
    }

    pub async fn run(&self) {

        poll_forum(&self.pool, &self.forum_api_key).await;

        // dau_by_mau has entry for every day, assume other tables have the same last_stamp
        let last_stamp: NaiveDate = match diesel::sql_query(
            "SELECT stamp FROM forum_report_dau_by_mau ORDER BY stamp DESC;"
        )
            .get_result::<Stamp>(&self.pool.get().unwrap()) {
                Err(_) => NaiveDate::parse_from_str("2021-05-01", "%Y-%m-%d").unwrap(),
                Ok(e) => e.stamp
            };
        println!("starting from stamp {}", last_stamp);

        let today = Utc::now().naive_local().date();
        let days = today.signed_duration_since(last_stamp).num_days();
        for n in (1..days).rev() {
            let date = Utc::now().checked_sub_signed(Duration::days(n)).unwrap().naive_local().date();
            poll_forum_reports(&self.pool, &date, &self.forum_api_key).await;
        }
    }
}

pub async fn poll_forum(p: &DbPool, forum_api_key: &str) {
    let v = fetch_forum_total_users().await;
    let forum_total_users = parse_forum_total_users(v).await;
    diesel::sql_query("insert into forum_total_users(amount) VALUES ($1);")
        .bind::<BigInt, _>(forum_total_users)
        .execute(&p.get().unwrap())
        .unwrap();

    let v = fetch_forum_active_users(forum_api_key).await;
    let forum_active_users = parse_forum_active_users(v).await;
    diesel::sql_query("insert into forum_active_users(amount) VALUES ($1);")
        .bind::<BigInt, _>(forum_active_users)
        .execute(&p.get().unwrap())
        .unwrap();

    let forum_topics = fetch_forum_topics(forum_api_key).await;

    let forum_total_topics = parse_forum_total_topics(&forum_topics).await;
    diesel::sql_query("insert into forum_total_topics(amount) VALUES ($1);")
        .bind::<BigInt, _>(forum_total_topics)
        .execute(&p.get().unwrap())
        .unwrap();

    let forum_total_posts = parse_forum_total_posts(&forum_topics).await;
    diesel::sql_query("insert into forum_total_posts(amount) VALUES ($1);")
        .bind::<BigInt, _>(forum_total_posts)
        .execute(&p.get().unwrap())
        .unwrap();
}

pub async fn poll_forum_reports(p: &DbPool, date: &NaiveDate, forum_api_key: &str) {

    let date_str = &format!("{}-{}-{}", date.year(), date.month(), date.day());

    let vs = fetch_report("daily_engaged_users", date_str, forum_api_key).await;
    let daily_engaged_users = parse_report_i64(vs).await;
    let result = diesel::sql_query(
        "insert into forum_report_daily_engaged_users(amount, stamp) VALUES ($1, $2);",
    )
    .bind::<BigInt, _>(daily_engaged_users)
    .bind::<Date, _>(date)
    .execute(&p.get().unwrap());
    match result {
        // ignore if already processed, panic otherwise
        Ok(_) => (),
        Err(DatabaseError(UniqueViolation, _)) => (),
        Err(e) => panic!("write to db: {:#?}, err {}", &daily_engaged_users, e),
    };

    let vs = fetch_report("likes", date_str, forum_api_key).await;
    let likes = parse_report_i64(vs).await;
    let result = diesel::sql_query("insert into forum_report_likes(amount, stamp) VALUES ($1, $2);")
        .bind::<BigInt, _>(likes)
        .bind::<Date, _>(date)
        .execute(&p.get().unwrap());
    match result {
        Ok(_) => (),
        Err(DatabaseError(UniqueViolation, _)) => (),
        Err(e) => panic!("write to db: {:#?}, err {}", &likes, e),
    };

    let vs = fetch_report("new_contributors", date_str, forum_api_key).await;
    let new_contributors = parse_report_i64(vs).await;
    let result = diesel::sql_query("insert into forum_report_new_contributors(amount, stamp) VALUES ($1, $2);")
        .bind::<BigInt, _>(new_contributors)
        .bind::<Date, _>(date)
        .execute(&p.get().unwrap());
    match result {
        Ok(_) => (),
        Err(DatabaseError(UniqueViolation, _)) => (),
        Err(e) => panic!("write to db: {:#?}, err {}", &new_contributors, e),
    };

    let vs = fetch_report("page_view_total_reqs", date_str, forum_api_key).await;
    let page_view_total_reqs = parse_report_i64(vs).await;
    let result = diesel::sql_query("insert into forum_report_page_views(amount, stamp) VALUES ($1, $2);")
        .bind::<BigInt, _>(page_view_total_reqs)
        .bind::<Date, _>(date)
        .execute(&p.get().unwrap());
    match result {
        Ok(_) => (),
        Err(DatabaseError(UniqueViolation, _)) => (),
        Err(e) => panic!("write to db: {:#?}, err {}", &page_view_total_reqs, e),
    };

    let vs = fetch_report("posts", date_str, forum_api_key).await;
    let posts = parse_report_i64(vs).await;
    let result = diesel::sql_query("insert into forum_report_posts(amount, stamp) VALUES ($1, $2);")
        .bind::<BigInt, _>(posts)
        .bind::<Date, _>(date)
        .execute(&p.get().unwrap());
    match result {
        Ok(_) => (),
        Err(DatabaseError(UniqueViolation, _)) => (),
        Err(e) => panic!("write to db: {:#?}, err {}", &posts, e),
    };

    let vs = fetch_report("signups", date_str, forum_api_key).await;
    let signups = parse_report_i64(vs).await;
    let result = diesel::sql_query("insert into forum_report_signups(amount, stamp) VALUES ($1, $2);")
        .bind::<BigInt, _>(signups)
        .bind::<Date, _>(date)
        .execute(&p.get().unwrap());
    match result {
        Ok(_) => (),
        Err(DatabaseError(UniqueViolation, _)) => (),
        Err(e) => panic!("write to db: {:#?}, err {}", &signups, e),
    };

    let vs = fetch_report("topics", date_str, forum_api_key).await;
    let topics = parse_report_i64(vs).await;
    let result = diesel::sql_query("insert into forum_report_topics(amount, stamp) VALUES ($1, $2);")
        .bind::<BigInt, _>(topics)
        .bind::<Date, _>(date)
        .execute(&p.get().unwrap());
    match result {
        Ok(_) => (),
        Err(DatabaseError(UniqueViolation, _)) => (),
        Err(e) => panic!("write to db: {:#?}, err {}", &topics, e),
    };

    let vs = fetch_report("visits", date_str, forum_api_key).await;
    let visits = parse_report_i64(vs).await;
    let result = diesel::sql_query("insert into forum_report_visits(amount, stamp) VALUES ($1, $2);")
        .bind::<BigInt, _>(visits)
        .bind::<Date, _>(date)
        .execute(&p.get().unwrap());
    match result {
        Ok(_) => (),
        Err(DatabaseError(UniqueViolation, _)) => (),
        Err(e) => panic!("write to db: {:#?}, err {}", &visits, e),
    };

    let vs = fetch_report("dau_by_mau", date_str, forum_api_key).await;
    let dau_by_mau = parse_report_f64(vs).await;
    let result = diesel::sql_query("insert into forum_report_dau_by_mau(amount, stamp) VALUES ($1, $2);")
        .bind::<Double, _>(dau_by_mau)
        .bind::<Date, _>(date)
        .execute(&p.get().unwrap());
    match result {
        Ok(_) => (),
        Err(DatabaseError(UniqueViolation, _)) => (),
        Err(e) => panic!("write to db: {:#?}, err {}", &dau_by_mau, e),
    };
}
