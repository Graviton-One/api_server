use serde_json::Value;

pub async fn fetch_forum_total_users() -> Value {
    let body: reqwest::Response = reqwest::get(
        "https://forum.graviton.one/directory_items.json?period=all&order=days_visited",
    )
    .await
    .unwrap();
    let body_text: String = body.text().await.unwrap();

    // parse the string of data into serde_json::Value
    serde_json::from_str(&body_text).unwrap()
}

pub async fn fetch_forum_active_users(forum_api_key: &str) -> Value {
    let client = reqwest::Client::new();
    let body = client
        .get("https://forum.graviton.one/admin/users/list/active.json")
        .header("Api-Key", forum_api_key)
        .header("Api-Username", "system")
        .send()
        .await
        .unwrap();
    let body_text: String = body.text().await.unwrap();

    // parse the string of data into a vector of Values
    serde_json::from_str(&body_text).unwrap()
}

pub async fn fetch_forum_topics(apikey: &str) -> Vec<Value> {
    let client = reqwest::Client::new();
    let mut topics: Vec<Value> = vec![];
    let mut index: i32 = 0;
    loop {
        let body = client
            .get(&format!("https://forum.graviton.one/latest?page={}", index))
            .header("Accept", "application/json")
            .header("Api-Key", apikey)
            .header("Api-Username", "system")
            .send()
            .await
            .unwrap();
        let body_text: String = body.text().await.unwrap();

        let v: Value = serde_json::from_str(&body_text).unwrap();

        let length = topics.len() as i64;

        let topics_new: &Vec<Value> = v["topic_list"]["topics"].as_array().unwrap();

        topics.append(&mut topics_new.clone());

        let max: i64 = v["topic_list"]["per_page"].as_i64().unwrap();

        if length < max {
            break;
        }

        index += 1;
    }
    topics
}

pub async fn fetch_report(report: &str, date: &str, apikey: &str) -> Vec<Value> {
    let client = reqwest::Client::new();
    let url: String = format!(
        "https://forum.graviton.one/admin/reports/{}.json?end_date={}&start_date={}",
        report, date, date
    );
    let body = client
        .get(&url)
        .header("Api-Key", apikey)
        .header("Api-Username", "system")
        .send()
        .await
        .unwrap();
    let body_text: String = body.text().await.unwrap();

    // parse the string of data into a vector of Values
    let v: Value = serde_json::from_str(&body_text).expect(&format!("{}, {}", report, date));

    let data: Vec<Value> = v["report"]["data"].as_array().unwrap().clone();

    data
}
