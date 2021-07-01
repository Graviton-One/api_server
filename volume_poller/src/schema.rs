// table! {
//     pool_volumes (id) {
//         id -> Int4,
//         uni -> Int8,
//         sushi -> Int8,
//         dodo -> Int8,
//         spirit_ftm -> Int8,
//         spirit_usdc -> Int8,
//         spirit_usdc -> Int8,
//         date -> Timestamp,
//     }
// }

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
    dodo_stats (id) {
        id -> Int4,
        tvl -> Int8,
        volume -> Int8,
        addresses_count -> Int4,
        apy -> Int4,
        date -> Timestamp,
    }
}

table! {
    sushi_stats (id) {
        id -> Int4,
        tvl -> Int8,
        volume -> Int8,
        addresses_count -> Int8,
        apy -> Int4,
        date -> Timestamp,
    }
}

table! {
    spooky_ftm_stats (id) {
        id -> Int4,
        tvl -> Int8,
        volume -> Int8,
        addresses_count -> Int4,
        apy -> Int4,
        date -> Timestamp,
    }
}

table! {
    spooky_usdc_stats (id) {
        id -> Int4,
        tvl -> Int8,
        volume -> Int8,
        addresses_count -> Int4,
        apy -> Int4,
        date -> Timestamp,
    }
}

table! {
    spirit_ftm_stats (id) {
        id -> Int4,
        tvl -> Int8,
        volume -> Int8,
        addresses_count -> Int4,
        apy -> Int4,
        date -> Timestamp,
    }
}

table! {
    spirit_usdc_stats (id) {
        id -> Int4,
        tvl -> Int8,
        volume -> Int8,
        addresses_count -> Int4,
        apy -> Int4,
        date -> Timestamp,
    }
}

table! {
    spirit_fusdt_stats (id) {
        id -> Int4,
        tvl -> Int8,
        volume -> Int8,
        addresses_count -> Int4,
        apy -> Int4,
        date -> Timestamp,
    }
}

table! {
    pancake_busd_stats (id) {
        id -> Int4,
        tvl -> Int8,
        volume -> Int8,
        addresses_count -> Int4,
        apy -> Int4,
        date -> Timestamp,
    }
}

table! {
    pancake_bnb_stats (id) {
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
    gton_price (id) {
        id -> Int4,
        price -> Float8,
        market_time -> Timestamp,
    }
}