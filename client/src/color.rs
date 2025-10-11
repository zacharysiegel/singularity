use raylib::color::Color;

pub const TEXT_COLOR: Color = Color {
    r: 0xf0,
    g: 0xf0,
    b: 0xf0,
    a: 0xff,
};
pub const TRANSPARENT: Color = Color {
    r: 0x00,
    g: 0x00,
    b: 0x00,
    a: 0x00,
};
pub const WHITE: Color = Color {
    r: 0xff,
    g: 0xff,
    b: 0xff,
    a: 0xff,
};

pub const MAP_BACKGROUND_COLOR: Color = Color {
    r: 0x1d,
    g: 0x1d,
    b: 0x1d,
    a: 0xff,
};
pub const HEX_OUTLINE_COLOR: Color = Color {
    r: 0xff,
    g: 0xff,
    b: 0xff,
    a: 0x80,
};
pub const METAL_BACKGROUND_COLOR: Color = Color {
    r: 0x60,
    g: 0x50,
    b: 0x70,
    a: 0xff,
};
pub const OIL_BACKGROUND_COLOR: Color = Color {
    r: 0x58,
    g: 0x58,
    b: 0x58,
    a: 0xff,
};

pub const FACILITY_OPERATING_COLOR: Color = Color {
    r: 0xb4,
    g: 0xb4,
    b: 0xb4,
    a: 0xff,
};
pub const FACILITY_PLACING_COLOR: Color = Color {
    a: 0x80,
    ..FACILITY_OPERATING_COLOR
};
pub const FACILITY_DESTROYED_COLOR: Color = Color {
    a: 0xf0,
    ..FACILITY_OPERATING_COLOR
};

pub const WINDOW_BORDER_COLOR: Color = Color {
    r: 0xb0,
    g: 0xb0,
    b: 0xb0,
    a: 0xff,
};
pub const WINDOW_INTERIOR_BORDER_COLOR: Color = Color {
    r: 0x80,
    g: 0x80,
    b: 0x80,
    a: 0xff,
};
pub const WINDOW_BACKGROUND_COLOR: Color = Color {
    r: 0x28,
    g: 0x2a,
    b: 0x2f,
    a: 0xff,
};
