use crate::color::WHITE;
use crate::map::coordinate::{HexCoord, RenderCoord};
use crate::state::Hex;
use crate::window;
use crate::window::{Window, WindowLayer};
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use raylib::RaylibHandle;
use std::ops::Add;

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
        Vector2 { x: 400., y: 300. }
    }

    fn layer(&self) -> WindowLayer {
        WindowLayer::HexWindowLayer
    }

    fn draw_content(&self, rl_draw: &mut RaylibDrawHandle) {
        draw_title(rl_draw, self);
    }

    fn handle_window_closed(&mut self) {
        self.close();
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
}

fn draw_title(rl_draw: &mut RaylibDrawHandle, window: &HexWindow) {
    let origin: RenderCoord = window.origin.unwrap();
    let hex: Hex = window.hex.unwrap();
    let hex_coord: HexCoord = hex.hex_coord;

    rl_draw.draw_text_ex(
        rl_draw.get_font_default(),
        &format!("Hex ({}, {})", hex_coord.i, hex_coord.j),
        origin.add(Vector2 { x: 20., y: 20. }),
        20.,
        2.,
        WHITE,
    );
}
