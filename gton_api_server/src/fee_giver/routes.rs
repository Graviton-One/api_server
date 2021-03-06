use actix_web::{web, HttpResponse};
use super::db::VoterInstance;
use crate::DbPool;
use super::{
    create_instance,
    check_balance,
    check_voting_id,
    transfer_fee,
};
use crate::ChainConfig;
use actix_web_dev::error::{
    Result,
    ApiError,
    ErrorType,
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
    if !check_balance(&data.user_address, instance.clone(),&config).await {
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
    if !data.check(&conn).await? {
        return Err(ApiError{
            code: 500,
            message: "something went wrong".to_string(),
            error_type: ErrorType::InternalError,
        });
    }
    transfer_fee(&data.user_address,instance.clone(),&config).await;
    Ok(HttpResponse::Ok().finish())
}
