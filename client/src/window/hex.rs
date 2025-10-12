use crate::color::WHITE;
use crate::map::coordinate::{HexCoord, MapCoord, RenderCoord};
use crate::state::Hex;
use crate::window;
use crate::window::{Window, WindowLayer};
use raylib::drawing::RaylibDrawHandle;
use raylib::ffi::{DrawTextEx, GetFontDefault};
use raylib::math::Vector2;
use std::ffi::CString;
use std::ops::Add;
use std::str::FromStr;

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

    fn draw(&self, rl_draw: &mut RaylibDrawHandle, _map_origin: &MapCoord) {
        if !self.is_open {
            return;
        }

        window::draw_window_base(rl_draw, self);
        draw_title(self);
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

    pub fn open(&mut self, origin: RenderCoord, hex: Hex) {
        self.is_open = true;
        self.origin = Some(origin);
        self.hex = Some(hex);
    }

    pub fn close(&mut self) {
        self.is_open = Self::DEFAULT.is_open;
        self.origin = Self::DEFAULT.origin;
        self.hex = Self::DEFAULT.hex;
    }
}

fn draw_title(window: &HexWindow) {
    let origin: RenderCoord = window.origin.unwrap();
    let hex: Hex = window.hex.unwrap();
    unsafe {
        let hex_coord: HexCoord = hex.hex_coord;
        let cstr: CString = CString::from_str(&format!("Hex ({}, {})", hex_coord.i, hex_coord.j)).unwrap();
        DrawTextEx(
            GetFontDefault(),
            cstr.as_ptr(),
            origin.add(Vector2 { x: 20., y: 20. }).into(),
            20.,
            2.,
            WHITE.into(),
        );
    }
}
