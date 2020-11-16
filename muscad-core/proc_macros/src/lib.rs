use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, GenericParam, ItemStruct, TypeParam};

#[proc_macro_derive(ErrStr)]
pub fn derive_as_str_fn(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let ItemStruct {
        mut generics,
        ident,
        ..
    } = input;

    for p in generics.params.iter_mut() {
        if let GenericParam::Type(v) = p {
            let s = v.ident.to_string();

            *v = syn::parse_str::<TypeParam>(&s).unwrap();
        }
    }

    (quote! {
        impl#generics ErrStr for #ident#generics {
            fn as_str() -> &'static str {
                stringify!(#ident)
            }
        }
    })
    .into()
}
