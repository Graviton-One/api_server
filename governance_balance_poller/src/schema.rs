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
    gton_farms (id) {
        id -> BigInt,
        pool_id -> BigInt,
        allocation -> Int4,
        farmed -> Float8,
        assigned -> Float8,
        apy -> Float8,
        active -> Bool,
        addresses_in -> Int4,
        lp_price -> Float8,
        lock_address -> Varchar,
        farm_linear -> Varchar,
        token_id -> Int4,
    }
}

table! {
    gton_price (id) {
        id -> Int4,
        price -> Float8,
        market_time -> Timestamp,
    }
}