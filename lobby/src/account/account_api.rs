use actix_web::{web, HttpRequest, HttpResponse};
use shared::error::AppError;
use shared::schema::account::CreateAccountRequest;
use sqlx::PgPool;

use crate::http;
use super::account_db;
use super::account_model::Account;

const BCRYPT_COST: u32 = 10;

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(web::scope("/account").route("", web::post().to(create_account)));
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
    let serial = shared::schema::account::AccountSerial::from(&account);
    http::serialize_response(&request, &serial)
}

fn hash_password(password: &str) -> Result<String, AppError> {
    bcrypt::hash(password, BCRYPT_COST).map_err(|error| AppError::from_error_default(Box::new(error)))
}
