create table events_erc20_approval_ftm(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
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
tx_from text not null,
tx_to text not null,
account text not null,
bindaddr text not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);


create table events_erc20_approval_bsc(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
owner text not null,
spender text not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_erc20_transfer_bsc(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
sender text not null,
receiver text not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_anyv4_swapin_bsc(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
account text not null,
amount numeric not null,
transfer_tx_hash text not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_anyv4_swapout_bsc(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
account text not null,
bindaddr text not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_erc20_approval_plg(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
owner text not null,
spender text not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_erc20_transfer_plg(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
sender text not null,
receiver text not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_anyv4_swapin_plg(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
account text not null,
amount numeric not null,
transfer_tx_hash text not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_anyv4_swapout_plg(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
account text not null,
bindaddr text not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_erc20_approval_eth(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
owner text not null,
spender text not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_erc20_transfer_eth(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
sender text not null,
receiver text not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_anyv4_transfer_eth(
id bigserial primary key,
tx_from text not null,
tx_to text not null,
sender text not null,
receiver text not null,
amount numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);


