use shared::environment::RuntimeEnvironment;
use shared::error::AppErrorStatic;
use shared::schema::account::{AccountPublicSerial, AccountSerial};
use uuid::Uuid;

use super::AccountState;
use crate::http;
use crate::state::STATE;

/// Fetches the authenticated user's account, sets `own_account_id`, and inserts the
/// corresponding public entry into the cache. Idempotent across reconnects.
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

/// Fills the cache with public account info for any of the given ids that aren't already
/// cached. Issues one `GET /account/{id}` per missing id concurrently and joins on all.
pub async fn fetch_missing_accounts(token: &str, account_ids: &[Uuid]) {
    let missing_account_ids: Vec<Uuid> = account_ids
        .iter()
        .copied()
        .filter(|account_id| !STATE.account.cache.contains_key(account_id))
        .collect();
    futures::future::join_all(
        missing_account_ids
            .into_iter()
            .map(|account_id| fetch_account(token, account_id)),
    )
    .await;
}

impl AccountState {
    /// Returns the cached username for `account_id` if present. On miss, spawns a deduped
    /// lazy fetch and returns `None`; subsequent reads (after the fetch completes) will
    /// see the cached value.
    pub fn request_username(&self, account_id: Uuid) -> Option<String> {
        if let Some(entry) = self.cache.get(&account_id) {
            return Some(entry.username.clone());
        }
        spawn_fetch_if_missing(account_id);
        None
    }
}

/// Deduped background fetch for a single account id. No-op if cached or already in flight.
fn spawn_fetch_if_missing(account_id: Uuid) {
    if STATE.account.cache.contains_key(&account_id) {
        return;
    }
    if STATE.account.in_flight_account_lookups.insert(account_id, ()).is_some() {
        return;
    }

    let token: Option<String> = STATE.ws.token.read().unwrap().clone();
    let Some(token) = token else {
        STATE.account.in_flight_account_lookups.remove(&account_id);
        return;
    };

    tokio::spawn(async move {
        fetch_account(&token, account_id).await;
        STATE.account.in_flight_account_lookups.remove(&account_id);
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
