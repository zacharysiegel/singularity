use crate::component::text_wrap;
use crate::component::time::format_relative_time;
use crate::component::vertical_scroll_region::VerticalScrollRegionUpdate;
use crate::conversation::draw::panel::{draw_panel_header, UNNAMED_CONVERSATION_PLACEHOLDER};
use crate::conversation::panel::{ChatPanel, CONTENT_PADDING};
use crate::conversation::state::{Conversation, ConversationEvent};
use crate::state::STATE;
use chrono::{DateTime, Local, Utc};
use dashmap::mapref::one::Ref;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use raylib::text::{RaylibFont, WeakFont};
use shared::color::{TEXT_COLOR, WINDOW_INTERIOR_BORDER_COLOR};
use shared::schema::conversation::ConversationMemberChange;
use shared::schema::conversation_message::ConversationMessage;
use std::sync::RwLockWriteGuard;
use uuid::Uuid;

const MESSAGE_FONT_SIZE: f32 = 13.;
const MESSAGE_FONT_SPACING: f32 = 1.5;
const SENDER_FONT_SIZE: f32 = 10.;
const SENDER_FONT_SPACING: f32 = 1.;
/// Vertical gap between consecutive wrapped lines within a single message body.
const MESSAGE_LINE_GAP: f32 = 4.;
/// Vertical gap between message bundles. One bundle = sender header line + all of that
/// message's wrapped body lines.
const MESSAGE_BUNDLE_GAP: f32 = 10.;
const SENDER_TO_MESSAGE_GAP: f32 = 2.;
const MESSAGE_MAX_WIDTH_RATIO: f32 = 0.88;

enum RenderableEvent {
    Message(MessageLines),
    System(SystemLine),
}

impl RenderableEvent {
    fn new(event: &ConversationEvent, font: &WeakFont, inner_width: f32) -> Self {
        match event {
            ConversationEvent::Chat(message) => {
                let max_width: f32 = inner_width * MESSAGE_MAX_WIDTH_RATIO;
                RenderableEvent::Message(MessageLines::new(message.clone(), font, max_width))
            }
            ConversationEvent::MemberJoined(change) => {
                RenderableEvent::System(SystemLine::member_joined(change, font, inner_width))
            }
            ConversationEvent::MemberLeft(change) => {
                RenderableEvent::System(SystemLine::member_left(change, font, inner_width))
            }
        }
    }

    fn height(&self) -> f32 {
        match self {
            RenderableEvent::Message(wrapped) => wrapped.height(),
            RenderableEvent::System(line) => line.height(),
        }
    }
}

struct MessageLines {
    message: ConversationMessage,
    sender_line: String,
    /// Content is hard-wrapped before being stored here
    content_lines: Vec<String>,
}

impl MessageLines {
    fn new(message: ConversationMessage, font: &WeakFont, max_width: f32) -> Self {
        let sender_line: String = format_sender_line(&message);
        let wrapped_lines: Vec<String> =
            text_wrap::wrap_text(&message.content, font, MESSAGE_FONT_SIZE, MESSAGE_FONT_SPACING, max_width);
        MessageLines {
            message,
            sender_line,
            content_lines: wrapped_lines,
        }
    }

    fn height(&self) -> f32 {
        SENDER_FONT_SIZE
            + SENDER_TO_MESSAGE_GAP
            + (MESSAGE_FONT_SIZE + MESSAGE_LINE_GAP) * self.content_lines.len().max(1) as f32
    }
}

struct SystemLine {
    wrapped_lines: Vec<String>,
}

impl SystemLine {
    fn new(body: String, timestamp: DateTime<Utc>, font: &WeakFont, max_width: f32) -> Self {
        let local_time: DateTime<Local> = timestamp.with_timezone(&Local);
        let absolute: String = local_time.format("%b %-d %H:%M:%S").to_string();
        let relative: String = format_relative_time(timestamp);
        let text: String = format!("{body} | {absolute} ({relative})");
        let wrapped_lines: Vec<String> =
            text_wrap::wrap_text(&text, font, SENDER_FONT_SIZE, SENDER_FONT_SPACING, max_width);
        SystemLine { wrapped_lines }
    }

    fn member_joined(change: &ConversationMemberChange, font: &WeakFont, max_width: f32) -> Self {
        SystemLine::new(format!("{} joined", format_username(change.account_id)), change.timestamp, font, max_width)
    }

    fn member_left(change: &ConversationMemberChange, font: &WeakFont, max_width: f32) -> Self {
        SystemLine::new(format!("{} left", format_username(change.account_id)), change.timestamp, font, max_width)
    }

    fn height(&self) -> f32 {
        SENDER_FONT_SIZE * self.wrapped_lines.len().max(1) as f32
            + MESSAGE_LINE_GAP * self.wrapped_lines.len().saturating_sub(1) as f32
    }
}

fn format_username(account_id: Uuid) -> String {
    STATE
        .account
        .request_username(account_id)
        .unwrap_or_else(|| account_id.to_string())
}

pub fn draw_conversation_view(rl_draw: &mut RaylibDrawHandle, panel_rect: Rectangle, conversation_id: Uuid) {
    let content_rect: Rectangle = ChatPanel::content_rectangle(panel_rect);
    let content_body_rect: Rectangle = ChatPanel::content_body_rectangle(panel_rect);

    let conversation_entry: Option<Ref<Uuid, Conversation>> = STATE.conversation.conversations.get(&conversation_id);
    let Some(conversation_entry) = conversation_entry else {
        return;
    };

    let font_factory: fn() -> WeakFont = || unsafe { WeakFont::from_raw(raylib::ffi::GetFontDefault()) };
    let inner_width: f32 = content_body_rect.width - CONTENT_PADDING * 2.;

    let (header_title, header_subtitle): (String, String) = format_conversation_header(&conversation_entry);
    let renderable_events: Vec<RenderableEvent> = conversation_entry
        .events
        .values()
        .map(|event| RenderableEvent::new(event, &font_factory(), inner_width))
        .collect();
    drop(conversation_entry);

    draw_panel_header(rl_draw, content_rect, &header_title, Some(&header_subtitle));

    let content_height: f32 = renderable_events
        .iter()
        .map(RenderableEvent::height)
        .sum::<f32>()
        + MESSAGE_BUNDLE_GAP * renderable_events.len().saturating_sub(1) as f32;

    let mut chat_panel: RwLockWriteGuard<ChatPanel> = STATE.conversation.chat_panel.write().unwrap();
    let view_state = chat_panel
        .conversation_view_states
        .get_mut(&conversation_id)
        .expect("conversation view state should exist for active tab");
    view_state.scroll_region.update(VerticalScrollRegionUpdate {
        viewport: Some(content_body_rect),
        content_height: Some(content_height),
        padding: None,
    });

    let own_account_id: Option<Uuid> = *STATE.account.own_account_id.read().unwrap();
    view_state.scroll_region.draw(rl_draw, |mut scissor_draw, y_offset| {
        let mut entry_top: f32 = content_body_rect.y + y_offset;
        for renderable in &renderable_events {
            if entry_top > content_body_rect.y + content_body_rect.height {
                break;
            }

            if entry_top + renderable.height() > content_body_rect.y {
                draw_renderable_event(&mut scissor_draw, content_body_rect, entry_top, renderable, own_account_id, &font_factory());
            }
            entry_top += renderable.height() + MESSAGE_BUNDLE_GAP;
        }
    });
}

fn format_conversation_header(conversation: &Conversation) -> (String, String) {
    let name: String = conversation.name.clone().unwrap_or_else(|| UNNAMED_CONVERSATION_PLACEHOLDER.to_string());
    let member_count: usize = conversation.members.len();
    let subtitle: String = format!("{member_count} member{}", if member_count == 1 { "" } else { "s" });
    (name, subtitle)
}

fn format_sender_line(message: &ConversationMessage) -> String {
    let sender_username: String = format_username(message.sender_account_id);
    let local_time: DateTime<Local> = message.created.with_timezone(&Local);
    let absolute: String = local_time.format("%b %-d %H:%M:%S").to_string();
    let relative: String = format_relative_time(message.created);
    format!("{sender_username} | {absolute} ({relative})")
}

fn draw_renderable_event(
    rl_draw: &mut impl RaylibDraw,
    viewport: Rectangle,
    top: f32,
    renderable: &RenderableEvent,
    own_account_id: Option<Uuid>,
    font: &WeakFont,
) {
    match renderable {
        RenderableEvent::Message(wrapped) => {
            let is_own: bool = own_account_id == Some(wrapped.message.sender_account_id);
            draw_message(rl_draw, viewport, top, wrapped, is_own, font);
        }
        RenderableEvent::System(row) => {
            draw_system_row(rl_draw, viewport, top, row, font);
        }
    }
}

fn draw_message(
    rl_draw: &mut impl RaylibDraw,
    viewport: Rectangle,
    top: f32,
    wrapped: &MessageLines,
    is_own: bool,
    font: &WeakFont,
) {
    let body_left: f32 = viewport.x + CONTENT_PADDING;
    let body_right: f32 = viewport.x + viewport.width - CONTENT_PADDING;

    let sender_measure: Vector2 = font.measure_text(&wrapped.sender_line, SENDER_FONT_SIZE, SENDER_FONT_SPACING);
    let sender_x: f32 = if is_own { body_right - sender_measure.x } else { body_left };
    rl_draw.draw_text_ex(
        font,
        &wrapped.sender_line,
        Vector2 { x: sender_x, y: top },
        SENDER_FONT_SIZE,
        SENDER_FONT_SPACING,
        WINDOW_INTERIOR_BORDER_COLOR,
    );

    let mut line_y: f32 = top + SENDER_FONT_SIZE + SENDER_TO_MESSAGE_GAP;
    for line in &wrapped.content_lines {
        let line_measure: Vector2 = font.measure_text(line, MESSAGE_FONT_SIZE, MESSAGE_FONT_SPACING);
        let line_x: f32 = if is_own { body_right - line_measure.x } else { body_left };
        rl_draw.draw_text_ex(
            font,
            line,
            Vector2 { x: line_x, y: line_y },
            MESSAGE_FONT_SIZE,
            MESSAGE_FONT_SPACING,
            TEXT_COLOR,
        );
        line_y += MESSAGE_FONT_SIZE + MESSAGE_LINE_GAP;
    }
}

fn draw_system_row(
    rl_draw: &mut impl RaylibDraw,
    viewport: Rectangle,
    top: f32,
    row: &SystemLine,
    font: &WeakFont,
) {
    let mut line_y: f32 = top;
    for line in &row.wrapped_lines {
        let measure: Vector2 = font.measure_text(line, SENDER_FONT_SIZE, SENDER_FONT_SPACING);
        let center_x: f32 = viewport.x + viewport.width / 2. - measure.x / 2.;
        rl_draw.draw_text_ex(
            font,
            line,
            Vector2 { x: center_x, y: line_y },
            SENDER_FONT_SIZE,
            SENDER_FONT_SPACING,
            WINDOW_INTERIOR_BORDER_COLOR,
        );
        line_y += SENDER_FONT_SIZE + MESSAGE_LINE_GAP;
    }
}
