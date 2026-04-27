use shared::error::AppError;
use shared::schema::account::CreateAccountRequest;
use shared::schema::session::LoginRequest;

const DEBUG_USERNAME: &str = "debug";
const DEBUG_EMAIL: &str = "singularity-debug@zach.ro";

async fn create_account(lobby_url: &str, password: &str) -> Result<(), AppError> {
    let url: String = format!("{}/account", lobby_url);
    let create_request: CreateAccountRequest = CreateAccountRequest {
        email: DEBUG_EMAIL.to_string(),
        username: DEBUG_USERNAME.to_string(),
        password: password.to_string(),
    };

    let client: reqwest::Client = reqwest::Client::new();
    let response: reqwest::Response = client
        .post(&url)
        .json(&create_request)
        .send()
        .await
        .map_err(|error| AppError::new(&format!("create account request failed: {error}")))?;

    if !response.status().is_success() {
        return Err(AppError::new(&format!(
            "create account failed with status {}",
            response.status()
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

    let client: reqwest::Client = reqwest::Client::new();
    let response: reqwest::Response = client
        .post(&url)
        .json(&login_request)
        .send()
        .await
        .map_err(|error| AppError::new(&format!("login request failed: {error}")))?;

    if !response.status().is_success() {
        return Err(AppError::new(&format!(
            "login failed with status {}",
            response.status()
        )));
    }

    let body: serde_json::Value = response
        .json()
        .await
        .map_err(|error| AppError::new(&format!("failed to parse login response: {error}")))?;

    let token: &str = body["token"].as_str().ok_or_else(|| AppError::new("login response missing token field"))?;

    Ok(token.to_string())
}

pub async fn debug_authenticate(lobby_url: &str) -> Result<String, AppError> {
    let password: String = dotenvy::var("DEBUG_ACCOUNT_PASSWORD")
        .map_err(|error| AppError::new(&format!("DEBUG_ACCOUNT_PASSWORD not set: {error}")))?;

    let first_login: Option<String> = login(lobby_url, &password).await.ok();
    if let Some(token) = first_login {
        return Ok(token);
    }

    log::info!("Debug account does not exist, creating it");
    create_account(lobby_url, &password).await?;
    login(lobby_url, &password).await
}
