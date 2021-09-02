use crate::schema::{
    refferal_offers,
    votings
};
use diesel::{
    sql_types::*,
    prelude::*
};
use actix_web_dev::error::{
    Result,
};
use serde::{
    Serialize,
    Deserialize,
};

#[derive(Serialize,Deserialize,Queryable,Clone,Debug)]
pub struct Voting {
    id: i32,
    title: String,
    date_from: String,
    date_to: String,
    description: String,
    details: String,
    proposer: String,
    forum_link: String,
}
#[derive(Insertable,Serialize,Deserialize,Queryable,Clone,Debug,AsChangeset)]
#[table_name = "votings"]
pub struct UpdateVoting {
    id: i32,
    title: Option<String>,
    date_from: Option<String>,
    date_to: Option<String>,
    description: Option<String>,
    details: Option<String>,
    proposer: Option<String>,
    forum_link: Option<String>,
    active: Option<bool>
}

#[derive(Insertable,Serialize,QueryableByName,Deserialize,Queryable,Clone,Debug,AsExpression)]
#[table_name = "votings"]
pub struct VotingInstance {
    #[sql_type="Integer"]
    id: i32,
    #[sql_type="Varchar"]
    title: String,
    #[sql_type="Varchar"]
    date_from: String,
    #[sql_type="Varchar"]
    date_to: String,
    #[sql_type="Varchar"]
    description: String,
    #[sql_type="Varchar"]
    details: String,
    #[sql_type="Varchar"]
    proposer: String,
    #[sql_type="Varchar"]
    forum_link: String,
    #[sql_type="Bool"]
    active: bool,
}

impl VotingInstance {
    pub async fn update(
        data: UpdateVoting,
        conn: &PgConnection, 
    ) -> Result<Vec<VotingInstance>> {
        diesel::update(votings::table)
            .filter(votings::id.eq(data.id))
            .set(data)
            .get_results::<VotingInstance>(conn)
            .map_err(|e|e.into())
    }

    pub async fn insert(
        &self,
        conn: &PgConnection, 
    ) -> Result<Vec<VotingInstance>> {
        diesel::insert_into(votings::table)
            .values(self)
            .get_results::<VotingInstance>(conn)
            .map_err(|e|e.into())
    }

    pub async fn get(
        id: i32,
        conn: &PgConnection, 
    ) -> Result<Vec<VotingInstance>> {
        votings::table
            .filter(votings::id.eq(id))
            .get_results::<VotingInstance>(conn)
            .map_err(|e|e.into())
    }

    pub async fn get_active(
        active: bool,
        conn: &PgConnection, 
    ) -> Result<Vec<VotingInstance>> {
        votings::table
            .filter(votings::active.eq(active))
            .get_results::<VotingInstance>(conn)
            .map_err(|e|e.into())
    }
    pub async fn get_all(
        conn: &PgConnection, 
    ) -> Result<Vec<VotingInstance>> {
        votings::table
            .get_results::<VotingInstance>(conn)
            .map_err(|e|e.into())
    }
}

use uuid::Uuid;

#[derive(Queryable,Clone,Debug)]
pub struct PoolsOfferTable {
    id: i64,
    pool_address: String,
    signature: String,
    secure_id: Uuid,
}

#[derive(Deserialize,Insertable)]
#[table_name="refferal_offers"]
pub struct PoolOffer {
    pool_address: String,
    signature: String,
}

impl PoolOffer {
    pub async fn load_secure_id(
        &self,
        conn: &PgConnection
    ) -> Result<String> {
        let d: PoolsOfferTable = diesel::insert_into(refferal_offers::table)
            .values(self)
            .get_result(conn)?;
        Ok(d.secure_id.to_string())
    }
}
