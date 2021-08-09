
create table events_balance_keeper_open_user(
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

create table events_balance_keeper_add(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
adder text not null,
user_id numeric not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table events_balance_keeper_subtract(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
adder text not null,
user_id numeric not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table events_lp_keeper_add(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
adder text not null,
token_id numeric not null,
user_id numeric not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table events_lp_keeper_subtract(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
adder text not null,
token_id numeric not null,
user_id numeric not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
