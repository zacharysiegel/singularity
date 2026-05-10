use crate::conversation::panel::{ChatPanel, ChatTab, RailButton};
use crate::input::{
    CharPressHandler, CharPressResult, ClickHandler, ClickResult, HoverHandler, HoverResult,
    KeyPressHandler, KeyPressResult, ScrollHandler, ScrollResult,
};
use crate::state::STATE;
use crate::window::{BORDER_GAP, BUTTON_WIDTH};
use raylib::consts::KeyboardKey;
use raylib::math::{Rectangle, Vector2};
use raylib::RaylibHandle;
use shared::map::RenderCoord;
use std::sync::RwLockWriteGuard;
use strum::IntoEnumIterator;

pub struct ChatPanelInput;

impl ScrollHandler for ChatPanelInput {
    fn scroll(&mut self, rl: &mut RaylibHandle, _scroll_v: Vector2, mouse_position: RenderCoord) -> ScrollResult {
        if !STATE.conversation.chat_panel.read().unwrap().open {
            return ScrollResult::Pass;
        }

        let mouse_hovered: bool = ChatPanel::panel_rectangle(rl.get_screen_width() as f32, rl.get_screen_height() as f32)
            .check_collision_point_rec(mouse_position);
        match mouse_hovered {
            true => ScrollResult::Consume,
            false => ScrollResult::Pass,
        }
    }
}
impl ClickHandler for ChatPanelInput {
    fn click(&mut self, rl: &mut RaylibHandle, press_position: RenderCoord, release_position: RenderCoord) -> ClickResult {
        if !STATE.conversation.chat_panel.read().unwrap().open {
            return ClickResult::Pass;
        }

        let panel_rect: Rectangle = ChatPanel::panel_rectangle(rl.get_screen_width() as f32, rl.get_screen_height() as f32);
        // Releasing outside after pressing inside should not propagate to underlying handlers.
        if !panel_rect.check_collision_point_rec(press_position) {
            return ClickResult::Pass;
        }

        let close_rect: Rectangle = Rectangle {
            x: panel_rect.x + panel_rect.width - BUTTON_WIDTH - BORDER_GAP,
            y: panel_rect.y + BORDER_GAP,
            width: BUTTON_WIDTH,
            height: BUTTON_WIDTH,
        };

        if close_rect.check_collision_point_rec(press_position)
            && close_rect.check_collision_point_rec(release_position)
        {
            let mut chat_panel: RwLockWriteGuard<ChatPanel> = STATE.conversation.chat_panel.write().unwrap();
            chat_panel.open = false;
        }

        ClickResult::Consume
    }
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

        let mut chat_panel: RwLockWriteGuard<ChatPanel> = STATE.conversation.chat_panel.write().unwrap();
        chat_panel.hovered_rail_button = hovered_button;

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
