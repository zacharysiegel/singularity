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
