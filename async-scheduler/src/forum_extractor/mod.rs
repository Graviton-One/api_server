use chrono::{DateTime, Datelike, Duration, Utc};

use crate::DbPool;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Double, Timestamp};

mod fetchers;
use fetchers::{
    fetch_forum_active_users, fetch_forum_topics, fetch_forum_total_users, fetch_report,
};
mod parsers;
use parsers::{
    parse_forum_active_users, parse_forum_total_posts, parse_forum_total_topics,
    parse_forum_total_users, parse_report_f64, parse_report_i64,
};

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
        poll_forum_reports(&self.pool, &self.forum_api_key).await;
    }
}

pub fn yesterday_utc() -> DateTime<Utc> {
    let now = Utc::now();
    return now.checked_sub_signed(Duration::days(1)).unwrap();
}

pub fn yesterday_str() -> String {
    let then = yesterday_utc();
    return format!("{}-{}-{}", then.year(), then.month(), then.day());
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

pub async fn poll_forum_reports(p: &DbPool, forum_api_key: &str) {
    let vs = fetch_report("dau_by_mau", &yesterday_str(), forum_api_key).await;
    let dau_by_mau = parse_report_f64(vs).await;
    diesel::sql_query("insert into forum_report_dau_by_mau(amount, stamp) VALUES ($1, $2);")
        .bind::<Double, _>(dau_by_mau)
        .bind::<Timestamp, _>(&yesterday_utc().naive_local())
        .execute(&p.get().unwrap())
        .unwrap();

    let vs = fetch_report("daily_engaged_users", &yesterday_str(), forum_api_key).await;
    let daily_engaged_users = parse_report_i64(vs).await;
    diesel::sql_query(
        "insert into forum_report_daily_engaged_users(amount, stamp) VALUES ($1, $2);",
    )
    .bind::<BigInt, _>(daily_engaged_users)
    .bind::<Timestamp, _>(&yesterday_utc().naive_local())
    .execute(&p.get().unwrap())
    .unwrap();

    let vs = fetch_report("likes", &yesterday_str(), forum_api_key).await;
    let likes = parse_report_i64(vs).await;
    diesel::sql_query("insert into forum_report_likes(amount, stamp) VALUES ($1, $2);")
        .bind::<BigInt, _>(likes)
        .bind::<Timestamp, _>(&yesterday_utc().naive_local())
        .execute(&p.get().unwrap())
        .unwrap();

    let vs = fetch_report("new_contributors", &yesterday_str(), forum_api_key).await;
    let new_contributors = parse_report_i64(vs).await;
    diesel::sql_query("insert into forum_report_new_contributors(amount, stamp) VALUES ($1, $2);")
        .bind::<BigInt, _>(new_contributors)
        .bind::<Timestamp, _>(&yesterday_utc().naive_local())
        .execute(&p.get().unwrap())
        .unwrap();

    let vs = fetch_report("page_view_total_reqs", &yesterday_str(), forum_api_key).await;
    let page_view_total_reqs = parse_report_i64(vs).await;
    diesel::sql_query("insert into forum_report_page_views(amount, stamp) VALUES ($1, $2);")
        .bind::<BigInt, _>(page_view_total_reqs)
        .bind::<Timestamp, _>(&yesterday_utc().naive_local())
        .execute(&p.get().unwrap())
        .unwrap();

    let vs = fetch_report("posts", &yesterday_str(), forum_api_key).await;
    let posts = parse_report_i64(vs).await;
    diesel::sql_query("insert into forum_report_posts(amount, stamp) VALUES ($1, $2);")
        .bind::<BigInt, _>(posts)
        .bind::<Timestamp, _>(&yesterday_utc().naive_local())
        .execute(&p.get().unwrap())
        .unwrap();

    let vs = fetch_report("signups", &yesterday_str(), forum_api_key).await;
    let signups = parse_report_i64(vs).await;
    diesel::sql_query("insert into forum_report_signups(amount, stamp) VALUES ($1, $2);")
        .bind::<BigInt, _>(signups)
        .bind::<Timestamp, _>(&yesterday_utc().naive_local())
        .execute(&p.get().unwrap())
        .unwrap();

    let vs = fetch_report("topics", &yesterday_str(), forum_api_key).await;
    let topics = parse_report_i64(vs).await;
    diesel::sql_query("insert into forum_report_topics(amount, stamp) VALUES ($1, $2);")
        .bind::<BigInt, _>(topics)
        .bind::<Timestamp, _>(&yesterday_utc().naive_local())
        .execute(&p.get().unwrap())
        .unwrap();

    let vs = fetch_report("visits", &yesterday_str(), forum_api_key).await;
    let visits = parse_report_i64(vs).await;
    diesel::sql_query("insert into forum_report_visits(amount, stamp) VALUES ($1, $2);")
        .bind::<BigInt, _>(visits)
        .bind::<Timestamp, _>(&yesterday_utc().naive_local())
        .execute(&p.get().unwrap())
        .unwrap();
}
