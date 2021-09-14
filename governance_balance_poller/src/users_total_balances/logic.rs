use std::sync::Arc;
use bigdecimal::BigDecimal;
use std::str::FromStr;

use web3::transports::Http;
use web3::types::*;
use web3::ethabi::{
    Topic,
    TopicFilter,
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

#[derive(Debug)]
pub struct Value {
    pub internal_address: String,
    pub amount: BigDecimal,
}
pub struct BalanceInOutValues (Vec<Value>);

impl BalanceInOutValues {
    pub fn add_to_db(
        &self,
        conn: Arc<Pool<ConnectionManager<PgConnection>>>,
    ) {
        for el in self.0.iter() {
            diesel::sql_query("select * from add_to_user_balance($1,$2)")
                .bind::<diesel::sql_types::Varchar,_>(el.internal_address.clone())
                .bind::<diesel::sql_types::Numeric,_>(el.amount.clone())
                .execute(&conn.get().unwrap())
                .unwrap();
        }
    }

    pub fn sub_to_db(
        &self,
        conn: Arc<Pool<ConnectionManager<PgConnection>>>,
    ) {
        for el in self.0.iter() {
            diesel::sql_query("select * from sub_to_user_balance($1,$2)")
                .bind::<diesel::sql_types::Varchar,_>(el.internal_address.clone())
                .bind::<diesel::sql_types::Numeric,_>(el.amount.clone())
                .execute(&conn.get().unwrap())
                .unwrap();
        }
    }
    pub async fn new(
        web3: &web3::Web3<Http>,
        prev_block: BlockNumber, 
        current_block: BlockNumber,
        method_topic: H256,
        balance_keeper: Address,
    ) -> Self {
        //getting add events
            let mut topics = TopicFilter::default();
            topics.topic0 = Topic::This(method_topic);

            let filter = FilterBuilder::default()
                        .from_block(prev_block) 
                        .to_block(current_block)
                        .address(vec![balance_keeper])
                        .topic_filter(topics)
                        .build();
            let result: Vec<web3::types::Log> = web3.eth().logs(filter).await.unwrap();
            let mut r: Vec<Value> = Vec::new();
            for block in result {
                use std::ops::Index;
                let to = block.topics[2].as_bytes();
                let to: U256 = to.into();
                let amount: U256 = block.data.0.index(0..32).into();
                let d = Value{
                    internal_address: to.to_string(),
                    amount: BigDecimal::from_str(&amount.to_string()).unwrap(),
                };
                println!("got {:?}",d);
                r.push(d);
            }
            BalanceInOutValues(r)
    }

}


