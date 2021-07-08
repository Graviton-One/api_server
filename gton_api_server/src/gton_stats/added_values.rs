use diesel::prelude::*;
use actix_web_dev::error::{
    Result,
};
use bigdecimal::BigDecimal;
use serde::{
    Serialize,
    Deserialize,
};
use diesel::sql_types::{
    Integer,
    Varchar,
    Bigint,
    Numeric,
};
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct UsersValuesSerde {
    pub sender_address: String,
    pub name: String,
    pub sender_id: i32,
    pub user_address: String,
    pub user_id: i32,
    pub amount: String,
}

#[derive(QueryableByName, Debug, Clone)]
pub struct UsersValues {
    #[sql_type="Varchar"]
    pub sender_address: String,
    #[sql_type="Varchar"]
    pub name: String,
    #[sql_type="Integer"]
    pub sender_id: i32,
    #[sql_type="Varchar"]
    pub user_address: String,
    #[sql_type="Integer"]
    pub user_id: i32,
    #[sql_type="Numeric"]
    pub amount: BigDecimal,
}

impl UsersValues {
    pub async fn get( 
        address: &str, 
        conn: &PgConnection,
    ) -> Result<Vec<UsersValuesSerde>> {
        let r = diesel::sql_query("SELECT * FROM get_users_values($1);")
            .bind::<diesel::sql_types::Varchar,_>(address)
            .get_results::<UsersValues>(conn)?;
        Ok(r
            .into_iter()
            .map(|el|{
                UsersValuesSerde {
                    sender_address: el.sender_address,
                    name: el.name,
                    sender_id: el.sender_id,
                    user_address: el.user_address,
                    user_id: el.user_id,
                    amount: el.amount.to_string()
                }
            })
            .collect())
    }
}


#[derive(Serialize, Deserialize)]
pub struct TotalValuesSerde {
    id: i32,
    address: String,
    name: String,
    amount: String,
}

#[derive(Queryable, Debug, Clone)]
pub struct TotalValues {
    id: i32,
    address: String,
    name: String,
    amount: BigDecimal,
}

impl TotalValues {
    pub async fn get( 
        conn: &PgConnection,
    ) -> Result<Vec<TotalValuesSerde>> {
        use crate::schema::value_senders;
        let r = value_senders::table
            .get_results::<TotalValues>(conn)?;
        Ok(r
            .into_iter()
            .map(|el|{
                TotalValuesSerde {
                    id: el.id,
                    address: el.address,
                    name: el.name,
                    amount: el.amount.to_string(),
                }
            })
            .collect())
    }
}
