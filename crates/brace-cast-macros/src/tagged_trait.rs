use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, ItemTrait, TypeParamBound};

pub fn expand(mut input: ItemTrait) -> TokenStream {
    let mut output = TokenStream::new();
    let from = &input.ident;

    for supertrait in &input.supertraits {
        if let TypeParamBound::Trait(trait_bound) = supertrait {
            let into = &trait_bound.path;

            output.extend(quote! {
                brace_cast::impl_cast_as!(trait #from : #into);
            });
        }
    }

    input.supertraits.push(parse_quote!(brace_cast::Cast));

    quote! {
        #input
        #output
    }
}
