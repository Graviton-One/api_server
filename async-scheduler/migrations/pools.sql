create table chains (
id primary key,
chain_name varchar not null,
chain_icon varchar not null,
chain_short varchar not null,
network_id numeric not null,
explorer varchar not null,
node_url varchar not null,
token varchar not null,
gton_address varchar not null
);

create table dexes (
id primary key,
chain_id numeric not null,
name varchar not null,
forign key (chain_id) references chains (id));

сreate table pools (
id primary key,
pool_address varchar not null,
name varchar not null,
swap_link varchar not null,
pair_link varchar not null,
gton_reserves DOUBLE PRECISION not null,
tvl DOUBLE PRECISION not null,
dex_id numeric not null,
FOREIGN KEY (dex_id) REFERENCES dexes (id));


insert into chains (chain_name, chain_icon, chain_short, network_id, explorer, node_url, token, gton_address, coingecko_id)
    values ("Ethereum", "/", "ETH", 1, "http://etherscan.io/", "https://mainnet.infura.io/v3/ec6afadb1810471dbb600f24b86391d2", "ETH", "0x01e0E2e61f554eCAaeC0cC933E739Ad90f24a86d", "ethereum");

insert into dexes (chain_id, name) values (1, "Sushiswap");

insert into pools values (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id)
        values ("0xBA38eca6DFdB92EC605C4281C3944fCcD9DeC898", "GTON_ETH", "", "", 0, 0, 1);
insert into pools values (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id)
        values ("0x0B3EcEa6bC79BE3eCC805528655C4fC173caC2DD", "GTON_USDC", "", "", 0, 0, 1);

insert into chains (chain_name, chain_icon, chain_short, network_id, explorer, node_url, token, gton_address, coingecko_id)
    values ("Binance", "/", "BSC", 56, "http://bscscan.com/", "https://bsc-dataseed.binance.org", "BNB", "0x64D5BaF5ac030e2b7c435aDD967f787ae94D0205", "binance-smart-chain");

insert into dexes (chain_id, name) values (2, "Pancakeswap");
insert into pools values (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id)
        values ("0xbe2c760aE00CbE6A5857cda719E74715edC22279", "GTON_BUSD", "", "", 0, 0, 2);
insert into pools values (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id)
        values ("0xA216571b69dd69600F50992f7c23b07B1980CfD8", "GTON_BNB", "", "", 0, 0, 2);
insert into pools values (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id)
        values ("0x84fc8e4aa7310b58f982f43cad6f2cbb97d54ac3", "GTON_USDC", "", "", 0, 0, 2);

insert into chains (chain_name, chain_icon, chain_short, network_id, explorer, node_url, token, gton_address, coingecko_id)
    values ("Fantom", "/", "FTM", 250, "http://ftmscan.com/", "https://rpcapi.fantom.network", "FTM", "0xC1Be9a4D5D45BeeACAE296a7BD5fADBfc14602C4", "fantom");

insert into dexes (chain_id, name) values (3, "Spiritswap");
insert into pools values (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id)
        values ("0x25F5B3840D414a21c4Fc46D21699e54d48F75FDD", "GTON_FTM", "", "", 0, 0, 3);
insert into pools values (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id)
        values ("0x8a5555c4996B72E5725Cf108Ad773Ce5E715DED4", "GTON_USDC", "", "", 0, 0, 3);
insert into pools values (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id)
        values ("0xe2cc82a90c1ab7c18c59cb640bce9dbf8663edf8", "GTON_Spirit", "", "", 0, 0, 3);
insert into pools values (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id)
        values ("0x070AB37714b96f1A938e75CAbbb64ED5F5748170", "GTON_fUSDT", "", "", 0, 0, 3);

insert into dexes (chain_id, name) values (4, "Spookyswap");
insert into pools values (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id)
        values ("0xcf9f857ffe6ff32b41b2a0d0b4448c16564886de", "GTON_FTM", "", "", 0, 0, 4);
insert into pools values (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id)
        values ("0xb9b452a71dd1cfb4952d90e03bf701a6c7ae263b", "GTON_USDC", "", "", 0, 0, 4);
insert into pools values (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id)
        values ("0xecaa956acb5f023da902df9eac5e799fb4716c2b", "GTON_USDT", "", "", 0, 0, 4);
insert into pools values (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id)
        values ("0x5afe9403dc982bb841d187c968abaebfaf5dba1a", "GTON_Spooky", "", "", 0, 0, 4);

insert into chains (chain_name, chain_icon, chain_short, network_id, explorer, node_url, token, gton_address, coingecko_id)
    values ("Polygon", "/", "MATIC", 1, "https://polygonscan.com/", "https://rpc-mainnet.maticvigil.com/", "MATIC", "0xf480f38c366daac4305dc484b2ad7a496ff00cea", "polygon-pos");

insert into dexes (chain_id, name) values (5, "Quickswap")

insert into pools values (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id)
        values ("0x7d49d50c886882220c428afbe60408904c72e2df", "GTON_MATIC", "", "", 0, 0, 5);
insert into pools values (pool_address, name, swap_link, pair_link, gton_reserves, tvl, dex_id)
        values ("0xf01a0a0424bda0acdd044a61af88a34636e0001c", "GTON_USDC", "", "", 0, 0, 5);


