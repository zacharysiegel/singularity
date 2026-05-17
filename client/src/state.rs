use crate::account::AccountState;
use crate::conversation::ConversationState;
use crate::stage::StageState;
use crate::texture::ScreenRenderTexture;
use crate::ws::state::WsState;
use http::header;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use shared::environment::RuntimeEnvironment;
use std::mem;
use std::sync::{LazyLock, RwLock};

pub static STATE: LazyLock<State> = LazyLock::new(|| State {
    stage: StageState::DEFAULT,
    frame_counter: RwLock::new(0),
    screen_texture: RwLock::new(unsafe { mem::zeroed() }),
    ws: WsState::default(),
    conversation: ConversationState::new(),
    account: AccountState::new(),
});

pub static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    let content_type: &str = RuntimeEnvironment::default().content_type();
    let mut default_headers: HeaderMap = HeaderMap::new();
    default_headers.insert(header::ACCEPT, HeaderValue::from_static(content_type));

    Client::builder().default_headers(default_headers).build().expect("failed to construct HTTP client")
});

#[derive(Debug)]
pub struct State {
    pub stage: StageState,
    pub frame_counter: RwLock<u64>,
    pub screen_texture: RwLock<ScreenRenderTexture>,
    pub ws: WsState,
    pub conversation: ConversationState,
    pub account: AccountState,
}

#[derive(Debug, PartialEq)]
pub enum Loading {
    Incomplete,
    Complete,
}
