use reqwest::Response;
use serde::de::DeserializeOwned;
use shared::error::AppError;
use shared::http::CONTENT_TYPE_MSGPACK;

/// Deserialization utility specifically suited for the "reqwest" library (used on the client)
pub async fn deserialize_response<T: DeserializeOwned>(
    response: Response,
) -> Result<T, AppError> {
    let content_type: Option<String> = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.to_string());

    let bytes: Vec<u8> = response.bytes().await?.to_vec();

    if content_type.as_deref().is_some_and(|ct| ct.contains(CONTENT_TYPE_MSGPACK)) {
        let result: T = rmp_serde::from_slice(&bytes)
            .map_err(|error| AppError::from_error_default(Box::new(error)))?;
        Ok(result)
    } else {
        let result: T = serde_json::from_slice(&bytes)
            .map_err(|error| AppError::from_error_default(Box::new(error)))?;
        Ok(result)
    }
}
