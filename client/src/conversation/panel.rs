use crate::component::vertical_scroll_region::VerticalScrollRegion;
use crate::window::{BORDER_GAP, BUTTON_WIDTH};
use raylib::math::Rectangle;
use strum::EnumIter;
use uuid::Uuid;

const PANEL_MAX_WIDTH: f32 = 600.;
pub const HEADER_HEIGHT: f32 = 46.;
pub const ENTRY_HEIGHT: f32 = 44.;
pub const RAIL_SEPARATOR_GAP: f32 = 6.;
pub const TAB_MINI_CLOSE_SIZE: f32 = 12.;
pub const TAB_MINI_CLOSE_MARGIN: f32 = 2.;

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
    pub hovered_rail_button: Option<RailButton>,
    pub hovered_list_entry: Option<usize>,
    pub hovered_conversation_tab: Option<usize>,
    pub hovered_conversation_tab_close: Option<usize>,
    pub hovered_tooltip: bool,
    pub list_scroll_region: VerticalScrollRegion,
}

/// Variant order determines top-to-bottom position in the rail via discriminant cast.
#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
#[repr(u8)]
pub enum RailButton {
    Close,
    New,
    List,
}

impl ChatPanel {
    pub const TOOLTIP_WIDTH: f32 = 200.;

    pub fn new() -> Self {
        ChatPanel {
            open: false,
            active_tab: ChatTab::ConversationList,
            conversation_tabs: Vec::new(),
            hovered_rail_button: None,
            hovered_list_entry: None,
            hovered_conversation_tab: None,
            hovered_conversation_tab_close: None,
            hovered_tooltip: false,
            list_scroll_region: VerticalScrollRegion::new(Rectangle::default(), 0.),
        }
    }

    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    pub fn open_conversation(&mut self, conversation_id: Uuid) {
        if !self.conversation_tabs.contains(&conversation_id) {
            self.conversation_tabs.push(conversation_id);
        }
        self.active_tab = ChatTab::Conversation(conversation_id);
    }

    pub fn dismiss_conversation_tab(&mut self, conversation_id: Uuid) {
        self.conversation_tabs.retain(|id| *id != conversation_id);
        if self.active_tab == ChatTab::Conversation(conversation_id) {
            self.active_tab = ChatTab::ConversationList;
        }
    }

    pub fn panel_width(screen_width: f32) -> f32 {
        f32::min(screen_width, PANEL_MAX_WIDTH)
    }

    pub fn panel_rect(screen_width: f32, screen_height: f32) -> Rectangle {
        let width: f32 = Self::panel_width(screen_width);
        Rectangle {
            x: screen_width - width,
            y: 0.,
            width,
            height: screen_height,
        }
    }

    /// Button rectangle for persistent rail buttons (e.g. close, new)
    pub fn rail_control_rect(panel_rect: Rectangle, button: RailButton) -> Rectangle {
        let rail_x: f32 = panel_rect.x + panel_rect.width - BUTTON_WIDTH - BORDER_GAP;
        let rail_y: f32 = panel_rect.y + BORDER_GAP;
        Rectangle {
            x: rail_x,
            y: rail_y + BUTTON_WIDTH * button as u8 as f32,
            width: BUTTON_WIDTH,
            height: BUTTON_WIDTH,
        }
    }

    /// Affordance rectangle for conversation rail buttons
    pub fn rail_conversation_rect(panel_rect: Rectangle, index: usize) -> Rectangle {
        let list_rect: Rectangle = Self::rail_control_rect(panel_rect, RailButton::List);
        let tab_area_y_start: f32 = list_rect.y + list_rect.height + RAIL_SEPARATOR_GAP;
        Rectangle {
            x: list_rect.x,
            y: tab_area_y_start + BUTTON_WIDTH * index as f32,
            width: BUTTON_WIDTH,
            height: BUTTON_WIDTH,
        }
    }

    pub fn tooltip_rect(panel_rect: Rectangle, tab_index: usize) -> Rectangle {
        let tab_rect: Rectangle = Self::rail_conversation_rect(panel_rect, tab_index);
        Rectangle {
            x: tab_rect.x - Self::TOOLTIP_WIDTH,
            y: tab_rect.y,
            width: Self::TOOLTIP_WIDTH,
            height: tab_rect.height,
        }
    }

    pub fn rail_conversation_close_rect(tab_rect: Rectangle) -> Rectangle {
        Rectangle {
            x: tab_rect.x + tab_rect.width - TAB_MINI_CLOSE_SIZE - TAB_MINI_CLOSE_MARGIN,
            y: tab_rect.y + TAB_MINI_CLOSE_MARGIN,
            width: TAB_MINI_CLOSE_SIZE,
            height: TAB_MINI_CLOSE_SIZE,
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

    /// The content rectangle less the header
    pub fn content_body_rectangle(panel_rect: Rectangle) -> Rectangle {
        let content_rect: Rectangle = ChatPanel::content_rectangle(panel_rect);
        Rectangle {
            x: content_rect.x,
            y: content_rect.y + HEADER_HEIGHT,
            width: content_rect.width,
            height: content_rect.height - HEADER_HEIGHT,
        }
    }
}
