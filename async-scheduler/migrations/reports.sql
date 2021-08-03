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
