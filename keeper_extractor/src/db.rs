use crate::schema::pollers_data;
use diesel::prelude::*;
use serde::{
    Serialize,
    Deserialize,
};

#[derive(Insertable,Serialize,Deserialize)]
#[table_name = "pollers_data"]
pub struct Blocks {
    pub block_id: i32,
    pub poller_id: i32,
}

#[derive(Serialize,Deserialize,Queryable)]
pub struct Data {
    id: i32,
    block_id: i32,
    poller_id: i32, 
}


impl Blocks {
    pub async fn save(
        &self,
        conn: &PgConnection, 
    ) -> Result<bool> {
        diesel::sql_query("INSERT INTO pollers_data (block_id, poller_id) VALUES ($1, $2)")
            .bind::<diesel::sql_types::Integer,_>(self.block_id)
            .bind::<diesel::sql_types::Integer,_>(self.poller_id)
            .get_result::<Data>(conn)
            .map_err(|e|e.into())
            .map(|r|r.res)
    }

    pub async fn get(
        &self,
        conn: &PgConnection, 
    ) -> Result<Data> {
        pollers_data::table
            .filter(pollers_data::poller_id.eq(self.poller_id))
            .get_result(conn)
            .map_err(|e|e.into())
    }
}