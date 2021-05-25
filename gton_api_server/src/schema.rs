table! {
    gton_price (id) {
        id -> Int4,
        price -> Float8,
        market_time -> Timestamp,
    }
}

table! {
    voters (id) {
        id -> Int4,
        round_id -> Int4,
        user_address -> Varchar,
        vote_times -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    gton_price,
    voters,
);
