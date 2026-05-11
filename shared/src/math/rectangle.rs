use raylib::math::{Rectangle, Vector2};

pub fn rect_origin(rectangle: Rectangle) -> Vector2 {
    Vector2 {
        x: rectangle.x,
        y: rectangle.y,
    }
}

pub fn rect_dimensions(rectangle: Rectangle) -> Vector2 {
    Vector2 {
        x: rectangle.width,
        y: rectangle.height,
    }
}

pub fn center_vertically(container_y: f32, container_height: f32, content_height: f32) -> f32 {
    container_y + (container_height - content_height) / 2.
}
