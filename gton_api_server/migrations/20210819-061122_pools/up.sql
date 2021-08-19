create table chains (
id BIGSERIAL primary key,
chain_name varchar not null,
chain_icon varchar not null,
chain_short varchar not null,
network_id numeric not null,
coingecko_id varchar not null,
explorer varchar not null,
node_url varchar not null,
token varchar not null,
gton_address varchar not null
);

create table dexes (
id BIGSERIAL primary key,
chain_id BIGINT references chains(id),
name varchar not null,
image varchar not null,
small_image varchar not null);

create table pools (
id BIGSERIAL primary key,
pool_address varchar not null,
name varchar not null,
image varchar not null,
swap_link varchar not null,
pair_link varchar not null,
gton_reserves DOUBLE PRECISION not null,
tvl DOUBLE PRECISION not null,
dex_id BIGINT REFERENCES dexes(id));


insert into chains (id, chain_name, chain_icon, chain_short, network_id, explorer, node_url, token, gton_address, coingecko_id)
    values (1, 'Ethereum', '/img/gton/table-images/chain/eth.svg', 'ETH', 1, 'http://etherscan.io/', 'https://mainnet.infura.io/v3/ec6afadb1810471dbb600f24b86391d2', 'ETH', '0x01e0E2e61f554eCAaeC0cC933E739Ad90f24a86d', 'ethereum');

insert into dexes (id, chain_id, name, image, small_image) values (1, 1, 'Sushiswap', '/img/gton/amm/sushi-s.svg', '/img/gton/table-images/sushi.svg');

insert into pools (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id, image)
        values ('0xBA38eca6DFdB92EC605C4281C3944fCcD9DeC898', 'GTON_ETH', 'https://analytics.sushi.com/tokens/0x01e0e2e61f554ecaaec0cc933e739ad90f24a86d', 'https://app.sushi.com/add/0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2/0x01e0E2e61f554eCAaeC0cC933E739Ad90f24a86d', 0, 0, 1, 'asd');
insert into pools (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id, image)
        values ('0x0B3EcEa6bC79BE3eCC805528655C4fC173caC2DD', 'GTON_USDC', 'https://analytics.sushi.com/tokens/0x01e0e2e61f554ecaaec0cc933e739ad90f24a86d', 'https://app.sushi.com/add/0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48/0x01e0E2e61f554eCAaeC0cC933E739Ad90f24a86d', 0, 0, 1, 'asd');

insert into chains (id, chain_name, chain_icon, chain_short, network_id, explorer, node_url, token, gton_address, coingecko_id)
    values (2, 'Binance', '/', 'BSC', 56, 'http://bscscan.com/', 'https://bsc-dataseed.binance.org', 'BNB', '0x64D5BaF5ac030e2b7c435aDD967f787ae94D0205', 'binance-smart-chain');

insert into dexes (id, chain_id, name, image, small_image) values (2, 2, 'Pancakeswap', '/img/gton/table-images/pancake.svg', '/img/gton/amm/pancake-s.svg');
insert into pools (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id, image)
        values ('0xbe2c760aE00CbE6A5857cda719E74715edC22279', 'GTON_BUSD', 'asfasf', 'asfasf', 0, 0, 2, 'asd');
insert into pools (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id, image)
        values ('0xA216571b69dd69600F50992f7c23b07B1980CfD8', 'GTON_BNB', 'asfasf', 'asfasf', 0, 0, 2, 'asd');
insert into pools (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id, image)
        values ('0x84fc8e4aa7310b58f982f43cad6f2cbb97d54ac3', 'GTON_USDC', 'asfasf', 'asfasf', 0, 0, 2, 'asd');

insert into chains (id, chain_name, chain_icon, chain_short, network_id, explorer, node_url, token, gton_address, coingecko_id)
    values (3, 'Fantom', '/', 'FTM', 250, 'http://ftmscan.com/', 'https://rpcapi.fantom.network', 'FTM', '0xC1Be9a4D5D45BeeACAE296a7BD5fADBfc14602C4', 'fantom');

insert into dexes (id, chain_id, name, image, small_image) values (3, 3, 'Spiritswap', '/img/gton/table-images/pancake.svg', '/img/gton/amm/pancake-s.svg');
insert into pools (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id, image)
        values ('0x25F5B3840D414a21c4Fc46D21699e54d48F75FDD', 'GTON_FTM', 'asfasf', 'asfasf', 0, 0, 3, 'asd');
insert into pools (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id, image)
        values ('0x8a5555c4996B72E5725Cf108Ad773Ce5E715DED4', 'GTON_USDC', 'asfasf', 'asfasf', 0, 0, 3, 'asd');
insert into pools (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id, image)
        values ('0xe2cc82a90c1ab7c18c59cb640bce9dbf8663edf8', 'GTON_Spirit', 'asfasf', 'asfasf', 0, 0, 3, 'asd');
insert into pools (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id, image)
        values ('0x070AB37714b96f1A938e75CAbbb64ED5F5748170', 'GTON_fUSDT', 'asfasf', 'asfasf', 0, 0, 3, 'asd');

insert into dexes (id, chain_id, name, image, small_image) values (4, 3, 'Spookyswap', '/img/gton/table-images/pancake.svg', '/img/gton/amm/pancake-s.svg');
insert into pools (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id, image)
        values ('0xcf9f857ffe6ff32b41b2a0d0b4448c16564886de', 'GTON_FTM', 'asfasf', 'asfasf', 0, 0, 4, 'asd');
insert into pools (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id, image)
        values ('0xb9b452a71dd1cfb4952d90e03bf701a6c7ae263b', 'GTON_USDC', 'asfasf', 'asfasf', 0, 0, 4, 'asd');
insert into pools (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id, image)
        values ('0xecaa956acb5f023da902df9eac5e799fb4716c2b', 'GTON_USDT', 'asfasf', 'asfasf', 0, 0, 4, 'asd');
insert into pools (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id, image)
        values ('0x5afe9403dc982bb841d187c968abaebfaf5dba1a', 'GTON_Spooky', 'asfasf', 'asfasf', 0, 0, 4, 'asd');

insert into chains (id, chain_name, chain_icon, chain_short, network_id, explorer, node_url, token, gton_address, coingecko_id)
    values (4, 'Polygon', '/', 'MATIC', 1, 'https://polygonscan.com/', 'https://rpc-mainnet.maticvigil.com/', 'MATIC', '0xf480f38c366daac4305dc484b2ad7a496ff00cea', 'polygon-pos');

insert into dexes (id, chain_id, name, image, small_image) values (5, 4, 'Quickswap', '/img/gton/table-images/pancake.svg', '/img/gton/amm/pancake-s.svg');

insert into pools (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id, image)
        values ('0x7d49d50c886882220c428afbe60408904c72e2df', 'GTON_MATIC', 'asfasf', 'asfasf', 0, 0, 5, 'asd');
insert into pools (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id, image)
        values ('0xf01a0a0424bda0acdd044a61af88a34636e0001c', 'GTON_USDC', 'asfasf', 'asfasf', 0, 0, 5, 'asd');


