use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use sqlx::PgPool;
use std::future::Future;
use std::pin::Pin;
use uuid::Uuid;

/// Actix-web extractor that validates the Authorization header and resolves
/// to an authenticated account. Add this as a handler parameter to require
/// authentication.
pub struct AuthenticatedAccount {
    pub account_id: Uuid,
    pub session_id: Uuid,
    pub token: String,
}

impl FromRequest for AuthenticatedAccount {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(request: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let pool = request.app_data::<web::Data<PgPool>>().cloned();
        let auth_header = request
            .headers()
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            .map(|string| string.to_string());

        Box::pin(async move {
            let pool = pool.ok_or_else(|| actix_web::error::ErrorInternalServerError("missing pool"))?;

            let header = auth_header
                .ok_or_else(|| actix_web::error::ErrorUnauthorized("missing authorization header"))?;

            let token = header
                .strip_prefix("Bearer ")
                .ok_or_else(|| actix_web::error::ErrorUnauthorized("invalid authorization format"))?;

            let session = super::session_db::get_session_by_token(pool.get_ref(), token)
                .await
                .map_err(|_| actix_web::error::ErrorInternalServerError("db error"))?
                .ok_or_else(|| actix_web::error::ErrorUnauthorized("invalid or expired session"))?;

            // Debounced sliding window: refresh when near expiry
            let refresh_threshold = chrono::Duration::days(super::SESSION_REFRESH_THRESHOLD_DAYS);
            if session.expires - chrono::Utc::now() < refresh_threshold {
                let new_expires = chrono::Utc::now() + chrono::Duration::days(super::SESSION_DURATION_DAYS);
                let _ = super::session_db::refresh_session(pool.get_ref(), token, new_expires).await;
            }

            Ok(AuthenticatedAccount {
                account_id: session.account_id,
                session_id: session.id,
                token: session.token,
            })
        })
    }
}
