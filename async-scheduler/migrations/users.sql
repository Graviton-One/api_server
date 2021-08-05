
create table events_open_user(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
opener text not null,
user_address text not null,
user_id numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
