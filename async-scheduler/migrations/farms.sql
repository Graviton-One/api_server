create table gton_farms(
    id BIGSERIAL primary key,
    pool_id BIGINT references pools(id) UNIQUE,
    allocation BIGINT NOT NULL,
    farmed DOUBLE PRECISION not null,
    assigned DOUBLE PRECISION not null,
    apy DOUBLE PRECISION not null,
    active BOOLEAN not null,
    addresses_in int not null,
    lp_price DOUBLE PRECISION not null,
    lock_address VARCHAR not null,
    farm_linear VARCHAR not null,
    token_id int not null
);


insert into gton_farms (pool_id, allocation, farmed, assigned, apy, active, addresses_in, lp_price, lock_address, farm_linear, token_id)
    values (2, 12, 12, 12, 12, true, 1, 12, '0xa69e5e2094e55b80b71c39849de8186ed9b88b38', '0x43a467dba03ce47952c118bb2efe1d49a5aa4815', 0);


insert into gton_farms (pool_id, allocation, farmed, assigned, apy, active, addresses_in, lp_price, lock_address, farm_linear, token_id)
    values (3, 12, 12, 12, 12, true, 1, 12, '0xf8405aebd87e37e60549d4f28a5a88deb38bea7b', '0xae775393f729df8d95e014588222abe980aad61c', 2);


insert into gton_farms (pool_id, allocation, farmed, assigned, apy, active, addresses_in, lp_price, lock_address, farm_linear, token_id)
    values (9, 12, 12, 12, 12, true, 1, 12, '0xf488b8d9a391f27d5e83fa421bda986b7d4da41a', '0x6db0774fefdc9e9cb5546c1aa67a92e3046b8074', 3);


insert into gton_farms (pool_id, allocation, farmed, assigned, apy, active, addresses_in, lp_price, lock_address, farm_linear, token_id)
    values (15, 12, 12, 12, 12, true, 1, 12, '0xbba98ea00ab995a467e9afabbb15dbddd29e1f44', '0xf3245fe3a1eabb840725b9888347ce0f01ded0b4', 1);