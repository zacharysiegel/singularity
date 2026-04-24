use actix_web::{HttpRequest, HttpResponse, web};
use shared::schema::follow::{FollowSerial, FollowSummarySerial};
use sqlx::PgPool;
use uuid::Uuid;

use super::follow_db;
use super::follow_model::{Follow, FollowEntity, FollowSummary};
use crate::http;
use crate::lobby_error::{LobbyError, OptionExt, ResultExt};
use crate::session::session_extractor::AuthenticatedAccount;

pub fn account_configurer(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/{account_id}")
            .route("/follow", web::post().to(follow_account))
            .route("/follow", web::delete().to(unfollow_account))
            .route("/followers", web::get().to(get_followers))
            .route("/following", web::get().to(get_following))
            .route("/mutuals", web::get().to(get_mutuals)),
    );
}

// TODO: prevent the client from issuing a self-follow request for efficiency
async fn follow_account(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
    auth: AuthenticatedAccount,
) -> Result<HttpResponse, LobbyError> {
    let (target_account_id_string,): (String,) = path.into_inner();
    let target_account_id: Uuid = Uuid::parse_str(&target_account_id_string).or_bad_request()?;

    let existing: Option<FollowEntity> = follow_db::get_follow(pool.get_ref(), auth.account_id, target_account_id).await?;
    existing.then_conflict("already following this account")?;

    let entity: FollowEntity = follow_db::create_follow(pool.get_ref(), auth.account_id, target_account_id).await?;
    let follow: Follow = Follow::from(entity);
    let serial: FollowSerial = FollowSerial::from(&follow);
    Ok(http::serialize_response(&request, &serial))
}

async fn unfollow_account(
    _request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
    auth: AuthenticatedAccount,
) -> Result<HttpResponse, LobbyError> {
    let (target_account_id_string,): (String,) = path.into_inner();
    let target_account_id: Uuid = Uuid::parse_str(&target_account_id_string).or_bad_request()?;

    let deleted: bool = follow_db::delete_follow(pool.get_ref(), auth.account_id, target_account_id).await?;
    if !deleted {
        return Err(LobbyError::not_found("follow relationship"));
    }

    Ok(HttpResponse::Ok().finish())
}

async fn get_followers(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, LobbyError> {
    let (account_id_string,): (String,) = path.into_inner();
    let account_id: Uuid = Uuid::parse_str(&account_id_string).or_bad_request()?;

    let follower_rows: Vec<FollowSummary> = follow_db::get_followers(pool.get_ref(), account_id).await?;
    let follower_serials: Vec<FollowSummarySerial> =
        follower_rows.iter().map(FollowSummarySerial::from).collect();

    Ok(http::serialize_response(&request, &follower_serials))
}

async fn get_following(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, LobbyError> {
    let (account_id_string,): (String,) = path.into_inner();
    let account_id: Uuid = Uuid::parse_str(&account_id_string).or_bad_request()?;

    let following_rows: Vec<FollowSummary> = follow_db::get_following(pool.get_ref(), account_id).await?;
    let following_serials: Vec<FollowSummarySerial> =
        following_rows.iter().map(FollowSummarySerial::from).collect();

    Ok(http::serialize_response(&request, &following_serials))
}

async fn get_mutuals(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, LobbyError> {
    let (account_id_string,): (String,) = path.into_inner();
    let account_id: Uuid = Uuid::parse_str(&account_id_string).or_bad_request()?;

    let mutual_rows: Vec<FollowSummary> = follow_db::get_mutuals(pool.get_ref(), account_id).await?;
    let mutual_serials: Vec<FollowSummarySerial> = mutual_rows.iter().map(FollowSummarySerial::from).collect();

    Ok(http::serialize_response(&request, &mutual_serials))
}
