use reqwest::Response;
use serde::de::DeserializeOwned;
use shared::error::{AppError, AppErrorStatic};
use shared::http::CONTENT_TYPE_MSGPACK;

use crate::state::HTTP_CLIENT;

const MAX_RETRY_ATTEMPTS: u32 = 3;

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

pub async fn fetch_standard<T: DeserializeOwned>(
    token: &str,
    url: &str,
    description: &str,
) -> Result<T, AppErrorStatic> {
    shared::http::with_retry(MAX_RETRY_ATTEMPTS, || async {
        let response: Response = HTTP_CLIENT
            .get(url)
            .bearer_auth(token)
            .send()
            .await
            .map_err(|error| AppErrorStatic::new(&error.to_string()))?;

        if !response.status().is_success() {
            return Err(AppErrorStatic::new(&format!(
                "fetch {description} failed; [{}]",
                response.status()
            )));
        }

        deserialize_response(response)
            .await
            .map_err(AppErrorStatic::from)
    })
    .await
}
