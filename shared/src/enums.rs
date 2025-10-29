/// Requires `#[derive(::strum::FromRepr)]` on the enum
/// See: [::strum::FromRepr]
#[macro_export]
macro_rules! try_from_repr {
    ($id:ident<$typ:ty>$(,)?) => {
        impl ::std::convert::TryFrom<$typ> for $id {
            type Error = $crate::error::AppErrorStatic;

            fn try_from(discriminant: $typ) -> Result<Self, Self::Error> {
                match $id::from_repr(discriminant) {
                    ::std::option::Option::Some(variant) => ::std::result::Result::Ok(variant),
                    ::std::option::Option::None => ::std::result::Result::Err($crate::error::AppErrorStatic::new(
                        &format!("Error parsing enumeration [{}]", discriminant,),
                    )),
                }
            }
        }
    };
}
