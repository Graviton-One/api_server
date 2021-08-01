use serde_json::Value;

pub async fn parse_forum_total_users(v: Value) -> i64 {
    let n: i64 = v["meta"]["total_rows_directory_items"].as_i64().unwrap();

    n
}

pub async fn parse_forum_active_users(v: Value) -> i64 {
    let users: &Vec<Value> = v.as_array().unwrap();

    users.len() as i64
}

pub async fn parse_forum_total_topics(topics: &Vec<Value>) -> i64 {
    topics.len() as i64
}

pub async fn parse_forum_total_posts(topics: &Vec<Value>) -> i64 {
    let mut count: i64 = 0;
    for topic in topics.into_iter() {
        let posts_count = topic["posts_count"].as_i64().unwrap();
        count += posts_count;
    }
    count
}

pub async fn parse_report_f64(vs: Vec<Value>) -> f64 {
    if vs.len() == 0 {
        return 0.0;
    } else {
        return vs[0]["y"].as_f64().unwrap();
    }
}

pub async fn parse_report_i64(vs: Vec<Value>) -> i64 {
    if vs.len() == 0 {
        return 0;
    } else {
        return vs[0]["y"].as_i64().unwrap();
    }
}
