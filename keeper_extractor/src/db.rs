use crate::schema::pollers_data;
use diesel::prelude::*;
use serde::{
    Serialize,
    Deserialize,
};

#[derive(Serialize,Deserialize,Queryable)]
pub struct Data {
    id: i32,
    block_id: i32,
    poller_id: i32, 
}


impl Blocks {
    pub async fn check(
        &self,
        conn: &PgConnection, 
    ) -> Result<bool> {
        diesel::sql_query("call GiveFee($1,$2)")
            .bind::<diesel::sql_types::Varchar,_>(self.user_address.clone())
            .bind::<diesel::sql_types::Integer,_>(self.round_id)
            .get_result::<Allowance>(conn)
            .map_err(|e|e.into())
            .map(|r|r.res)
    }

    pub async fn get(
        &self,
        conn: &PgConnection, 
    ) -> Result<Voter> {
        pollers_data::table
            .filter(pollers_data::round_id.eq(self.round_id))
            .get_result(conn)
            .map_err(|e|e.into())
    }
}