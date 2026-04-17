use actix_web::{web, HttpRequest, HttpResponse};
use shared::schema::session::{LoginRequest, LoginResponse};
use sqlx::PgPool;

use crate::account::account_db;
use crate::account::account_model::AccountEntity;
use crate::http;
use crate::password;
use super::session_db;
use super::session_extractor::AuthenticatedAccount;

use super::SESSION_DURATION_DAYS;

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/session")
            .route("", web::post().to(login))
            .route("", web::delete().to(logout)),
    );
}

async fn login(request: HttpRequest, pool: web::Data<PgPool>, body: web::Bytes) -> HttpResponse {
    let payload: LoginRequest = unwrap_or_400!(http::deserialize_request(&request, &body));

    if payload.email.is_empty() || payload.password.is_empty() {
        return HttpResponse::BadRequest().finish();
    }

    let account_entity: Option<AccountEntity> = unwrap_or_500!(account_db::get_account_by_email(pool.get_ref(), &payload.email).await);
    let account_entity: AccountEntity = unwrap_or_404!(account_entity);

    let password_valid: bool = unwrap_or_500!(password::verify(&payload.password, &account_entity.password_hash));
    if !password_valid {
        return HttpResponse::Unauthorized().finish();
    }

    let session_id: uuid::Uuid = uuid::Uuid::now_v7();
    let token: String = format!("{}", uuid::Uuid::now_v7().as_simple());
    let expires: chrono::DateTime<chrono::Utc> = chrono::Utc::now() + chrono::Duration::days(SESSION_DURATION_DAYS);

    unwrap_or_500!(
        session_db::create_session(pool.get_ref(), session_id, account_entity.id, &token, expires).await
    );

    let response: LoginResponse = LoginResponse { token };
    http::serialize_response(&request, &response)
}

async fn logout(_request: HttpRequest, pool: web::Data<PgPool>, auth: AuthenticatedAccount) -> HttpResponse {
    unwrap_or_500!(session_db::delete_session_by_token(pool.get_ref(), &auth.token).await);
    HttpResponse::Ok().finish()
}
