use crate::component::frame::BORDER_THICKNESS;
use crate::component::text::Text;
use crate::component::text_truncate;
use crate::component::vertical_scroll_region::VerticalScrollRegionUpdate;
use crate::conversation::draw::panel::{draw_panel_header, DETAIL_FONT_SIZE, NAME_FONT_SIZE, UNNAMED_CONVERSATION_PLACEHOLDER};
use crate::conversation::panel::{ChatPanel, CONTENT_PADDING, ENTRY_HEIGHT};
use crate::state::STATE;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use raylib::text::{RaylibFont, WeakFont};
use shared::color::{GREEN, TEXT_COLOR, WINDOW_BACKGROUND_COLOR, WINDOW_INTERIOR_BORDER_COLOR};
use shared::math;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};
use uuid::Uuid;

const ENTRY_END_MARGIN: f32 = 80.;
const ENTRY_LINE_GAP: f32 = 4.;

pub fn draw_conversation_list(rl_draw: &mut RaylibDrawHandle, panel_rect: Rectangle) {
    let content_rect: Rectangle = ChatPanel::content_rectangle(panel_rect);
    let conversation_order: RwLockReadGuard<Vec<Uuid>> = STATE.conversation.display_order();
    let hovered_list_entry: Option<usize> = STATE.conversation.chat_panel.read().unwrap().conversation_list_state.hovered_entry;

    draw_panel_header(rl_draw, content_rect, "Conversations", None);

    let scroll_viewport: Rectangle = ChatPanel::content_body_rectangle(panel_rect);
    let conversation_count: usize = conversation_order.len();
    let logical_height: f32 = conversation_count as f32 * ENTRY_HEIGHT;

    let mut chat_panel: RwLockWriteGuard<ChatPanel> = STATE.conversation.chat_panel.write().unwrap();
    chat_panel.conversation_list_state.scroll_region.update(VerticalScrollRegionUpdate {
        viewport: Some(scroll_viewport),
        content_height: Some(logical_height),
        padding: None,
    });
    chat_panel.conversation_list_state.scroll_region.draw(rl_draw, |mut scissor_draw, y_offset| {
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
    let name_max_width: f32 = content_rect.width - CONTENT_PADDING * 2. - ENTRY_END_MARGIN;
    let name_text: Text = Text {
        content: name.to_string(),
        font_size: NAME_FONT_SIZE,
        font_spacing: 2.,
        color: TEXT_COLOR,
    };
    let font = || unsafe { WeakFont::from_raw(raylib::ffi::GetFontDefault()) };
    let truncated_name: String = text_truncate::truncate_text(&name_text, &font(), name_max_width);

    let total_text_height: f32 = NAME_FONT_SIZE + DETAIL_FONT_SIZE + ENTRY_LINE_GAP;
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
        Vector2 { x: name_x, y: text_top + NAME_FONT_SIZE + ENTRY_LINE_GAP },
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

    rl_draw.draw_line_ex(
        Vector2 { x: content_rect.x, y: y + ENTRY_HEIGHT },
        Vector2 { x: content_rect.x + content_rect.width, y: y + ENTRY_HEIGHT },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
}
