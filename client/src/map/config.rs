use crate::math::SIN_FRAC_PI_3;
use std::sync::LazyLock;

pub const HEX_COUNT_SQRT: i16 = 64;
pub const HEX_COUNT: i16 = HEX_COUNT_SQRT * HEX_COUNT_SQRT;
pub const HEX_SIDES: u8 = 6;
pub const HEX_RADIUS: f32 = 32.;
pub const HEX_SIDE_LENGTH: f32 = HEX_RADIUS;
pub const HEX_HEIGHT: LazyLock<f32> = LazyLock::new(|| *SIN_FRAC_PI_3 as f32 * f32::from(HEX_RADIUS) * 2_f32);
pub const HEX_ROTATION: f32 = 30_f32;
