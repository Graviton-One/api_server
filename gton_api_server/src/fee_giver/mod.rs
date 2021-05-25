pub mod db;
pub mod routes;

use web3::contract::{Contract, Options};
use ethcontract::prelude::*;
use super::ChainConfig;
pub type Web3Instance = web3::Web3<ethcontract::Http>;

pub fn create_instance(rpc_url: &str) -> Web3Instance {
    let http = web3::transports::Http::new(rpc_url)
        .expect("error creating web3 instance");
    web3::Web3::new(http)
}

pub async fn check_balance(
    address: &str, 
    web3: Web3Instance,
    cfg: &ChainConfig,
) -> bool {
    let contract = Contract::from_json(
        web3.eth(),
        cfg.balance_keeper,
        include_bytes!("../abi/balance_keeper.json"),
    ).expect("error contract creating");

    let user_address: H160 = address.parse().unwrap();

    let res: U256 = contract
        .query("userBalance", user_address, None, Options::default(), None)
        .await
        .expect("error getting user balance");

    res.gt(&U256::zero())
}

pub async fn check_voting_id(
    round_id: i32, 
    web3: Web3Instance,
    cfg: &ChainConfig,
) -> bool {
    let contract = Contract::from_json(
        web3.eth(),
        cfg.governance_vote,
        include_bytes!("../abi/voter.json"),
    ).expect("error contract creating");

    let round_id: U256 = round_id.into();
    contract
        .query("isActiveRound", round_id, None, Options::default(), None)
        .await
        .expect("error getting active rounds")
}

pub async fn transfer_fee(
    user_address: &str, 
    web3: Web3Instance,
    cfg: &ChainConfig,
) {
    let chain_id = 0xFA.into();
    let sender = Account::Offline(cfg.fee_giver_key.to_owned(), chain_id);
    
    let to_address: Address = user_address.parse().unwrap();
    ethcontract::transaction::TransactionBuilder::new(web3)
        .from(sender)
        .to(to_address)
        .value(1_000_000_000_000_000_000u64.into())
        .send()
        .await
        .expect("error sending transaction");
}
