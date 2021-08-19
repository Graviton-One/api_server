table! {
    achievements (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        icon -> Varchar,
    }
}

table! {
    achievements_to_users (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        achievement_id -> Nullable<Int4>,
    }
}

table! {
    gton_price (id) {
        id -> Int4,
        price -> Float8,
        market_time -> Timestamp,
    }
}

table! {
    uni_stats (id) {
        id -> Int4,
        tvl -> Int8,
        volume -> Int8,
        addresses_count -> Int4,
        apy -> Int4,
        date -> Timestamp,
    }
}

table! {
    pollers_data (id) {
        id -> Int4,
        block_id -> Int8,
        poller_id -> Int4,
    }
}

table! {
    total_values_for_users (id) {
        id -> Int4,
        user_id -> Int4,
        sender_id -> Int4,
        amount -> Numeric,
    }
}

table! {
    users (id) {
        id -> Int4,
        address -> Varchar,
        twitter_account -> Nullable<Varchar>,
    }
}

table! {
    value_senders (id) {
        id -> Int4,
        address -> Varchar,
        name -> Varchar,
        amount -> Numeric,
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

table! {
    votings (id) {
        id -> Int4,
        title -> Varchar,
        date_from -> Varchar,
        date_to -> Varchar,
        description -> Text,
        details -> Text,
        proposer -> Varchar,
        forum_link -> Varchar,
        active -> Bool,
    }
}

table! {
    chains (id) {
        id -> BigInt,
        coingecko_id -> Varchar,
        chain_name -> Varchar,
        chain_icon -> Varchar,
        chain_short -> Varchar,
        network_id -> Int4,
        explorer -> Varchar,
        node_url -> Varchar,
        token -> Varchar,
        gton_address -> Varchar,
    }
}
table! {
    dexes (id) {
        id -> BigInt,
        chain_id -> BigInt,
        name -> Varchar,
    }
}
table! {
    pools (id) {
        id -> BigInt,
        dex_id -> BigInt,
        name -> Varchar,
        pool_address -> Varchar,
        swap_link -> Varchar,
        pair_link -> Varchar,
        gton_reserves -> Float8,
        tvl -> Float8,
    }
}

joinable!(achievements_to_users -> achievements (achievement_id));
joinable!(achievements_to_users -> users (user_id));
joinable!(total_values_for_users -> users (user_id));
joinable!(total_values_for_users -> value_senders (sender_id));
joinable!(dexes -> chains (chain_id));
joinable!(pools -> dexes (dex_id));

allow_tables_to_appear_in_same_query!(
    achievements,
    achievements_to_users,
    gton_price,
    pollers_data,
    total_values_for_users,
    users,
    value_senders,
    voters,
);
