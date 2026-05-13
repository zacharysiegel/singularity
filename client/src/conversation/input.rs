use crate::conversation::draw;
use crate::conversation::panel::{ChatPanel, ChatTab, RailButton, ENTRY_HEIGHT};
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

        let panel_rect: Rectangle = ChatPanel::panel_rectangle(rl.get_screen_width() as f32, rl.get_screen_height() as f32);
        if !panel_rect.check_collision_point_rec(mouse_position) {
            return ScrollResult::Pass;
        }

        let mut chat_panel: RwLockWriteGuard<ChatPanel> = STATE.conversation.chat_panel.write().unwrap();
        chat_panel.list_scroll_region.scroll(rl, scroll_v, mouse_position);

        ScrollResult::Consume
    }
}
impl ClickHandler for ChatPanelInput {
    fn click(&mut self, rl: &mut RaylibHandle, button: ClickButton, press_position: RenderCoord, release_position: RenderCoord) -> ClickResult {
        if !STATE.conversation.chat_panel.read().unwrap().open {
            return ClickResult::Pass;
        }

        let panel_rect: Rectangle = ChatPanel::panel_rectangle(rl.get_screen_width() as f32, rl.get_screen_height() as f32);

        match button {
            ClickButton::Left => on_left_click(panel_rect, press_position, release_position),
            ClickButton::Middle => on_middle_click(panel_rect, release_position),
            ClickButton::Right => ClickResult::Pass,
        }
    }
}

fn on_left_click(panel_rect: Rectangle, press_position: RenderCoord, release_position: RenderCoord) -> ClickResult {
    if !panel_rect.check_collision_point_rec(press_position) {
        return ClickResult::Pass;
    }

    let close_rect: Rectangle = ChatPanel::rail_button_rect(panel_rect, RailButton::Close);
    if close_rect.check_collision_point_rec(press_position)
        && close_rect.check_collision_point_rec(release_position)
    {
        STATE.conversation.chat_panel.write().unwrap().open = false;
        return ClickResult::Consume;
    }

    let list_rect: Rectangle = ChatPanel::rail_button_rect(panel_rect, RailButton::List);
    if list_rect.check_collision_point_rec(press_position)
        && list_rect.check_collision_point_rec(release_position)
    {
        STATE.conversation.chat_panel.write().unwrap().active_tab = ChatTab::ConversationList;
        return ClickResult::Consume;
    }

    let new_rect: Rectangle = ChatPanel::rail_button_rect(panel_rect, RailButton::New);
    if new_rect.check_collision_point_rec(press_position)
        && new_rect.check_collision_point_rec(release_position)
    {
        STATE.conversation.chat_panel.write().unwrap().active_tab = ChatTab::NewConversation;
        return ClickResult::Consume;
    }

    let conversation_tabs: Vec<Uuid> =
        STATE.conversation.chat_panel.read().unwrap().conversation_tabs.clone();
    for (index, conversation_id) in conversation_tabs.iter().enumerate() {
        let tab_rect: Rectangle = ChatPanel::conversation_tab_rect(panel_rect, index);
        let tooltip_rect: Rectangle = ChatPanel::tooltip_name_area_rect(panel_rect, index);
        let rect_contains_both = |r: Rectangle| -> bool {
            r.check_collision_point_rec(press_position) && r.check_collision_point_rec(release_position)
        };
        let clicked_tab: bool = rect_contains_both(tab_rect);
        let clicked_tooltip: bool = rect_contains_both(tooltip_rect);
        if !clicked_tab && !clicked_tooltip {
            continue;
        }

        let close_rect: Rectangle = draw::tab_mini_close_rect(tab_rect);
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

    let scroll_viewport: Rectangle = ChatPanel::content_body_rectangle(panel_rect);
    if scroll_viewport.check_collision_point_rec(press_position)
        && scroll_viewport.check_collision_point_rec(release_position)
    {
        on_content_click(panel_rect, press_position, release_position);
    }

    ClickResult::Consume
}

fn on_middle_click(panel_rect: Rectangle, position: RenderCoord) -> ClickResult {
    if !panel_rect.check_collision_point_rec(position) {
        return ClickResult::Pass;
    }

    let conversation_tabs: Vec<Uuid> =
        STATE.conversation.chat_panel.read().unwrap().conversation_tabs.clone();
    for (index, conversation_id) in conversation_tabs.iter().enumerate() {
        let tab_rect: Rectangle = ChatPanel::conversation_tab_rect(panel_rect, index);
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
    let scroll_offset: f32 = STATE.conversation.chat_panel.read().unwrap().list_scroll_region.scroll_offset();

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

        let panel_rect: Rectangle = ChatPanel::panel_rectangle(
            rl.get_screen_width() as f32,
            rl.get_screen_height() as f32,
        );
        if !panel_rect.check_collision_point_rec(mouse_position) {
            return HoverResult::Pass;
        }

        let hovered_button: Option<RailButton> = RailButton::iter()
            .find(|button| {
                ChatPanel::rail_button_rect(panel_rect, *button)
                    .check_collision_point_rec(mouse_position)
            });

        let scroll_viewport: Rectangle = ChatPanel::content_body_rectangle(panel_rect);
        let scroll_offset: f32 = STATE.conversation.chat_panel.read().unwrap().list_scroll_region.scroll_offset();
        let entry_count: usize = STATE.conversation.display_order().len();

        let conversation_tab_count: usize = STATE.conversation.chat_panel.read().unwrap().conversation_tabs.len();
        let previous_hovered_conversation_tab: Option<usize> =
            STATE.conversation.chat_panel.read().unwrap().hovered_conversation_tab;
        let tab_under_cursor: Option<usize> = (0..conversation_tab_count).find(|index| {
            ChatPanel::conversation_tab_rect(panel_rect, *index).check_collision_point_rec(mouse_position)
        });
        let hovered_conversation_tab: Option<usize> = match (tab_under_cursor, previous_hovered_conversation_tab) {
            (Some(index), _) => Some(index),
            (None, Some(previous_index)) => {
                let tooltip_rect: Rectangle = ChatPanel::tooltip_name_area_rect(panel_rect, previous_index);
                if tooltip_rect.check_collision_point_rec(mouse_position) {
                    Some(previous_index)
                } else {
                    None
                }
            }
            (None, None) => None,
        };
        let hovered_conversation_tab_close: Option<usize> = match hovered_conversation_tab {
            Some(index) => {
                let tab_rect: Rectangle = ChatPanel::conversation_tab_rect(panel_rect, index);
                let close_rect: Rectangle = draw::tab_mini_close_rect(tab_rect);
                if close_rect.check_collision_point_rec(mouse_position) {
                    Some(index)
                } else {
                    None
                }
            }
            None => None,
        };
        let hovered_tooltip: bool = match hovered_conversation_tab {
            Some(index) => ChatPanel::tooltip_name_area_rect(panel_rect, index)
                .check_collision_point_rec(mouse_position),
            None => false,
        };

        let hovered_list_entry: Option<usize> = if hovered_conversation_tab.is_none()
            && scroll_viewport.check_collision_point_rec(mouse_position)
        {
            let mouse_offset_from_body: f32 = mouse_position.y - scroll_viewport.y + scroll_offset;
            if mouse_offset_from_body >= 0. {
                let index: usize = (mouse_offset_from_body / ENTRY_HEIGHT) as usize;
                if index < entry_count {
                    Some(index)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        let mut chat_panel: RwLockWriteGuard<ChatPanel> = STATE.conversation.chat_panel.write().unwrap();
        chat_panel.hovered_rail_button = hovered_button;
        chat_panel.hovered_list_entry = hovered_list_entry;
        chat_panel.hovered_conversation_tab = hovered_conversation_tab;
        chat_panel.hovered_conversation_tab_close = hovered_conversation_tab_close;
        chat_panel.hovered_tooltip = hovered_tooltip;

        HoverResult::Consume
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
