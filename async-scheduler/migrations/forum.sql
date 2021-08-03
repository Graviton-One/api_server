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

INSERT INTO forum_active_users (amount, stamp) VALUES (99, '2021-07-22 00:00:00.000000');
INSERT INTO forum_active_users (amount, stamp) VALUES (100, '2021-07-23 00:00:00.000000');
INSERT INTO forum_active_users (amount, stamp) VALUES (100, '2021-07-24 00:00:00.000000');
INSERT INTO forum_active_users (amount, stamp) VALUES (100, '2021-07-25 00:00:00.000000');
INSERT INTO forum_active_users (amount, stamp) VALUES (100, '2021-07-26 00:00:00.000000');

INSERT INTO forum_total_posts (amount, stamp) VALUES (116, '2021-07-22 00:00:00.000000');
INSERT INTO forum_total_posts (amount, stamp) VALUES (125, '2021-07-23 00:00:00.000000');
INSERT INTO forum_total_posts (amount, stamp) VALUES (135, '2021-07-24 00:00:00.000000');
INSERT INTO forum_total_posts (amount, stamp) VALUES (135, '2021-07-25 00:00:00.000000');
INSERT INTO forum_total_posts (amount, stamp) VALUES (136, '2021-07-26 00:00:00.000000');

INSERT INTO forum_total_topics (amount, stamp) VALUES (18, '2021-07-22 00:00:00.000000');
INSERT INTO forum_total_topics (amount, stamp) VALUES (19, '2021-07-23 00:00:00.000000');
INSERT INTO forum_total_topics (amount, stamp) VALUES (20, '2021-07-24 00:00:00.000000');
INSERT INTO forum_total_topics (amount, stamp) VALUES (20, '2021-07-25 00:00:00.000000');
INSERT INTO forum_total_topics (amount, stamp) VALUES (20, '2021-07-26 00:00:00.000000');

INSERT INTO forum_total_users (amount, stamp) VALUES (93, '2021-07-22 00:00:00.000000');
INSERT INTO forum_total_users (amount, stamp) VALUES (94, '2021-07-23 00:00:00.000000');
INSERT INTO forum_total_users (amount, stamp) VALUES (96, '2021-07-24 00:00:00.000000');
INSERT INTO forum_total_users (amount, stamp) VALUES (97, '2021-07-25 00:00:00.000000');
INSERT INTO forum_total_users (amount, stamp) VALUES (100, '2021-07-26 00:00:00.000000');
