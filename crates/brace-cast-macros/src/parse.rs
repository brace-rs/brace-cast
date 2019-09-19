use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Attribute, Error, ItemImpl, ItemTrait, Token, Visibility};

pub enum Input {
    Trait(ItemTrait),
    Impl(ItemImpl),
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attrs = Attribute::parse_outer(input)?;
        let ahead = input.fork();

        ahead.parse::<Visibility>()?;
        ahead.parse::<Option<Token![unsafe]>>()?;

        if ahead.peek(Token![trait]) {
            let mut item: ItemTrait = input.parse()?;

            attrs.extend(item.attrs);
            item.attrs = attrs;

            return Ok(Input::Trait(item));
        }

        if ahead.peek(Token![impl]) {
            let mut item: ItemImpl = input.parse()?;

            if item.trait_.is_none() {
                let impl_token = item.impl_token;
                let ty = item.self_ty;
                let span = quote!(#impl_token #ty);
                let msg = "expected impl Trait for Type";

                return Err(Error::new_spanned(span, msg));
            }

            attrs.extend(item.attrs);
            item.attrs = attrs;

            return Ok(Input::Impl(item));
        }

        Err(input.error("expected trait Trait or impl Trait for Type"))
    }
}
