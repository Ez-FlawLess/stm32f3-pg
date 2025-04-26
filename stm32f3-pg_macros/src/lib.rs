use proc_macro::TokenStream;

mod utils;
mod vector_table;

#[proc_macro_attribute]
pub fn vector_table(attr: TokenStream, item: TokenStream) -> TokenStream {
    vector_table::vector_table_macro(attr, item)
}
