use crate::map::coordinate::{HexCoord, MapCoord, RenderCoord};
use crate::state::Hex;
use crate::window::{Window, WindowLayer};
use raylib::ffi::{Color, DrawRectangleV, DrawTextEx, GetFontDefault};
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

    fn draw(&self, _map_origin: &MapCoord) {
        if !self.is_open {
            return;
        }

        let origin: RenderCoord = self.origin.unwrap();
        let hex: Hex = self.hex.unwrap();
        unsafe {
            DrawRectangleV(
                origin.into(),
                self.dimensions().into(),
                Color {
                    r: 0x0f,
                    g: 0x0f,
                    b: 0x0f,
                    a: 0xff,
                },
            );

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
