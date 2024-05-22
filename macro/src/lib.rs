//! The macros
use proc_macro::{Delimiter, Group, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use regex::Regex;
use utils::{compile_error, Input};

// pub(crate) mod parse_str;
pub(crate) mod utils;

mod util {}

/// Regex at compiletime
#[proc_macro]
pub fn cotirema(input: TokenStream) -> TokenStream {
    match inner(&mut Input::from(input)) {
        Ok(tt) | Err(tt) => tt,
    }
}

#[inline]
fn inner(input: &mut Input) -> Result<TokenStream, TokenStream> {
    let re = input.extract_str().into_err()?;
    input.extract_punct(',').into_err()?;
    let haystack = input.extract_str().into_err()?;
    let mut tokens = Vec::<TokenTree>::new();
    for find in Regex::new(&re)
        .map_err(|e| compile_error(Span::mixed_site(), &e.to_string()))?
        .find_iter(&haystack)
    {
        tokens.push(Literal::string(find.as_str()).into());
        tokens.push(Punct::new(',', Spacing::Alone).into());
    }
    Ok(TokenStream::from(TokenTree::from(Group::new(
        Delimiter::Bracket,
        TokenStream::from_iter(tokens),
    ))))
}
