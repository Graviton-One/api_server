use web3::types::*;
use web3::ethabi::{
    Topic,
    TopicFilter,
};
pub type Web3Instance = web3::Web3<ethcontract::Http>;

pub fn create_instance(rpc_url: &str) -> Web3Instance {
    let http = web3::transports::Http::new(rpc_url)
        .expect("error creating web3 instance");
    web3::Web3::new(http)
}

pub fn build_filter(method_topic: H256, address: Address, prev_block: BlockNumber, current_block: BlockNumber) -> Filter {
    let mut topics = TopicFilter::default();
    topics.topic0 = Topic::This(method_topic);

    FilterBuilder::default()
                .from_block(prev_block) 
                .to_block(current_block)
                .address(vec![address])
                .topic_filter(topics)
                .build()
}

pub fn prepare_volume(num: U256, dig: U256) -> f64 {
    let amount = amount.checked_div(U256::from_dec_str("10")
    .unwrap().pow(gton_dig)).unwrap();
    amount.as_u128() as f64
}