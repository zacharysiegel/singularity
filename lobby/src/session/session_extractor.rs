use super::session_db;
use super::session_model::SessionEntity;
use crate::lobby_error::LobbyError;
use crate::http;
use actix_web::{FromRequest, HttpRequest, dev::Payload, web};
use chrono::{DateTime, Utc};
use shared::error::AppError;
use sqlx::PgPool;
use std::future::Future;
use std::pin::Pin;
use uuid::Uuid;

/// Actix-web extractor which validates the Authorization header and resolves to an authenticated account.
/// Add this as a handler parameter to require authentication.
pub struct AuthenticatedAccount {
    pub account_id: Uuid,
    pub session_id: Uuid,
    pub token: String,
}

impl FromRequest for AuthenticatedAccount {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(request: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let pool: web::Data<PgPool> = match request.app_data::<web::Data<PgPool>>().cloned() {
            Some(pool) => pool,
            None => {
                return Box::pin(async {
                    let error: LobbyError = LobbyError::internal("missing database connection pool");
                    Err(error.into())
                });
            }
        };

        let token: String = match http::extract_bearer_token(request) {
            Some(token) => token.to_string(),
            None => {
                return Box::pin(async {
                    let error: LobbyError = LobbyError::unauthorized("missing or invalid authorization header");
                    Err(error.into())
                });
            }
        };

        Box::pin(async move {
            let session: SessionEntity = session_db::get_session_by_token(pool.get_ref(), &token)
                .await
                .map_err(|error| {
                    let lobby_error: LobbyError = LobbyError::Internal(error);
                    actix_web::Error::from(lobby_error)
                })?
                .ok_or_else(|| {
                    let lobby_error: LobbyError =
                        LobbyError::unauthorized(&format!("invalid or expired session [{}]", token));
                    actix_web::Error::from(lobby_error)
                })?;

            // Debounced sliding window: refresh when near expiry
            if (session.expiry - Utc::now()) < super::SESSION_REFRESH_THRESHOLD {
                let new_expiry: DateTime<Utc> = Utc::now() + super::SESSION_DURATION;
                let _ = session_db::refresh_session(pool.get_ref(), &token, new_expiry).await;
            }

            Ok(AuthenticatedAccount {
                account_id: session.account_id,
                session_id: session.id,
                token: session.token,
            })
        })
    }
}
