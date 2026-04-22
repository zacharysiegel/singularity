use actix_web::http::header;
use actix_web::{FromRequest, HttpRequest, dev::Payload, web};
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
            None => return Box::pin(async { Err(actix_web::error::ErrorInternalServerError("missing pool")) }),
        };

        let auth_header: String = match request
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .map(|string| string.to_string())
        {
            Some(header) => header,
            None => return Box::pin(async { Err(actix_web::error::ErrorUnauthorized("missing authorization header")) }),
        };

        let token: String = match auth_header.strip_prefix("Bearer ") {
            Some(token) => token.to_string(),
            None => return Box::pin(async { Err(actix_web::error::ErrorUnauthorized("invalid authorization format")) }),
        };

        Box::pin(async move {
            let session: super::session_model::SessionEntity =
                super::session_db::get_session_by_token(pool.get_ref(), &token)
                    .await
                    .map_err(|_| actix_web::error::ErrorInternalServerError("db error"))?
                    .ok_or_else(|| actix_web::error::ErrorUnauthorized("invalid or expired session"))?;

            // Debounced sliding window: refresh when near expiry
            let refresh_threshold: chrono::Duration = chrono::Duration::days(super::SESSION_REFRESH_THRESHOLD_DAYS);
            if session.expires - chrono::Utc::now() < refresh_threshold {
                let new_expires: chrono::DateTime<chrono::Utc> =
                    chrono::Utc::now() + chrono::Duration::days(super::SESSION_DURATION_DAYS);
                let _ = super::session_db::refresh_session(pool.get_ref(), &token, new_expires).await;
            }

            Ok(AuthenticatedAccount {
                account_id: session.account_id,
                session_id: session.id,
                token: session.token,
            })
        })
    }
}
