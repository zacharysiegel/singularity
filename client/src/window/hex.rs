use crate::map::coordinate::{HexCoord, MapCoord, RenderCoord};
use crate::state::Hex;
use crate::window::{Window, WindowLayer};
use raylib::ffi::{Color, DrawLineEx, DrawRectangleLinesEx, DrawRectangleRec, DrawTextEx, GetFontDefault};
use raylib::math::{Rectangle, Vector2};
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

    fn draw(&self, _map_origin: &MapCoord) {
        if !self.is_open {
            return;
        }

        draw_background(self);
        draw_title(self);
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

fn draw_background(window: &HexWindow) {
    const INNER_BORDER_GAP: f32 = 10.;
    const BORDER_COLOR: Color = Color {
        r: 0x80,
        g: 0x80,
        b: 0x80,
        a: 0xff,
    };
    const BACKGROUND_COLOR: Color = Color {
        r: 0x28,
        g: 0x2a,
        b: 0x2f,
        a: 0xff,
    };

    let origin: RenderCoord = window.origin.unwrap();
    let full: Rectangle = Rectangle {
        x: origin.x,
        y: origin.y,
        width: window.dimensions().x,
        height: window.dimensions().y,
    };
    let inner_border: Rectangle = Rectangle {
        x: origin.x + INNER_BORDER_GAP,
        y: origin.y + INNER_BORDER_GAP,
        width: window.dimensions().x - INNER_BORDER_GAP * 2.,
        height: window.dimensions().y - INNER_BORDER_GAP * 2.,
    };

    unsafe {
        DrawRectangleRec(full.into(), BACKGROUND_COLOR);
        DrawLineEx(
            Vector2 {
                x: origin.x,
                y: origin.y + INNER_BORDER_GAP,
            }
            .into(),
            Vector2 {
                x: origin.x + window.dimensions().x,
                y: origin.y + INNER_BORDER_GAP,
            }
            .into(),
            1.,
            BORDER_COLOR,
        );
        DrawLineEx(
            Vector2 {
                x: origin.x,
                y: origin.y + window.dimensions().y - INNER_BORDER_GAP,
            }
            .into(),
            Vector2 {
                x: origin.x + window.dimensions().x,
                y: origin.y + window.dimensions().y - INNER_BORDER_GAP,
            }
            .into(),
            1.,
            BORDER_COLOR,
        );
        DrawLineEx(
            Vector2 {
                x: origin.x + INNER_BORDER_GAP,
                y: origin.y,
            }
            .into(),
            Vector2 {
                x: origin.x + INNER_BORDER_GAP,
                y: origin.y + window.dimensions().y,
            }
            .into(),
            1.,
            BORDER_COLOR,
        );
        DrawLineEx(
            Vector2 {
                x: origin.x + window.dimensions().x - INNER_BORDER_GAP,
                y: origin.y,
            }
            .into(),
            Vector2 {
                x: origin.x + window.dimensions().x - INNER_BORDER_GAP,
                y: origin.y + window.dimensions().y,
            }
            .into(),
            1.,
            BORDER_COLOR,
        );
        DrawRectangleLinesEx(
            full.into(),
            1.,
            Color {
                r: 0xb0,
                g: 0xb0,
                b: 0xb0,
                a: 0xff,
            },
        );
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
            Color {
                r: 0xff,
                g: 0xff,
                b: 0xff,
                a: 0xff,
            },
        );
    }
}
