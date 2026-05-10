use crate::window::{BORDER_GAP, BUTTON_WIDTH};
use raylib::math::Rectangle;
use strum::EnumIter;
use uuid::Uuid;

const PANEL_MAX_WIDTH: f32 = 600.;
pub const HEADER_HEIGHT: f32 = 36.;
pub const ENTRY_HEIGHT: f32 = 44.;

#[derive(Debug, Clone, PartialEq)]
pub enum ChatTab {
    NewConversation,
    ConversationList,
    Conversation(Uuid),
}

#[derive(Debug)]
pub struct ChatPanel {
    pub open: bool,
    pub active_tab: ChatTab,
    /// Conversation IDs of open tabs in the rail, in top-down display order.
    pub conversation_tabs: Vec<Uuid>,
    pub expanded: bool,
    pub hovered_rail_button: Option<RailButton>,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
#[repr(u8)]
pub enum RailButton {
    Close,
    New,
    List,
}

impl ChatPanel {
    pub fn new() -> Self {
        ChatPanel {
            open: false,
            active_tab: ChatTab::ConversationList,
            conversation_tabs: Vec::new(),
            expanded: false,
            hovered_rail_button: None,
        }
    }

    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    pub fn panel_width(screen_width: f32) -> f32 {
        f32::min(screen_width, PANEL_MAX_WIDTH)
    }

    pub fn panel_rectangle(screen_width: f32, screen_height: f32) -> Rectangle {
        let width: f32 = Self::panel_width(screen_width);
        Rectangle {
            x: screen_width - width,
            y: 0.,
            width,
            height: screen_height,
        }
    }

    pub fn rail_button_rect(panel_rect: Rectangle, button: RailButton) -> Rectangle {
        let rail_x: f32 = panel_rect.x + panel_rect.width - BUTTON_WIDTH - BORDER_GAP;
        let rail_y: f32 = panel_rect.y + BORDER_GAP;
        Rectangle {
            x: rail_x,
            y: rail_y + BUTTON_WIDTH * button as u8 as f32,
            width: BUTTON_WIDTH,
            height: BUTTON_WIDTH,
        }
    }

    /// The area to the left of the rail buttons, inside the panel border. Used for rendering
    /// conversation list entries, message views, etc.
    pub fn content_rectangle(panel_rect: Rectangle) -> Rectangle {
        Rectangle {
            x: panel_rect.x + BORDER_GAP,
            y: panel_rect.y + BORDER_GAP,
            width: panel_rect.width - BUTTON_WIDTH - BORDER_GAP * 3.,
            height: panel_rect.height - BORDER_GAP * 2.,
        }
    }
}
