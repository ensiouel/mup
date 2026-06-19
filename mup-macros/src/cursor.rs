use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Span, TokenStream, TokenTree};
use quote::{ToTokens, quote};

#[derive(Clone)]
pub struct Cursor {
    tokens: Vec<TokenTree>,
    pos: usize,
}

impl Cursor {
    pub fn new(tokens: TokenStream) -> Self {
        Self {
            tokens: tokens.into_iter().collect(),
            pos: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    pub fn peek(&self) -> Option<&TokenTree> {
        self.tokens.get(self.pos)
    }

    pub fn peek_n(&self, n: usize) -> Option<&TokenTree> {
        self.tokens.get(self.pos + n)
    }

    pub fn next(&mut self) -> Option<TokenTree> {
        let token = self.tokens.get(self.pos).cloned();
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    pub fn span(&self) -> Span {
        self.peek().map_or_else(Span::call_site, TokenTree::span)
    }

    pub fn peek_ident(&self, value: &str) -> bool {
        matches!(self.peek(), Some(TokenTree::Ident(ident)) if ident == value)
    }

    pub fn consume_ident(&mut self, value: &str) -> Option<Ident> {
        if self.peek_ident(value) {
            match self.next() {
                Some(TokenTree::Ident(ident)) => Some(ident),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn peek_punct(&self, value: char) -> bool {
        matches!(self.peek(), Some(TokenTree::Punct(punct)) if punct.as_char() == value)
    }

    pub fn peek_punct_n(&self, n: usize, value: char) -> bool {
        matches!(self.peek_n(n), Some(TokenTree::Punct(punct)) if punct.as_char() == value)
    }

    pub fn consume_punct(&mut self, value: char) -> Option<Punct> {
        if self.peek_punct(value) {
            match self.next() {
                Some(TokenTree::Punct(punct)) => Some(punct),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn consume_double_punct(&mut self, first: char, second: char) -> bool {
        if self.peek_punct(first) && self.peek_punct_n(1, second) {
            self.next();
            self.next();
            true
        } else {
            false
        }
    }

    pub fn peek_group(&self, delimiter: Delimiter) -> bool {
        matches!(self.peek(), Some(TokenTree::Group(group)) if group.delimiter() == delimiter)
    }

    pub fn consume_group(&mut self, delimiter: Delimiter) -> Option<Group> {
        if self.peek_group(delimiter) {
            match self.next() {
                Some(TokenTree::Group(group)) => Some(group),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn next_ident(&mut self) -> syn::Result<Ident> {
        match self.next() {
            Some(TokenTree::Ident(ident)) => Ok(ident),
            Some(token) => Err(syn::Error::new(token.span(), "expected identifier")),
            None => Err(syn::Error::new(Span::call_site(), "expected identifier")),
        }
    }

    pub fn next_literal(&mut self) -> syn::Result<Literal> {
        match self.next() {
            Some(TokenTree::Literal(lit)) => Ok(lit),
            Some(token) => Err(syn::Error::new(token.span(), "expected literal")),
            None => Err(syn::Error::new(Span::call_site(), "expected literal")),
        }
    }

    pub fn collect_until_semicolon(&mut self) -> syn::Result<TokenStream> {
        let mut out = TokenStream::new();
        while let Some(token) = self.next() {
            if matches!(&token, TokenTree::Punct(punct) if punct.as_char() == ';') {
                return Ok(out);
            }
            token.to_tokens(&mut out);
        }
        Err(syn::Error::new(
            Span::call_site(),
            "expected `;` before end of markup input",
        ))
    }

    pub fn collect_until_brace_group(&mut self) -> syn::Result<(TokenStream, Group)> {
        let mut out = TokenStream::new();
        while let Some(token) = self.next() {
            match token {
                TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => {
                    return Ok((out, group));
                }
                token => token.to_tokens(&mut out),
            }
        }
        Err(syn::Error::new(
            Span::call_site(),
            "expected `{ ... }` before end of markup input",
        ))
    }

    pub fn collect_until_arrow(&mut self) -> syn::Result<TokenStream> {
        let mut out = TokenStream::new();
        while !self.is_empty() {
            if self.consume_double_punct('=', '>') {
                return Ok(out);
            }
            if let Some(token) = self.next() {
                token.to_tokens(&mut out);
            }
        }
        Err(syn::Error::new(
            Span::call_site(),
            "expected `=>` in component enum render arm",
        ))
    }

    pub fn skip_optional_comma(&mut self) {
        let _ = self.consume_punct(',');
    }

    pub fn unexpected(&self, message: &str) -> syn::Error {
        syn::Error::new(self.span(), message)
    }
}

pub fn token_stream_from_group(group: &Group) -> TokenStream {
    let stream = group.stream();
    match group.delimiter() {
        Delimiter::Parenthesis => quote!((#stream)),
        Delimiter::Brace => quote!({ #stream }),
        Delimiter::Bracket => quote!([#stream]),
        Delimiter::None => stream,
    }
}
