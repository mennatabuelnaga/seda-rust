mod adapter;
mod call_self;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Adapter, attributes(adapter))]
pub fn adapter(input: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(input as DeriveInput);
    let name = &derive.ident;
    let actions = match adapter::AdapterActions::from_attrs(derive.attrs) {
        Ok(actions) => actions,
        Err(err) => return err.to_compile_error().into(),
    };
    let db = actions.db.unwrap();
    let http = actions.http.unwrap();

    let adapter_trait_impl = quote::quote!(
        impl HostAdapterTypes for #name {
          type Database = #db;
          type Http = #http;
        }
    );

    adapter_trait_impl.into()
}

#[proc_macro_attribute]
pub fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    call_self::main(args, item)
}
