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


insert into chains (chain_name, chain_icon, chain_short, network_id, explorer, node_url, token, gton_address)
    values ("Ethereum", "/", "ETH", 1, "http://etherscan.io/", "https://mainnet.infura.io/v3/ec6afadb1810471dbb600f24b86391d2", "ETH", "0x01e0E2e61f554eCAaeC0cC933E739Ad90f24a86d");

insert into chains (chain_name, chain_icon, chain_short, network_id, explorer, node_url, token, gton_address)
    values ("Binance", "/", "BSC", 56, "http://bscscan.com/", "https://bsc-dataseed.binance.org", "BNB", "0x64D5BaF5ac030e2b7c435aDD967f787ae94D0205");

insert into chains (chain_name, chain_icon, chain_short, network_id, explorer, node_url, token, gton_address)
    values ("Fantom", "/", "FTM", 250, "http://ftmscan.com/", "https://rpcapi.fantom.network", "FTM", "0xC1Be9a4D5D45BeeACAE296a7BD5fADBfc14602C4");

insert into chains (chain_name, chain_icon, chain_short, network_id, explorer, node_url, token, gton_address)
    values ("Polygon", "/", "MATIC", 1, "https://polygonscan.com/", "https://rpc-mainnet.maticvigil.com/", "MATIC", "0xf480f38c366daac4305dc484b2ad7a496ff00cea");