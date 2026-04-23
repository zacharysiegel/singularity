use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use shared::schema::session::{LoginRequest, LoginResponse};
use sqlx::PgPool;
use uuid::Uuid;
use crate::account::account_db;
use crate::account::account_model::AccountEntity;
use crate::http;
use crate::password;
use super::session_db;

use super::SESSION_DURATION;

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/session")
            .route("", web::post().to(login))
            .route("", web::delete().to(logout)),
    );
}

async fn login(request: HttpRequest, pool: web::Data<PgPool>, body: web::Bytes) -> HttpResponse {
    let payload: LoginRequest = unwrap_or_400!(http::deserialize_request(&request, &body));

    if !payload.is_valid() {
        return HttpResponse::BadRequest().finish();
    }

    let account_entity: Option<AccountEntity> = unwrap_or_500!(account_db::get_account_by_email(pool.get_ref(), &payload.email).await);
    let account_entity: AccountEntity = unwrap_or_404!(account_entity);

    let password_valid: bool = unwrap_or_500!(password::verify(&payload.password, &account_entity.password_hash));
    if !password_valid {
        return HttpResponse::Unauthorized().finish();
    }

    let token: String = format!("{}", Uuid::now_v7().as_simple());
    let expiry: DateTime<Utc> = Utc::now() + SESSION_DURATION;

    unwrap_or_500!(
        session_db::create_session(pool.get_ref(), account_entity.id, &token, expiry).await
    );

    let response: LoginResponse = LoginResponse { token };
    http::serialize_response(&request, &response)
}

async fn logout(request: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    let token: &str = match http::extract_bearer_token(&request) {
        Some(token) => token,
        None => return HttpResponse::BadRequest().finish(),
    };

    unwrap_or_500!(session_db::delete_session_by_token(pool.get_ref(), token).await);
    HttpResponse::Ok().finish()
}
