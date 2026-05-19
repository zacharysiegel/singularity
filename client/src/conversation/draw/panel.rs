use crate::component::frame::{BORDER_THICKNESS, draw_rail_button_frame, draw_side_button_accent_filled, draw_window_frame};
use crate::component::icon::{draw_close_x, draw_donut_ring, draw_hamburger, draw_plus};
use crate::component::text::Text;
use crate::component::{text_truncate, DEFAULT_FONT_SPACING};
use crate::conversation::draw::event_view::draw_conversation_view;
use crate::conversation::draw::list_view::draw_conversation_list;
use crate::conversation::panel::{ChatPanel, ChatTab, CONTENT_PADDING, HEADER_HEIGHT, RailControl};
use crate::state::STATE;
use crate::window::BUTTON_WIDTH;
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle, RaylibScissorModeExt};
use raylib::math::{Rectangle, Vector2};
use raylib::text::{RaylibFont, WeakFont};
use raylib::RaylibThread;
use shared::color::{TEXT_COLOR, WINDOW_BACKGROUND_COLOR, WINDOW_INTERIOR_BORDER_COLOR};
use shared::math;
use std::sync::RwLockReadGuard;
use strum::IntoEnumIterator;
use uuid::Uuid;

pub const NAME_FONT_SIZE: f32 = 14.;
pub const HEADER_FONT_SIZE: f32 = NAME_FONT_SIZE + 4.;
pub const DETAIL_FONT_SIZE: f32 = 10.;
pub const UNNAMED_CONVERSATION_PLACEHOLDER: &str = "Unnamed";
pub const DONUT_PLACEHOLDER_COLOR: Color = Color { r: 0xa0, g: 0xa0, b: 0xa0, a: 0xff };

pub const PANEL_BACKGROUND_ALPHA: u8 = 0xF0;
const TAB_MINI_CLOSE_BACKGROUND_X_THICKNESS: f32 = 1.5;

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
        ChatTab::Conversation(conversation_id) => draw_conversation_view(rl_draw, panel_rect, conversation_id),
        ChatTab::NewConversation => {}
    }

    draw_rail(rl_draw, panel_rect);
}

fn draw_rail(rl_draw: &mut RaylibDrawHandle, panel_rect: Rectangle) {
    let chat_panel: RwLockReadGuard<ChatPanel> = STATE.conversation.chat_panel.read().unwrap();
    let hovered_button: Option<RailControl> = chat_panel.hovered_control_button;
    let active_rail_button: Option<RailControl> = match &chat_panel.active_tab {
        ChatTab::ConversationList => Some(RailControl::List),
        ChatTab::NewConversation => Some(RailControl::New),
        ChatTab::Conversation(_) => None,
    };
    drop(chat_panel);

    let buttons: Vec<(Rectangle, RailControl)> = RailControl::iter()
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
    if conversation_tabs.is_empty() {
        return;
    }

    let chat_panel: RwLockReadGuard<ChatPanel> = STATE.conversation.chat_panel.read().unwrap();
    let active_conversation_id: Option<Uuid> = match chat_panel.active_tab {
        ChatTab::Conversation(id) => Some(id),
        _ => None,
    };
    let hovered_index: Option<usize> = chat_panel.hovered_conversation_tab;
    let close_hovered_index: Option<usize> = chat_panel.hovered_conversation_tab_close;
    drop(chat_panel);

    let clip_rect: Rectangle = ChatPanel::rail_conversation_clip_rect(panel_rect);
    rl_draw.draw_scissor_mode(clip_rect.x as i32, clip_rect.y as i32, clip_rect.width as i32, clip_rect.height as i32, |mut scissor_draw| {
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
                &mut scissor_draw,
                tab_rect,
                tooltip_rect,
                name,
                is_active,
                is_hovered,
                is_hovered && close_hovered_index == Some(index),
            );
        }
    });
}

/// Renders a conversation tab. With `tooltip_rect = None`, draws only the icon slot (rail
/// tab). With `Some(rect)`, widens the tab's frame to include the name label in that rect
/// (hover tooltip presentation).
pub fn draw_conversation_tab(
    rl_draw: &mut impl RaylibDraw,
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

fn draw_tooltip(rl_draw: &mut impl RaylibDraw, tooltip_rect: Rectangle, name: String) {
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

fn draw_tab_mini_close(rl_draw: &mut impl RaylibDraw, tab_rect: Rectangle, hovered: bool) {
    let close_rect: Rectangle = ChatPanel::rail_conversation_close_rect(tab_rect);
    let background_color: Color = if hovered {
        math::color_add(&WINDOW_BACKGROUND_COLOR, &shared::color::DIFF_HOVER_BUTTON)
    } else {
        WINDOW_BACKGROUND_COLOR
    };
    rl_draw.draw_rectangle_rec(close_rect, background_color);
    rl_draw.draw_rectangle_lines_ex(close_rect, BORDER_THICKNESS, WINDOW_INTERIOR_BORDER_COLOR);
    draw_close_x(rl_draw, close_rect, TAB_MINI_CLOSE_BACKGROUND_X_THICKNESS, shared::color::RED);
}

/// Renders a panel section header: a vertically centered title text, followed by a double border line.
/// Used by all chat panel tab views (conversation list, message view, new conversation).
pub fn draw_panel_header(rl_draw: &mut RaylibDrawHandle, content_rect: Rectangle, title: &str, subtitle: Option<&str>) {
    let double_border_separation: f32 = 4.;
    let header_text_area_height: f32 = HEADER_HEIGHT - double_border_separation;
    let line_gap: f32 = 4.;
    let title_max_width: f32 = content_rect.width - CONTENT_PADDING * 2.;

    let title_text: Text = Text {
        content: title.to_string(),
        font_size: HEADER_FONT_SIZE,
        font_spacing: 2.,
        color: TEXT_COLOR,
    };
    let truncated_title: String = text_truncate::truncate_text(&title_text, &rl_draw.get_font_default(), title_max_width);
    let title_height: f32 = rl_draw.get_font_default().measure_text(&truncated_title, HEADER_FONT_SIZE, 2.).y;

    let truncated_subtitle: Option<String> = subtitle.map(|subtitle| {
        let subtitle_text: Text = Text {
            content: subtitle.to_string(),
            font_size: DETAIL_FONT_SIZE,
            font_spacing: 1.,
            color: WINDOW_INTERIOR_BORDER_COLOR,
        };
        text_truncate::truncate_text(&subtitle_text, &rl_draw.get_font_default(), title_max_width)
    });

    let header_content_height: f32 = match &truncated_subtitle {
        Some(_) => title_height + line_gap + DETAIL_FONT_SIZE,
        None => title_height,
    };
    let title_top_y: f32 = math::center_vertically(content_rect.y, header_text_area_height, header_content_height);
    rl_draw.draw_text_ex(
        rl_draw.get_font_default(),
        &truncated_title,
        Vector2 { x: content_rect.x + CONTENT_PADDING, y: title_top_y },
        HEADER_FONT_SIZE,
        DEFAULT_FONT_SPACING,
        TEXT_COLOR,
    );

    if let Some(subtitle) = truncated_subtitle {
        rl_draw.draw_text_ex(
            rl_draw.get_font_default(),
            &subtitle,
            Vector2 { x: content_rect.x + CONTENT_PADDING, y: title_top_y + title_height + line_gap },
            DETAIL_FONT_SIZE,
            1.,
            WINDOW_INTERIOR_BORDER_COLOR,
        );
    }

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
