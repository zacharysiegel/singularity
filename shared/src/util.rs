use std::sync::LazyLock;

pub const SIN_PI_DIV_3: LazyLock<f64> = LazyLock::new(|| std::f64::consts::FRAC_PI_3.sin());
pub const SIN_PI_DIV_6: LazyLock<f64> = LazyLock::new(|| std::f64::consts::FRAC_PI_6.sin());

#[macro_export]
macro_rules! modular_add {
    ($m:expr, $a:expr, $b:expr) => {
        (($a.rem_euclid($m)) + ($b.rem_euclid($m))).rem_euclid($m)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn modular_addition() {
        assert_eq!(0, modular_add!(2, 0_i32, 0_i32));
        assert_eq!(1, modular_add!(2, 0_i32, 1_i32));
        assert_eq!(1, modular_add!(2, 1_i32, 0_i32));
        assert_eq!(1, modular_add!(3, 0_i32, 1_i32));
        assert_eq!(0, modular_add!(2, 1_i32, 1_i32));
        assert_eq!(2, modular_add!(3, 1_i32, 1_i32));
        assert_eq!(1, modular_add!(3, 1_i32, 3_i32));
        assert_eq!(2, modular_add!(3, 1_i32, -2_i32));
        assert_eq!(1, modular_add!(3, 1_i32, -6_i32));
    }
}
