create table events_erc20_approval_ftm(
id bigserial primary key,
tx_origin text not null,
owner text not null,
spender text not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_erc20_transfer_ftm(
id bigserial primary key,
tx_origin text not null,
sender text not null,
receiver text not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_anyv4_transfer(
id bigserial primary key,
tx_origin text not null,
sender text not null,
receiver text not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_anyv4_swapin_ftm(
id bigserial primary key,
tx_origin text not null,
account text not null,
amount numeric not null,
transfer_tx_hash text not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_anyv4_swapout_ftm(
id bigserial primary key,
tx_origin text not null,
account text not null,
bindaddr text not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_univ2_pair_created_spirit(
id bigserial primary key,
tx_origin text not null,
address text not null,
token0 text not null,
token1 text not null,
gtonToken0 bool not null,
title text not null,
index numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_univ2_transfer_spirit(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_spirit(id),
sender text not null,
receiver text not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_univ2_swap_spirit(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_spirit(id),
sender text not null,
receiver text not null,
amount0_in numeric not null,
amount1_in numeric not null,
amount0_out numeric not null,
amount1_out numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_univ2_mint_spirit(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_spirit(id),
sender text not null,
amount0 numeric not null,
amount1 numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_univ2_burn_spirit(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_spirit(id),
sender text not null,
receiver text not null,
amount0 numeric not null,
amount1 numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table univ2_buy_spirit(
id bigserial primary key,
swap_id bigInt references events_univ2_swap_spirit(id),
pair_id bigInt references events_univ2_pair_created_spirit(id),
pair_title text not null,
tx_origin text not null,
amount_token_in numeric not null,
amount_gton_out numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table univ2_sell_spirit(
id bigserial primary key,
swap_id bigInt references events_univ2_swap_spirit(id),
pair_id bigInt references events_univ2_pair_created_spirit(id),
pair_title text not null,
tx_origin text not null,
amount_gton_in numeric not null,
amount_token_out numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table univ2_lp_add(
id bigserial primary key,
mint_id bigInt references events_univ2_mint_spirit(id),
pair_id bigInt references events_univ2_pair_created_spirit(id),
pair_title text not null,
tx_origin text not null,
amount_gton_in numeric not null,
amount_token_out numeric not null,
amount_lp_out numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
