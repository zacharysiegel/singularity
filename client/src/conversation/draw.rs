use std::sync::{RwLockReadGuard, RwLockWriteGuard};
use crate::component::frame::{BORDER_THICKNESS, draw_side_button_accent_filled, draw_rail_button_frame, draw_window_frame};
use crate::component::icon::{draw_close_x, draw_donut_ring, draw_hamburger, draw_plus};
use crate::component::text::Text;
use crate::component::text_truncate;
use crate::conversation::panel::{ChatPanel, ChatTab, RailButton, ENTRY_HEIGHT, HEADER_HEIGHT};
use crate::state::STATE;
use crate::window::BUTTON_WIDTH;
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use raylib::text::{RaylibFont, WeakFont};
use raylib::RaylibThread;
use shared::color::{GREEN, TEXT_COLOR, WINDOW_BACKGROUND_COLOR, WINDOW_INTERIOR_BORDER_COLOR};
use strum::IntoEnumIterator;
use uuid::Uuid;
use shared::math;
use crate::component::vertical_scroll_region::VerticalScrollRegionUpdate;

const PANEL_BACKGROUND_ALPHA: u8 = 0xE8;
const CONTENT_PADDING: f32 = 12.;
const NAME_FONT_SIZE: f32 = 14.;
const HEADER_FONT_SIZE: f32 = NAME_FONT_SIZE + 4.;
const DETAIL_FONT_SIZE: f32 = 10.;
const DONUT_PLACEHOLDER_COLOR: Color = Color { r: 0xa0, g: 0xa0, b: 0xa0, a: 0xff };
const UNNAMED_CONVERSATION_PLACEHOLDER: &str = "Unnamed";

pub fn draw(rl_draw: &mut RaylibDrawHandle, _rl_thread: &RaylibThread) {
    if !STATE.conversation.chat_panel.read().unwrap().open {
        return;
    }

    let panel_rect: Rectangle = ChatPanel::panel_rect(rl_draw.get_screen_width() as f32, rl_draw.get_screen_height() as f32);
    draw_window_frame(rl_draw, panel_rect, Color {
        a: PANEL_BACKGROUND_ALPHA,
        ..WINDOW_BACKGROUND_COLOR
    });

    let active_tab: ChatTab = STATE.conversation.chat_panel.read().unwrap().active_tab.clone();
    match active_tab {
        ChatTab::ConversationList => draw_conversation_list(rl_draw, panel_rect),
        ChatTab::Conversation(_) => {}
        ChatTab::NewConversation => {}
    }

    draw_rail(rl_draw, panel_rect);
}

fn draw_rail(rl_draw: &mut RaylibDrawHandle, panel_rect: Rectangle) {
    let chat_panel: RwLockReadGuard<ChatPanel> = STATE.conversation.chat_panel.read().unwrap();
    let hovered_button: Option<RailButton> = chat_panel.hovered_rail_button;
    let active_rail_button: Option<RailButton> = match &chat_panel.active_tab {
        ChatTab::ConversationList => Some(RailButton::List),
        ChatTab::NewConversation => Some(RailButton::New),
        ChatTab::Conversation(_) => None,
    };
    drop(chat_panel);

    let buttons: Vec<(Rectangle, RailButton)> = RailButton::iter()
        .map(|button| (ChatPanel::rail_control_rect(panel_rect, button), button))
        .collect();

    for (rect, button) in &buttons {
        draw_rail_button_frame(rl_draw, *rect, hovered_button == Some(*button));
        if active_rail_button == Some(*button) {
            draw_side_button_accent_filled(rl_draw, *rect, shared::color::WINDOW_BORDER_FOCUSED_COLOR);
        }
    }

    let close_rect: Rectangle = buttons[0].0;
    let new_rect: Rectangle = buttons[1].0;
    let list_rect: Rectangle = buttons[2].0;

    draw_close_x(rl_draw, close_rect, 4.5, shared::color::RED);
    draw_plus(rl_draw, new_rect, 2., TEXT_COLOR);
    draw_hamburger(rl_draw, list_rect, 2., TEXT_COLOR);

    let conversation_tabs: Vec<Uuid> = STATE.conversation.chat_panel.read().unwrap().conversation_tabs.clone();
    if !conversation_tabs.is_empty() {
        draw_rail_separator(rl_draw, close_rect.x, list_rect.y + list_rect.height);
    }
    draw_conversation_tabs(rl_draw, panel_rect, &conversation_tabs);
}

fn draw_rail_separator(rl_draw: &mut RaylibDrawHandle, x: f32, y: f32) {
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

fn draw_conversation_tabs(
    rl_draw: &mut RaylibDrawHandle,
    panel_rect: Rectangle,
    conversation_tabs: &[Uuid],
) {
    let chat_panel: RwLockReadGuard<ChatPanel> = STATE.conversation.chat_panel.read().unwrap();
    let active_conversation_id: Option<Uuid> = match chat_panel.active_tab {
        ChatTab::Conversation(id) => Some(id),
        _ => None,
    };
    let hovered_index: Option<usize> = chat_panel.hovered_conversation_tab;
    let close_hovered_index: Option<usize> = chat_panel.hovered_conversation_tab_close;
    drop(chat_panel);

    for (index, conversation_id) in conversation_tabs.iter().enumerate() {
        let tab_rect: Rectangle = ChatPanel::rail_conversation_rect(panel_rect, index);
        let is_active: bool = active_conversation_id == Some(*conversation_id);
        let is_hovered: bool = hovered_index == Some(index);
        let name: String = STATE.conversation.conversations
            .get(conversation_id)
            .map(|conversation| conversation.name.clone().unwrap_or_else(|| UNNAMED_CONVERSATION_PLACEHOLDER.to_string()))
            .unwrap_or_default();
        let tooltip_rect: Option<Rectangle> = if is_hovered {
            Some(ChatPanel::tooltip_rect(panel_rect, index))
        } else {
            None
        };

        draw_conversation_tab(
            rl_draw,
            tab_rect,
            tooltip_rect,
            name,
            is_active,
            is_hovered,
            is_hovered && close_hovered_index == Some(index),
        );
    }
}

/// Renders a conversation tab. With `tooltip_rect = None`, draws only the icon slot (rail
/// tab). With `Some(rect)`, widens the tab's frame to include the name label in that rect
/// (hover tooltip presentation).
pub fn draw_conversation_tab(
    rl_draw: &mut RaylibDrawHandle,
    icon_rect: Rectangle,
    tooltip_rect: Option<Rectangle>,
    name: String,
    active: bool,
    row_hovered: bool,
    mini_close_hovered: bool,
) {
    let container: Rectangle = match tooltip_rect {
        Some(tooltip_rect) => Rectangle {
            width: tooltip_rect.width + icon_rect.width,
            ..tooltip_rect
        },
        None => icon_rect,
    };

    draw_rail_button_frame(rl_draw, container, row_hovered);
    if active {
        draw_side_button_accent_filled(rl_draw, container, shared::color::WINDOW_BORDER_FOCUSED_COLOR);
    }

    draw_donut_ring(rl_draw, icon_rect, DONUT_PLACEHOLDER_COLOR);

    if let Some(tooltip_rect) = tooltip_rect {
        draw_tooltip(rl_draw, tooltip_rect, name);
    }

    if row_hovered {
        draw_tab_mini_close(rl_draw, icon_rect, mini_close_hovered);
    }
}

fn draw_tooltip(rl_draw: &mut RaylibDrawHandle, tooltip_rect: Rectangle, name: String) {
    let text: Text = Text {
        content: name,
        font_size: NAME_FONT_SIZE,
        font_spacing: 2.,
        color: TEXT_COLOR,
    };
    let font_factory: fn() -> WeakFont = || unsafe { WeakFont::from_raw(raylib::ffi::GetFontDefault()) };
    let name_max_width: f32 = tooltip_rect.width - CONTENT_PADDING * 2.;
    let truncated_name: String = text_truncate::truncate_text(&text, &font_factory(), name_max_width);
    let text_height: f32 = NAME_FONT_SIZE;
    let text_y: f32 = math::center_vertically(tooltip_rect.y, tooltip_rect.height, text_height);
    rl_draw.draw_text_ex(
        font_factory(),
        &truncated_name,
        Vector2 {
            x: tooltip_rect.x + CONTENT_PADDING,
            y: text_y,
        },
        NAME_FONT_SIZE,
        2.,
        TEXT_COLOR,
    );
}

fn draw_tab_mini_close(rl_draw: &mut RaylibDrawHandle, tab_rect: Rectangle, hovered: bool) {
    let close_rect: Rectangle = ChatPanel::rail_conversation_close_rect(tab_rect);
    let background_color: Color = if hovered {
        math::color_add(&WINDOW_BACKGROUND_COLOR, &shared::color::DIFF_HOVER_BUTTON)
    } else {
        WINDOW_BACKGROUND_COLOR
    };
    rl_draw.draw_rectangle_rec(close_rect, background_color);
    rl_draw.draw_rectangle_lines_ex(close_rect, BORDER_THICKNESS, WINDOW_INTERIOR_BORDER_COLOR);
    draw_close_x(rl_draw, close_rect, 1.5, shared::color::RED);
}

/// Renders a panel section header: a vertically centered title text, followed by a double border line.
/// Used by all chat panel tab views (conversation list, message view, new conversation).
pub fn draw_panel_header(rl_draw: &mut RaylibDrawHandle, content_rect: Rectangle, title: &str) {
    let double_border_separation: f32 = 4.;
    let header_text_area_height: f32 = HEADER_HEIGHT - double_border_separation;

    let text_height: f32 = rl_draw.get_font_default().measure_text(title, HEADER_FONT_SIZE, 2.).y;
    let header_text_y: f32 = math::center_vertically(content_rect.y, header_text_area_height, text_height);
    rl_draw.draw_text_ex(
        rl_draw.get_font_default(),
        title,
        Vector2 { x: content_rect.x + CONTENT_PADDING, y: header_text_y },
        HEADER_FONT_SIZE,
        2.,
        TEXT_COLOR,
    );

    let top_y: f32 = content_rect.y + HEADER_HEIGHT - double_border_separation;
    let bottom_y: f32 = content_rect.y + HEADER_HEIGHT;
    let left_x: f32 = content_rect.x;
    let right_x: f32 = content_rect.x + content_rect.width;
    rl_draw.draw_spline_linear(
        &[
            Vector2 { x: left_x, y: top_y },
            Vector2 { x: right_x, y: top_y },
            Vector2 { x: right_x, y: bottom_y },
            Vector2 { x: left_x, y: bottom_y },
        ],
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
}

fn draw_conversation_list(rl_draw: &mut RaylibDrawHandle, panel_rect: Rectangle) {
    let content_rect: Rectangle = ChatPanel::content_rectangle(panel_rect);
    let conversation_order: RwLockReadGuard<Vec<Uuid>> = STATE.conversation.display_order();
    let hovered_list_entry: Option<usize> = STATE.conversation.chat_panel.read().unwrap().hovered_list_entry;

    draw_panel_header(rl_draw, content_rect, "Conversations");

    let scroll_viewport: Rectangle = ChatPanel::content_body_rectangle(panel_rect);
    let conversation_count: usize = conversation_order.len();
    let logical_height: f32 = conversation_count as f32 * ENTRY_HEIGHT;

    let mut chat_panel: RwLockWriteGuard<ChatPanel> = STATE.conversation.chat_panel.write().unwrap();
    chat_panel.list_scroll_region.update(VerticalScrollRegionUpdate {
        viewport: Some(scroll_viewport),
        content_height: Some(logical_height),
        padding: None,
    });
    chat_panel.list_scroll_region.draw(rl_draw, |mut scissor_draw, y_offset| {
        let mut entry_top: f32 = scroll_viewport.y + y_offset;
        for (index, conversation_id) in conversation_order.iter().enumerate() {
            if entry_top > scroll_viewport.y + scroll_viewport.height {
                break;
            }

            if entry_top + ENTRY_HEIGHT > scroll_viewport.y {
                let hovered: bool = hovered_list_entry == Some(index);
                if let Some(conversation) = STATE.conversation.conversations.get(conversation_id) {
                    let name: String = conversation.name.clone().unwrap_or_else(|| UNNAMED_CONVERSATION_PLACEHOLDER.to_string());
                    let member_count: usize = conversation.members.len();
                    let unread_count: u32 = conversation.unread_count;
                    drop(conversation);
                    draw_conversation_entry(&mut scissor_draw, content_rect, entry_top, &name, member_count, unread_count, hovered);
                }
            }

            entry_top += ENTRY_HEIGHT;
        }
    });
}

fn draw_conversation_entry(
    rl_draw: &mut impl RaylibDraw,
    content_rect: Rectangle,
    y: f32,
    name: &str,
    member_count: usize,
    unread_count: u32,
    hovered: bool,
) {
    if hovered {
        let hover_rect: Rectangle = Rectangle {
            x: content_rect.x,
            y,
            width: content_rect.width,
            height: ENTRY_HEIGHT,
        };
        rl_draw.draw_rectangle_rec(hover_rect, math::color_add(&WINDOW_BACKGROUND_COLOR, &shared::color::DIFF_HOVER_BUTTON));
        // Redraw borders that the hover tint painted over
        let left_x: f32 = content_rect.x;
        let right_x: f32 = content_rect.x + content_rect.width;
        let bottom_y: f32 = y + ENTRY_HEIGHT;
        rl_draw.draw_spline_linear(
            &[
                Vector2 { x: left_x, y },
                Vector2 { x: right_x, y },
                Vector2 { x: right_x, y: bottom_y },
                Vector2 { x: left_x, y: bottom_y },
                Vector2 { x: left_x, y },
            ],
            BORDER_THICKNESS,
            WINDOW_INTERIOR_BORDER_COLOR,
        );
    }

    let name_x: f32 = content_rect.x + CONTENT_PADDING;
    let name_max_width: f32 = content_rect.width - CONTENT_PADDING * 2. - 80.; // Subtract additional end margin
    let name_text: Text = Text {
        content: name.to_string(),
        font_size: NAME_FONT_SIZE,
        font_spacing: 2.,
        color: TEXT_COLOR,
    };
    // RaylibDraw does not provide get_font_default. This will need to change soon anyway when we use our own fonts.
    let font = || unsafe { WeakFont::from_raw(raylib::ffi::GetFontDefault()) };
    let truncated_name: String = text_truncate::truncate_text(&name_text, &font(), name_max_width);

    let line_gap: f32 = 4.;
    let total_text_height: f32 = NAME_FONT_SIZE + DETAIL_FONT_SIZE + line_gap;
    let text_top: f32 = math::center_vertically(y, ENTRY_HEIGHT, total_text_height);

    rl_draw.draw_text_ex(
        font(),
        &truncated_name,
        Vector2 { x: name_x, y: text_top },
        NAME_FONT_SIZE,
        2.,
        TEXT_COLOR,
    );

    let detail_text: String = format!("{member_count} members");
    rl_draw.draw_text_ex(
        font(),
        &detail_text,
        Vector2 { x: name_x, y: text_top + NAME_FONT_SIZE + line_gap },
        DETAIL_FONT_SIZE,
        1.,
        WINDOW_INTERIOR_BORDER_COLOR,
    );

    if unread_count > 0 {
        let unread_text: String = format!("{unread_count} unread");
        let unread_measure: Vector2 = font().measure_text(&unread_text, DETAIL_FONT_SIZE, 1.);
        rl_draw.draw_text_ex(
            font(),
            &unread_text,
            Vector2 {
                x: content_rect.x + content_rect.width - CONTENT_PADDING - unread_measure.x,
                y: text_top,
            },
            DETAIL_FONT_SIZE,
            1.,
            GREEN,
        );
    }

    // Border bottom
    rl_draw.draw_line_ex(
        Vector2 { x: content_rect.x, y: y + ENTRY_HEIGHT },
        Vector2 { x: content_rect.x + content_rect.width, y: y + ENTRY_HEIGHT },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
}
