use crate::color::TEXT_COLOR;
use crate::input::ClickResult;
use crate::map::coordinate::RenderCoord;
use crate::state::{Hex, ResourceType};
use crate::window;
use crate::window::{Window, WindowLayer};
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::WeakFont;
use raylib::text::RaylibFont;
use raylib::RaylibHandle;
use std::ops::Add;
use window::draw::BORDER_GAP;

const FONT_SPACING: f32 = 2.;

#[derive(Debug)]
pub struct HexWindow {
    pub is_open: bool,
    pub origin: Option<RenderCoord>,
    pub hex: Option<Hex>,
}

impl Window for HexWindow {
    fn is_open(&self) -> bool {
        self.is_open
    }

    fn origin(&self) -> Option<RenderCoord> {
        self.origin
    }

    fn dimensions(&self) -> Vector2 {
        Vector2 { x: 300., y: 200. }
    }

    fn layer(&self) -> WindowLayer {
        WindowLayer::HexWindowLayer
    }

    fn draw_content(&self, rl_draw: &mut RaylibDrawHandle) {
        self.draw_title(rl_draw);
        self.draw_buttons(rl_draw);
        self.draw_footer(rl_draw);
    }

    fn handle_window_closed(&mut self) {
        self.close();
    }

    fn handle_window_clicked(&mut self, _rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult {
        let b1_rect: Rectangle = window::draw::side_button_rectangle(self, 1);
        if b1_rect.check_collision_point_rec(mouse_position) {
            log::debug!("Clicked button 1");
        }
        ClickResult::Consume
    }
}

impl HexWindow {
    pub const DEFAULT: HexWindow = HexWindow {
        is_open: false,
        origin: None,
        hex: None,
    };

    pub fn open(&mut self, rl: &mut RaylibHandle, mut origin: RenderCoord, hex: Hex) {
        self.is_open = true;
        self.origin = Some(window::bounded_origin(rl, &mut origin, self.dimensions()));
        self.hex = Some(hex);
    }

    pub fn close(&mut self) {
        self.is_open = Self::DEFAULT.is_open;
        self.origin = Self::DEFAULT.origin;
        self.hex = Self::DEFAULT.hex;
    }

    pub fn get_title(&self) -> Option<&'static str> {
        Some(match self.hex?.resource_type {
            ResourceType::None => "Empty Hex",
            ResourceType::Metal => "Resource: METAL",
            ResourceType::Oil => "Resource: OIL",
        })
    }

    fn draw_title(&self, rl_draw: &mut RaylibDrawHandle) {
        let origin: RenderCoord = self.origin.unwrap();
        let title_text: &str = self.get_title().unwrap();

        const FONT_SIZE: f32 = 20.;

        rl_draw.draw_text_ex(
            rl_draw.get_font_default(),
            title_text,
            origin.add(Vector2 { x: 20., y: 20. }),
            FONT_SIZE,
            FONT_SPACING,
            TEXT_COLOR,
        );
    }

    fn draw_buttons(&self, rl_draw: &mut RaylibDrawHandle) {
        window::draw::draw_side_button(rl_draw, self, 1);
    }

    fn draw_footer(&self, rl_draw: &mut RaylibDrawHandle) {
        let hex: Hex = self.hex.unwrap();
        let origin: RenderCoord = self.origin().unwrap();

        const FONT_SIZE: f32 = 12.;
        const FOOTER_HEIGHT: f32 = 20.;
        const FOOTER_MARGIN: f32 = 8.;

        let font: WeakFont = rl_draw.get_font_default();
        let footer_origin: RenderCoord = RenderCoord(Vector2 {
            x: origin.x + BORDER_GAP,
            y: origin.y + self.dimensions().y - BORDER_GAP - FOOTER_HEIGHT,
        });
        let footer_width: f32 = self.dimensions().x - BORDER_GAP * 2.;

        let coord_text: String = format!("({}, {})", hex.hex_coord.i, hex.hex_coord.j);
        let text_measurement: Vector2 = font.measure_text(&coord_text, FONT_SIZE, FONT_SPACING);
        let coord_location: Vector2 = Vector2 {
            x: footer_origin.x + footer_width - text_measurement.x - FOOTER_MARGIN,
            y: footer_origin.y,
        };
        rl_draw.draw_text_ex(font, &coord_text, coord_location, FONT_SIZE, FONT_SPACING, TEXT_COLOR);
    }
}
