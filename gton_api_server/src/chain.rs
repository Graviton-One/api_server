use ethcontract;
use ethcontract::web3;

pub type Web3Instance = web3::Web3<ethcontract::Http>;

pub enum ChainID {
    Ethereum = 1,
    Binance = 56,
    Fantom = 250,
    Polygon = 137,
}

impl ChainID {
    pub fn node_url(&self) -> String {
        match self {
            Self::Ethereum => String::from("https://mainnet.infura.io/v3/ec6afadb1810471dbb600f24b86391d2"),
            Self::Binance => String::from("https://bsc-dataseed1.binance.org"),
            Self::Fantom => String::from("https://rpcapi.fantom.network"),
            Self::Polygon => String::from("https://rpc-mainnet.maticvigil.com"),
        }
    }

    pub fn web3_rpc(&self) -> Web3Instance {
        let http = web3::transports::Http::new(self.node_url().as_str())
        .expect("error creating web3 instance");
         web3::Web3::new(http)
    }
}
