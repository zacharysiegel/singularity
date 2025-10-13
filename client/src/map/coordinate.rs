use crate::map::config::{HEX_COUNT_SQRT, HEX_HEIGHT, HEX_RADIUS, HEX_SIDE_LENGTH};
use crate::map::state::Hex;
use crate::state::STATE;
use raylib::prelude::Vector2;
use shared::error::AppError;
use std::ops::{Deref, DerefMut, Rem, Sub};
use crate::math::{SIN_FRAC_PI_6, TAN_FRAC_PI_6};

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

    /// This conversion finds a hex coordinate based on the rectangular layout of hexes during map initialization.
    /// This conversion is unrelated to hex bounding boxes and also does not find the nearest hex center to the map coordinate. See [MapCoord::containing_hex].
    pub fn hex_coord_rect(&self) -> HexCoord {
        let mut j: i16 = ((self.y - (HEX_SIDE_LENGTH / 2.)) / (HEX_RADIUS + HEX_SIDE_LENGTH / 2.)) as i16;
        let even_row: bool = j % 2 == 0;
        let mut i: i16 = ((self.x - if even_row { 0. } else { *HEX_HEIGHT / 2. }) / *HEX_HEIGHT) as i16;

        while i < 0 {
            i += HEX_COUNT_SQRT;
        }
        while j < 0 {
            j += HEX_COUNT_SQRT;
        }

        HexCoord { i, j }
    }

    pub fn containing_hex(&self) -> Hex {
        // Rather than check the entire map, limit search to a subset of possible candidates based on the truncated hex coord conversion
        let hex_coord_rect: HexCoord = self.hex_coord_rect();
        const N_CANDIDATES: usize = 4;
        let candidate_hex_coords: [HexCoord; N_CANDIDATES] = [
            hex_coord_rect,
            HexCoord {
                i: (hex_coord_rect.i + 1).rem(HEX_COUNT_SQRT),
                j: hex_coord_rect.j,
            },
            HexCoord {
                i: hex_coord_rect.i,
                j: (hex_coord_rect.j + 1).rem(HEX_COUNT_SQRT),
            },
            HexCoord {
                i: (hex_coord_rect.i + 1).rem(HEX_COUNT_SQRT),
                j: (hex_coord_rect.j + 1).rem(HEX_COUNT_SQRT),
            },
        ];

        let mut matched_i: Option<usize> = None;
        for i in 0..N_CANDIDATES {
            let center: MapCoord = candidate_hex_coords[i].map_coord();
            let top_left: MapCoord = MapCoord(Vector2 {
                x: center.x - *HEX_HEIGHT / 2.,
                y: center.y - HEX_RADIUS,
            });
            let offset: MapCoord = MapCoord(self.sub(top_left.0)).overflow_adjusted();

            if (0. <= offset.x) && (offset.x < *HEX_HEIGHT) {
                let start: f32 = 0.;
                let partition_one: f32 = HEX_RADIUS * (*SIN_FRAC_PI_6 as f32);
                let partition_two: f32 = partition_one + HEX_RADIUS;
                let end: f32 = HEX_RADIUS * 2.;

                let matched: bool = {
                    if (start <= offset.y) && (offset.y < partition_one) {
                        let abs_evaluation: f32 = (*TAN_FRAC_PI_6 as f32) * (offset.x - *HEX_HEIGHT / 2.).abs() + 0.;
                        offset.y >= abs_evaluation
                    } else if (partition_one <= offset.y) && (offset.y < partition_two) {
                        true
                    } else if (partition_two <= offset.y) && (offset.y < end) {
                        let abs_evaluation: f32 =
                            -1. * (*TAN_FRAC_PI_6 as f32) * (offset.x - *HEX_HEIGHT / 2.).abs() + (2. * HEX_RADIUS);
                        offset.y < abs_evaluation
                    } else {
                        false
                    }
                };
                if matched {
                    matched_i = Some(i);
                    break;
                }
            }
        }

        if matched_i.is_none() {
            let error = AppError::new(&format!(
                "Failed to match containing hex among candidates; [self: {:?}] [naive: {:?}]",
                self, hex_coord_rect
            ));
            panic!("{:#}", error);
        }

        let matched_i: usize = matched_i.unwrap();

        let hexes = STATE.stage.map.hexes.read().expect("poisoned global state");
        let matched_hex: Hex = hexes[candidate_hex_coords[matched_i].map_index()];
        drop(hexes);

        matched_hex
    }

    pub fn render_coord(&self, map_origin: &MapCoord) -> RenderCoord {
        let mut x: f32 = self.x - map_origin.x;
        let mut y: f32 = self.y - map_origin.y;

        if x < -*HEX_HEIGHT / 2. {
            x += get_map_width_pixels();
        }
        if y < -HEX_RADIUS {
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

impl From<RenderCoord> for raylib::ffi::Vector2 {
    fn from(value: RenderCoord) -> Self {
        raylib::ffi::Vector2::from(value.0)
    }
}

impl RenderCoord {
    pub fn map_coord(&self, map_origin: &MapCoord) -> MapCoord {
        MapCoord(Vector2 {
            x: self.x + map_origin.x,
            y: self.y + map_origin.y,
        })
        .overflow_adjusted()
    }

    pub fn containing_hex(&self, map_origin: &MapCoord) -> Hex {
        self.map_coord(map_origin).containing_hex()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HexCoord {
    pub i: i16,
    pub j: i16,
}

impl Default for HexCoord {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl HexCoord {
    pub const DEFAULT: HexCoord = HexCoord { i: 0, j: 0 };

    pub fn clone_map_hex(&self) -> Option<Hex> {
        let hexes = STATE.stage.map.hexes.read().expect("global state poisoned");
        hexes.get(self.map_index()).map(|hex| hex.clone())
    }

    pub const fn map_index(&self) -> usize {
        (self.i + self.j * HEX_COUNT_SQRT) as usize
    }

    pub fn map_coord(&self) -> MapCoord {
        let even_row: bool = self.j % 2 == 0;
        let x: f32 = (f32::from(self.i) * *HEX_HEIGHT) + (if even_row { 0_f32 } else { *HEX_HEIGHT / 2. });
        let y: f32 = f32::from(self.j) * (HEX_RADIUS + HEX_SIDE_LENGTH / 2.);
        MapCoord(Vector2 { x, y })
    }
}

pub fn get_hex_count_width(pixels: f32) -> u16 {
    (pixels / *HEX_HEIGHT).ceil() as u16
}

pub fn get_hex_count_height(pixels: f32) -> u16 {
    (pixels / (HEX_RADIUS + HEX_SIDE_LENGTH / 2.)).ceil() as u16
}

pub fn get_hex_width_pixels(hex_count: i16) -> f32 {
    let mut result: f32 = f32::from(hex_count) * *HEX_HEIGHT;
    if hex_count % 2 == 1 {
        result -= *HEX_HEIGHT / 2_f32;
    }
    result
}

pub fn get_hex_height_pixels(hex_count: i16) -> f32 {
    f32::from(hex_count) * (HEX_RADIUS + HEX_SIDE_LENGTH / 2.)
}

pub fn get_map_width_pixels() -> f32 {
    get_hex_width_pixels(HEX_COUNT_SQRT)
}

pub fn get_map_height_pixels() -> f32 {
    get_hex_height_pixels(HEX_COUNT_SQRT)
}
