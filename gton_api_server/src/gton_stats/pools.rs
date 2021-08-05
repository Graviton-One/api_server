use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::mpsc;

use web3::{
    self,
    Transport,
    contract::{Contract, Options},
};
use ethcontract::prelude::*;

use crate::chain::{
    ChainID,
    Web3Instance,
};


#[derive(Default, Debug, Clone)]
pub struct PoolStats {
    pub token_a: Address,
    pub token_b: Address,
    pub token_a_reserves: U256,
    pub token_b_reserves: U256,
}


async fn retrieve_token<T: Transport>(contract: &Contract<T>, property: &str) -> Result<Address, web3::contract::Error> {
    contract
        .query(property, (), None, Options::default(), None).await
}

pub async fn get_pool_reserves(
    pool_address: &str,
    web3: Web3Instance,
) -> Result<PoolStats, Box<dyn Error>> {
    let contract = Contract::from_json(
        web3.eth(),
        pool_address.parse().unwrap(),
        include_bytes!("../abi/pancakeV2pair.json"),
    ).expect("error contract creating");

    let (token_a_reserves, token_b_reserves, _): (U256, U256, U256) = contract
        .query("getReserves", (), None, Options::default(), None).await?;


    let (token_a, token_b) = (
        retrieve_token(&contract, "token0").await?,
        retrieve_token(&contract, "token1").await?,
    );
    
    // PoolStats::default()
    let pool_stats = PoolStats {
        token_a,
        token_b,
        token_a_reserves,
        token_b_reserves,
    };

    Ok(pool_stats)
}

#[async_trait]
trait ReservesHolder {
    async fn get_pool_reserves(&'static self) -> Result<Vec<PoolStats>, Box<dyn Error>>;
}

pub enum DEXName {
    Uniswap,
    Spirit,
    Spooky,
    Sushi,
    Pancake,
    Raydium,
    Serum
}


pub struct DEXPool<'a> {
    pub address: &'a str,
    pub pair: &'a str,
}


pub struct DEXPools<'a> {
    pub chain_id: ChainID,
    pub pools: Vec<DEXPool<'a>>,
    pub name: DEXName,
}

#[async_trait]
impl<'a> ReservesHolder for DEXPools<'a> {
    async fn get_pool_reserves(&'static self) -> Result<Vec<PoolStats>, Box<dyn Error>> {
        let n = self.pools.len();
        let (tx, mut rx) = mpsc::channel(n);

        for pool in &self.pools {
            let mut tx = tx.clone();

            tokio::spawn(async move {
                // tx.send("sending from first handle").await;
                let x_reserves = get_pool_reserves(pool.address.clone(), self.chain_id.web3_rpc()).await.unwrap();
                tx.send(x_reserves).await.unwrap();
            });
        }

        let mut reserves = vec![];

        for _ in 0..n {
            let x_reserves = rx.recv().await.unwrap();
            reserves.push(x_reserves)
        }

        Ok(reserves)
    }
}

pub mod list {
    use super::*;

    pub fn total_dex<'a>() -> Vec<DEXPools<'a>> {
        vec![
            self::uniswap(),
            self::sushiswap(),
            self::spookyswap(),
            self::spirit(),
            self::pancake(),
        ]
    }

    pub fn uniswap() -> DEXPools<'static> {
        DEXPools {
            chain_id: ChainID::Ethereum,
            name: DEXName::Uniswap,
            pools: vec![
                DEXPool {
                    pair: "GTON/USDC",
                    address: "0xE40a2eAB69D4dE66BcCb0Ac8E2517a230c6312E8",
                },
            ],
        }
    }

    pub fn sushiswap() -> DEXPools<'static> {
        DEXPools {
            chain_id: ChainID::Ethereum,
            name: DEXName::Sushi,
            pools: vec![
                DEXPool {
                    pair: "GTON/WETH",
                    address: "0xBA38eca6DFdB92EC605C4281C3944fCcD9DeC898",
                },
            ],
        }
    }

    pub fn spookyswap() -> DEXPools<'static> {
        DEXPools {
            chain_id: ChainID::Fantom,
            name: DEXName::Spooky,
            pools: vec![
                DEXPool {
                    pair: "GTON/USDC",
                    address: "0xcf9f857ffe6ff32b41b2a0d0b4448c16564886de",
                },
                DEXPool {
                    pair: "GTON/FTM",
                    address: "0xb9b452a71dd1cfb4952d90e03bf701a6c7ae263b",
                },
            ],
        }
    }

    pub fn spirit() -> DEXPools<'static> {
        DEXPools {
            chain_id: ChainID::Fantom,
            name: DEXName::Spirit,
            pools: vec![
                DEXPool {
                    pair: "GTON/FTM",
                    address: "0x25F5B3840D414a21c4Fc46D21699e54d48F75FDD",
                },
                DEXPool {
                    pair: "GTON/USDC",
                    address: "0x8a5555c4996B72E5725Cf108Ad773Ce5E715DED4",
                },
                DEXPool {
                    pair: "GTON/fUSDT",
                    address: "0x070AB37714b96f1A938e75CAbbb64ED5F5748170",
                },
            ],
        }
    }

    pub fn pancake() -> DEXPools<'static> {
        DEXPools {
            chain_id: ChainID::Binance,
            name: DEXName::Pancake,
            pools: vec![
                DEXPool {
                    pair: "GTON/BUSD",
                    address: "0xbe2c760aE00CbE6A5857cda719E74715edC22279",
                },
                DEXPool {
                    pair: "GTON/WBNB",
                    address: "0xA216571b69dd69600F50992f7c23b07B1980CfD8",
                },
            ],
        }
    }
}
pub struct PoolsProvider;

impl PoolsProvider {
    fn tokio() -> tokio::runtime::Runtime {
        tokio::runtime::Runtime::new().unwrap()
    }

    // pub async fn get_pool_test() -> Result<u64, Box<dyn std::error::Error>> {
    //     let rt = Self::tokio();
    //     rt.block_on(async {
    //         let bsc_gton_pool = &pools_list::pancake().pools[0];
    //         let node_url = &pools_list::pancake().chain_id.node_url();

    //         let transport = web3::transports::Http::new(node_url.as_str())?;
    //         let web3 = web3::Web3::new(transport);
        
    //         println!("Calling accounts.");
    //         let mut accounts = web3.eth().accounts().await?;
    //         println!("Accounts: {:?}", accounts);
    //         // accounts.push(bsc_gton_pool.address.parse().unwrap());
    //         accounts.push("0xCed486E3905F8FE1E8aF5d1791F5E7Ad7915f01a".parse().unwrap());
        
    //         println!("Calling balance.");
    //         for account in accounts {

    //             let balance = web3.eth().balance_of(account, None).await?;
    //             println!("Balance of {:?}: {}", account, balance);
    //             return Ok(balance.as_u64());
    //         }
            
    //         Ok(0)
    //     })
    // }
    // pub async fn get_pool_reserves() -> Result<(), Box<dyn std::error::Error>> {
    //     let transport = web3::transports::Http::new("http://localhost:8545")?;
    //     let web3 = web3::Web3::new(transport);
    
    //     println!("Calling accounts.");
    //     let mut accounts = web3.eth().accounts().await?;
    //     println!("Accounts: {:?}", accounts);
    //     accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());
    
    //     println!("Calling balance.");
    //     for account in accounts {
    //         let balance = web3.eth().balance(account, None).await?;
    //         println!("Balance of {:?}: {}", account, balance);
    //     }
        
    //     Ok(())
    // }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    type WrappedResult<T> = Result<T, Box<dyn Error>>;

    fn new_runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Runtime::new().unwrap()
    }

    #[test]
    fn test_pool_reserves_retrieval() -> WrappedResult<()> {
        let mut rt = self::new_runtime();
        rt.block_on(async {
            let pancake_pool = list::pancake();
            let first_pool = &pancake_pool.pools[0];
            let pool_reserves = get_pool_reserves(first_pool.address, pancake_pool.chain_id.web3_rpc()).await?;

            println!("retrieved pool reserves successfully");
            println!("pool reserves: {:?} \n", pool_reserves);

            // panic!();
            Ok(())
        })
    }

    // #[test]
    // fn test_all_pool_reserves_retrieval() -> WrappedResult<()> {
    //     let mut rt = self::new_runtime();
    //     rt.block_on(async {
    //         // let pancake_pool = list::pancake();
    //         // let first_pool = &pancake_pool.pools[0];
    //         // let pool_reserves = get_pool_reserves(first_pool.address, pancake_pool.chain_id.web3_rpc()).await?;
    //         let mut all_pools_reserves: Vec<PoolStats> = vec![];
    //         let all_dex = list::total_dex();

    //         let mut pool_reserves: Vec<PoolStats>;
    //         for dex in all_dex {
    //             pool_reserves = dex.get_pool_reserves().await?;
    //             // println!("pool reserves: {:?} \n", &pool_reserves);
    //             // let pool_reserves = pool_reserves.iter().flatten().collect();
    //             // let mut pool_reserves = &mut pool_reserves.clone();
    //             // all_pools_reserves.append(pool_reserves);
    //             all_pools_reserves.append(&mut pool_reserves);
    //         }

    //         println!("retrieved all pool reserves successfully");

    //         panic!();
    //         Ok(())
    //     })
    // }
}