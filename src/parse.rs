use proc_macro2::Span;
use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream, Result},
    Attribute, Error, ItemFn, ItemImpl, ItemStruct, ItemTrait, ItemType,
};

pub enum Item {
    Trait(ItemTrait),
    Impl(ItemImpl),
    Fn(ItemFn),
    Struct(ItemStruct),
    Type(ItemType),
}

macro_rules! fork {
    ($fork:ident = $input:ident) => {{
        $fork = $input.fork();
        &$fork
    }};
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let mut fork;
        let item = if let Ok(mut item) = fork!(fork = input).parse::<ItemImpl>() {
            if item.trait_.is_none() {
                return Err(Error::new(Span::call_site(), "expected a trait impl"));
            }
            item.attrs = attrs;
            Item::Impl(item)
        } else if let Ok(mut item) = fork!(fork = input).parse::<ItemTrait>() {
            item.attrs = attrs;
            Item::Trait(item)
        } else if let Ok(mut item) = fork!(fork = input).parse::<ItemFn>() {
            item.attrs = attrs;
            Item::Fn(item)
        } else if let Ok(mut item) = fork!(fork = input).parse::<ItemStruct>() {
            item.attrs = attrs;
            Item::Struct(item)
        } else if let Ok(mut item) = fork!(fork = input).parse::<ItemType>() {
            item.attrs = attrs;
            Item::Type(item)
        } else {
            return Err(Error::new(
                Span::call_site(),
                "expected trait impl, trait, fn, struct, or type",
            ));
        };
        input.advance_to(&fork);
        Ok(item)
    }
}
