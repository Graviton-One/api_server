create table total_staked(
id bigserial primary key,
amount numeric not null,
stamp timestamp default CURRENT_TIMESTAMP not null);
create table total_stakers(
id bigserial primary key,
amount numeric not null,
stamp timestamp default CURRENT_TIMESTAMP not null);
create table forum_total_users(
id bigserial primary key,
amount bigInt not null,
stamp timestamp default CURRENT_TIMESTAMP not null);
create table forum_active_users(
id bigserial primary key,
amount bigInt not null,
stamp timestamp default CURRENT_TIMESTAMP not null);
create table forum_total_topics(
id bigserial primary key,
amount bigInt not null,
stamp timestamp default CURRENT_TIMESTAMP not null);
create table forum_total_posts(
id bigserial primary key,
amount bigInt not null,
stamp timestamp default CURRENT_TIMESTAMP not null);
create table forum_report_dau_by_mau(
id bigserial primary key,
amount double precision not null,
stamp date not null,
unique (stamp));
create table forum_report_daily_engaged_users(
id bigserial primary key,
amount bigInt not null,
stamp date not null,
unique (stamp));
create table forum_report_likes(
id bigserial primary key,
amount bigInt not null,
stamp date not null,
unique (stamp));
create table forum_report_new_contributors(
id bigserial primary key,
amount bigInt not null,
stamp date not null,
unique (stamp));
create table forum_report_page_views(
id bigserial primary key,
amount bigInt not null,
stamp date not null,
unique (stamp));
create table forum_report_posts(
id bigserial primary key,
amount bigInt not null,
stamp date not null,
unique (stamp));
create table forum_report_signups(
id bigserial primary key,
amount bigInt not null,
stamp date not null,
unique (stamp));
create table forum_report_topics(
id bigserial primary key,
amount bigInt not null,
stamp date not null,
unique (stamp));
create table forum_report_visits(
id bigserial primary key,
amount bigInt not null,
stamp date not null,
unique (stamp));
create table events_erc20_approval_ftm(
id bigserial primary key,
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
address text not null,
token0 text not null,
token1 text not null,
index numeric not null,
stamp timestamp not null,
block_number bigInt not null,
tx_hash text not null,
log_index bigInt not null,
unique (tx_hash, log_index)
);
create table events_univ2_swap_spirit(
id bigserial primary key,
pair bigInt references events_univ2_pair_created_spirit(id),
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
pair bigInt references events_univ2_pair_created_spirit(id),
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
pair bigInt references events_univ2_pair_created_spirit(id),
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
INSERT INTO forum_active_users (id, amount, stamp) VALUES (1, 99, '2021-07-22 00:00:00.000000');
INSERT INTO forum_active_users (id, amount, stamp) VALUES (2, 100, '2021-07-23 00:00:00.000000');
INSERT INTO forum_active_users (id, amount, stamp) VALUES (3, 100, '2021-07-24 00:00:00.000000');
INSERT INTO forum_active_users (id, amount, stamp) VALUES (4, 100, '2021-07-25 00:00:00.000000');
INSERT INTO forum_active_users (id, amount, stamp) VALUES (5, 100, '2021-07-26 00:00:00.000000');

INSERT INTO forum_total_posts (id, amount, stamp) VALUES (1, 116, '2021-07-22 00:00:00.000000');
INSERT INTO forum_total_posts (id, amount, stamp) VALUES (2, 125, '2021-07-23 00:00:00.000000');
INSERT INTO forum_total_posts (id, amount, stamp) VALUES (3, 135, '2021-07-24 00:00:00.000000');
INSERT INTO forum_total_posts (id, amount, stamp) VALUES (4, 135, '2021-07-25 00:00:00.000000');
INSERT INTO forum_total_posts (id, amount, stamp) VALUES (5, 136, '2021-07-26 00:00:00.000000');

INSERT INTO forum_total_topics (id, amount, stamp) VALUES (1, 18, '2021-07-22 00:00:00.000000');
INSERT INTO forum_total_topics (id, amount, stamp) VALUES (2, 19, '2021-07-23 00:00:00.000000');
INSERT INTO forum_total_topics (id, amount, stamp) VALUES (3, 20, '2021-07-24 00:00:00.000000');
INSERT INTO forum_total_topics (id, amount, stamp) VALUES (4, 20, '2021-07-25 00:00:00.000000');
INSERT INTO forum_total_topics (id, amount, stamp) VALUES (5, 20, '2021-07-26 00:00:00.000000');

INSERT INTO forum_total_users (id, amount, stamp) VALUES (1, 93, '2021-07-22 00:00:00.000000');
INSERT INTO forum_total_users (id, amount, stamp) VALUES (2, 94, '2021-07-23 00:00:00.000000');
INSERT INTO forum_total_users (id, amount, stamp) VALUES (3, 96, '2021-07-24 00:00:00.000000');
INSERT INTO forum_total_users (id, amount, stamp) VALUES (4, 97, '2021-07-25 00:00:00.000000');
INSERT INTO forum_total_users (id, amount, stamp) VALUES (5, 100, '2021-07-26 00:00:00.000000');
