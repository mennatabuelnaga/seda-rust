use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Attribute, DeriveInput, Ident, Token};

#[derive(Clone, Default)]
struct AdapterActions {
    pub db:   Option<Ident>,
    pub http: Option<Ident>,
}

impl IntoIterator for AdapterActions {
    type IntoIter = std::array::IntoIter<Self::Item, 2>;
    type Item = (&'static str, Option<Ident>);

    fn into_iter(self) -> Self::IntoIter {
        [("database", self.db), ("http", self.http)].into_iter()
    }
}

impl AdapterActions {
    fn merge(self, other: Self) -> syn::Result<Self> {
        let either = |a: Option<Ident>, b: Option<Ident>| match (a, b) {
            (None, None) => Ok(None),
            (Some(val), None) | (None, Some(val)) => Ok(Some(val)),
            (Some(lhs), Some(rhs)) => {
                let mut error = syn::Error::new_spanned(rhs, "redundant attribute argument");
                error.combine(syn::Error::new_spanned(lhs, "note: first one here"));
                Err(error)
            }
        };

        Ok(Self {
            db:   either(self.db, other.db)?,
            http: either(self.http, other.http)?,
        })
    }

    fn from_attrs(attrs: Vec<Attribute>) -> syn::Result<AdapterActions> {
        let actions = attrs
            .into_iter()
            .filter(|attr| attr.path.is_ident("adapter"))
            .try_fold(AdapterActions::default(), |act, attr| {
                let list: Punctuated<AdapterActions, Token![,]> = attr.parse_args_with(Punctuated::parse_terminated)?;

                list.into_iter().try_fold(act, AdapterActions::merge)
            })?;

        actions
            .clone()
            .into_iter()
            .try_for_each(|(field, action)| match action {
                Some(_) => Ok(()),
                None => Err(syn::Error::new_spanned(
                    quote!(
                        #field
                    ),
                    format!("Missing Adapter Type: {field}."),
                )),
            })?;
        Ok(actions)
    }
}

mod keywords {
    syn::custom_keyword!(database);
    syn::custom_keyword!(http);
}

impl Parse for AdapterActions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(keywords::database) {
            input.parse::<keywords::database>()?;
            input.parse::<syn::Token![=]>()?;
            let db = input.parse::<syn::Ident>()?;
            Ok(Self {
                db:   Some(db),
                http: None,
            })
        } else if input.peek(keywords::http) {
            input.parse::<keywords::http>()?;
            input.parse::<syn::Token![=]>()?;
            let http = input.parse::<syn::Ident>()?;
            Ok(Self {
                http: Some(http),
                db:   None,
            })
        } else {
            Err(syn::Error::new(
                input.span(),
                "Unknown adapter type or empty parse stream",
            ))
        }
    }
}

#[proc_macro_derive(Adapter, attributes(adapter))]
pub fn adapter(input: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(input as DeriveInput);
    let name = &derive.ident;
    let actions = match AdapterActions::from_attrs(derive.attrs) {
        Ok(actions) => actions,
        Err(err) => return err.to_compile_error().into(),
    };
    let db = actions.db.unwrap();
    let http = actions.http.unwrap();

    let adapter_trait_impl = quote!(
        impl HostAdapterTypes for #name {
          type Database = #db;
          type Http = #http;
        }
    );

    adapter_trait_impl.into()
}
