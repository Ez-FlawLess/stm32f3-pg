use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

use crate::utils::error;

pub fn vector_table_macro(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let Data::Struct(data_struct) = &input.data else {
        return error!(input, "#[vector_table] can only be used on structs");
    };

    let Fields::Named(fields) = &data_struct.fields else {
        return error!(data_struct.fields, "can only be used on Named Structs");
    };

    quote! {}.into()
}
