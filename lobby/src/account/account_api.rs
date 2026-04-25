use actix_web::{HttpRequest, HttpResponse, web};
use shared::schema::account::{
    AccountPublicSerial, AccountSerial, ChangePasswordRequest, CreateAccountRequest, UpdateAccountRequest,
};
use sqlx::PgPool;

use super::account_db;
use super::account_model::{Account, AccountEntity};
use crate::http;
use crate::lobby_error::{LobbyError, OptionExtLobbyError, ResultExtLobbyError};
use crate::password;
use crate::session::session_extractor::AuthenticatedAccount;

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/account")
            .route("", web::get().to(get_own_account))
            .route("", web::post().to(create_account))
            .route("", web::patch().to(update_account))
            .route("", web::delete().to(delete_account))
            .route("/password", web::patch().to(change_password))
            .route("/{account_id}", web::get().to(get_account_public))
            .configure(crate::accolade::accolade_api::account_configurer)
            .configure(crate::statistic::statistic_api::account_configurer)
            .configure(crate::follow::follow_api::account_configurer),
    );
}

async fn get_own_account(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    auth: AuthenticatedAccount,
) -> Result<HttpResponse, LobbyError> {
    let entity: Option<AccountEntity> = account_db::get_account_by_id(pool.get_ref(), auth.account_id).await?;
    let entity: AccountEntity = entity.or_not_found()?;

    let account: Account = Account::from(entity);
    let serial: AccountSerial = AccountSerial::from(&account);
    Ok(http::serialize_response(&request, &serial))
}

async fn get_account_public(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, LobbyError> {
    let (account_id_string,): (String,) = path.into_inner();
    let account_id: uuid::Uuid = uuid::Uuid::parse_str(&account_id_string).or_bad_request()?;

    let entity: Option<AccountEntity> = account_db::get_account_by_id(pool.get_ref(), account_id).await?;
    let entity: AccountEntity = entity.or_not_found()?;

    let account: Account = Account::from(entity);
    let serial: AccountPublicSerial = AccountPublicSerial::from(&account);
    Ok(http::serialize_response(&request, &serial))
}

async fn create_account(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Bytes,
) -> Result<HttpResponse, LobbyError> {
    let payload: CreateAccountRequest = http::deserialize_request(&request, &body).or_bad_request()?;

    if !payload.is_valid() {
        return Err(LobbyError::bad_request("invalid create account request"));
    }

    let password_hash: String = password::hash(&payload.password)?;

    let entity: AccountEntity =
        account_db::create_account(pool.get_ref(), &payload.email, &payload.username, &password_hash).await?;

    let account: Account = Account::from(entity);
    let serial: AccountSerial = AccountSerial::from(&account);
    Ok(http::serialize_response(&request, &serial))
}

async fn update_account(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Bytes,
    auth: AuthenticatedAccount,
) -> Result<HttpResponse, LobbyError> {
    let payload: UpdateAccountRequest = http::deserialize_request(&request, &body).or_bad_request()?;

    if !payload.is_valid() {
        return Err(LobbyError::bad_request("invalid update account request"));
    }

    let entity: AccountEntity = account_db::update_account(
        pool.get_ref(),
        auth.account_id,
        payload.username.as_deref(),
        payload.email.as_deref(),
    )
    .await?;

    let account: Account = Account::from(entity);
    let serial: AccountSerial = AccountSerial::from(&account);
    Ok(http::serialize_response(&request, &serial))
}

async fn change_password(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Bytes,
    auth: AuthenticatedAccount,
) -> Result<HttpResponse, LobbyError> {
    let payload: ChangePasswordRequest = http::deserialize_request(&request, &body).or_bad_request()?;

    if !payload.is_valid() {
        return Err(LobbyError::bad_request("invalid change password request"));
    }

    let entity: Option<AccountEntity> = account_db::get_account_by_id(pool.get_ref(), auth.account_id).await?;
    let entity: AccountEntity = entity.or_not_found()?;

    let old_password_valid: bool = password::verify(&payload.old_password, &entity.password_hash)?;
    if !old_password_valid {
        return Err(LobbyError::unauthorized("incorrect password"));
    }

    let new_password_hash: String = password::hash(&payload.new_password)?;
    account_db::update_password_hash(pool.get_ref(), auth.account_id, &new_password_hash).await?;

    Ok(HttpResponse::Ok().finish())
}

async fn delete_account(
    _request: HttpRequest,
    pool: web::Data<PgPool>,
    auth: AuthenticatedAccount,
) -> Result<HttpResponse, LobbyError> {
    account_db::soft_delete_account(pool.get_ref(), auth.account_id).await?;
    Ok(HttpResponse::Ok().finish())
}
