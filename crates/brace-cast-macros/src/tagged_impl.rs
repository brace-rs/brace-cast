use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemImpl;

pub fn expand(input: ItemImpl) -> TokenStream {
    let from = &input.self_ty;
    let into = &input.trait_.as_ref().unwrap().1;

    quote! {
        #input
        brace_cast::impl_cast_as!(struct #from : #into);
    }
}
