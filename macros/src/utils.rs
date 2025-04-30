macro_rules! error {
    ($item: expr, $message: expr) => {
        syn::Error::new(syn::spanned::Spanned::span(&$item), $message)
            .to_compile_error()
            .into()
    };
}

pub(crate) use error;
