use actix_web::{web, HttpRequest, HttpResponse};
use shared::schema::account::{
    AccountPublicSerial, AccountSerial, ChangePasswordRequest, CreateAccountRequest, UpdateAccountRequest,
};
use sqlx::PgPool;

use crate::http;
use crate::password;
use crate::session::session_extractor::AuthenticatedAccount;
use super::account_db;
use super::account_model::{Account, AccountEntity};

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
    let entity: Option<AccountEntity> = unwrap_or_500!(account_db::get_account_by_id(pool.get_ref(), auth.account_id).await);
    let entity: AccountEntity = unwrap_or_404!(entity);

    let account: Account = Account::from(entity);
    let serial: AccountSerial = AccountSerial::from(&account);
    http::serialize_response(&request, &serial)
}

async fn get_account_public(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
) -> HttpResponse {
    let (account_id_string,): (String,) = path.into_inner();
    let account_id: uuid::Uuid = unwrap_or_400!(uuid::Uuid::parse_str(&account_id_string));

    let entity: Option<AccountEntity> = unwrap_or_500!(account_db::get_account_by_id(pool.get_ref(), account_id).await);
    let entity: AccountEntity = unwrap_or_404!(entity);

    let account: Account = Account::from(entity);
    let serial: AccountPublicSerial = AccountPublicSerial::from(&account);
    http::serialize_response(&request, &serial)
}

async fn create_account(request: HttpRequest, pool: web::Data<PgPool>, body: web::Bytes) -> HttpResponse {
    let payload: CreateAccountRequest = unwrap_or_400!(http::deserialize_request(&request, &body));

    if !payload.is_valid() {
        return HttpResponse::BadRequest().finish();
    }

    let password_hash: String = unwrap_or_500!(password::hash(&payload.password));

    let entity: AccountEntity = unwrap_or_500!(
        account_db::create_account(pool.get_ref(), &payload.email, &payload.username, &password_hash).await
    );

    let account: Account = Account::from(entity);
    let serial: AccountSerial = AccountSerial::from(&account);
    http::serialize_response(&request, &serial)
}

async fn update_account(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Bytes,
    auth: AuthenticatedAccount,
) -> HttpResponse {
    let payload: UpdateAccountRequest = unwrap_or_400!(http::deserialize_request(&request, &body));

    if !payload.is_valid() {
        return HttpResponse::BadRequest().finish();
    }

    let entity: AccountEntity = unwrap_or_500!(
        account_db::update_account(
            pool.get_ref(),
            auth.account_id,
            payload.username.as_deref(),
            payload.email.as_deref(),
        )
        .await
    );

    let account: Account = Account::from(entity);
    let serial: AccountSerial = AccountSerial::from(&account);
    http::serialize_response(&request, &serial)
}

async fn change_password(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Bytes,
    auth: AuthenticatedAccount,
) -> HttpResponse {
    let payload: ChangePasswordRequest = unwrap_or_400!(http::deserialize_request(&request, &body));

    if !payload.is_valid() {
        return HttpResponse::BadRequest().finish();
    }

    let entity: Option<AccountEntity> = unwrap_or_500!(account_db::get_account_by_id(pool.get_ref(), auth.account_id).await);
    let entity: AccountEntity = unwrap_or_404!(entity);

    let old_password_valid: bool = unwrap_or_500!(password::verify(&payload.old_password, &entity.password_hash));
    if !old_password_valid {
        return HttpResponse::Unauthorized().finish();
    }

    let new_password_hash: String = unwrap_or_500!(password::hash(&payload.new_password));
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
