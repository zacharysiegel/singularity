use proc_macro::TokenStream;

mod facility_collection;

#[proc_macro_attribute]
pub fn facility_collection(attr: TokenStream, item: TokenStream) -> TokenStream {
    facility_collection::facility_collection_impl(attr, item)
}
