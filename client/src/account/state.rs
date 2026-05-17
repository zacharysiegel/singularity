use dashmap::DashMap;
use shared::schema::account::AccountPublicSerial;
use std::sync::RwLock;
use uuid::Uuid;
use super::catchup;

#[derive(Debug)]
pub struct AccountState {
    /// The authenticated user's account id. Set during catch-up after fetching
    /// `GET /account`; overwritten if the user re-authenticates (e.g. after logout
    /// or reconnect as a different account). Used to determine which messages are
    /// own (right-aligned) vs other (left-aligned).
    pub own_account_id: RwLock<Option<Uuid>>,
    /// Public account info (id + username) keyed by account_id. Filled eagerly during
    /// catch-up for known members and lazily on demand for unknown senders of messages
    /// which arrive via real-time events.
    pub cache: DashMap<Uuid, AccountPublicSerial>,
    /// Account IDs with an in-flight `GET /account/{id}` request. Inserted before dispatch,
    /// removed after the request completes (success or failure). Prevents fan-out when
    /// multiple events from the same uncached sender arrive concurrently.
    pub in_flight_account_lookups: DashMap<Uuid, ()>,
}

impl AccountState {
    pub fn new() -> Self {
        AccountState {
            own_account_id: RwLock::new(None),
            cache: DashMap::new(),
            in_flight_account_lookups: DashMap::new(),
        }
    }

    /// Returns the cached username for `account_id` if present. On miss, spawns a deduped
    /// lazy fetch and returns `None`; subsequent reads (after the fetch completes) will
    /// see the cached value.
    pub fn request_username(&self, account_id: Uuid) -> Option<String> {
        if let Some(entry) = self.cache.get(&account_id) {
            return Some(entry.username.clone());
        }

        catchup::spawn_fetch_if_missing(account_id);
        None
    }
}
