use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use shared::schema::session::{LoginRequest, LoginResponse};
use sqlx::PgPool;
use uuid::Uuid;
use crate::account::account_db;
use crate::account::account_model::AccountEntity;
use crate::lobby_error::{LobbyError, OptionExtLobbyError, ResultExtLobbyError};
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

async fn login(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Bytes,
) -> Result<HttpResponse, LobbyError> {
    let payload: LoginRequest = http::deserialize_request(&request, &body).or_bad_request()?;

    if !payload.is_valid() {
        return Err(LobbyError::bad_request("invalid login request"));
    }

    let account_entity: Option<AccountEntity> =
        account_db::get_account_by_email(pool.get_ref(), &payload.email).await?;
    let account_entity: AccountEntity = account_entity.or_not_found()?;

    let password_valid: bool = password::verify(&payload.password, &account_entity.password_hash)?;
    if !password_valid {
        return Err(LobbyError::unauthorized("incorrect password"));
    }

    let token: String = format!("{}", Uuid::now_v7().as_simple());
    let expiry: DateTime<Utc> = Utc::now() + SESSION_DURATION;

    session_db::create_session(pool.get_ref(), account_entity.id, &token, expiry).await?;

    let response: LoginResponse = LoginResponse { token };
    Ok(http::serialize_response(&request, &response))
}

async fn logout(request: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, LobbyError> {
    let token: &str = match http::extract_bearer_token(&request) {
        Some(token) => token,
        None => return Err(LobbyError::bad_request("missing authorization header")),
    };

    session_db::delete_session_by_token(pool.get_ref(), token).await?;
    Ok(HttpResponse::Ok().finish())
}
