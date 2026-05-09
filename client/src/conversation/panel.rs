use uuid::Uuid;

const PANEL_MAX_WIDTH: f32 = 600.;

#[derive(Debug, Clone, PartialEq)]
pub enum ChatTab {
    ConversationList,
    Conversation(Uuid),
    NewConversation,
}

#[derive(Debug)]
pub struct ChatPanel {
    pub open: bool,
    pub active_tab: ChatTab,
    /// Conversation IDs of open tabs in the rail, in display order.
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
        screen_width.min(PANEL_MAX_WIDTH)
    }

    pub fn panel_rectangle(screen_width: f32, screen_height: f32) -> raylib::math::Rectangle {
        let width: f32 = Self::panel_width(screen_width);
        raylib::math::Rectangle {
            x: screen_width - width,
            y: 0.,
            width,
            height: screen_height,
        }
    }
}
