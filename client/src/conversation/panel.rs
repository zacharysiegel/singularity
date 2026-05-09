use raylib::math::Rectangle;
use uuid::Uuid;

const PANEL_MAX_WIDTH: f32 = 600.;

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
}

impl ChatPanel {
    pub fn new() -> Self {
        ChatPanel {
            open: false,
            active_tab: ChatTab::ConversationList,
            conversation_tabs: Vec::new(),
            expanded: false,
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
}
