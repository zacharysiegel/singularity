use crate::button::RectangularButton;
use crate::input::ScrollResult;
use crate::map::RenderCoord;
use crate::map::{Hex, ResourceType};
use crate::player::Player;
use crate::state::STATE;
use crate::window;
use crate::window::state::WindowLayer;
use crate::window::Window;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use raylib::text::RaylibFont;
use raylib::{RaylibHandle, RaylibThread};
use std::ops::Add;
use std::sync::RwLockReadGuard;
use window::draw::BORDER_GAP;

const FONT_SPACING: f32 = 2.;

#[derive(Debug)]
pub struct HexWindow {
    pub origin: Option<RenderCoord>,
    pub hex: Option<Hex>,
    pub close_button: RectangularButton,
}

impl Window for HexWindow {
    fn is_open(&self) -> bool {
        self.origin.is_some()
    }

    fn close(&mut self) {
        self.origin = Self::DEFAULT.origin;
        self.hex = Self::DEFAULT.hex;
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

    fn close_button(&self) -> &RectangularButton {
        &self.close_button
    }

    fn close_button_mut(&mut self) -> &mut RectangularButton {
        &mut self.close_button
    }

    fn draw_content(&self, rl_draw: &mut RaylibDrawHandle, _rl_thread: &RaylibThread) {
        self.draw_title(rl_draw);
        self.draw_footer(rl_draw);
    }

    fn handle_window_scroll(&mut self, _rl: &mut RaylibHandle, _scroll_v: Vector2) -> ScrollResult {
        ScrollResult::Pass
    }
}

impl HexWindow {
    pub const DEFAULT: HexWindow = HexWindow {
        origin: None,
        hex: None,
        close_button: RectangularButton::DEFAULT,
    };

    pub fn open(&mut self, rl: &mut RaylibHandle, origin: RenderCoord, hex: Hex) {
        self.origin = Some(window::bounded_origin(rl, origin, self.dimensions()));
        self.hex = Some(hex);
        self.close_button = RectangularButton::new(window::side_button_rectangle(self, 0));
    }

    fn resource_text(&self) -> Option<&'static str> {
        match self.hex?.resource_type {
            ResourceType::None => None,
            ResourceType::Metal => Some("Resource: METAL"),
            ResourceType::Oil => Some("Resource: OIL"),
        }
    }

    fn facility_text(&self) -> Option<&'static str> {
        let players: RwLockReadGuard<Vec<Player>> = STATE.stage.game.player.players.read().unwrap();
        for player in players.iter() {
            match player.facilities.at(self.hex?.hex_coord) {
                None => continue,
                Some(facility) => return Some(facility.display_name()),
            }
        }
        None
    }

    fn title(&self) -> String {
        let resource_text: Option<&str> = self.resource_text();
        let facility_text: Option<&str> = self.facility_text();

        let mut title = String::new();
        if let Some(text) = resource_text {
            title.push_str(text);
        }
        if let Some(text) = facility_text {
            if !title.is_empty() {
                title.push('\n');
            }
            title.push_str(text);
        }

        if title.is_empty() {
            title.push_str("Empty");
        }
        title
    }
}

mod draw {
    use crate::color::TEXT_COLOR;
    use crate::map::{Hex, RenderCoord};
    use crate::window::hex::FONT_SPACING;
    use crate::window::{HexWindow, Window, BORDER_GAP};
    use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
    use raylib::math::Vector2;
    use raylib::prelude::{RaylibFont, WeakFont};
    use std::ops::Add;

    impl HexWindow {
        pub fn draw_title(&self, rl_draw: &mut RaylibDrawHandle) {
            let origin: RenderCoord = self.origin.unwrap();
            const FONT_SIZE: f32 = 20.;

            rl_draw.draw_text_ex(
                rl_draw.get_font_default(),
                &self.title(),
                origin.add(Vector2 { x: 20., y: 20. }),
                FONT_SIZE,
                FONT_SPACING,
                TEXT_COLOR,
            );
        }

        pub fn draw_footer(&self, rl_draw: &mut RaylibDrawHandle) {
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
}
