create table events_univ2_pair_created_ftm_spirit(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
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

create table events_univ2_pair_created_ftm_spooky(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
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

create table events_univ2_pair_created_bsc_pancake(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
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

create table events_univ2_pair_created_plg_sushi(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
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

create table events_univ2_pair_created_plg_quick(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
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

create table events_univ2_pair_created_eth_sushi(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
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
create table events_univ2_transfer_eth_sushi(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
pair_id bigInt references events_univ2_pair_created_eth_sushi(id),
sender text not null,
receiver text not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_univ2_swap_eth_sushi(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
pair_id bigInt references events_univ2_pair_created_eth_sushi(id),
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
create table events_univ2_mint_eth_sushi(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
pair_id bigInt references events_univ2_pair_created_eth_sushi(id),
sender text not null,
amount0 numeric not null,
amount1 numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_univ2_burn_eth_sushi(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
pair_id bigInt references events_univ2_pair_created_eth_sushi(id),
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
