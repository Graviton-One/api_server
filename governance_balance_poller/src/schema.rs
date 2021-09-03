table! {
    pollers_data (id) {
        id -> Int4,
        block_id -> Int8,
        poller_id -> Int4,
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

table! {
    farm_transactions (id) {
        id -> BigInt,
        farm_id -> BigInt,
        amount -> Numeric,
        tx_type -> Varchar,
        tx_hash -> Varchar,
        stamp -> Timestamp,
        user_address -> Varchar,
    }
}