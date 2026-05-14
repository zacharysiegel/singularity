use crate::input::{ClickButton, ClickHandler, ClickResult, HoverHandler, HoverResult, ScrollHandler, ScrollResult};
use crate::state::STATE;
use crate::window;
use crate::window::HexWindow;
use raylib::RaylibHandle;
use raylib::math::Vector2;
use shared::map::{HexCoord, MapCoord, RenderCoord};
use std::ops::{Add, Mul};
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

pub struct MapInput;

impl ScrollHandler for MapInput {
    fn scroll(&mut self, _rl: &mut RaylibHandle, scroll_v: Vector2, _mouse_position: RenderCoord) -> ScrollResult {
        let mut map_origin: RwLockWriteGuard<MapCoord> =
            STATE.stage.game.map.map_origin.write().expect("global state poisoned");

        *map_origin = scrolled_map_origin(*map_origin, scroll_v);
        ScrollResult::Consume
    }
}

impl ClickHandler for MapInput {
    fn click(&mut self, rl: &mut RaylibHandle, button: ClickButton, press_position: RenderCoord, release_position: RenderCoord) -> ClickResult {
        if button != ClickButton::Left {
            return ClickResult::Pass;
        }

        let map_origin: RwLockReadGuard<MapCoord> = STATE.stage.game.map.map_origin.read().unwrap();
        let press_hex: HexCoord = press_position.containing_hex(&*map_origin);
        let release_hex: HexCoord = release_position.containing_hex(&*map_origin);
        drop(map_origin);

        if press_hex != release_hex {
            return ClickResult::Pass;
        }

        let mut hex_window: RwLockWriteGuard<HexWindow> = STATE.stage.game.window.hex.write().unwrap();
        hex_window.open(
            rl,
            RenderCoord(Vector2::from(release_position)),
            super::clone_hex(release_hex).unwrap(),
        );
        drop(hex_window);

        ClickResult::Consume
    }
}

impl HoverHandler for MapInput {
    fn hover(&mut self, _rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
        if window::any_window_open() {
            return HoverResult::Pass;
        }

        let containing_hex_coord: HexCoord = {
            let map_origin: RwLockReadGuard<MapCoord> = STATE.stage.game.map.map_origin.read().unwrap();
            mouse_position.containing_hex(&*map_origin)
        };

        let mut hovered_hex_coord: RwLockWriteGuard<Option<HexCoord>> =
            STATE.stage.game.map.hovered_hex_coord.write().unwrap();
        *hovered_hex_coord = Some(containing_hex_coord);

        HoverResult::Consume
    }
}

fn scrolled_map_origin(map_origin: MapCoord, scroll_v: Vector2) -> MapCoord {
    let scroll_inverted: Vector2 = scroll_v.mul(Vector2 { x: -1., y: -1. });
    let unchecked_origin: Vector2 = map_origin.add(scroll_inverted);
    MapCoord(unchecked_origin).overflow_adjusted()
}
