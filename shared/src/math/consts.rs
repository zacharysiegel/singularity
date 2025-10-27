use std::sync::LazyLock;

pub const SIN_FRAC_PI_3: LazyLock<f64> = LazyLock::new(|| std::f64::consts::FRAC_PI_3.sin());
pub const SIN_FRAC_PI_4: LazyLock<f64> = LazyLock::new(|| std::f64::consts::FRAC_PI_4.sin());
pub const SIN_FRAC_PI_6: LazyLock<f64> = LazyLock::new(|| std::f64::consts::FRAC_PI_6.sin());
pub const TAN_FRAC_PI_6: LazyLock<f64> = LazyLock::new(|| std::f64::consts::FRAC_PI_6.tan());
