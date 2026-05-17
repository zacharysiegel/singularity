use crate::conversation::panel::{ChatPanel, ChatTab, RailControl, ENTRY_HEIGHT};
use crate::input::{
    CharPressHandler, CharPressResult, ClickButton, ClickHandler, ClickResult, HoverHandler, HoverResult,
    KeyPressHandler, KeyPressResult, ScrollHandler, ScrollResult,
};
use crate::state::STATE;
use raylib::consts::KeyboardKey;
use raylib::math::{Rectangle, Vector2};
use raylib::RaylibHandle;
use shared::map::RenderCoord;
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
            ClickButton::Left => handle_left_click(panel_rect, press_position, release_position),
            ClickButton::Middle => handle_middle_click(panel_rect, release_position),
            ClickButton::Right => ClickResult::Pass,
        }
    }
}

fn handle_left_click(panel_rect: Rectangle, press_position: RenderCoord, release_position: RenderCoord) -> ClickResult {
    if !panel_rect.check_collision_point_rec(press_position) {
        return ClickResult::Pass;
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
    let scroll_offset: f32 = STATE.conversation.chat_panel.read().unwrap().conversation_list_state.scroll_region.scroll_offset();

    let press_offset_from_body: f32 = press_position.y - content_body_rect.y + scroll_offset;
    let release_offset_from_entries: f32 = release_position.y - content_body_rect.y + scroll_offset;
    if press_offset_from_body < 0. || release_offset_from_entries < 0. {
        return;
    }

    let press_index: usize = (press_offset_from_body / ENTRY_HEIGHT) as usize;
    let release_index: usize = (release_offset_from_entries / ENTRY_HEIGHT) as usize;
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

        let mut chat_panel: RwLockWriteGuard<ChatPanel> = STATE.conversation.chat_panel.write().unwrap();
        chat_panel.hovered_control_button = hovered_rail_button;
        chat_panel.hovered_conversation_tab = hovered_conversation_tab;
        chat_panel.hovered_conversation_tab_close = hovered_conversation_tab_close;
        chat_panel.hovered_tooltip = hovered_tooltip;
        drop(chat_panel);

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

    let scroll_offset: f32 = STATE.conversation.chat_panel.read().unwrap().conversation_list_state.scroll_region.scroll_offset();
    let entry_count: usize = STATE.conversation.display_order().len();

    let mouse_offset_from_content: f32 = mouse_position.y - content_viewport.y + scroll_offset;
    if mouse_offset_from_content < 0. {
        return None;
    }

    let index: usize = (mouse_offset_from_content / ENTRY_HEIGHT) as usize;
    if index < entry_count {
        Some(index)
    } else {
        None
    }
}

impl KeyPressHandler for ChatPanelInput {
    fn key_press(&mut self, _rl: &mut RaylibHandle, key: KeyboardKey) -> KeyPressResult {
        let mut chat_panel: RwLockWriteGuard<ChatPanel> = STATE.conversation.chat_panel.write().unwrap();

        if key == KeyboardKey::KEY_T {
            chat_panel.toggle();
            return KeyPressResult::Consume;
        }

        if !chat_panel.open {
            return KeyPressResult::Pass;
        }

        if key == KeyboardKey::KEY_ESCAPE {
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

        KeyPressResult::Pass
    }
}

impl CharPressHandler for ChatPanelInput {
    fn char_press(&mut self, _rl: &mut RaylibHandle, _ch: char) -> CharPressResult {
        CharPressResult::Pass
    }
}
