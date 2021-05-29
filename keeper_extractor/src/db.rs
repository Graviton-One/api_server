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
        conn: &PgConnection,
        poller_id: i32,
        new_block_id: i32,
    ) -> Result<()> {
            diesel::update(pollers_data::table)
                .filter(pollers_data::poller_id.eq(poller_id))
                .set(
                    pollers_data::block_id.eq(new_block_id),
                )
                .execute(conn)
                .unwrap();
                Ok(())
    }

    pub async fn get(
        conn: &PgConnection, 
        poller_id: i32
    ) -> Result<Self> {
        pollers_data::table
            .filter(pollers_data::poller_id.eq(poller_id))
            .get_result(conn)
            .map_err(|e|e.into())
    }
}