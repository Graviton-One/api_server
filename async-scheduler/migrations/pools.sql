create table gton_pools_chains (
    id BIGSERIAL primary key,
    chain_name varchar not null,
    chain_icon varchar not null,
    chain_short varchar not null,
    network_id numeric not null,
    explorer varchar not null,
    node_url varchar not null,
    token varchar not null,
    gton_address varchar not null
);

create table gton_pools_dexes (
    id  BIGSERIAL primary key,
    name varchar not null,
    chain_id BIGINT references gton_pools_chains(id)
);

CREATE table gton_pools (
    id BIGSERIAL primary key,
    pool_address varchar not null,
    name varchar not null,
    swap_link varchar not null,
    pair_link varchar not null,
    gton_reserves NUMERIC not null,
    second_token_reserves NUMERIC NOT NULL,
    second_token_name VARCHAR NOT NULL,
    tvl NUMERIC not null,
    dex_id BIGINT REFERENCES gton_pools_dexes(id)
);
