use diesel::prelude::*;
use actix_web_dev::error::{
    Result,
    ApiError,
    ErrorType,
};
use serde::{
    Serialize,
    Deserialize,
};
use chrono::NaiveDateTime;
use diesel::sql_types::{
    Integer,
    Varchar,
    Nullable,
    Bigint,
};

#[derive(Serialize, Deserialize, QueryableByName, Debug, Clone)]
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
    #[sql_type="Bigint"]
    pub amount: i64,
}

impl UsersValues {
    pub async fn get( 
        address: &str, 
        conn: &PgConnection,
    ) -> Result<Vec<Self>> {
        println!("u adr: {}",address);
        diesel::sql_query("SELECT * FROM get_users_values($1);")
            .bind::<diesel::sql_types::Varchar,_>(address)
            .get_results::<UsersValues>(conn)
            .map_err(|e|e.into())
    }
}



#[derive(Serialize, Deserialize, Queryable, Debug, Clone)]
pub struct TotalValues {
    id: i32,
    address: String,
    name: String,
    amount: i32,
}

impl TotalValues {
    pub async fn get( 
        conn: &PgConnection,
    ) -> Result<Vec<Self>> {
        use crate::schema::value_senders;
        value_senders::table
            .get_results(conn)
            .map_err(|e|e.into())
    }
}
