use shared::environment::RuntimeEnvironment;
use shared::error::AppErrorStatic;
use shared::schema::account::{AccountPublicSerial, AccountSerial};
use uuid::Uuid;

use crate::http;

pub async fn fetch_own_account(token: &str) -> Result<AccountSerial, AppErrorStatic> {
    let lobby_http_origin: String = RuntimeEnvironment::default().lobby_http_origin();
    let url: String = format!("{lobby_http_origin}/account");
    http::fetch_standard(token, &url, "own account").await
}

pub async fn fetch_account(token: &str, account_id: Uuid) -> Result<AccountPublicSerial, AppErrorStatic> {
    let lobby_http_origin: String = RuntimeEnvironment::default().lobby_http_origin();
    let url: String = format!("{lobby_http_origin}/account/{account_id}");
    http::fetch_standard(token, &url, &format!("account; [{account_id}]")).await
}
