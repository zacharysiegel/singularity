use actix_web::{web, HttpRequest, HttpResponse};
use shared::error::AppError;
use shared::schema::account::{
    AccountPublicSerial, AccountSerial, ChangePasswordRequest, CreateAccountRequest, UpdateAccountRequest,
};
use sqlx::PgPool;

use crate::http;
use crate::session::session_extractor::AuthenticatedAccount;
use super::account_db;
use super::account_model::Account;

const BCRYPT_COST: u32 = 10;

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/account")
            .route("", web::get().to(get_own_account))
            .route("", web::post().to(create_account))
            .route("", web::patch().to(update_account))
            .route("", web::delete().to(delete_account))
            .route("/password", web::patch().to(change_password))
            .route("/{account_id}", web::get().to(get_account_public)),
    );
}

async fn get_own_account(request: HttpRequest, pool: web::Data<PgPool>, auth: AuthenticatedAccount) -> HttpResponse {
    let entity = unwrap_or_500!(account_db::get_account_by_id(pool.get_ref(), auth.account_id).await);
    let entity = unwrap_or_404!(entity);

    let account = Account::from(entity);
    let serial = AccountSerial::from(&account);
    http::serialize_response(&request, &serial)
}

async fn get_account_public(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let account_id = unwrap_or_400!(uuid::Uuid::parse_str(&path.into_inner()));

    let entity = unwrap_or_500!(account_db::get_account_by_id(pool.get_ref(), account_id).await);
    let entity = unwrap_or_404!(entity);

    let account = Account::from(entity);
    let serial = AccountPublicSerial::from(&account);
    http::serialize_response(&request, &serial)
}

async fn create_account(request: HttpRequest, pool: web::Data<PgPool>, body: web::Bytes) -> HttpResponse {
    let payload: CreateAccountRequest = unwrap_or_400!(http::deserialize_request(&request, &body));

    if payload.email.is_empty() || payload.username.is_empty() || payload.password.is_empty() {
        return HttpResponse::BadRequest().finish();
    }

    let password_hash: String = unwrap_or_500!(hash_password(&payload.password));
    let id = uuid::Uuid::now_v7();

    let entity = unwrap_or_500!(
        account_db::create_account(pool.get_ref(), id, &payload.email, &payload.username, &password_hash).await
    );

    let account = Account::from(entity);
    let serial = AccountSerial::from(&account);
    http::serialize_response(&request, &serial)
}

async fn update_account(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Bytes,
    auth: AuthenticatedAccount,
) -> HttpResponse {
    let payload: UpdateAccountRequest = unwrap_or_400!(http::deserialize_request(&request, &body));

    if payload.username.is_none() && payload.email.is_none() {
        return HttpResponse::BadRequest().finish();
    }

    let entity = unwrap_or_500!(
        account_db::update_account(
            pool.get_ref(),
            auth.account_id,
            payload.username.as_deref(),
            payload.email.as_deref(),
        )
        .await
    );

    let account = Account::from(entity);
    let serial = AccountSerial::from(&account);
    http::serialize_response(&request, &serial)
}

async fn change_password(
    _request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Bytes,
    auth: AuthenticatedAccount,
) -> HttpResponse {
    let payload: ChangePasswordRequest = unwrap_or_400!(http::deserialize_request(&_request, &body));

    if payload.new_password.is_empty() {
        return HttpResponse::BadRequest().finish();
    }

    // Verify old password
    let entity = unwrap_or_500!(account_db::get_account_by_id(pool.get_ref(), auth.account_id).await);
    let entity = unwrap_or_404!(entity);

    let old_password_valid = unwrap_or_500!(verify_password(&payload.old_password, &entity.password_hash));
    if !old_password_valid {
        return HttpResponse::Unauthorized().finish();
    }

    let new_password_hash: String = unwrap_or_500!(hash_password(&payload.new_password));
    unwrap_or_500!(account_db::update_password_hash(pool.get_ref(), auth.account_id, &new_password_hash).await);

    HttpResponse::Ok().finish()
}

async fn delete_account(
    _request: HttpRequest,
    pool: web::Data<PgPool>,
    auth: AuthenticatedAccount,
) -> HttpResponse {
    unwrap_or_500!(account_db::soft_delete_account(pool.get_ref(), auth.account_id).await);
    HttpResponse::Ok().finish()
}

fn hash_password(password: &str) -> Result<String, AppError> {
    bcrypt::hash(password, BCRYPT_COST).map_err(|error| AppError::from_error_default(Box::new(error)))
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, password_hash).map_err(|error| AppError::from_error_default(Box::new(error)))
}
