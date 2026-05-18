use crate::conversation::panel::{ChatPanel, ChatTab, ConversationViewState, RailControl, ENTRY_HEIGHT};
use crate::input::{
    CharPressHandler, CharPressResult, ClickButton, ClickHandler, ClickResult, HoverHandler, HoverResult,
    KeyPressHandler, KeyPressResult, ScrollHandler, ScrollResult,
};
use crate::state::STATE;
use crate::ws;
use raylib::consts::KeyboardKey;
use raylib::math::{Rectangle, Vector2};
use raylib::RaylibHandle;
use shared::map::RenderCoord;
use shared::schema::ws_message::{ConnectionType, WsRequest};
use std::sync::RwLockWriteGuard;
use strum::IntoEnumIterator;
use uuid::Uuid;

pub struct ChatPanelInput;

impl ScrollHandler for ChatPanelInput {
    fn scroll(&mut self, rl: &mut RaylibHandle, scroll_v: Vector2, mouse_position: RenderCoord) -> ScrollResult {
        if !STATE.conversation.chat_panel.read().unwrap().open {
            return ScrollResult::Pass;
        }

        let panel_rect: Rectangle = ChatPanel::panel_rect(rl.get_screen_width() as f32, rl.get_screen_height() as f32);
        if !panel_rect.check_collision_point_rec(mouse_position) {
            return ScrollResult::Pass;
        }

        let active_tab: ChatTab = STATE.conversation.chat_panel.read().unwrap().active_tab.clone();
        let mut chat_panel: RwLockWriteGuard<ChatPanel> = STATE.conversation.chat_panel.write().unwrap();
        match active_tab {
            ChatTab::ConversationList => {
                chat_panel.conversation_list_state.scroll_region.scroll(rl, scroll_v, mouse_position);
            }
            ChatTab::Conversation(conversation_id) => {
                if let Some(view_state) = chat_panel.conversation_view_states.get_mut(&conversation_id) {
                    view_state.scroll_region.scroll(rl, scroll_v, mouse_position);
                }
            }
            ChatTab::NewConversation => {}
        }

        ScrollResult::Consume
    }
}

impl ClickHandler for ChatPanelInput {
    fn click(&mut self, rl: &mut RaylibHandle, button: ClickButton, press_position: RenderCoord, release_position: RenderCoord) -> ClickResult {
        if !STATE.conversation.chat_panel.read().unwrap().open {
            return ClickResult::Pass;
        }

        let panel_rect: Rectangle = ChatPanel::panel_rect(rl.get_screen_width() as f32, rl.get_screen_height() as f32);

        match button {
            ClickButton::Left => handle_left_click(rl, panel_rect, press_position, release_position),
            ClickButton::Middle => handle_middle_click(panel_rect, release_position),
            ClickButton::Right => ClickResult::Pass,
        }
    }
}

fn handle_left_click(rl: &mut RaylibHandle, panel_rect: Rectangle, press_position: RenderCoord, release_position: RenderCoord) -> ClickResult {
    let send_rect: Rectangle = ChatPanel::send_button_rect(panel_rect);
    if matches!(active_conversation_id(), Some(_))
        && send_rect.check_collision_point_rec(press_position)
        && send_rect.check_collision_point_rec(release_position)
    {
        send_active_message();
        return ClickResult::Consume;
    }

    // Always let the active conversation's message input see Left clicks: it focuses on
    // inside-rect clicks and unfocuses on outside ones. Doing this before the panel-rect
    // check means a click that drags out of the panel still unfocuses the input.
    let input_result: ClickResult = delegate_message_input_click(rl, ClickButton::Left, press_position, release_position);

    if !panel_rect.check_collision_point_rec(press_position) {
        return ClickResult::Pass;
    }

    if let ClickResult::Consume = input_result {
        return ClickResult::Consume;
    }

    if let ClickResult::Consume = handle_rail_control_click(panel_rect, press_position, release_position) {
        return ClickResult::Consume;
    }

    if let ClickResult::Consume = handle_conversation_tab_click(panel_rect, press_position, release_position) {
        return ClickResult::Consume;
    }

    let scroll_viewport: Rectangle = ChatPanel::content_body_rectangle(panel_rect);
    if scroll_viewport.check_collision_point_rec(press_position)
        && scroll_viewport.check_collision_point_rec(release_position)
    {
        on_content_click(panel_rect, press_position, release_position);
    }

    ClickResult::Consume
}

fn handle_rail_control_click(panel_rect: Rectangle, press_position: RenderCoord, release_position: RenderCoord) -> ClickResult {
    for button in RailControl::iter() {
        let rect: Rectangle = ChatPanel::rail_control_rect(panel_rect, button);
        if !(rect.check_collision_point_rec(press_position) && rect.check_collision_point_rec(release_position)) {
            continue;
        }

        let mut chat_panel: RwLockWriteGuard<ChatPanel> = STATE.conversation.chat_panel.write().unwrap();
        match button {
            RailControl::Close => chat_panel.open = false,
            RailControl::List => chat_panel.active_tab = ChatTab::ConversationList,
            RailControl::New => chat_panel.active_tab = ChatTab::NewConversation,
        }
        return ClickResult::Consume;
    }
    ClickResult::Pass
}

fn handle_conversation_tab_click(panel_rect: Rectangle, press_position: RenderCoord, release_position: RenderCoord) -> ClickResult {
    let conversation_tabs: Vec<Uuid> =
        STATE.conversation.chat_panel.read().unwrap().conversation_tabs.clone();
    let hovered_conversation_tab: Option<usize> =
        STATE.conversation.chat_panel.read().unwrap().hovered_conversation_tab;
    for (index, conversation_id) in conversation_tabs.iter().enumerate() {
        let tab_rect: Rectangle = ChatPanel::rail_conversation_rect(panel_rect, index);
        let rect_pressed_and_released = |r: Rectangle| -> bool {
            r.check_collision_point_rec(press_position) && r.check_collision_point_rec(release_position)
        };

        let clicked_tab: bool = rect_pressed_and_released(tab_rect);
        let clicked_tooltip: bool = if hovered_conversation_tab == Some(index) {
            rect_pressed_and_released(ChatPanel::tooltip_rect(panel_rect, index))
        } else {
            false
        };
        if !clicked_tab && !clicked_tooltip {
            continue;
        }

        let close_rect: Rectangle = ChatPanel::rail_conversation_close_rect(tab_rect);
        if close_rect.check_collision_point_rec(press_position)
            && close_rect.check_collision_point_rec(release_position)
        {
            STATE.conversation.chat_panel.write().unwrap().dismiss_conversation_tab(*conversation_id);
            return ClickResult::Consume;
        }

        STATE.conversation.chat_panel.write().unwrap().open_conversation(*conversation_id);
        STATE.conversation.mark_as_read(*conversation_id);
        return ClickResult::Consume;
    }
    ClickResult::Pass
}

fn handle_middle_click(panel_rect: Rectangle, position: RenderCoord) -> ClickResult {
    if !panel_rect.check_collision_point_rec(position) {
        return ClickResult::Pass;
    }

    let conversation_tabs: Vec<Uuid> =
        STATE.conversation.chat_panel.read().unwrap().conversation_tabs.clone();
    for (index, conversation_id) in conversation_tabs.iter().enumerate() {
        let tab_rect: Rectangle = ChatPanel::rail_conversation_rect(panel_rect, index);
        if tab_rect.check_collision_point_rec(position) {
            STATE.conversation.chat_panel.write().unwrap().dismiss_conversation_tab(*conversation_id);
            return ClickResult::Consume;
        }
    }

    ClickResult::Consume
}

fn on_content_click(panel_rect: Rectangle, press_position: RenderCoord, release_position: RenderCoord) {
    let active_tab: ChatTab = STATE.conversation.chat_panel.read().unwrap().active_tab.clone();
    match active_tab {
        ChatTab::ConversationList => on_conversation_list_click(panel_rect, press_position, release_position),
        _ => {}
    }
}

fn on_conversation_list_click(panel_rect: Rectangle, press_position: RenderCoord, release_position: RenderCoord) {
    let content_body_rect: Rectangle = ChatPanel::content_body_rectangle(panel_rect);
    if !content_body_rect.check_collision_point_rec(press_position)
        || !content_body_rect.check_collision_point_rec(release_position)
    {
        return;
    }

    let chat_panel: std::sync::RwLockReadGuard<ChatPanel> = STATE.conversation.chat_panel.read().unwrap();
    let press_content_y: Option<f32> = chat_panel.conversation_list_state.scroll_region.screen_to_content_y(press_position.y);
    let release_content_y: Option<f32> = chat_panel.conversation_list_state.scroll_region.screen_to_content_y(release_position.y);
    drop(chat_panel);

    let (Some(press_content_y), Some(release_content_y)) = (press_content_y, release_content_y) else {
        return;
    };

    let press_index: usize = (press_content_y / ENTRY_HEIGHT) as usize;
    let release_index: usize = (release_content_y / ENTRY_HEIGHT) as usize;
    if press_index != release_index {
        return;
    }

    let conversation_id: Option<Uuid> = STATE.conversation.display_order()
        .get(release_index)
        .copied();
    let Some(conversation_id) = conversation_id else {
        log::error!("Conversation not found; [{release_index}]");
        return;
    };

    STATE.conversation.chat_panel.write().unwrap().open_conversation(conversation_id);
    STATE.conversation.mark_as_read(conversation_id);
}

impl HoverHandler for ChatPanelInput {
    fn hover(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
        if !STATE.conversation.chat_panel.read().unwrap().open {
            return HoverResult::Pass;
        }

        let panel_rect: Rectangle = ChatPanel::panel_rect(
            rl.get_screen_width() as f32,
            rl.get_screen_height() as f32,
        );
        if !panel_rect.check_collision_point_rec(mouse_position) {
            return HoverResult::Pass;
        }

        let hovered_rail_button: Option<RailControl> = find_hovered_rail_button(panel_rect, mouse_position);
        let (hovered_conversation_tab, hovered_conversation_tab_close, hovered_tooltip): (Option<usize>, Option<usize>, bool) =
            find_conversation_tab_hover_state(panel_rect, mouse_position);
        let send_button_hovered: bool = matches!(active_conversation_id(), Some(_))
            && ChatPanel::send_button_rect(panel_rect).check_collision_point_rec(mouse_position);

        let mut chat_panel: RwLockWriteGuard<ChatPanel> = STATE.conversation.chat_panel.write().unwrap();
        chat_panel.hovered_control_button = hovered_rail_button;
        chat_panel.hovered_conversation_tab = hovered_conversation_tab;
        chat_panel.hovered_conversation_tab_close = hovered_conversation_tab_close;
        chat_panel.hovered_tooltip = hovered_tooltip;
        chat_panel.send_button_hovered = send_button_hovered;
        drop(chat_panel);

        with_active_message_input(|input| input.hover(rl, mouse_position));

        handle_content_hover(panel_rect, mouse_position);

        HoverResult::Consume
    }
}

fn find_hovered_rail_button(panel_rect: Rectangle, mouse_position: RenderCoord) -> Option<RailControl> {
    RailControl::iter().find(|button| {
        ChatPanel::rail_control_rect(panel_rect, *button).check_collision_point_rec(mouse_position)
    })
}

fn find_conversation_tab_hover_state(
    panel_rect: Rectangle,
    mouse_position: RenderCoord,
) -> (Option<usize>, Option<usize>, bool) {
    let chat_panel: std::sync::RwLockReadGuard<ChatPanel> = STATE.conversation.chat_panel.read().unwrap();
    let conversation_tab_count: usize = chat_panel.conversation_tabs.len();
    let previous_hovered_conversation_tab: Option<usize> = chat_panel.hovered_conversation_tab;
    drop(chat_panel);

    let tab_under_cursor: Option<usize> = (0..conversation_tab_count).find(|index| {
        ChatPanel::rail_conversation_rect(panel_rect, *index).check_collision_point_rec(mouse_position)
    });
    let hovered_conversation_tab: Option<usize> = match (tab_under_cursor, previous_hovered_conversation_tab) {
        (Some(index), _) => Some(index),
        (None, Some(previous_index)) => {
            let tooltip_rect: Rectangle = ChatPanel::tooltip_rect(panel_rect, previous_index);
            if tooltip_rect.check_collision_point_rec(mouse_position) {
                Some(previous_index)
            } else {
                None
            }
        }
        (None, None) => None,
    };

    let (hovered_conversation_tab_close, hovered_tooltip): (Option<usize>, bool) = match hovered_conversation_tab {
        Some(index) => {
            let tab_rect: Rectangle = ChatPanel::rail_conversation_rect(panel_rect, index);
            let close_rect: Rectangle = ChatPanel::rail_conversation_close_rect(tab_rect);
            let tooltip_rect: Rectangle = ChatPanel::tooltip_rect(panel_rect, index);
            let close_hovered: Option<usize> = if close_rect.check_collision_point_rec(mouse_position) {
                Some(index)
            } else {
                None
            };
            (
                close_hovered,
                tooltip_rect.check_collision_point_rec(mouse_position),
            )
        }
        None => (None, false),
    };

    (hovered_conversation_tab, hovered_conversation_tab_close, hovered_tooltip)
}

fn handle_content_hover(panel_rect: Rectangle, mouse_position: RenderCoord) {
    let active_tab: ChatTab = STATE.conversation.chat_panel.read().unwrap().active_tab.clone();
    match active_tab {
        ChatTab::ConversationList => handle_conversation_list_hover(panel_rect, mouse_position),
        ChatTab::NewConversation => {}
        ChatTab::Conversation(_) => {}
    }
}

fn handle_conversation_list_hover(panel_rect: Rectangle, mouse_position: RenderCoord) {
    let occluded_by_tab: bool = STATE.conversation.chat_panel.read().unwrap().hovered_conversation_tab.is_some();
    let hovered_entry: Option<usize> = if occluded_by_tab {
        None
    } else {
        find_conversation_list_hovered_entry(panel_rect, mouse_position)
    };
    STATE.conversation.chat_panel.write().unwrap().conversation_list_state.hovered_entry = hovered_entry;
}

fn find_conversation_list_hovered_entry(panel_rect: Rectangle, mouse_position: RenderCoord) -> Option<usize> {
    let content_viewport: Rectangle = ChatPanel::content_body_rectangle(panel_rect);
    if !content_viewport.check_collision_point_rec(mouse_position) {
        return None;
    }

    let chat_panel: std::sync::RwLockReadGuard<ChatPanel> = STATE.conversation.chat_panel.read().unwrap();
    let content_y: f32 = chat_panel.conversation_list_state.scroll_region.screen_to_content_y(mouse_position.y)?;
    drop(chat_panel);

    let entry_count: usize = STATE.conversation.display_order().len();
    let index: usize = (content_y / ENTRY_HEIGHT) as usize;
    if index < entry_count {
        Some(index)
    } else {
        None
    }
}

impl KeyPressHandler for ChatPanelInput {
    fn key_press(&mut self, rl: &mut RaylibHandle, key: KeyboardKey) -> KeyPressResult {
        let active_input_focused: bool = is_active_message_input_focused();

        if key == KeyboardKey::KEY_T && !active_input_focused {
            STATE.conversation.chat_panel.write().unwrap().toggle();
            return KeyPressResult::Consume;
        }

        if !STATE.conversation.chat_panel.read().unwrap().open {
            return KeyPressResult::Pass;
        }

        if key == KeyboardKey::KEY_ESCAPE {
            if active_input_focused {
                with_active_message_input(|input| input.focused = false);
                return KeyPressResult::Consume;
            }
            let mut chat_panel: RwLockWriteGuard<ChatPanel> = STATE.conversation.chat_panel.write().unwrap();
            match &chat_panel.active_tab {
                ChatTab::ConversationList => {
                    chat_panel.open = false;
                }
                ChatTab::Conversation(_) | ChatTab::NewConversation => {
                    chat_panel.active_tab = ChatTab::ConversationList;
                }
            }
            return KeyPressResult::Consume;
        }

        if active_input_focused && key == KeyboardKey::KEY_ENTER {
            send_active_message();
            return KeyPressResult::Consume;
        }

        if active_input_focused {
            return delegate_message_input_key_press(rl, key);
        }

        KeyPressResult::Pass
    }
}

impl CharPressHandler for ChatPanelInput {
    fn char_press(&mut self, rl: &mut RaylibHandle, ch: char) -> CharPressResult {
        if !is_active_message_input_focused() {
            return CharPressResult::Pass;
        }
        delegate_message_input_char_press(rl, ch)
    }
}

fn active_conversation_id() -> Option<Uuid> {
    match STATE.conversation.chat_panel.read().unwrap().active_tab {
        ChatTab::Conversation(id) => Some(id),
        _ => None,
    }
}

fn is_active_message_input_focused() -> bool {
    let chat_panel: std::sync::RwLockReadGuard<ChatPanel> = STATE.conversation.chat_panel.read().unwrap();
    let ChatTab::Conversation(id) = chat_panel.active_tab else {
        return false;
    };
    chat_panel
        .conversation_view_states
        .get(&id)
        .map(|view_state| view_state.message_input.focused)
        .unwrap_or(false)
}

fn with_active_message_input<F, R>(action: F) -> Option<R>
where
    F: FnOnce(&mut crate::component::text_input::TextInput) -> R,
{
    let id: Uuid = active_conversation_id()?;
    let mut chat_panel: RwLockWriteGuard<ChatPanel> = STATE.conversation.chat_panel.write().unwrap();
    let view_state: &mut ConversationViewState = chat_panel.conversation_view_states.get_mut(&id)?;
    Some(action(&mut view_state.message_input))
}

fn delegate_message_input_key_press(rl: &mut RaylibHandle, key: KeyboardKey) -> KeyPressResult {
    with_active_message_input(|input| input.key_press(rl, key)).unwrap_or(KeyPressResult::Pass)
}

fn delegate_message_input_char_press(rl: &mut RaylibHandle, ch: char) -> CharPressResult {
    with_active_message_input(|input| input.char_press(rl, ch)).unwrap_or(CharPressResult::Pass)
}

fn delegate_message_input_click(
    rl: &mut RaylibHandle,
    button: ClickButton,
    press_position: RenderCoord,
    release_position: RenderCoord,
) -> ClickResult {
    with_active_message_input(|input| input.click(rl, button, press_position, release_position))
        .unwrap_or(ClickResult::Pass)
}

fn send_active_message() {
    let id: Option<Uuid> = active_conversation_id();
    let Some(id) = id else { return };

    let content: String = with_active_message_input(|input| {
        let body: String = input.text.content.clone();
        input.clear();
        body
    })
    .unwrap_or_default();
    if content.trim().is_empty() {
        return;
    }

    ws::lifecycle::send(ConnectionType::Lobby, WsRequest::Chat { conversation_id: id, content });
}
