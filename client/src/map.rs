use crate::config::{
    HEX_COUNT_SQRT, HEX_HEIGHT, HEX_OUTLINE_COLOR, HEX_RADIUS, HEX_ROTATION, HEX_SIDE_LENGTH,
    HEX_SIDES, get_hex_count_height, get_hex_count_width, get_hex_width_pixels,
    get_map_height_pixels, get_map_width_pixels,
};
use crate::state::{
    Facility, FacilityState, FacilityType, Hex, HexCoord, ResourceType, STATE, State,
};
use raylib::color::Color;
use raylib::ffi::{DrawPoly, DrawPolyLinesEx, DrawText, GetScreenHeight, GetScreenWidth, Vector2};
use std::ffi::{CString, c_int};
use std::ops::{Deref, DerefMut};
use std::sync::RwLockWriteGuard;

#[derive(Debug, Copy, Clone)]
pub struct MapCoord(pub Vector2);

impl Deref for MapCoord {
    type Target = Vector2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MapCoord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for MapCoord {
    fn default() -> Self {
        MapCoord::DEFAULT
    }
}

impl MapCoord {
    pub const DEFAULT: MapCoord = MapCoord(Vector2 { x: 0.0, y: 0.0 });

    pub fn hex_coord(&self) -> HexCoord {
        let mut j: i16 = ((self.y - f32::from(HEX_SIDE_LENGTH / 2))
            / f32::from(HEX_RADIUS + HEX_SIDE_LENGTH / 2)) as i16;
        let even_row: bool = j % 2 == 0;
        let mut i: i16 =
            ((self.x - if even_row { 0. } else { *HEX_HEIGHT / 2. }) / *HEX_HEIGHT) as i16;

        while i < 0 {
            i += HEX_COUNT_SQRT as i16;
        }
        while j < 0 {
            j += HEX_COUNT_SQRT as i16;
        }

        HexCoord {
            i: i as u16,
            j: j as u16,
        }
    }

    pub fn render_coord(&self, map_origin: &MapCoord) -> RenderCoord {
        let mut x: f32 = self.x - map_origin.x;
        let mut y: f32 = self.y - map_origin.y;

        if x < -*HEX_HEIGHT / 2. {
            x += get_map_width_pixels();
        }
        if y < -f32::from(HEX_RADIUS) {
            y += get_map_height_pixels();
        }

        RenderCoord(Vector2 { x, y })
    }

    pub fn overflow_adjusted(&mut self) -> Self {
        let map_width_pixels: f32 = get_map_width_pixels();
        let map_height_pixels: f32 = get_map_height_pixels();

        while self.x < 0. {
            self.x += map_width_pixels;
        }
        while self.y < 0. {
            self.y += map_height_pixels;
        }

        while self.x >= map_width_pixels {
            self.x -= map_width_pixels;
        }
        while self.y >= map_height_pixels {
            self.y -= map_height_pixels;
        }

        *self
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RenderCoord(pub Vector2);

impl Deref for RenderCoord {
    type Target = Vector2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RenderCoord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for RenderCoord {
    fn default() -> Self {
        RenderCoord(Vector2 { x: 0.0, y: 0.0 })
    }
}

impl From<RenderCoord> for Vector2 {
    fn from(value: RenderCoord) -> Self {
        value.0
    }
}

pub fn init_map() {
    for i in 0..HEX_COUNT_SQRT {
        for j in 0..HEX_COUNT_SQRT {
            let hex_coord: HexCoord = HexCoord { i, j };
            let resource_type: ResourceType = init_resource_type_from_hex_coord(&hex_coord);
            let hex: Hex = Hex {
                hex_coord,
                resource_type,
            };
            let mut state_g: RwLockWriteGuard<State> =
                STATE.write().expect("global state poisoned");
            let i: usize = hex.hex_coord.map_index();
            state_g.hexes[i] = hex;
        }
    }
}

// todo: implement planned strategy (plan.md)
const fn init_resource_type_from_hex_coord(hex_coord: &HexCoord) -> ResourceType {
    if (hex_coord.i % (HEX_COUNT_SQRT / 4)) == 10 && hex_coord.j % (HEX_COUNT_SQRT / 4) == 4 {
        ResourceType::Metal
    } else if (hex_coord.i % (HEX_COUNT_SQRT / 4) == 2 && hex_coord.j % (HEX_COUNT_SQRT / 4) == 12)
    {
        ResourceType::Oil
    } else {
        ResourceType::None
    }
}

pub fn draw_map(map_origin: &MapCoord) {
    let screen_width: i32 = unsafe { GetScreenWidth() };
    let screen_height: i32 = unsafe { GetScreenHeight() };
    let origin_hex_coord: HexCoord = map_origin.hex_coord();
    let min_hex_coord: HexCoord = HexCoord {
        i: if i32::from(origin_hex_coord.i) - 1 < 0 {
            HEX_COUNT_SQRT - 1
        } else {
            origin_hex_coord.i - 1
        },
        j: if i32::from(origin_hex_coord.j) - 1 < 0 {
            HEX_COUNT_SQRT - 1
        } else {
            origin_hex_coord.j - 1
        },
    };
    let mut hex_coord: HexCoord = min_hex_coord;

    let max_hexes_i: u16 = get_hex_count_width(screen_width as f32);
    let max_hexes_j: u16 = get_hex_count_height(screen_height as f32);
    for _hexes_drawn_j in 0..(max_hexes_j + 2) {
        for _hexes_drawn_i in 0..(max_hexes_i + 2) {
            draw_map_hex(map_origin, &hex_coord);

            hex_coord.i += 1;
            if hex_coord.i >= HEX_COUNT_SQRT {
                hex_coord.i = 0;
            }
        }

        hex_coord.i = min_hex_coord.i;
        hex_coord.j += 1;
        if hex_coord.j >= HEX_COUNT_SQRT {
            hex_coord.j = 0;
        }
    }
}

fn draw_map_hex(map_origin: &MapCoord, hex_coord: &HexCoord) {
    let map_coord: MapCoord = hex_coord.map_coord();
    let render_coord: RenderCoord = map_coord.render_coord(map_origin);
    let Some(hex): Option<Hex> = hex_coord.clone_map_hex() else {
        panic!("Invalid hex coord: {:?}", hex_coord);
    };
    let color: Color = hex.resource_type.color();

    match hex.resource_type {
        ResourceType::None => {}
        _ => unsafe {
            DrawPoly(
                render_coord.into(),
                HEX_SIDES as c_int,
                f32::from(HEX_RADIUS),
                HEX_ROTATION,
                color.into(),
            );
        },
    }
    unsafe {
        DrawPolyLinesEx(
            render_coord.into(),
            HEX_SIDES as c_int,
            f32::from(HEX_RADIUS),
            HEX_ROTATION,
            1.,
            HEX_OUTLINE_COLOR.into(),
        );
    }
}

pub fn draw_players(map_origin: &MapCoord) {
    let state = STATE.read().expect("global state poisoned");
    for player in &state.players {
        for facility in &player.facilities {
            draw_facility(map_origin, facility);
        }
    }
}

fn draw_facility(map_origin: &MapCoord, facility: &Facility) {
    let map_coord: MapCoord = facility.location.map_coord();
    let render_coord: RenderCoord = map_coord.render_coord(map_origin);
    let color: Color = {
        let mut color = Color {
            r: 0xb4,
            g: 0xb4,
            b: 0xb4,
            a: 0xff,
        };
        match facility.facility_state {
            FacilityState::Operating => {}
            FacilityState::Placing => color.a = 0x80,
            FacilityState::Destroyed => color.a = 0xf0,
        }
        color
    };

    match facility.facility_type {
        FacilityType::ControlCenter => unsafe {
            let cstr = CString::new("CC").unwrap();
            DrawText(
                cstr.as_ptr(),
                render_coord.x as i32 - 10,
                render_coord.y as i32 - 10,
                10,
                color.into(),
            );
        },
        FacilityType::MetalExtractor => unsafe {
            let cstr = CString::new("ME").unwrap();
            DrawText(
                cstr.as_ptr(),
                render_coord.x as i32 - 10,
                render_coord.y as i32 - 10,
                10,
                color.into(),
            );
        },
        FacilityType::OilExtractor => unsafe {
            let cstr = CString::new("OE").unwrap();
            DrawText(
                cstr.as_ptr(),
                render_coord.x as i32 - 10,
                render_coord.y as i32 - 10,
                10,
                color.into(),
            );
        },
    }
}
