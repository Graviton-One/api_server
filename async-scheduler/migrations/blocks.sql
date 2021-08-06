create table blocks(
id bigserial primary key,
name_table text not null,
block_number bigint not null,
unique (name_table)
);
