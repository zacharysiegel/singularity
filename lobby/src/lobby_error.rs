use actix_web::http::StatusCode;
use actix_web::mime::APPLICATION_JSON;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use shared::error::{AppError, AppErrorStatic};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Serialize)]
struct ErrorResponseBody {
    status_short: u16,
    status_long: String,
    message: String,
}

pub enum LobbyError {
    BadRequest(AppError),
    Unauthorized(AppError),
    Forbidden(AppError),
    NotFound(AppError),
    Conflict(AppError),
    Internal(AppError),
}

impl LobbyError {
    pub fn bad_request(message: &str) -> Self {
        Self::BadRequest(AppError::new(message))
    }

    pub fn unauthorized(message: &str) -> Self {
        Self::Unauthorized(AppError::new(message))
    }

    pub fn forbidden(message: &str) -> Self {
        Self::Forbidden(AppError::new(message))
    }

    pub fn not_found(message: &str) -> Self {
        Self::NotFound(AppError::new(message))
    }

    pub fn conflict(message: &str) -> Self {
        Self::Conflict(AppError::new(message))
    }

    pub fn internal(message: &str) -> Self {
        Self::Internal(AppError::new(message))
    }

    fn app_error(&self) -> &AppError {
        match self {
            LobbyError::BadRequest(error) => error,
            LobbyError::Unauthorized(error) => error,
            LobbyError::Forbidden(error) => error,
            LobbyError::NotFound(error) => error,
            LobbyError::Conflict(error) => error,
            LobbyError::Internal(error) => error,
        }
    }
}

impl Display for LobbyError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self.app_error(), formatter)
    }
}

impl Debug for LobbyError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.app_error(), formatter)
    }
}

impl ResponseError for LobbyError {
    fn status_code(&self) -> StatusCode {
        match self {
            LobbyError::BadRequest(_) => StatusCode::BAD_REQUEST,
            LobbyError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            LobbyError::Forbidden(_) => StatusCode::FORBIDDEN,
            LobbyError::NotFound(_) => StatusCode::NOT_FOUND,
            LobbyError::Conflict(_) => StatusCode::CONFLICT,
            LobbyError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status: StatusCode = self.status_code();

        match self {
            LobbyError::Internal(error) => log::error!("{}", error),
            LobbyError::BadRequest(error) => log::warn!("{}", error),
            _ => {}
        }

        let body: ErrorResponseBody = ErrorResponseBody {
            status_short: status.as_u16(),
            status_long: status.to_string(),
            message: self.app_error().message.clone(),
        };

        HttpResponse::build(status).content_type(APPLICATION_JSON).json(body)
    }
}

impl From<AppError> for LobbyError {
    fn from(error: AppError) -> Self {
        LobbyError::Internal(error)
    }
}

impl From<AppErrorStatic> for LobbyError {
    fn from(error: AppErrorStatic) -> Self {
        LobbyError::Internal(AppError::from(error))
    }
}

pub trait ResultExt<T> {
    fn or_bad_request(self) -> Result<T, LobbyError>;
}

impl<T, E: Display> ResultExt<T> for Result<T, E> {
    fn or_bad_request(self) -> Result<T, LobbyError> {
        self.map_err(|error| LobbyError::bad_request(&error.to_string()))
    }
}

pub trait OptionExt<T> {
    fn or_not_found(self) -> Result<T, LobbyError>;
    fn or_forbidden(self, message: &str) -> Result<T, LobbyError>;
    fn then_conflict(self, message: &str) -> Result<(), LobbyError>;
}

impl<T> OptionExt<T> for Option<T> {
    fn or_not_found(self) -> Result<T, LobbyError> {
        self.ok_or_else(|| LobbyError::not_found("resource"))
    }

    fn or_forbidden(self, message: &str) -> Result<T, LobbyError> {
        self.ok_or_else(|| LobbyError::forbidden(message))
    }

    fn then_conflict(self, message: &str) -> Result<(), LobbyError> {
        match self {
            Some(_) => Err(LobbyError::conflict(message)),
            None => Ok(()),
        }
    }
}
