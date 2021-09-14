create table farms_transactions (
    id BIGSERIAL primary key,
    amount numeric not null,
    tx_type varchar not null,
    tx_hash varchar not null,
    user_address varchar not null,
    farm_id BIGSERIAL not null,
    stamp Timestamp not null
)