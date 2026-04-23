use actix_web::http::{StatusCode, header};
use actix_web::{HttpRequest, HttpResponse};
use serde::Serialize;
use shared::error::AppError;

const CONTENT_TYPE_JSON: &str = "application/json";
const CONTENT_TYPE_MSGPACK: &str = "application/msgpack";

pub fn serialize_response<T: Serialize>(request: &HttpRequest, body: &T) -> HttpResponse {
    serialize_response_with_status(request, body, StatusCode::OK)
}

pub fn serialize_response_with_status<T: Serialize>(
    request: &HttpRequest,
    body: &T,
    status: StatusCode,
) -> HttpResponse {
    let accept: &str = request
        .headers()
        .get(header::ACCEPT)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");

    if accept.contains(CONTENT_TYPE_MSGPACK) {
        match rmp_serde::to_vec_named(body) {
            Ok(bytes) => HttpResponse::build(status).content_type(CONTENT_TYPE_MSGPACK).body(bytes),
            Err(error) => {
                log::error!("MessagePack serialization error: {}", error);
                HttpResponse::InternalServerError().finish()
            }
        }
    } else {
        match serde_json::to_vec(body) {
            Ok(bytes) => HttpResponse::build(status).content_type(CONTENT_TYPE_JSON).body(bytes),
            Err(error) => {
                log::error!("JSON serialization error: {}", error);
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}

pub fn deserialize_request<T: serde::de::DeserializeOwned>(
    request: &HttpRequest,
    bytes: &[u8],
) -> Result<T, AppError> {
    let content_type: &str = request
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| AppError::new("missing or invalid content-type header"))?;

    if content_type.contains(CONTENT_TYPE_MSGPACK) {
        rmp_serde::from_slice(bytes).map_err(|error| AppError::from_error_default(Box::new(error)))
    } else if content_type.contains(CONTENT_TYPE_JSON) {
        serde_json::from_slice(bytes).map_err(|error| AppError::from_error_default(Box::new(error)))
    } else {
        Err(AppError::new(&format!("unsupported content-type [{}]", content_type)))
    }
}

pub fn extract_bearer_token<'a>(request: &'a HttpRequest) -> Option<&'a str> {
    let header_value: &str = request
        .headers()
        .get(header::AUTHORIZATION)?
        .to_str()
        .ok()?;
    header_value.strip_prefix("Bearer ")
}

macro_rules! unwrap_or_400 {
    ($expr:expr) => {
        match $expr {
            Ok(value) => value,
            Err(error) => {
                log::warn!("400 Bad Request: {}", error);
                return HttpResponse::BadRequest().finish();
            }
        }
    };
}

macro_rules! unwrap_or_404 {
    ($expr:expr) => {
        match $expr {
            Some(value) => value,
            None => {
                return HttpResponse::NotFound().finish();
            }
        }
    };
}

macro_rules! unwrap_or_500 {
    ($expr:expr) => {
        match $expr {
            Ok(value) => value,
            Err(error) => {
                log::error!("500 Internal Server Error: {}", error);
                return HttpResponse::InternalServerError().finish();
            }
        }
    };
}
