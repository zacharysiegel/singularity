use raylib::math::Rectangle;

pub const DEFAULT_RECTANGLE: Rectangle = Rectangle {
    x: 0.,
    y: 0.,
    width: 0.,
    height: 0.,
};

#[macro_export]
macro_rules! default_const_impl {
    ($typ:ty) => {
        impl ::std::default::Default for $typ {
            fn default() -> Self {
                Self::DEFAULT
            }
        }
    };
}
