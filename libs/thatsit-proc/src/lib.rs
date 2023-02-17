extern crate proc_macro;
use proc_macro::TokenStream;

mod impls;

macro_rules! export_macro {
    ($name:ident) => {
        #[proc_macro_attribute]
        pub fn $name (attrs: TokenStream, input: TokenStream) -> TokenStream {
            crate::impls::$name(attrs.into(), input.into()).into()
        }
    }
}

export_macro!(widget);

export_macro!(literate);
