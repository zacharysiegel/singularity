use crate::input::{
    CharPressHandler, CharPressResult,
    ClickHandler, ClickResult, HoverHandler, HoverResult, KeyPressHandler, KeyPressResult, ScrollHandler, ScrollResult,
};
use crate::window::Window;
use raylib::RaylibHandle;
use raylib::consts::KeyboardKey;
use raylib::math::{Rectangle, Vector2};
use shared::map::RenderCoord;

impl<T: Window> ScrollHandler for T {
    fn scroll(&mut self, rl: &mut RaylibHandle, scroll_v: Vector2) -> ScrollResult {
        if self.is_open() {
            self.handle_window_scroll(rl, scroll_v)
        } else {
            ScrollResult::Pass
        }
    }
}

impl<T: Window> ClickHandler for T {
    fn click(&mut self, rl: &mut RaylibHandle, press_position: RenderCoord, release_position: RenderCoord) -> ClickResult {
        let press_outside: bool = !window_contains_render_coord(self, press_position);
        let release_outside: bool = !window_contains_render_coord(self, release_position);

        if press_outside && release_outside {
            return if self.is_open() {
                self.close();
                ClickResult::Consume
            } else {
                ClickResult::Pass
            };
        }

        if press_outside {
            return ClickResult::Pass;
        }

        if release_outside {
            return ClickResult::Consume;
        }

        if let ClickResult::Consume = self.close_button_mut().click(rl, press_position, release_position) {
            self.close();
            return ClickResult::Consume;
        }

        self.handle_window_click(rl, press_position, release_position);
        ClickResult::Consume
    }
}

impl<T: Window> HoverHandler for T {
    fn hover(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
        if !window_contains_render_coord(self, mouse_position) {
            return HoverResult::Pass;
        }

        self.close_button_mut().hover(rl, mouse_position);
        self.handle_window_hover(rl, mouse_position)
    }
}

impl<T: Window> KeyPressHandler for T {
    fn key_press(&mut self, rl: &mut RaylibHandle, key: KeyboardKey) -> KeyPressResult {
        if self.is_open() {
            if key == KeyboardKey::KEY_ESCAPE {
                self.close();
                KeyPressResult::Consume
            } else {
                self.handle_window_key_press(rl, key)
            }
        } else {
            KeyPressResult::Pass
        }
    }
}

impl<T: Window> CharPressHandler for T {
    fn char_press(&mut self, rl: &mut RaylibHandle, ch: char) -> CharPressResult {
        if self.is_open() {
            self.handle_window_char_press(rl, ch)
        } else {
            CharPressResult::Pass
        }
    }
}

fn window_contains_render_coord(window: &dyn Window, render_coord: RenderCoord) -> bool {
    if !window.is_open() {
        return false;
    }

    let rectangle: Rectangle = window.try_to_rectangle().unwrap();
    rectangle.check_collision_point_rec(Vector2::from(render_coord))
}
