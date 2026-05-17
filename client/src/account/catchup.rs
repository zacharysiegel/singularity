use shared::environment::RuntimeEnvironment;
use shared::error::AppErrorStatic;
use shared::schema::account::{AccountPublicSerial, AccountSerial};
use uuid::Uuid;

use crate::http;
use crate::state::STATE;

/// Fetches the authenticated user's account, sets `own_account_id`, and inserts the
/// corresponding public entry into the cache. Idempotent across reconnects (`OnceLock`
/// only sets the first time, cache insert is a write).
pub async fn fetch_own_account(token: &str) {
    let lobby_http_origin: String = RuntimeEnvironment::default().lobby_http_origin();
    let url: String = format!("{lobby_http_origin}/account");
    let result: Result<AccountSerial, AppErrorStatic> =
        http::fetch_standard(token, &url, "own account").await;
    let account: AccountSerial = match result {
        Ok(account) => account,
        Err(error) => {
            log::warn!("Failed to fetch own account; [{error}]");
            return;
        }
    };

    let _ = STATE.account.own_account_id.write().unwrap().replace(account.id);
    let public: AccountPublicSerial = AccountPublicSerial {
        id: account.id,
        username: account.username,
    };
    STATE.account.cache.insert(public.id, public);
}

/// Fills the cache with public account info for any of the given ids that aren't
/// already cached. Performs one `GET /account/{id}` per missing id sequentially —
/// catch-up isn't on a hot path.
pub async fn fetch_missing_accounts(token: &str, account_ids: &[Uuid]) {
    for account_id in account_ids {
        if STATE.account.cache.contains_key(account_id) {
            continue;
        }
        fetch_account(token, *account_id).await;
    }
}

/// Spawn a deduped lazy fetch for an account that isn't currently cached. Safe to call
/// from synchronous contexts (WS event handlers). No-op if the id is already cached or
/// already in flight.
pub fn spawn_fetch_if_missing(account_id: Uuid) {
    if STATE.account.cache.contains_key(&account_id) {
        return;
    }
    if STATE.account.in_flight.insert(account_id, ()).is_some() {
        return;
    }

    let token: Option<String> = STATE.ws.token.read().unwrap().clone();
    let Some(token) = token else {
        STATE.account.in_flight.remove(&account_id);
        return;
    };

    tokio::spawn(async move {
        fetch_account(&token, account_id).await;
        STATE.account.in_flight.remove(&account_id);
    });
}

async fn fetch_account(token: &str, account_id: Uuid) {
    let lobby_http_origin: String = RuntimeEnvironment::default().lobby_http_origin();
    let url: String = format!("{lobby_http_origin}/account/{account_id}");
    let result: Result<AccountPublicSerial, AppErrorStatic> =
        http::fetch_standard(token, &url, &format!("account; [{account_id}]")).await;
    match result {
        Ok(public) => {
            STATE.account.cache.insert(public.id, public);
        }
        Err(error) => {
            log::warn!("Failed to fetch account; [{account_id}] [{error}]");
        }
    }
}
