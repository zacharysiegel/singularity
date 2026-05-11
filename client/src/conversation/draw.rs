use crate::component::frame::{BORDER_THICKNESS, draw_side_button_frame, draw_window_frame};
use crate::component::icon::{draw_close_x, draw_hamburger, draw_plus};
use crate::component::text::Text;
use crate::component::text_truncate;
use crate::conversation::panel::{ChatPanel, ChatTab, RailButton, ENTRY_HEIGHT, HEADER_HEIGHT};
use crate::conversation::state::Conversation;
use crate::state::STATE;
use crate::window::BUTTON_WIDTH;
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use raylib::text::RaylibFont;
use raylib::RaylibThread;
use shared::color::{GREEN, TEXT_COLOR, WINDOW_BACKGROUND_COLOR, WINDOW_INTERIOR_BORDER_COLOR};
use strum::IntoEnumIterator;

const PANEL_BACKGROUND_ALPHA: u8 = 0xE8;
const CONTENT_PADDING: f32 = 12.;
const NAME_FONT_SIZE: f32 = 14.;
const DETAIL_FONT_SIZE: f32 = 10.;

pub fn draw(rl_draw: &mut RaylibDrawHandle, _rl_thread: &RaylibThread) {
    if !STATE.conversation.chat_panel.read().unwrap().open {
        return;
    }

    let panel_rect: Rectangle = ChatPanel::panel_rectangle(rl_draw.get_screen_width() as f32, rl_draw.get_screen_height() as f32);
    draw_window_frame(rl_draw, panel_rect, Color {
        a: PANEL_BACKGROUND_ALPHA,
        ..WINDOW_BACKGROUND_COLOR
    });
    draw_rail_action_buttons(rl_draw, panel_rect);

    let active_tab: ChatTab = STATE.conversation.chat_panel.read().unwrap().active_tab.clone();
    match active_tab {
        ChatTab::ConversationList => draw_conversation_list(rl_draw, panel_rect),
        ChatTab::Conversation(_) => {}
        ChatTab::NewConversation => {}
    }
}

fn draw_rail_action_buttons(rl_draw: &mut RaylibDrawHandle, panel_rect: Rectangle) {
    let hovered_button: Option<RailButton> = STATE.conversation.chat_panel.read().unwrap().hovered_rail_button;

    let buttons: Vec<(Rectangle, RailButton)> = RailButton::iter()
        .map(|button| (ChatPanel::rail_button_rect(panel_rect, button), button))
        .collect();

    for (rect, button) in &buttons {
        draw_side_button_frame(rl_draw, *rect, hovered_button == Some(*button));
    }

    let close_rect: Rectangle = buttons[0].0;
    let new_rect: Rectangle = buttons[1].0;
    let list_rect: Rectangle = buttons[2].0;

    draw_close_x(rl_draw, close_rect, 4.5, shared::color::RED);
    draw_plus(rl_draw, new_rect, 2., TEXT_COLOR);
    draw_hamburger(rl_draw, list_rect, 2., TEXT_COLOR);

    let has_conversation_tabs: bool = !STATE.conversation.chat_panel.read().unwrap().conversation_tabs.is_empty();
    if has_conversation_tabs {
        draw_double_separator(rl_draw, close_rect.x, list_rect.y + list_rect.height);
    }
}

fn draw_double_separator(rl_draw: &mut RaylibDrawHandle, x: f32, y: f32) {
    rl_draw.draw_line_ex(
        Vector2 { x, y },
        Vector2 { x: x + BUTTON_WIDTH, y },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
    rl_draw.draw_line_ex(
        Vector2 { x, y: y + 3. },
        Vector2 { x: x + BUTTON_WIDTH, y: y + 3. },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
}

fn draw_conversation_list(rl_draw: &mut RaylibDrawHandle, panel_rect: Rectangle) {
    let content_rect: Rectangle = ChatPanel::content_rectangle(panel_rect);
    let hovered_list_entry: Option<usize> = STATE.conversation.chat_panel.read().unwrap().hovered_list_entry;

    draw_conversation_list_header(rl_draw, content_rect);

    let mut y: f32 = content_rect.y + HEADER_HEIGHT;
    for (index, entry) in STATE.conversation.conversations.iter().enumerate() {
        if y + ENTRY_HEIGHT > content_rect.y + content_rect.height {
            break;
        }
        let hovered: bool = hovered_list_entry == Some(index);
        draw_conversation_entry(rl_draw, content_rect, y, entry.value(), hovered);
        y += ENTRY_HEIGHT;
    }
}

fn draw_conversation_list_header(rl_draw: &mut RaylibDrawHandle, content_rect: Rectangle) {
    rl_draw.draw_text_ex(
        rl_draw.get_font_default(),
        "Conversations",
        Vector2 { x: content_rect.x + CONTENT_PADDING, y: content_rect.y + 10. },
        NAME_FONT_SIZE + 2.,
        2.,
        TEXT_COLOR,
    );
    rl_draw.draw_line_ex(
        Vector2 { x: content_rect.x, y: content_rect.y + HEADER_HEIGHT },
        Vector2 { x: content_rect.x + content_rect.width, y: content_rect.y + HEADER_HEIGHT },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
}

fn draw_conversation_entry(
    rl_draw: &mut RaylibDrawHandle,
    content_rect: Rectangle,
    y: f32,
    conversation: &Conversation,
    hovered: bool,
) {
    if hovered {
        let hover_rect: Rectangle = Rectangle {
            x: content_rect.x,
            y,
            width: content_rect.width,
            height: ENTRY_HEIGHT,
        };
        rl_draw.draw_rectangle_rec(hover_rect, shared::math::color_add(&WINDOW_BACKGROUND_COLOR, &shared::color::DIFF_HOVER_BUTTON));
        rl_draw.draw_line_ex(
            Vector2 { x: content_rect.x, y },
            Vector2 { x: content_rect.x, y: y + ENTRY_HEIGHT },
            BORDER_THICKNESS,
            WINDOW_INTERIOR_BORDER_COLOR,
        );
        rl_draw.draw_line_ex(
            Vector2 { x: content_rect.x + content_rect.width, y },
            Vector2 { x: content_rect.x + content_rect.width, y: y + ENTRY_HEIGHT },
            BORDER_THICKNESS,
            WINDOW_INTERIOR_BORDER_COLOR,
        );
    }

    let font = rl_draw.get_font_default();
    let name: &str = conversation.name.as_deref().unwrap_or("Unnamed");
    let member_count: usize = conversation.members.len();
    let unread_count: u32 = conversation.unread_count;

    let name_x: f32 = content_rect.x + CONTENT_PADDING;
    let name_max_width: f32 = content_rect.width - CONTENT_PADDING * 2. - 80.;
    let name_text: Text = Text {
        content: name.to_string(),
        font_size: NAME_FONT_SIZE,
        font_spacing: 2.,
        color: TEXT_COLOR,
    };
    let truncated_name: String = text_truncate::truncate_text(&name_text, &font, name_max_width);

    rl_draw.draw_text_ex(
        rl_draw.get_font_default(),
        &truncated_name,
        Vector2 { x: name_x, y: y + 6. },
        NAME_FONT_SIZE,
        2.,
        TEXT_COLOR,
    );

    let member_text: String = format!("{member_count} members");
    rl_draw.draw_text_ex(
        rl_draw.get_font_default(),
        &member_text,
        Vector2 { x: name_x, y: y + 24. },
        DETAIL_FONT_SIZE,
        1.,
        WINDOW_INTERIOR_BORDER_COLOR,
    );

    if unread_count > 0 {
        let unread_text: String = format!("{unread_count} unread");
        let unread_measure: Vector2 = font.measure_text(&unread_text, DETAIL_FONT_SIZE, 1.);
        rl_draw.draw_text_ex(
            rl_draw.get_font_default(),
            &unread_text,
            Vector2 {
                x: content_rect.x + content_rect.width - CONTENT_PADDING - unread_measure.x,
                y: y + 6.,
            },
            DETAIL_FONT_SIZE,
            1.,
            GREEN,
        );
    }

    rl_draw.draw_line_ex(
        Vector2 { x: content_rect.x, y: y + ENTRY_HEIGHT },
        Vector2 { x: content_rect.x + content_rect.width, y: y + ENTRY_HEIGHT },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
}
