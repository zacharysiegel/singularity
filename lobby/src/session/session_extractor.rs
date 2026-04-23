use super::session_db;
use super::session_model::SessionEntity;
use crate::http;
use actix_web::{FromRequest, HttpRequest, dev::Payload, web};
use chrono::{DateTime, Utc};
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
                    Err(actix_web::error::ErrorInternalServerError(
                        "missing database connection pool",
                    ))
                });
            }
        };

        let token: String = match http::extract_bearer_token(request) {
            Some(token) => token.to_string(),
            None => {
                return Box::pin(async {
                    Err(actix_web::error::ErrorUnauthorized(
                        "missing or invalid authorization header",
                    ))
                });
            }
        };

        Box::pin(async move {
            let session: SessionEntity = session_db::get_session_by_token(pool.get_ref(), &token)
                .await
                .map_err(|_| {
                    actix_web::error::ErrorInternalServerError(format!("database error fetching session [{}]", token))
                })?
                .ok_or_else(|| {
                    actix_web::error::ErrorUnauthorized(format!("invalid or expired session [{}]", token))
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
