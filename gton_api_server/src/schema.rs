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
    blocks (id) {
        id -> Int8,
        name_table -> Text,
        block_number -> Int8,
    }
}

table! {
    chains (id) {
        id -> Int8,
        chain_name -> Varchar,
        chain_icon -> Varchar,
        chain_short -> Varchar,
        network_id -> Numeric,
        coingecko_id -> Varchar,
        explorer -> Varchar,
        node_url -> Varchar,
        token -> Varchar,
        gton_address -> Varchar,
    }
}

table! {
    dexes (id) {
        id -> Int8,
        chain_id -> Nullable<Int8>,
        name -> Varchar,
        image -> Varchar,
        small_image -> Varchar,
    }
}

table! {
    events_anyv4_swapin_bsc (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        account -> Text,
        amount -> Numeric,
        transfer_tx_hash -> Text,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_anyv4_swapin_ftm (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        account -> Text,
        amount -> Numeric,
        transfer_tx_hash -> Text,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_anyv4_swapin_ftm_bk (id) {
        id -> Int8,
        account -> Text,
        amount -> Numeric,
        transfer_tx_hash -> Text,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_anyv4_swapin_plg (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        account -> Text,
        amount -> Numeric,
        transfer_tx_hash -> Text,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_anyv4_swapout_bsc (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        account -> Text,
        bindaddr -> Text,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_anyv4_swapout_ftm (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        account -> Text,
        bindaddr -> Text,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_anyv4_swapout_ftm_bk (id) {
        id -> Int8,
        account -> Text,
        bindaddr -> Text,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_anyv4_swapout_plg (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        account -> Text,
        bindaddr -> Text,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_anyv4_transfer_bk (id) {
        id -> Int8,
        sender -> Text,
        receiver -> Text,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_anyv4_transfer_eth (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        sender -> Text,
        receiver -> Text,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_balance_keeper_add (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        adder -> Text,
        user_id -> Numeric,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_balance_keeper_open_user (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        opener -> Text,
        user_id -> Numeric,
        user_chain -> Text,
        user_address -> Text,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_balance_keeper_subtract (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        subtractor -> Text,
        user_id -> Numeric,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_erc20_approval_bsc (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        owner -> Text,
        spender -> Text,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_erc20_approval_eth (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        owner -> Text,
        spender -> Text,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_erc20_approval_ftm (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        owner -> Text,
        spender -> Text,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_erc20_approval_ftm_bk (id) {
        id -> Int8,
        owner -> Text,
        spender -> Text,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_erc20_approval_plg (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        owner -> Text,
        spender -> Text,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_erc20_transfer_bsc (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        sender -> Text,
        receiver -> Text,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_erc20_transfer_eth (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        sender -> Text,
        receiver -> Text,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_erc20_transfer_ftm (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        sender -> Text,
        receiver -> Text,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_erc20_transfer_ftm_bk (id) {
        id -> Int8,
        sender -> Text,
        receiver -> Text,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_erc20_transfer_plg (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        sender -> Text,
        receiver -> Text,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_lp_keeper_add (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        adder -> Text,
        token_id -> Numeric,
        user_id -> Numeric,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_lp_keeper_subtract (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        subtractor -> Text,
        token_id -> Numeric,
        user_id -> Numeric,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_burn_bsc_pancake (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        pair_id -> Nullable<Int8>,
        sender -> Text,
        receiver -> Text,
        amount0 -> Numeric,
        amount1 -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_burn_eth_sushi (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        pair_id -> Nullable<Int8>,
        sender -> Text,
        receiver -> Text,
        amount0 -> Numeric,
        amount1 -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_burn_ftm_spirit (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        pair_id -> Nullable<Int8>,
        sender -> Text,
        receiver -> Text,
        amount0 -> Numeric,
        amount1 -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_burn_ftm_spooky (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        pair_id -> Nullable<Int8>,
        sender -> Text,
        receiver -> Text,
        amount0 -> Numeric,
        amount1 -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_burn_plg_quick (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        pair_id -> Nullable<Int8>,
        sender -> Text,
        receiver -> Text,
        amount0 -> Numeric,
        amount1 -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_burn_plg_sushi (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        pair_id -> Nullable<Int8>,
        sender -> Text,
        receiver -> Text,
        amount0 -> Numeric,
        amount1 -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_burn_spirit_bk (id) {
        id -> Int8,
        pair -> Nullable<Int8>,
        sender -> Text,
        receiver -> Text,
        amount0 -> Numeric,
        amount1 -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_mint_bsc_pancake (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        pair_id -> Nullable<Int8>,
        sender -> Text,
        amount0 -> Numeric,
        amount1 -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_mint_eth_sushi (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        pair_id -> Nullable<Int8>,
        sender -> Text,
        amount0 -> Numeric,
        amount1 -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_mint_ftm_spirit (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        pair_id -> Nullable<Int8>,
        sender -> Text,
        amount0 -> Numeric,
        amount1 -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_mint_ftm_spooky (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        pair_id -> Nullable<Int8>,
        sender -> Text,
        amount0 -> Numeric,
        amount1 -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_mint_plg_quick (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        pair_id -> Nullable<Int8>,
        sender -> Text,
        amount0 -> Numeric,
        amount1 -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_mint_plg_sushi (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        pair_id -> Nullable<Int8>,
        sender -> Text,
        amount0 -> Numeric,
        amount1 -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_mint_spirit_bk (id) {
        id -> Int8,
        pair -> Nullable<Int8>,
        sender -> Text,
        amount0 -> Numeric,
        amount1 -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_pair_created_bsc_pancake (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        address -> Text,
        token0 -> Text,
        token1 -> Text,
        gtontoken0 -> Bool,
        title -> Text,
        index -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_pair_created_eth_sushi (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        address -> Text,
        token0 -> Text,
        token1 -> Text,
        gtontoken0 -> Bool,
        title -> Text,
        index -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_pair_created_ftm_spirit (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        address -> Text,
        token0 -> Text,
        token1 -> Text,
        gtontoken0 -> Bool,
        title -> Text,
        index -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_pair_created_ftm_spooky (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        address -> Text,
        token0 -> Text,
        token1 -> Text,
        gtontoken0 -> Bool,
        title -> Text,
        index -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_pair_created_plg_quick (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        address -> Text,
        token0 -> Text,
        token1 -> Text,
        gtontoken0 -> Bool,
        title -> Text,
        index -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_pair_created_plg_sushi (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        address -> Text,
        token0 -> Text,
        token1 -> Text,
        gtontoken0 -> Bool,
        title -> Text,
        index -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_pair_created_spirit_bk (id) {
        id -> Int8,
        address -> Text,
        token0 -> Text,
        token1 -> Text,
        index -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_swap_bsc_pancake (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        pair_id -> Nullable<Int8>,
        sender -> Text,
        receiver -> Text,
        amount0_in -> Numeric,
        amount1_in -> Numeric,
        amount0_out -> Numeric,
        amount1_out -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_swap_eth_sushi (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        pair_id -> Nullable<Int8>,
        sender -> Text,
        receiver -> Text,
        amount0_in -> Numeric,
        amount1_in -> Numeric,
        amount0_out -> Numeric,
        amount1_out -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_swap_ftm_spirit (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        pair_id -> Nullable<Int8>,
        sender -> Text,
        receiver -> Text,
        amount0_in -> Numeric,
        amount1_in -> Numeric,
        amount0_out -> Numeric,
        amount1_out -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_swap_ftm_spooky (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        pair_id -> Nullable<Int8>,
        sender -> Text,
        receiver -> Text,
        amount0_in -> Numeric,
        amount1_in -> Numeric,
        amount0_out -> Numeric,
        amount1_out -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_swap_plg_quick (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        pair_id -> Nullable<Int8>,
        sender -> Text,
        receiver -> Text,
        amount0_in -> Numeric,
        amount1_in -> Numeric,
        amount0_out -> Numeric,
        amount1_out -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_swap_spirit_bk (id) {
        id -> Int8,
        pair -> Nullable<Int8>,
        sender -> Text,
        receiver -> Text,
        amount0_in -> Numeric,
        amount1_in -> Numeric,
        amount0_out -> Numeric,
        amount1_out -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_transfer_bsc_pancake (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        pair_id -> Nullable<Int8>,
        sender -> Text,
        receiver -> Text,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_transfer_eth_sushi (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        pair_id -> Nullable<Int8>,
        sender -> Text,
        receiver -> Text,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_transfer_ftm_spirit (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        pair_id -> Nullable<Int8>,
        sender -> Text,
        receiver -> Text,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_transfer_ftm_spooky (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        pair_id -> Nullable<Int8>,
        sender -> Text,
        receiver -> Text,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    events_univ2_transfer_plg_quick (id) {
        id -> Int8,
        tx_from -> Text,
        tx_to -> Text,
        pair_id -> Nullable<Int8>,
        sender -> Text,
        receiver -> Text,
        amount -> Numeric,
        stamp -> Timestamp,
        block_number -> Int8,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    forum_active_users (id) {
        id -> Int8,
        amount -> Int8,
        stamp -> Timestamp,
    }
}

table! {
    forum_report_daily_engaged_users (id) {
        id -> Int8,
        amount -> Int8,
        stamp -> Date,
    }
}

table! {
    forum_report_dau_by_mau (id) {
        id -> Int8,
        amount -> Float8,
        stamp -> Date,
    }
}

table! {
    forum_report_likes (id) {
        id -> Int8,
        amount -> Int8,
        stamp -> Date,
    }
}

table! {
    forum_report_new_contributors (id) {
        id -> Int8,
        amount -> Int8,
        stamp -> Date,
    }
}

table! {
    forum_report_page_views (id) {
        id -> Int8,
        amount -> Int8,
        stamp -> Date,
    }
}

table! {
    forum_report_posts (id) {
        id -> Int8,
        amount -> Int8,
        stamp -> Date,
    }
}

table! {
    forum_report_signups (id) {
        id -> Int8,
        amount -> Int8,
        stamp -> Date,
    }
}

table! {
    forum_report_topics (id) {
        id -> Int8,
        amount -> Int8,
        stamp -> Date,
    }
}

table! {
    forum_report_visits (id) {
        id -> Int8,
        amount -> Int8,
        stamp -> Date,
    }
}

table! {
    forum_total_posts (id) {
        id -> Int8,
        amount -> Int8,
        stamp -> Timestamp,
    }
}

table! {
    forum_total_topics (id) {
        id -> Int8,
        amount -> Int8,
        stamp -> Timestamp,
    }
}

table! {
    forum_total_users (id) {
        id -> Int8,
        amount -> Int8,
        stamp -> Timestamp,
    }
}

table! {
    gton_pools (id) {
        id -> Int8,
        pool_address -> Varchar,
        name -> Varchar,
        swap_link -> Varchar,
        pair_link -> Varchar,
        gton_reserves -> Numeric,
        second_token_reserves -> Numeric,
        second_token_name -> Varchar,
        tvl -> Numeric,
        dex_id -> Nullable<Int8>,
    }
}

table! {
    gton_pools_chains (id) {
        id -> Int8,
        chain_name -> Varchar,
        chain_icon -> Varchar,
        chain_short -> Varchar,
        network_id -> Numeric,
        explorer -> Varchar,
        node_url -> Varchar,
        token -> Varchar,
        gton_address -> Varchar,
    }
}

table! {
    gton_pools_dexes (id) {
        id -> Int8,
        name -> Varchar,
        chain_id -> Nullable<Int8>,
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
    pollers_data (id) {
        id -> Int4,
        block_id -> Int8,
        poller_id -> Int4,
    }
}

table! {
    pools (id) {
        id -> Int8,
        pool_address -> Varchar,
        name -> Varchar,
        image -> Varchar,
        swap_link -> Varchar,
        pair_link -> Varchar,
        gton_reserves -> Float8,
        tvl -> Float8,
        dex_id -> Nullable<Int8>,
        token_pair_image -> String,
    }
}

table! {
    refferal_offers (id) {
        id -> Int8,
        pool_address -> Varchar,
        signature -> Varchar,
        secure_id -> Uuid,
    }
}

table! {
    total_staked (id) {
        id -> Int8,
        amount -> Numeric,
        stamp -> Timestamp,
    }
}

table! {
    total_stakers (id) {
        id -> Int8,
        amount -> Numeric,
        stamp -> Timestamp,
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
    univ2_buy_amount_daily_eth (id) {
        stamp -> Nullable<Date>,
        sum -> Nullable<Numeric>,
        id -> Int8,
    }
}

table! {
    univ2_buy_amount_daily_other (id) {
        id -> Int8,
        stamp -> Nullable<Date>,
        sum -> Nullable<Numeric>,
    }
}

table! {
    univ2_buy_bsc_pancake (id) {
        id -> Int8,
        swap_id -> Nullable<Int8>,
        pair_id -> Nullable<Int8>,
        pair_title -> Text,
        tx_from -> Text,
        amount_token_in -> Numeric,
        amount_gton_out -> Numeric,
        stamp -> Timestamp,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    univ2_buy_eth_sushi (id) {
        id -> Int8,
        swap_id -> Nullable<Int8>,
        pair_id -> Nullable<Int8>,
        pair_title -> Text,
        tx_from -> Text,
        amount_token_in -> Numeric,
        amount_gton_out -> Numeric,
        stamp -> Timestamp,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    univ2_buy_eth_sushi_bk (id) {
        id -> Int8,
        swap_id -> Nullable<Int8>,
        pair_id -> Nullable<Int8>,
        pair_title -> Text,
        tx_from -> Text,
        amount_token_in -> Numeric,
        amount_gton_out -> Numeric,
        stamp -> Timestamp,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    univ2_buy_ftm_amount_daily_eth (id) {
        stamp -> Nullable<Date>,
        sum -> Nullable<Numeric>,
        id -> Int8,
    }
}

table! {
    univ2_buy_ftm_spirit (id) {
        id -> Int8,
        swap_id -> Nullable<Int8>,
        pair_id -> Nullable<Int8>,
        pair_title -> Text,
        tx_from -> Text,
        amount_token_in -> Numeric,
        amount_gton_out -> Numeric,
        stamp -> Timestamp,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    univ2_buy_ftm_spooky (id) {
        id -> Int8,
        swap_id -> Nullable<Int8>,
        pair_id -> Nullable<Int8>,
        pair_title -> Text,
        tx_from -> Text,
        amount_token_in -> Numeric,
        amount_gton_out -> Numeric,
        stamp -> Timestamp,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    univ2_buy_plg_quick (id) {
        id -> Int8,
        swap_id -> Nullable<Int8>,
        pair_id -> Nullable<Int8>,
        pair_title -> Text,
        tx_from -> Text,
        amount_token_in -> Numeric,
        amount_gton_out -> Numeric,
        stamp -> Timestamp,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    univ2_buyers_running_count_eth (id) {
        day -> Nullable<Date>,
        users -> Nullable<Int8>,
        id -> Int8,
    }
}

table! {
    univ2_buyers_running_count_other (id) {
        id -> Int8,
        day -> Nullable<Date>,
        users -> Nullable<Int8>,
    }
}

table! {
    univ2_lp_add_bsc_pancake (id) {
        id -> Int8,
        mint_id -> Nullable<Int8>,
        pair_id -> Nullable<Int8>,
        pair_title -> Text,
        tx_from -> Text,
        amount_gton_in -> Numeric,
        amount_token_in -> Numeric,
        amount_lp_out -> Nullable<Numeric>,
        stamp -> Timestamp,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    univ2_lp_add_eth_sushi (id) {
        id -> Int8,
        mint_id -> Nullable<Int8>,
        pair_id -> Nullable<Int8>,
        pair_title -> Text,
        tx_from -> Text,
        amount_gton_in -> Numeric,
        amount_token_in -> Numeric,
        amount_lp_out -> Nullable<Numeric>,
        stamp -> Timestamp,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    univ2_lp_add_ftm_spirit (id) {
        id -> Int8,
        mint_id -> Nullable<Int8>,
        pair_id -> Nullable<Int8>,
        pair_title -> Text,
        tx_from -> Text,
        amount_gton_in -> Numeric,
        amount_token_in -> Numeric,
        amount_lp_out -> Nullable<Numeric>,
        stamp -> Timestamp,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    univ2_lp_add_ftm_spooky (id) {
        id -> Int8,
        mint_id -> Nullable<Int8>,
        pair_id -> Nullable<Int8>,
        pair_title -> Text,
        tx_from -> Text,
        amount_gton_in -> Numeric,
        amount_token_in -> Numeric,
        amount_lp_out -> Nullable<Numeric>,
        stamp -> Timestamp,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    univ2_lp_add_plg_quick (id) {
        id -> Int8,
        mint_id -> Nullable<Int8>,
        pair_id -> Nullable<Int8>,
        pair_title -> Text,
        tx_from -> Text,
        amount_gton_in -> Numeric,
        amount_token_in -> Numeric,
        amount_lp_out -> Nullable<Numeric>,
        stamp -> Timestamp,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    univ2_lp_remove_bsc_pancake (id) {
        id -> Int8,
        burn_id -> Nullable<Int8>,
        pair_id -> Nullable<Int8>,
        pair_title -> Text,
        tx_from -> Text,
        amount_gton_out -> Numeric,
        amount_token_out -> Numeric,
        amount_lp_in -> Nullable<Numeric>,
        stamp -> Timestamp,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    univ2_lp_remove_eth_sushi (id) {
        id -> Int8,
        burn_id -> Nullable<Int8>,
        pair_id -> Nullable<Int8>,
        pair_title -> Text,
        tx_from -> Text,
        amount_gton_out -> Numeric,
        amount_token_out -> Numeric,
        amount_lp_in -> Nullable<Numeric>,
        stamp -> Timestamp,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    univ2_lp_remove_ftm_spirit (id) {
        id -> Int8,
        burn_id -> Nullable<Int8>,
        pair_id -> Nullable<Int8>,
        pair_title -> Text,
        tx_from -> Text,
        amount_gton_out -> Numeric,
        amount_token_out -> Numeric,
        amount_lp_in -> Nullable<Numeric>,
        stamp -> Timestamp,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    univ2_lp_remove_ftm_spooky (id) {
        id -> Int8,
        burn_id -> Nullable<Int8>,
        pair_id -> Nullable<Int8>,
        pair_title -> Text,
        tx_from -> Text,
        amount_gton_out -> Numeric,
        amount_token_out -> Numeric,
        amount_lp_in -> Nullable<Numeric>,
        stamp -> Timestamp,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    univ2_lp_remove_plg_quick (id) {
        id -> Int8,
        burn_id -> Nullable<Int8>,
        pair_id -> Nullable<Int8>,
        pair_title -> Text,
        tx_from -> Text,
        amount_gton_out -> Numeric,
        amount_token_out -> Numeric,
        amount_lp_in -> Nullable<Numeric>,
        stamp -> Timestamp,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    univ2_sell_amount_daily_eth (id) {
        stamp -> Nullable<Date>,
        sum -> Nullable<Numeric>,
        id -> Int8,
    }
}

table! {
    univ2_sell_amount_daily_other (id) {
        id -> Int8,
        stamp -> Nullable<Date>,
        sum -> Nullable<Numeric>,
    }
}

table! {
    univ2_sell_bsc_pancake (id) {
        id -> Int8,
        swap_id -> Nullable<Int8>,
        pair_id -> Nullable<Int8>,
        pair_title -> Text,
        tx_from -> Text,
        amount_gton_in -> Numeric,
        amount_token_out -> Numeric,
        stamp -> Timestamp,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    univ2_sell_eth_sushi (id) {
        id -> Int8,
        swap_id -> Nullable<Int8>,
        pair_id -> Nullable<Int8>,
        pair_title -> Text,
        tx_from -> Text,
        amount_gton_in -> Numeric,
        amount_token_out -> Numeric,
        stamp -> Timestamp,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    univ2_sell_eth_sushi_bk (id) {
        id -> Int8,
        swap_id -> Nullable<Int8>,
        pair_id -> Nullable<Int8>,
        pair_title -> Text,
        tx_from -> Text,
        amount_gton_in -> Numeric,
        amount_token_out -> Numeric,
        stamp -> Timestamp,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    univ2_sell_ftm_spirit (id) {
        id -> Int8,
        swap_id -> Nullable<Int8>,
        pair_id -> Nullable<Int8>,
        pair_title -> Text,
        tx_from -> Text,
        amount_gton_in -> Numeric,
        amount_token_out -> Numeric,
        stamp -> Timestamp,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    univ2_sell_ftm_spooky (id) {
        id -> Int8,
        swap_id -> Nullable<Int8>,
        pair_id -> Nullable<Int8>,
        pair_title -> Text,
        tx_from -> Text,
        amount_gton_in -> Numeric,
        amount_token_out -> Numeric,
        stamp -> Timestamp,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    univ2_sell_plg_quick (id) {
        id -> Int8,
        swap_id -> Nullable<Int8>,
        pair_id -> Nullable<Int8>,
        pair_title -> Text,
        tx_from -> Text,
        amount_gton_in -> Numeric,
        amount_token_out -> Numeric,
        stamp -> Timestamp,
        tx_hash -> Text,
        log_index -> Int8,
    }
}

table! {
    univ2_sellers_running_count_eth (id) {
        day -> Nullable<Date>,
        users -> Nullable<Int8>,
        id -> Int8,
    }
}

table! {
    univ2_sellers_running_count_other (id) {
        id -> Int8,
        day -> Nullable<Date>,
        users -> Nullable<Int8>,
    }
}

table! {
    users (id) {
        id -> Int4,
        address -> Varchar,
        twitter_account -> Nullable<Varchar>,
        external_address -> Nullable<Varchar>,
        chain_type -> Nullable<Varchar>,
        governance_balance -> Numeric,
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
        details -> Varchar,
        proposer -> Varchar,
        forum_link -> Varchar,
        active -> Bool,
    }
}

joinable!(achievements_to_users -> achievements (achievement_id));
joinable!(achievements_to_users -> users (user_id));
joinable!(dexes -> chains (chain_id));
joinable!(events_univ2_burn_bsc_pancake -> events_univ2_pair_created_bsc_pancake (pair_id));
joinable!(events_univ2_burn_eth_sushi -> events_univ2_pair_created_eth_sushi (pair_id));
joinable!(events_univ2_burn_ftm_spirit -> events_univ2_pair_created_ftm_spirit (pair_id));
joinable!(events_univ2_burn_ftm_spooky -> events_univ2_pair_created_ftm_spooky (pair_id));
joinable!(events_univ2_burn_plg_quick -> events_univ2_pair_created_plg_quick (pair_id));
joinable!(events_univ2_burn_plg_sushi -> events_univ2_pair_created_plg_sushi (pair_id));
joinable!(events_univ2_burn_spirit_bk -> events_univ2_pair_created_spirit_bk (pair));
joinable!(events_univ2_mint_bsc_pancake -> events_univ2_pair_created_bsc_pancake (pair_id));
joinable!(events_univ2_mint_eth_sushi -> events_univ2_pair_created_eth_sushi (pair_id));
joinable!(events_univ2_mint_ftm_spirit -> events_univ2_pair_created_ftm_spirit (pair_id));
joinable!(events_univ2_mint_ftm_spooky -> events_univ2_pair_created_ftm_spooky (pair_id));
joinable!(events_univ2_mint_plg_quick -> events_univ2_pair_created_plg_quick (pair_id));
joinable!(events_univ2_mint_plg_sushi -> events_univ2_pair_created_plg_sushi (pair_id));
joinable!(events_univ2_mint_spirit_bk -> events_univ2_pair_created_spirit_bk (pair));
joinable!(events_univ2_swap_bsc_pancake -> events_univ2_pair_created_bsc_pancake (pair_id));
joinable!(events_univ2_swap_eth_sushi -> events_univ2_pair_created_eth_sushi (pair_id));
joinable!(events_univ2_swap_ftm_spirit -> events_univ2_pair_created_ftm_spirit (pair_id));
joinable!(events_univ2_swap_ftm_spooky -> events_univ2_pair_created_ftm_spooky (pair_id));
joinable!(events_univ2_swap_plg_quick -> events_univ2_pair_created_plg_quick (pair_id));
joinable!(events_univ2_swap_spirit_bk -> events_univ2_pair_created_spirit_bk (pair));
joinable!(events_univ2_transfer_bsc_pancake -> events_univ2_pair_created_bsc_pancake (pair_id));
joinable!(events_univ2_transfer_eth_sushi -> events_univ2_pair_created_eth_sushi (pair_id));
joinable!(events_univ2_transfer_ftm_spirit -> events_univ2_pair_created_ftm_spirit (pair_id));
joinable!(events_univ2_transfer_ftm_spooky -> events_univ2_pair_created_ftm_spooky (pair_id));
joinable!(events_univ2_transfer_plg_quick -> events_univ2_pair_created_plg_quick (pair_id));
joinable!(gton_pools -> gton_pools_dexes (dex_id));
joinable!(gton_pools_dexes -> gton_pools_chains (chain_id));
joinable!(pools -> dexes (dex_id));
joinable!(total_values_for_users -> users (user_id));
joinable!(total_values_for_users -> value_senders (sender_id));
joinable!(univ2_buy_bsc_pancake -> events_univ2_pair_created_bsc_pancake (pair_id));
joinable!(univ2_buy_bsc_pancake -> events_univ2_swap_bsc_pancake (swap_id));
joinable!(univ2_buy_eth_sushi -> events_univ2_pair_created_eth_sushi (pair_id));
joinable!(univ2_buy_eth_sushi -> events_univ2_swap_eth_sushi (swap_id));
joinable!(univ2_buy_eth_sushi_bk -> events_univ2_pair_created_eth_sushi (pair_id));
joinable!(univ2_buy_eth_sushi_bk -> events_univ2_swap_eth_sushi (swap_id));
joinable!(univ2_buy_ftm_spirit -> events_univ2_pair_created_ftm_spirit (pair_id));
joinable!(univ2_buy_ftm_spirit -> events_univ2_swap_ftm_spirit (swap_id));
joinable!(univ2_buy_ftm_spooky -> events_univ2_pair_created_ftm_spooky (pair_id));
joinable!(univ2_buy_ftm_spooky -> events_univ2_swap_ftm_spooky (swap_id));
joinable!(univ2_buy_plg_quick -> events_univ2_pair_created_plg_quick (pair_id));
joinable!(univ2_buy_plg_quick -> events_univ2_swap_plg_quick (swap_id));
joinable!(univ2_lp_add_bsc_pancake -> events_univ2_mint_bsc_pancake (mint_id));
joinable!(univ2_lp_add_bsc_pancake -> events_univ2_pair_created_bsc_pancake (pair_id));
joinable!(univ2_lp_add_eth_sushi -> events_univ2_mint_eth_sushi (mint_id));
joinable!(univ2_lp_add_eth_sushi -> events_univ2_pair_created_eth_sushi (pair_id));
joinable!(univ2_lp_add_ftm_spirit -> events_univ2_mint_ftm_spirit (mint_id));
joinable!(univ2_lp_add_ftm_spirit -> events_univ2_pair_created_ftm_spirit (pair_id));
joinable!(univ2_lp_add_ftm_spooky -> events_univ2_mint_ftm_spooky (mint_id));
joinable!(univ2_lp_add_ftm_spooky -> events_univ2_pair_created_ftm_spooky (pair_id));
joinable!(univ2_lp_add_plg_quick -> events_univ2_mint_plg_quick (mint_id));
joinable!(univ2_lp_add_plg_quick -> events_univ2_pair_created_plg_quick (pair_id));
joinable!(univ2_lp_remove_bsc_pancake -> events_univ2_burn_bsc_pancake (burn_id));
joinable!(univ2_lp_remove_bsc_pancake -> events_univ2_pair_created_bsc_pancake (pair_id));
joinable!(univ2_lp_remove_eth_sushi -> events_univ2_burn_eth_sushi (burn_id));
joinable!(univ2_lp_remove_eth_sushi -> events_univ2_pair_created_eth_sushi (pair_id));
joinable!(univ2_lp_remove_ftm_spirit -> events_univ2_burn_ftm_spirit (burn_id));
joinable!(univ2_lp_remove_ftm_spirit -> events_univ2_pair_created_ftm_spirit (pair_id));
joinable!(univ2_lp_remove_ftm_spooky -> events_univ2_burn_ftm_spooky (burn_id));
joinable!(univ2_lp_remove_ftm_spooky -> events_univ2_pair_created_ftm_spooky (pair_id));
joinable!(univ2_lp_remove_plg_quick -> events_univ2_burn_plg_quick (burn_id));
joinable!(univ2_lp_remove_plg_quick -> events_univ2_pair_created_plg_quick (pair_id));
joinable!(univ2_sell_bsc_pancake -> events_univ2_pair_created_bsc_pancake (pair_id));
joinable!(univ2_sell_bsc_pancake -> events_univ2_swap_bsc_pancake (swap_id));
joinable!(univ2_sell_eth_sushi -> events_univ2_pair_created_eth_sushi (pair_id));
joinable!(univ2_sell_eth_sushi -> events_univ2_swap_eth_sushi (swap_id));
joinable!(univ2_sell_eth_sushi_bk -> events_univ2_pair_created_eth_sushi (pair_id));
joinable!(univ2_sell_eth_sushi_bk -> events_univ2_swap_eth_sushi (swap_id));
joinable!(univ2_sell_ftm_spirit -> events_univ2_pair_created_ftm_spirit (pair_id));
joinable!(univ2_sell_ftm_spirit -> events_univ2_swap_ftm_spirit (swap_id));
joinable!(univ2_sell_ftm_spooky -> events_univ2_pair_created_ftm_spooky (pair_id));
joinable!(univ2_sell_ftm_spooky -> events_univ2_swap_ftm_spooky (swap_id));
joinable!(univ2_sell_plg_quick -> events_univ2_pair_created_plg_quick (pair_id));
joinable!(univ2_sell_plg_quick -> events_univ2_swap_plg_quick (swap_id));

allow_tables_to_appear_in_same_query!(
    achievements,
    achievements_to_users,
    blocks,
    chains,
    dexes,
    events_anyv4_swapin_bsc,
    events_anyv4_swapin_ftm,
    events_anyv4_swapin_ftm_bk,
    events_anyv4_swapin_plg,
    events_anyv4_swapout_bsc,
    events_anyv4_swapout_ftm,
    events_anyv4_swapout_ftm_bk,
    events_anyv4_swapout_plg,
    events_anyv4_transfer_bk,
    events_anyv4_transfer_eth,
    events_balance_keeper_add,
    events_balance_keeper_open_user,
    events_balance_keeper_subtract,
    events_erc20_approval_bsc,
    events_erc20_approval_eth,
    events_erc20_approval_ftm,
    events_erc20_approval_ftm_bk,
    events_erc20_approval_plg,
    events_erc20_transfer_bsc,
    events_erc20_transfer_eth,
    events_erc20_transfer_ftm,
    events_erc20_transfer_ftm_bk,
    events_erc20_transfer_plg,
    events_lp_keeper_add,
    events_lp_keeper_subtract,
    events_univ2_burn_bsc_pancake,
    events_univ2_burn_eth_sushi,
    events_univ2_burn_ftm_spirit,
    events_univ2_burn_ftm_spooky,
    events_univ2_burn_plg_quick,
    events_univ2_burn_plg_sushi,
    events_univ2_burn_spirit_bk,
    events_univ2_mint_bsc_pancake,
    events_univ2_mint_eth_sushi,
    events_univ2_mint_ftm_spirit,
    events_univ2_mint_ftm_spooky,
    events_univ2_mint_plg_quick,
    events_univ2_mint_plg_sushi,
    events_univ2_mint_spirit_bk,
    events_univ2_pair_created_bsc_pancake,
    events_univ2_pair_created_eth_sushi,
    events_univ2_pair_created_ftm_spirit,
    events_univ2_pair_created_ftm_spooky,
    events_univ2_pair_created_plg_quick,
    events_univ2_pair_created_plg_sushi,
    events_univ2_pair_created_spirit_bk,
    events_univ2_swap_bsc_pancake,
    events_univ2_swap_eth_sushi,
    events_univ2_swap_ftm_spirit,
    events_univ2_swap_ftm_spooky,
    events_univ2_swap_plg_quick,
    events_univ2_swap_spirit_bk,
    events_univ2_transfer_bsc_pancake,
    events_univ2_transfer_eth_sushi,
    events_univ2_transfer_ftm_spirit,
    events_univ2_transfer_ftm_spooky,
    events_univ2_transfer_plg_quick,
    forum_active_users,
    forum_report_daily_engaged_users,
    forum_report_dau_by_mau,
    forum_report_likes,
    forum_report_new_contributors,
    forum_report_page_views,
    forum_report_posts,
    forum_report_signups,
    forum_report_topics,
    forum_report_visits,
    forum_total_posts,
    forum_total_topics,
    forum_total_users,
    gton_pools,
    gton_pools_chains,
    gton_pools_dexes,
    gton_price,
    pollers_data,
    pools,
    refferal_offers,
    total_staked,
    total_stakers,
    total_values_for_users,
    univ2_buy_amount_daily_eth,
    univ2_buy_amount_daily_other,
    univ2_buy_bsc_pancake,
    univ2_buy_eth_sushi,
    univ2_buy_eth_sushi_bk,
    univ2_buy_ftm_amount_daily_eth,
    univ2_buy_ftm_spirit,
    univ2_buy_ftm_spooky,
    univ2_buy_plg_quick,
    univ2_buyers_running_count_eth,
    univ2_buyers_running_count_other,
    univ2_lp_add_bsc_pancake,
    univ2_lp_add_eth_sushi,
    univ2_lp_add_ftm_spirit,
    univ2_lp_add_ftm_spooky,
    univ2_lp_add_plg_quick,
    univ2_lp_remove_bsc_pancake,
    univ2_lp_remove_eth_sushi,
    univ2_lp_remove_ftm_spirit,
    univ2_lp_remove_ftm_spooky,
    univ2_lp_remove_plg_quick,
    univ2_sell_amount_daily_eth,
    univ2_sell_amount_daily_other,
    univ2_sell_bsc_pancake,
    univ2_sell_eth_sushi,
    univ2_sell_eth_sushi_bk,
    univ2_sell_ftm_spirit,
    univ2_sell_ftm_spooky,
    univ2_sell_plg_quick,
    univ2_sellers_running_count_eth,
    univ2_sellers_running_count_other,
    users,
    value_senders,
    voters,
    votings,
);
