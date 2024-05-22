use std::borrow::Cow;

use litrs::InvalidToken;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

pub struct Input {
    index: usize,
    tokens: Vec<TokenTree>,
}

impl From<TokenStream> for Input {
    fn from(value: TokenStream) -> Self {
        Self {
            index: 0,
            tokens: value.into_iter().collect(),
        }
    }
}

impl Input {
    pub fn extract_str(&mut self) -> Extracted<Cow<'static, str>> {
        let Some(token) = self.tokens.get(self.index) else {
            return Extracted::Ended(
                self.tokens[..self.index]
                    .last()
                    .map_or_else(Span::mixed_site, TokenTree::span),
            );
        };
        let lit = match token {
            TokenTree::Literal(lit) => lit,
            t => return Extracted::WrongToken(t.clone()),
        };
        let a = match litrs::StringLit::try_from(lit) {
            Ok(a) => a,
            Err(e) => return Extracted::ParseError(e),
        };
        self.index += 1;
        Extracted::Right(a.into_value())
    }

    pub fn extract_punct(&mut self, ch: char) -> Extracted<()> {
        let Some(token) = self.tokens.get(self.index) else {
            return Extracted::Ended(
                self.tokens[..self.index]
                    .last()
                    .map_or_else(Span::mixed_site, TokenTree::span),
            );
        };
        match token {
            TokenTree::Punct(punct) if punct == &ch => {
                self.index += 1;
                Extracted::Right(())
            }
            t => Extracted::WrongToken(t.clone()),
        }
    }
}

pub enum Extracted<T> {
    Right(T),
    Ended(Span),
    WrongToken(TokenTree),
    ParseError(InvalidToken),
}

impl<T> Extracted<T> {
    pub fn into_err(self) -> Result<T, TokenStream> {
        match self {
            Self::Right(t) => Ok(t),
            Self::Ended(span) => Err(compile_error(span, "Proc macro ended early")),
            Self::WrongToken(t) => Err(compile_error(t.span(), "Invalid token found")),
            Self::ParseError(e) => Err(e.to_compile_error()),
        }
    }
}

/// creates a [`TokenStream`] of a [`core::compile_error`]
/// with the given [`Span`] & message
#[inline]
pub fn compile_error(span: Span, m: &str) -> TokenStream {
    macro_rules! stream {
        ($span:ident, $($unit:expr),+) => {{
            [$({
                let mut unit = TokenTree::from($unit);
                unit.set_span($span);
                unit
            },)*].into_iter().collect::<TokenStream>()
        }};
    }
    stream!(
        span,
        Punct::new(':', Spacing::Joint),
        Punct::new(':', Spacing::Alone),
        Ident::new("core", span),
        Punct::new(':', Spacing::Joint),
        Punct::new(':', Spacing::Alone),
        Ident::new("compile_error", span),
        Punct::new('!', Spacing::Alone),
        Group::new(Delimiter::Parenthesis, stream!(span, Literal::string(m)))
    )
}
