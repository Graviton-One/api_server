use actix_web::{web, HttpResponse};
use super::db::{VoterInstance, check_balance};
use crate::DbPool;
use super::{
    create_instance,
    check_voting_id,
    transfer_fee,
};
use crate::ChainConfig;
use actix_web_dev::error::{
    Result,
    ApiError,
    ErrorType,
};
use serde::{
    Serialize,
    Deserialize,
};

pub async fn get_vote_count (
    pool: web::Data<DbPool>, 
    data: web::Query<VoterInstance>,
) -> Result<HttpResponse> {
    let conn = pool.get()?;
    let r = data.get_times(&conn).await?;
    Ok(HttpResponse::Ok().json(r))
}

pub async fn check_vote (
    pool: web::Data<DbPool>, 
    data: web::Json<VoterInstance>,
    config: web::Data<std::sync::Arc<ChainConfig>>,
) -> Result<HttpResponse> {
    let conn = pool.get()?;
    let instance = create_instance("https://rpcapi.fantom.network");
    if !check_balance(&data.user_address, &conn) {
        return Err(ApiError{
            code: 500,
            message: "something went wrong".to_string(),
            error_type: ErrorType::InternalError,
        });
    }
    if !check_voting_id(data.round_id, instance.clone(),&config).await {
        return Err(ApiError{
            code: 500,
            message: "something went wrong".to_string(),
            error_type: ErrorType::InternalError,
        });
    }
    let address = &data.user_address;
    let voter = VoterInstance {
        round_id: data.round_id,
        user_address: address.to_string()
    };
    if !voter.check(&conn).await? {
        return Err(ApiError{
            code: 500,
            message: "something went wrong".to_string(),
            error_type: ErrorType::InternalError,
        });
    }
    transfer_fee(&data.user_address,instance.clone(),&config).await;
    Ok(HttpResponse::Ok().finish())
}
