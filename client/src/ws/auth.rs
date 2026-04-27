use crate::state::HTTP_CLIENT;
use shared::error::AppError;
use shared::schema::account::CreateAccountRequest;
use shared::schema::session::LoginRequest;

const DEBUG_USERNAME: &str = "debug";
const DEBUG_EMAIL: &str = "singularity-debug@zach.ro";
const DEBUG_ACCOUNT_PASSWORD_KEY: &str = "DEBUG_ACCOUNT_PASSWORD";

async fn create_account(lobby_url: &str, password: &str) -> Result<(), AppError> {
    let url: String = format!("{}/account", lobby_url);
    let create_request: CreateAccountRequest = CreateAccountRequest {
        email: DEBUG_EMAIL.to_string(),
        username: DEBUG_USERNAME.to_string(),
        password: password.to_string(),
    };

    let response: reqwest::Response = HTTP_CLIENT
        .post(&url)
        .json(&create_request)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(AppError::new(&format!(
            "create account failed with status {}; [{}]",
            response.status(),
            response.text().await?
        )));
    }
    Ok(())
}

async fn login(lobby_url: &str, password: &str) -> Result<String, AppError> {
    let url: String = format!("{}/session", lobby_url);
    let login_request: LoginRequest = LoginRequest {
        email: DEBUG_EMAIL.to_string(),
        password: password.to_string(),
    };

    let response: reqwest::Response = HTTP_CLIENT
        .post(&url)
        .json(&login_request)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(AppError::new(&format!(
            "login failed with status {}",
            response.status()
        )));
    }

    let body: serde_json::Value = response.json().await?;
    let token: &str = body["token"].as_str()
        .ok_or_else(|| AppError::new("login response missing token field"))?;

    Ok(token.to_string())
}

pub async fn debug_authenticate(lobby_http_origin: &str) -> Result<String, AppError> {
    let password: String = dotenvy::var(DEBUG_ACCOUNT_PASSWORD_KEY)
        .map_err(|error| AppError::new(&format!("{DEBUG_ACCOUNT_PASSWORD_KEY} not set; [{error}]")))?;

    let token: Option<String> = login(lobby_http_origin, &password).await.ok();
    if let Some(token) = token {
        return Ok(token);
    }

    log::info!("Debug account does not exist, creating it");
    create_account(lobby_http_origin, &password).await?;
    login(lobby_http_origin, &password).await
}
