use anyhow::{Result, Context};
use crate::DbPool;
use diesel::prelude::*;

pub async fn report_buy_amount_daily_other(
    pool: &DbPool,
) -> Result<()> {
    diesel::sql_query(
       "DROP TABLE IF EXISTS univ2_buy_amount_daily_other"
    )
        .execute(&pool.get().context("execute sql query")?);
    diesel::sql_query(
       "CREATE TABLE univ2_buy_ftm_amount_daily_other AS \
        SELECT stamp::date, sum(\"amount_gton_out\") \
        FROM univ2_buy_ftm_spirit \
            NATURAL FULL OUTER JOIN univ2_buy_ftm_spooky
            NATURAL FULL OUTER JOIN univ2_buy_bsc_pancake
            NATURAL FULL OUTER JOIN univ2_buy_plg_sushi
            NATURAL FULL OUTER JOIN univ2_buy_plg_quick
        GROUP BY 1 ORDER BY 1 ASC;"
    )
        .execute(&pool.get().context("execute sql query")?);
    Ok(())
}

pub async fn report_buy_amount_daily_eth(
    pool: &DbPool,
) -> Result<()> {
    diesel::sql_query(
       "DROP TABLE IF EXISTS univ2_buy_amount_daily_eth"
    )
        .execute(&pool.get().context("execute sql query")?);
    diesel::sql_query(
       "CREATE TABLE univ2_buy_amount_daily_eth AS \
        SELECT stamp::date, sum(\"amount_gton_out\") \
        FROM univ2_buy_eth_sushi \
        GROUP BY 1 ORDER BY 1 ASC;"
    )
        .execute(&pool.get().context("execute sql query")?);
    Ok(())
}

pub async fn report_sell_amount_daily_other(
    pool: &DbPool,
) -> Result<()> {
    diesel::sql_query(
       "DROP TABLE IF EXISTS univ2_sell_amount_daily_other"
    )
        .execute(&pool.get().context("execute sql query")?);
    diesel::sql_query(
       "CREATE TABLE univ2_sell_amount_daily_other AS \
        SELECT stamp::date, sum(\"amount_gton_in\") \
        FROM univ2_sell_ftm_spirit \
            NATURAL FULL OUTER JOIN univ2_sell_ftm_spooky
            NATURAL FULL OUTER JOIN univ2_sell_bsc_pancake
            NATURAL FULL OUTER JOIN univ2_sell_plg_sushi
            NATURAL FULL OUTER JOIN univ2_sell_plg_quick
        GROUP BY 1 ORDER BY 1 ASC;"
    )
        .execute(&pool.get().context("execute sql query")?);
    Ok(())
}

pub async fn report_sell_amount_daily_eth(
    pool: &DbPool,
) -> Result<()> {
    diesel::sql_query(
       "DROP TABLE IF EXISTS univ2_sell_amount_daily_eth"
    )
        .execute(&pool.get().context("execute sql query")?);
    diesel::sql_query(
       "CREATE TABLE univ2_sell_amount_daily_eth AS \
        SELECT stamp::date, sum(\"amount_gton_in\") \
        FROM univ2_sell_eth_sushi \
        GROUP BY 1 ORDER BY 1 ASC;"
    )
        .execute(&pool.get().context("execute sql query")?);
    Ok(())
}

pub async fn report_unique_buyers_eth(pool: &DbPool) -> Result<()> {

    diesel::sql_query(
       "DROP TABLE IF EXISTS univ2_buyers_running_count_eth"
    )
        .execute(&pool.get().context("execute sql query")?);
    diesel::sql_query(
       "CREATE TABLE univ2_buyers_running_count_eth AS \
        SELECT \
            day, \
            ( \
              SELECT \
                  COUNT(DISTINCT tx_from) AS users \
              FROM \
                  univ2_buy_eth_sushi AS events \
              WHERE \
                  events.stamp::Date BETWEEN b.day - 7 AND b.day + 1 \
            ) \
        FROM  (SELECT \
                generate_series( \
                                MIN(DATE_TRUNC('day', stamp)::DATE), \
                                MAX(DATE_TRUNC('day', stamp)::DATE), \
                                '1d')::date as day \
                FROM univ2_buy_eth_sushi AS events \
              ) as b \
        GROUP BY day \
        ORDER BY day;"
    )
        .execute(&pool.get().context("execute sql query")?);
    Ok(())
}

pub async fn report_unique_sellers_eth(pool: &DbPool) -> Result<()> {

    diesel::sql_query(
       "DROP TABLE IF EXISTS univ2_sellers_running_count_eth"
    )
        .execute(&pool.get().context("execute sql query")?);
    diesel::sql_query(
       "CREATE TABLE univ2_sellers_running_count_eth AS \
        SELECT \
            day, \
            ( \
              SELECT \
                  COUNT(DISTINCT tx_from) AS users \
              FROM \
                  univ2_sell_eth_sushi AS events \
              WHERE \
                  events.stamp::Date BETWEEN b.day - 7 AND b.day + 1 \
            ) \
        FROM  (SELECT \
                generate_series( \
                                MIN(DATE_TRUNC('day', stamp)::DATE), \
                                MAX(DATE_TRUNC('day', stamp)::DATE), \
                                '1d')::date as day \
                FROM univ2_sell_eth_sushi AS events \
              ) as b \
        GROUP BY day \
        ORDER BY day;"
    )
        .execute(&pool.get().context("execute sql query")?);
    Ok(())
}

pub async fn report_unique_buyers_other(pool: &DbPool) -> Result<()> {

    diesel::sql_query(
       "DROP TABLE IF EXISTS univ2_buyers_running_count_other"
    )
        .execute(&pool.get().context("execute sql query")?);
    diesel::sql_query(
       "CREATE TABLE univ2_buyers_running_count_other AS \
        SELECT
            day,
            (
              SELECT
                  COUNT(DISTINCT tx_from) AS users
              FROM (univ2_buy_ftm_spirit
                    NATURAL FULL OUTER JOIN univ2_buy_ftm_spooky
                    NATURAL FULL OUTER JOIN univ2_buy_bsc_pancake
                    NATURAL FULL OUTER JOIN univ2_buy_plg_sushi
                    NATURAL FULL OUTER JOIN univ2_buy_plg_quick)
                        AS events
              WHERE
                  events.stamp::Date BETWEEN b.day - 7 AND b.day + 1
            )
        FROM  (SELECT
                generate_series(
                                MIN(DATE_TRUNC('day', stamp)::DATE),
                                MAX(DATE_TRUNC('day', stamp)::DATE),
                                '1d')::date as day
              FROM (univ2_buy_ftm_spirit
                    NATURAL FULL OUTER JOIN univ2_buy_ftm_spooky
                    NATURAL FULL OUTER JOIN univ2_buy_bsc_pancake
                    NATURAL FULL OUTER JOIN univ2_buy_plg_sushi
                    NATURAL FULL OUTER JOIN univ2_buy_plg_quick)
                        AS events
            ) as b
        GROUP BY day
        ORDER BY day;"
    )
        .execute(&pool.get().context("execute sql query")?);
    Ok(())
}

pub async fn report_unique_sellers_other(pool: &DbPool) -> Result<()> {

    diesel::sql_query(
       "DROP TABLE IF EXISTS univ2_sellers_running_count_other"
    )
        .execute(&pool.get().context("execute sql query")?);
    diesel::sql_query(
       "CREATE TABLE univ2_sellers_running_count_other AS \
        SELECT
            day,
            (
              SELECT
                  COUNT(DISTINCT tx_from) AS users
              FROM (univ2_sell_ftm_spirit
                    NATURAL FULL OUTER JOIN univ2_sell_ftm_spooky
                    NATURAL FULL OUTER JOIN univ2_sell_bsc_pancake
                    NATURAL FULL OUTER JOIN univ2_sell_plg_sushi
                    NATURAL FULL OUTER JOIN univ2_sell_plg_quick)
                        AS events
              WHERE
                  events.stamp::Date BETWEEN b.day - 7 AND b.day + 1
            )
        FROM  (SELECT
                generate_series(
                                MIN(DATE_TRUNC('day', stamp)::DATE),
                                MAX(DATE_TRUNC('day', stamp)::DATE),
                                '1d')::date as day
              FROM (univ2_sell_ftm_spirit
                    NATURAL FULL OUTER JOIN univ2_sell_ftm_spooky
                    NATURAL FULL OUTER JOIN univ2_sell_bsc_pancake
                    NATURAL FULL OUTER JOIN univ2_sell_plg_sushi
                    NATURAL FULL OUTER JOIN univ2_sell_plg_quick)
                        AS events
            ) as b
        GROUP BY day
        ORDER BY day;"
    )
        .execute(&pool.get().context("execute sql query")?);
    Ok(())
}
