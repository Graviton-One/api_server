create table events_univ2_pair_created_ftm_spirit(
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
create table events_univ2_transfer_ftm_spirit(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_ftm_spirit(id),
sender text not null,
receiver text not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_univ2_swap_ftm_spirit(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_ftm_spirit(id),
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
create table events_univ2_mint_ftm_spirit(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_ftm_spirit(id),
sender text not null,
amount0 numeric not null,
amount1 numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_univ2_burn_ftm_spirit(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_ftm_spirit(id),
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

create table univ2_buy_ftm_spirit(
id bigserial primary key,
swap_id bigInt references events_univ2_swap_ftm_spirit(id),
pair_id bigInt references events_univ2_pair_created_ftm_spirit(id),
pair_title text not null,
tx_origin text not null,
amount_token_in numeric not null,
amount_gton_out numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table univ2_sell_ftm_spirit(
id bigserial primary key,
swap_id bigInt references events_univ2_swap_ftm_spirit(id),
pair_id bigInt references events_univ2_pair_created_ftm_spirit(id),
pair_title text not null,
tx_origin text not null,
amount_gton_in numeric not null,
amount_token_out numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table univ2_lp_add_ftm_spirit(
id bigserial primary key,
mint_id bigInt references events_univ2_mint_ftm_spirit(id),
pair_id bigInt references events_univ2_pair_created_ftm_spirit(id),
pair_title text not null,
tx_origin text not null,
amount_gton_in numeric not null,
amount_token_in numeric not null,
amount_lp_out numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table univ2_lp_remove_ftm_spirit(
id bigserial primary key,
burn_id bigInt references events_univ2_burn_ftm_spirit(id),
pair_id bigInt references events_univ2_pair_created_ftm_spirit(id),
pair_title text not null,
tx_origin text not null,
amount_gton_out numeric not null,
amount_token_out numeric not null,
amount_lp_in numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table events_univ2_pair_created_ftm_spooky(
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
create table events_univ2_transfer_ftm_spooky(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_ftm_spooky(id),
sender text not null,
receiver text not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_univ2_swap_ftm_spooky(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_ftm_spooky(id),
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
create table events_univ2_mint_ftm_spooky(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_ftm_spooky(id),
sender text not null,
amount0 numeric not null,
amount1 numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_univ2_burn_ftm_spooky(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_ftm_spooky(id),
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

create table univ2_buy_ftm_spooky(
id bigserial primary key,
swap_id bigInt references events_univ2_swap_ftm_spooky(id),
pair_id bigInt references events_univ2_pair_created_ftm_spooky(id),
pair_title text not null,
tx_origin text not null,
amount_token_in numeric not null,
amount_gton_out numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table univ2_sell_ftm_spooky(
id bigserial primary key,
swap_id bigInt references events_univ2_swap_ftm_spooky(id),
pair_id bigInt references events_univ2_pair_created_ftm_spooky(id),
pair_title text not null,
tx_origin text not null,
amount_gton_in numeric not null,
amount_token_out numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table univ2_lp_add_ftm_spooky(
id bigserial primary key,
mint_id bigInt references events_univ2_mint_ftm_spooky(id),
pair_id bigInt references events_univ2_pair_created_ftm_spooky(id),
pair_title text not null,
tx_origin text not null,
amount_gton_in numeric not null,
amount_token_in numeric not null,
amount_lp_out numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table univ2_lp_remove_ftm_spooky(
id bigserial primary key,
burn_id bigInt references events_univ2_burn_ftm_spooky(id),
pair_id bigInt references events_univ2_pair_created_ftm_spooky(id),
pair_title text not null,
tx_origin text not null,
amount_gton_out numeric not null,
amount_token_out numeric not null,
amount_lp_in numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table events_univ2_pair_created_bsc_pancake(
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
create table events_univ2_transfer_bsc_pancake(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_bsc_pancake(id),
sender text not null,
receiver text not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_univ2_swap_bsc_pancake(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_bsc_pancake(id),
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
create table events_univ2_mint_bsc_pancake(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_bsc_pancake(id),
sender text not null,
amount0 numeric not null,
amount1 numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_univ2_burn_bsc_pancake(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_bsc_pancake(id),
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

create table univ2_buy_bsc_pancake(
id bigserial primary key,
swap_id bigInt references events_univ2_swap_bsc_pancake(id),
pair_id bigInt references events_univ2_pair_created_bsc_pancake(id),
pair_title text not null,
tx_origin text not null,
amount_token_in numeric not null,
amount_gton_out numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table univ2_sell_bsc_pancake(
id bigserial primary key,
swap_id bigInt references events_univ2_swap_bsc_pancake(id),
pair_id bigInt references events_univ2_pair_created_bsc_pancake(id),
pair_title text not null,
tx_origin text not null,
amount_gton_in numeric not null,
amount_token_out numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table univ2_lp_add_bsc_pancake(
id bigserial primary key,
mint_id bigInt references events_univ2_mint_bsc_pancake(id),
pair_id bigInt references events_univ2_pair_created_bsc_pancake(id),
pair_title text not null,
tx_origin text not null,
amount_gton_in numeric not null,
amount_token_in numeric not null,
amount_lp_out numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table univ2_lp_remove_bsc_pancake(
id bigserial primary key,
burn_id bigInt references events_univ2_burn_bsc_pancake(id),
pair_id bigInt references events_univ2_pair_created_bsc_pancake(id),
pair_title text not null,
tx_origin text not null,
amount_gton_out numeric not null,
amount_token_out numeric not null,
amount_lp_in numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_univ2_pair_created_plg_sushi(
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
create table events_univ2_transfer_plg_sushi(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_plg_sushi(id),
sender text not null,
receiver text not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_univ2_swap_plg_sushi(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_plg_sushi(id),
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
create table events_univ2_mint_plg_sushi(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_plg_sushi(id),
sender text not null,
amount0 numeric not null,
amount1 numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_univ2_burn_plg_sushi(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_plg_sushi(id),
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

create table univ2_buy_plg_sushi(
id bigserial primary key,
swap_id bigInt references events_univ2_swap_plg_sushi(id),
pair_id bigInt references events_univ2_pair_created_plg_sushi(id),
pair_title text not null,
tx_origin text not null,
amount_token_in numeric not null,
amount_gton_out numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table univ2_sell_plg_sushi(
id bigserial primary key,
swap_id bigInt references events_univ2_swap_plg_sushi(id),
pair_id bigInt references events_univ2_pair_created_plg_sushi(id),
pair_title text not null,
tx_origin text not null,
amount_gton_in numeric not null,
amount_token_out numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table univ2_lp_add_plg_sushi(
id bigserial primary key,
mint_id bigInt references events_univ2_mint_plg_sushi(id),
pair_id bigInt references events_univ2_pair_created_plg_sushi(id),
pair_title text not null,
tx_origin text not null,
amount_gton_in numeric not null,
amount_token_in numeric not null,
amount_lp_out numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table univ2_lp_remove_plg_sushi(
id bigserial primary key,
burn_id bigInt references events_univ2_burn_plg_sushi(id),
pair_id bigInt references events_univ2_pair_created_plg_sushi(id),
pair_title text not null,
tx_origin text not null,
amount_gton_out numeric not null,
amount_token_out numeric not null,
amount_lp_in numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_univ2_pair_created_plg_quick(
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
create table events_univ2_transfer_plg_quick(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_plg_quick(id),
sender text not null,
receiver text not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_univ2_swap_plg_quick(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_plg_quick(id),
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
create table events_univ2_mint_plg_quick(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_plg_quick(id),
sender text not null,
amount0 numeric not null,
amount1 numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_univ2_burn_plg_quick(
id bigserial primary key,
tx_origin text not null,
pair_id bigInt references events_univ2_pair_created_plg_quick(id),
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

create table univ2_buy_plg_quick(
id bigserial primary key,
swap_id bigInt references events_univ2_swap_plg_quick(id),
pair_id bigInt references events_univ2_pair_created_plg_quick(id),
pair_title text not null,
tx_origin text not null,
amount_token_in numeric not null,
amount_gton_out numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table univ2_sell_plg_quick(
id bigserial primary key,
swap_id bigInt references events_univ2_swap_plg_quick(id),
pair_id bigInt references events_univ2_pair_created_plg_quick(id),
pair_title text not null,
tx_origin text not null,
amount_gton_in numeric not null,
amount_token_out numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table univ2_lp_add_plg_quick(
id bigserial primary key,
mint_id bigInt references events_univ2_mint_plg_quick(id),
pair_id bigInt references events_univ2_pair_created_plg_quick(id),
pair_title text not null,
tx_origin text not null,
amount_gton_in numeric not null,
amount_token_in numeric not null,
amount_lp_out numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);

create table univ2_lp_remove_plg_quick(
id bigserial primary key,
burn_id bigInt references events_univ2_burn_plg_quick(id),
pair_id bigInt references events_univ2_pair_created_plg_quick(id),
pair_title text not null,
tx_origin text not null,
amount_gton_out numeric not null,
amount_token_out numeric not null,
amount_lp_in numeric not null,
stamp timestamp not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
