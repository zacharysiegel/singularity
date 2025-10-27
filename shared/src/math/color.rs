use raylib::color::Color;

pub fn color_add(a: &Color, b: &Color) -> Color {
    Color {
        r: a.r + b.r,
        g: a.g + b.g,
        b: a.b + b.b,
        a: a.a + b.a,
    }
}
