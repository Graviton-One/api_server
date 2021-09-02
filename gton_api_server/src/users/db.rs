use diesel::prelude::*;
use actix_web_dev::error::{
    Result,
};
use serde::{
    Serialize,
    Deserialize,
};

#[derive(Serialize,Deserialize,Queryable,Clone,Debug)]
pub struct Users {
    id: i32,
    address: String,
    twitter_account: Option<String>, 
}
use diesel::sql_types::{
    Integer,
    Varchar,
};

#[derive(Serialize,Deserialize,QueryableByName,Clone,Debug)]
pub struct Achievements {
    #[sql_type="Integer"]
    user_id: i32,
    #[sql_type="Varchar"]
    address: String,
    #[sql_type="Integer"]
    id: i32,
    #[sql_type="Varchar"]
    name: String,
    #[sql_type="Varchar"]
    description: String,
    #[sql_type="Varchar"]
    icon: String,
    #[sql_type="Varchar"]
    external_address: String,
    #[sql_type="Varchar"]
    chain_type: String,
}

impl Achievements {
    pub async fn get(
        address: &str,
        chain: &str,
        conn: &PgConnection,
    ) -> Result<Vec<Self>> {
        println!("{}",address);
        diesel::sql_query("SELECT * FROM user_achievements WHERE 
            external_address=lower($1) and chain_type=upper($2);")
            .bind::<diesel::sql_types::Varchar,_>(address)
            .bind::<diesel::sql_types::Varchar,_>(chain)
            .get_results::<Achievements>(conn)
            .map_err(|e|e.into())
    }
}

