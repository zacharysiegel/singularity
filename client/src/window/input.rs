use crate::input::{ClickHandler, ClickResult, HoverHandler, HoverResult};
use crate::map::RenderCoord;
use crate::window::Window;
use raylib::RaylibHandle;
use raylib::math::{Rectangle, Vector2};

impl<T: Window> ClickHandler for T {
    fn click(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult {
        if !window_contains_render_coord(self, mouse_position) {
            return if self.is_open() {
                self.close();
                ClickResult::Consume
            } else {
                ClickResult::Pass
            };
        }

        if let ClickResult::Consume = self.close_button_mut().click(rl, mouse_position) {
            self.close();
            return ClickResult::Consume;
        }

        self.handle_window_clicked(rl, mouse_position);
        ClickResult::Consume
    }
}

impl<T: Window> HoverHandler for T {
    fn hover(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
        if !window_contains_render_coord(self, mouse_position) {
            return HoverResult::Pass;
        }

        self.close_button_mut().hover(rl, mouse_position);
        self.handle_window_hovered(rl, mouse_position)
    }
}

fn window_contains_render_coord(window: &dyn Window, render_coord: RenderCoord) -> bool {
    if !window.is_open() {
        return false;
    }

    let rectangle: Rectangle = window.try_to_rectangle().unwrap();
    rectangle.check_collision_point_rec(Vector2::from(render_coord))
}
