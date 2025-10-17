use proc_macro::TokenStream;
use syn::{parse_macro_input, Error, Fields, Ident, ItemEnum};

enum X {
    A(Ident),
    B({x: Ident})
}

pub fn facility_collection_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args: Ident = parse_macro_input!(attr as Ident);
    let item_enum: ItemEnum = parse_macro_input!(item as ItemEnum);

    let enum_name: &Ident = &item_enum.ident;

    for variant in &item_enum.variants {
        let fields: &Fields = &variant.fields;
        match fields {
            Fields::Named(_) => {
                unreachable!("Named fields are not used in enum definitions");
            }
            Fields::Unnamed(fields) => {
                
            }
            Fields::Unit => {}
        }
    }

    todo!()
}

/*
return Error::new_spanned(
    &input_struct.ident,
    "hello attribute can only be used on structs with named fields"
).to_compile_error().into();
 */
