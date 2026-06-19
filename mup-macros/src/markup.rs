use crate::cursor::{Cursor, token_stream_from_group};
use crate::mup_path;
use proc_macro2::{Delimiter, Group, Ident, Literal, Span, TokenStream, TokenTree};
use quote::{ToTokens, format_ident, quote};
use syn::LitStr;

pub fn expand(input: TokenStream) -> TokenStream {
    match expand_result(input) {
        Ok(tokens) => tokens,
        Err(error) => error.to_compile_error(),
    }
}

fn expand_result(input: TokenStream) -> syn::Result<TokenStream> {
    let crate_path = mup_path::get();
    let builder = format_ident!("__markup_template");
    let body = expand_body_with(&crate_path, &builder, None, input)?;

    Ok(quote! {{
        let mut #builder = #crate_path::template::TemplateBuilder::new();
        #body
        #builder.finish()
    }})
}

pub fn expand_body_with(
    crate_path: &TokenStream,
    builder: &Ident,
    children: Option<&Ident>,
    input: TokenStream,
) -> syn::Result<TokenStream> {
    let parser = MarkupParser {
        crate_path: crate_path.clone(),
        builder: builder.clone(),
        children: children.cloned(),
    };
    parser.parse(input)
}

#[derive(Clone)]
struct MarkupParser {
    crate_path: TokenStream,
    builder: Ident,
    children: Option<Ident>,
}

#[derive(Clone)]
enum Tag {
    Static(String),
    Dynamic(TokenStream),
}

enum Attr {
    Spread(TokenStream),
    Static {
        prefix: Option<String>,
        segments: Vec<String>,
        value: Option<TokenStream>,
    },
    Literal {
        name: Literal,
        value: Option<TokenStream>,
    },
    Dynamic {
        name: TokenStream,
        value: Option<TokenStream>,
    },
}

struct ExprLike {
    tokens: TokenStream,
    simple_path: bool,
}

impl MarkupParser {
    fn parse(&self, input: TokenStream) -> syn::Result<TokenStream> {
        let mut cursor = Cursor::new(input);
        self.parse_nodes(&mut cursor)
    }

    fn parse_nodes(&self, cursor: &mut Cursor) -> syn::Result<TokenStream> {
        let mut out = TokenStream::new();

        while !cursor.is_empty() {
            let node = self.parse_node(cursor)?;
            out.extend(node);
        }

        Ok(out)
    }

    fn parse_node(&self, cursor: &mut Cursor) -> syn::Result<TokenStream> {
        match cursor.peek() {
            Some(TokenTree::Literal(_)) => {
                let literal = cursor.next_literal()?;
                Ok(self.emit_render(quote!(#literal)))
            }
            Some(TokenTree::Punct(punct)) if punct.as_char() == '@' => self.parse_at(cursor),
            Some(TokenTree::Punct(punct)) if punct.as_char() == '.' => {
                cursor.next();
                self.parse_element_after_selector(
                    Tag::Static("div".to_owned()),
                    SelectorStart::Class,
                    cursor,
                )
            }
            Some(TokenTree::Punct(punct)) if punct.as_char() == '#' => {
                cursor.next();
                self.parse_element_after_selector(
                    Tag::Static("div".to_owned()),
                    SelectorStart::Id,
                    cursor,
                )
            }
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
                if self.paren_starts_dynamic_element(cursor) {
                    let group = cursor.consume_group(Delimiter::Parenthesis).unwrap();
                    self.parse_element(Tag::Dynamic(group.stream()), cursor)
                } else {
                    let group = cursor.consume_group(Delimiter::Parenthesis).unwrap();
                    Ok(self.emit_render(token_stream_from_group(&group)))
                }
            }
            Some(TokenTree::Ident(_)) => {
                let tag = self.parse_static_tag(cursor)?;
                self.parse_element(Tag::Static(tag), cursor)
            }
            Some(token) => Err(syn::Error::new(
                token.span(),
                format!("unexpected token in markup: `{token}`"),
            )),
            None => Err(syn::Error::new(
                Span::call_site(),
                "unexpected end of input",
            )),
        }
    }

    fn parse_at(&self, cursor: &mut Cursor) -> syn::Result<TokenStream> {
        cursor.consume_punct('@');

        if cursor.consume_ident("let").is_some() {
            let tokens = cursor.collect_until_semicolon()?;
            return Ok(quote! { let #tokens; });
        }

        if cursor.consume_ident("const").is_some() {
            let tokens = cursor.collect_until_semicolon()?;
            return Ok(quote! { const #tokens; });
        }

        if cursor.consume_ident("fn").is_some() {
            let (head, body) = cursor.collect_until_brace_group()?;
            let body = token_stream_from_group(&body);
            return Ok(quote! { fn #head #body });
        }

        if cursor.consume_ident("if").is_some() {
            return self.parse_if(cursor);
        }

        if cursor.consume_ident("for").is_some() {
            let (head, body) = cursor.collect_until_brace_group()?;
            let body = self.parse_group_body(body)?;
            return Ok(quote! {
                for #head {
                    #body
                }
            });
        }

        if cursor.consume_ident("while").is_some() {
            let (head, body) = cursor.collect_until_brace_group()?;
            let body = self.parse_group_body(body)?;
            return Ok(quote! {
                while #head {
                    #body
                }
            });
        }

        if cursor.consume_ident("match").is_some() {
            return self.parse_match(cursor);
        }

        if cursor.consume_ident("break").is_some() {
            let tokens = cursor.collect_until_semicolon()?;
            return Ok(quote! { break #tokens; });
        }

        if cursor.consume_ident("continue").is_some() {
            let tokens = cursor.collect_until_semicolon()?;
            return Ok(quote! { continue #tokens; });
        }

        if self.children.is_some() && cursor.peek_ident("children") {
            let mut lookahead = cursor.clone();
            let _ = lookahead.next();
            if !matches!(
                lookahead.peek(),
                Some(TokenTree::Punct(punct))
                    if matches!(punct.as_char(), '.' | ':' | '!' | '<')
            ) && !lookahead.peek_group(Delimiter::Parenthesis)
                && !lookahead.peek_group(Delimiter::Bracket)
            {
                let _ = cursor.next();
                return Ok(self.emit_children_slot());
            }
        }

        if self.children.is_some() && self.consume_markup_slot_call(cursor) {
            return Ok(self.emit_children_slot());
        }

        self.parse_rendered_value(cursor)
    }

    fn parse_if(&self, cursor: &mut Cursor) -> syn::Result<TokenStream> {
        let (condition, then_body) = cursor.collect_until_brace_group()?;
        let then_body = self.parse_group_body(then_body)?;
        let else_body = self.parse_else(cursor)?;

        Ok(quote! {
            if #condition {
                #then_body
            }
            #else_body
        })
    }

    fn parse_else(&self, cursor: &mut Cursor) -> syn::Result<TokenStream> {
        let mut lookahead = cursor.clone();
        if lookahead.consume_punct('@').is_none() || !lookahead.peek_ident("else") {
            return Ok(TokenStream::new());
        }

        cursor.consume_punct('@');
        cursor.consume_ident("else");

        if cursor.consume_ident("if").is_some() {
            let nested = self.parse_if(cursor)?;
            return Ok(quote! { else #nested });
        }

        let Some(group) = cursor.consume_group(Delimiter::Brace) else {
            return Err(cursor.unexpected("expected `{ ... }` after `@else`"));
        };
        let body = self.parse_group_body(group)?;
        Ok(quote! {
            else {
                #body
            }
        })
    }

    fn parse_match(&self, cursor: &mut Cursor) -> syn::Result<TokenStream> {
        let (scrutinee, arms_group) = cursor.collect_until_brace_group()?;
        let mut arms = Cursor::new(arms_group.stream());
        let mut rendered_arms = TokenStream::new();

        while !arms.is_empty() {
            let pattern = arms.collect_until_arrow()?;
            let Some(body_group) = arms.consume_group(Delimiter::Brace) else {
                return Err(arms.unexpected("expected `{ ... }` in match arm"));
            };
            let body = self.parse_group_body(body_group)?;
            arms.skip_optional_comma();
            rendered_arms.extend(quote! {
                #pattern => {
                    #body
                },
            });
        }

        Ok(quote! {
            match #scrutinee {
                #rendered_arms
            }
        })
    }

    fn parse_rendered_value(&self, cursor: &mut Cursor) -> syn::Result<TokenStream> {
        let expr = self.parse_expr_like(cursor)?;

        if expr.simple_path && cursor.peek_group(Delimiter::Brace) {
            let props = cursor.consume_group(Delimiter::Brace).unwrap();
            if group_looks_struct_literal(&props) {
                let props = token_stream_from_group(&props);
                let expr_tokens = expr.tokens;
                let value = quote!(#expr_tokens #props);
                let children = cursor.consume_group(Delimiter::Brace);
                return Ok(self.emit_render_with_optional_children(value, children));
            }

            return Ok(self.emit_render_with_optional_children(expr.tokens, Some(props)));
        }

        let children = cursor.consume_group(Delimiter::Brace);
        Ok(self.emit_render_with_optional_children(expr.tokens, children))
    }

    fn parse_group_body(&self, group: Group) -> syn::Result<TokenStream> {
        self.parse(group.stream())
    }

    fn emit_render(&self, expr: TokenStream) -> TokenStream {
        // Use method call so it auto-derefs for both owned `TemplateBuilder` and `&mut TemplateBuilder`
        // (the render_into_builder streaming path uses &mut, free-function push_render would need &mut &mut).
        let builder = &self.builder;

        quote! {
            #builder.push_render(&(#expr));
        }
    }

    fn emit_render_with_optional_children(
        &self,
        expr: TokenStream,
        children: Option<Group>,
    ) -> TokenStream {
        let crate_path = &self.crate_path;
        let builder = &self.builder;

        if let Some(children) = children {
            let children_markup = self.emit_children_markup(children.stream());
            quote! {{
                let __markup_children = #children_markup;
                let __markup_markup =
                    #crate_path::template::render(
                        &(#expr),
                        ::std::option::Option::Some(__markup_children),
                    );
                #builder.push_markup(&__markup_markup);
            }}
        } else {
            self.emit_render(expr)
        }
    }

    fn emit_children_markup(&self, body: TokenStream) -> TokenStream {
        let crate_path = &self.crate_path;
        let builder = format_ident!("__markup_children_template");
        let body = expand_body_with(&self.crate_path, &builder, self.children.as_ref(), body)
            .unwrap_or_else(syn::Error::into_compile_error);

        quote! {{
            let mut #builder = #crate_path::template::TemplateBuilder::new();
            #body
            #builder.finish()
        }}
    }

    fn emit_children_slot(&self) -> TokenStream {
        let Some(children) = self.children.as_ref() else {
            return TokenStream::new();
        };
        let builder = &self.builder;

        quote! {{
            if let ::std::option::Option::Some(__markup_children) = #children.as_ref() {
                #builder.push_markup(__markup_children);
            }
        }}
    }

    fn parse_static_tag(&self, cursor: &mut Cursor) -> syn::Result<String> {
        let first = cursor.next_ident()?;
        let mut segments = vec![first.to_string()];

        while cursor.consume_punct('-').is_some() {
            segments.push(cursor.next_ident()?.to_string());
        }

        Ok(segments.join("-"))
    }

    fn parse_element_after_selector(
        &self,
        tag: Tag,
        selector: SelectorStart,
        cursor: &mut Cursor,
    ) -> syn::Result<TokenStream> {
        let mut classes = Vec::new();
        let mut id = None;
        match selector {
            SelectorStart::Class => classes.push(self.parse_selector_value(cursor)?),
            SelectorStart::Id => id = Some(self.parse_selector_value(cursor)?),
        }
        self.parse_element_rest(tag, classes, id, cursor)
    }

    fn parse_element(&self, tag: Tag, cursor: &mut Cursor) -> syn::Result<TokenStream> {
        self.parse_element_rest(tag, Vec::new(), None, cursor)
    }

    fn parse_element_rest(
        &self,
        tag: Tag,
        mut classes: Vec<TokenStream>,
        mut id: Option<TokenStream>,
        cursor: &mut Cursor,
    ) -> syn::Result<TokenStream> {
        let mut attrs = Vec::new();

        loop {
            if cursor.peek_punct('.') && cursor.peek_punct_n(1, '.') {
                attrs.push(self.parse_attr(cursor)?);
                continue;
            }

            if cursor.consume_punct('.').is_some() {
                classes.push(self.parse_selector_value(cursor)?);
                continue;
            }

            if cursor.consume_punct('#').is_some() {
                id = Some(self.parse_selector_value(cursor)?);
                continue;
            }

            if cursor.consume_punct(';').is_some() {
                return Ok(self.emit_element(tag, classes, id, attrs, None, true));
            }

            if let Some(body) = cursor.consume_group(Delimiter::Brace) {
                let body = self.parse_group_body(body)?;
                return Ok(self.emit_element(tag, classes, id, attrs, Some(body), false));
            }

            if cursor.is_empty() {
                return Err(cursor.unexpected("expected element body `{ ... }` or `;`"));
            }

            attrs.push(self.parse_attr(cursor)?);
        }
    }

    fn parse_selector_value(&self, cursor: &mut Cursor) -> syn::Result<TokenStream> {
        if let Some(group) = cursor.consume_group(Delimiter::Parenthesis) {
            return Ok(token_stream_from_group(&group));
        }
        let value = self.parse_name_segments(cursor)?.join("-");
        let literal = LitStr::new(&value, Span::call_site());
        Ok(quote!(#literal))
    }

    fn parse_optional_attr_value(&self, cursor: &mut Cursor) -> syn::Result<Option<TokenStream>> {
        if cursor.consume_punct('=').is_some() {
            Ok(Some(self.parse_attr_value(cursor)?))
        } else {
            Ok(None)
        }
    }

    fn parse_attr(&self, cursor: &mut Cursor) -> syn::Result<Attr> {
        if cursor.consume_double_punct('.', '.') {
            return Ok(Attr::Spread(self.parse_spread_expr(cursor)?));
        }

        if let Some(group) = cursor.consume_group(Delimiter::Parenthesis) {
            let name = group.stream();
            let value = self.parse_optional_attr_value(cursor)?;
            return Ok(Attr::Dynamic { name, value });
        }

        if matches!(cursor.peek(), Some(TokenTree::Literal(_))) {
            let name = cursor.next_literal()?;
            let value = self.parse_optional_attr_value(cursor)?;
            return Ok(Attr::Literal { name, value });
        }

        if cursor.peek_punct(':') || cursor.peek_punct('@') {
            let prefix = cursor.next().unwrap().to_string();
            let segments = self.parse_name_segments(cursor)?;
            let value = self.parse_optional_attr_value(cursor)?;
            return Ok(Attr::Static {
                prefix: Some(prefix),
                segments,
                value,
            });
        }

        if matches!(cursor.peek(), Some(TokenTree::Ident(_))) {
            let segments = self.parse_name_segments(cursor)?;
            let value = self.parse_optional_attr_value(cursor)?;
            return Ok(Attr::Static {
                prefix: None,
                segments,
                value,
            });
        }

        Err(cursor.unexpected("unexpected token in attributes"))
    }

    fn parse_spread_expr(&self, cursor: &mut Cursor) -> syn::Result<TokenStream> {
        if let Some(group) = cursor.consume_group(Delimiter::Bracket) {
            let inner = group.stream();
            return Ok(quote!([#inner]));
        }

        if let Some(group) = cursor.consume_group(Delimiter::Parenthesis) {
            return Ok(token_stream_from_group(&group));
        }

        Ok(self.parse_expr_like(cursor)?.tokens)
    }

    fn parse_attr_value(&self, cursor: &mut Cursor) -> syn::Result<TokenStream> {
        if let Some(group) = cursor.consume_group(Delimiter::Parenthesis) {
            return Ok(token_stream_from_group(&group));
        }

        if matches!(cursor.peek(), Some(TokenTree::Literal(_))) {
            let literal = cursor.next_literal()?;
            return Ok(quote!(#literal));
        }

        Ok(self.parse_expr_like(cursor)?.tokens)
    }

    fn parse_name_segments(&self, cursor: &mut Cursor) -> syn::Result<Vec<String>> {
        let first = cursor.next_ident()?;
        let mut segments = vec![first.to_string()];

        while cursor.consume_punct('-').is_some() {
            segments.push(cursor.next_ident()?.to_string());
        }

        Ok(segments)
    }

    fn emit_element(
        &self,
        tag: Tag,
        classes: Vec<TokenStream>,
        id: Option<TokenStream>,
        attrs: Vec<Attr>,
        body: Option<TokenStream>,
        void: bool,
    ) -> TokenStream {
        let crate_path = &self.crate_path;
        let builder = &self.builder;

        let classes = self.emit_classes(classes);
        let id = id.map(|id| {
            quote! {
                #crate_path::template::push_static_attr(&mut #builder.current, "id", &(#id));
            }
        });
        let attrs = attrs.into_iter().map(|attr| self.emit_attr(attr));

        match tag {
            Tag::Static(tag) => {
                // ponytail: macro-parsed tag, skip runtime assert_valid_tag_name
                let tag = LitStr::new(&tag, Span::call_site());
                if void {
                    quote! {{
                        #crate_path::template::push_start_tag_static(&mut #builder.current, #tag);
                        #classes
                        #id
                        #(#attrs)*
                        #crate_path::template::finish_void_tag_static(&mut #builder.current, #tag);
                    }}
                } else {
                    let body = body.unwrap_or_default();
                    quote! {{
                        #crate_path::template::push_start_tag_static(&mut #builder.current, #tag);
                        #classes
                        #id
                        #(#attrs)*
                        #crate_path::template::finish_start_tag(&mut #builder.current);
                        #body
                        #crate_path::template::push_end_tag_static(&mut #builder.current, #tag);
                    }}
                }
            }
            Tag::Dynamic(expr) => {
                if void {
                    quote! {{
                        let __markup_tag = &(#expr);
                        #crate_path::template::push_start_tag(&mut #builder.current, __markup_tag);
                        #classes
                        #id
                        #(#attrs)*
                        #crate_path::template::finish_void_tag(&mut #builder.current, __markup_tag);
                    }}
                } else {
                    let body = body.unwrap_or_default();
                    quote! {{
                        let __markup_tag = &(#expr);
                        #crate_path::template::push_start_tag(&mut #builder.current, __markup_tag);
                        #classes
                        #id
                        #(#attrs)*
                        #crate_path::template::finish_start_tag(&mut #builder.current);
                        #body
                        #crate_path::template::push_end_tag(&mut #builder.current, __markup_tag);
                    }}
                }
            }
        }
    }

    fn emit_classes(&self, classes: Vec<TokenStream>) -> TokenStream {
        if classes.is_empty() {
            return TokenStream::new();
        }

        let crate_path = &self.crate_path;
        let builder = &self.builder;

        // ponytail: all static → fold at compile time, single push_str, zero allocation
        if let Some(folded) = fold_static_class_literals(&classes) {
            let lit = LitStr::new(&folded, Span::call_site());
            return quote! {
                #crate_path::template::push_class_attr(&mut #builder.current, #lit);
            };
        }

        // ponytail: has a static prefix followed by dynamic classes → write directly into
        // builder.current: eliminates the intermediate String::new() + malloc per element
        let static_count = classes.iter().take_while(|c| is_str_literal(c)).count();
        let all_rest_dynamic = classes[static_count..].iter().all(|c| !is_str_literal(c));

        if all_rest_dynamic {
            let (static_classes, dynamic_classes) = classes.split_at(static_count);
            let static_prefix: String = static_classes
                .iter()
                .filter_map(extract_str_literal)
                .collect::<Vec<_>>()
                .join(" ");

            if static_prefix.is_empty() {
                // All dynamic: track whether any class was written
                return quote! {{
                    let __class_attr_start = #builder.current.len();
                    #builder.current.push_str(" class=\"");
                    let mut __class_sep_needed = false;
                    #({
                        let __sep = #builder.current.len();
                        if __class_sep_needed { #builder.current.push(' '); }
                        if #crate_path::template::push_class_direct(
                            &mut #builder.current, &(#dynamic_classes)
                        ) {
                            __class_sep_needed = true;
                        } else if __class_sep_needed {
                            #builder.current.truncate(__sep);
                        }
                    })*
                    if __class_sep_needed {
                        #builder.current.push('"');
                    } else {
                        #builder.current.truncate(__class_attr_start);
                    }
                }};
            } else {
                // Has static prefix (always non-empty): write it inline, append dynamic
                let class_attr_prefix = format!(" class=\"{}", static_prefix);
                let prefix_lit = LitStr::new(&class_attr_prefix, Span::call_site());
                return quote! {{
                    #builder.current.push_str(#prefix_lit);
                    #({
                        let __sep = #builder.current.len();
                        #builder.current.push(' ');
                        if !#crate_path::template::push_class_direct(
                            &mut #builder.current, &(#dynamic_classes)
                        ) {
                            #builder.current.truncate(__sep);
                        }
                    })*
                    #builder.current.push('"');
                }};
            }
        }

        // Fallback: interleaved static/dynamic classes (rare) — use intermediate String
        quote! {{
            let mut __markup_classes = ::std::string::String::new();
            #(
                #crate_path::template::push_class_value(&mut __markup_classes, &(#classes));
            )*
            #crate_path::template::push_class_attr(&mut #builder.current, &__markup_classes);
        }}
    }

    fn emit_attr(&self, attr: Attr) -> TokenStream {
        let crate_path = &self.crate_path;
        let builder = &self.builder;

        match attr {
            Attr::Spread(expr) => quote! {
                #crate_path::template::push_attrs(&mut #builder.current, &(#expr));
            },
            Attr::Static {
                prefix,
                segments,
                value,
            } => {
                // ponytail: fold attr name at compile time, use static variants to skip runtime validation
                let name = match &prefix {
                    Some(p) => format!("{}{}", p, segments.join("-")),
                    None => segments.join("-"),
                };
                let name_lit = LitStr::new(&name, Span::call_site());
                match value {
                    Some(value) => quote! {
                        #crate_path::template::push_static_attr(
                            &mut #builder.current,
                            #name_lit,
                            &(#value),
                        );
                    },
                    None => quote! {
                        #crate_path::template::push_static_bool_attr(
                            &mut #builder.current,
                            #name_lit,
                        );
                    },
                }
            }
            Attr::Literal { name, value } => match value {
                Some(value) => quote! {
                    #crate_path::template::push_attr(&mut #builder.current, &#name, &(#value));
                },
                None => quote! {
                    #crate_path::template::push_bool_attr(&mut #builder.current, &#name);
                },
            },
            Attr::Dynamic { name, value } => match value {
                Some(value) => quote! {
                    #crate_path::template::push_attr(&mut #builder.current, &(#name), &(#value));
                },
                None => quote! {
                    #crate_path::template::push_bool_attr(&mut #builder.current, &(#name));
                },
            },
        }
    }

    fn parse_expr_like(&self, cursor: &mut Cursor) -> syn::Result<ExprLike> {
        let mut out = TokenStream::new();
        let mut simple_path = false;
        let mut can_consume_call = false;

        match cursor.next() {
            Some(TokenTree::Ident(ident)) => {
                simple_path = true;
                can_consume_call = true;
                ident.to_tokens(&mut out);
            }
            Some(TokenTree::Literal(literal)) => {
                literal.to_tokens(&mut out);
            }
            Some(TokenTree::Group(group)) => {
                token_stream_from_group(&group).to_tokens(&mut out);
            }
            Some(token) => {
                return Err(syn::Error::new(
                    token.span(),
                    "expected Rust expression in markup",
                ));
            }
            None => {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "expected Rust expression in markup",
                ));
            }
        }

        loop {
            if cursor.consume_double_punct(':', ':') {
                quote!(::).to_tokens(&mut out);
                if cursor.peek_punct('<') {
                    self.collect_angle_args(cursor)?.to_tokens(&mut out);
                } else {
                    cursor.next_ident()?.to_tokens(&mut out);
                }
                can_consume_call = true;
                continue;
            }

            if can_consume_call {
                if cursor.peek_group(Delimiter::Parenthesis) && cursor.peek_punct_n(1, '=') {
                    break;
                }
                if let Some(group) = cursor.consume_group(Delimiter::Parenthesis) {
                    simple_path = false;
                    can_consume_call = false;
                    token_stream_from_group(&group).to_tokens(&mut out);
                    continue;
                }
            }

            if let Some(group) = cursor.consume_group(Delimiter::Bracket) {
                simple_path = false;
                can_consume_call = false;
                token_stream_from_group(&group).to_tokens(&mut out);
                continue;
            }

            if cursor.consume_punct('.').is_some() {
                simple_path = false;
                can_consume_call = true;
                quote!(.).to_tokens(&mut out);
                match cursor.next() {
                    Some(TokenTree::Ident(ident)) => ident.to_tokens(&mut out),
                    Some(TokenTree::Literal(literal)) => literal.to_tokens(&mut out),
                    Some(token) => {
                        return Err(syn::Error::new(
                            token.span(),
                            "expected field or method after `.`",
                        ));
                    }
                    None => {
                        return Err(syn::Error::new(
                            Span::call_site(),
                            "expected field or method after `.`",
                        ));
                    }
                }
                continue;
            }

            if cursor.consume_punct('!').is_some() {
                simple_path = false;
                can_consume_call = false;
                quote!(!).to_tokens(&mut out);
                match cursor.next() {
                    Some(TokenTree::Group(group)) => {
                        token_stream_from_group(&group).to_tokens(&mut out)
                    }
                    Some(token) => {
                        return Err(syn::Error::new(
                            token.span(),
                            "expected macro invocation body after `!`",
                        ));
                    }
                    None => {
                        return Err(syn::Error::new(
                            Span::call_site(),
                            "expected macro invocation body after `!`",
                        ));
                    }
                }
                continue;
            }

            break;
        }

        Ok(ExprLike {
            tokens: out,
            simple_path,
        })
    }

    fn collect_angle_args(&self, cursor: &mut Cursor) -> syn::Result<TokenStream> {
        let mut depth = 0usize;
        let mut out = TokenStream::new();

        while let Some(token) = cursor.next() {
            match &token {
                TokenTree::Punct(punct) if punct.as_char() == '<' => {
                    depth += 1;
                    token.to_tokens(&mut out);
                }
                TokenTree::Punct(punct) if punct.as_char() == '>' => {
                    if depth == 0 {
                        return Err(syn::Error::new(token.span(), "unexpected `>`"));
                    }
                    depth -= 1;
                    token.to_tokens(&mut out);
                    if depth == 0 {
                        return Ok(out);
                    }
                }
                token => token.to_tokens(&mut out),
            }
        }

        Err(syn::Error::new(
            Span::call_site(),
            "unterminated generic argument list",
        ))
    }

    fn paren_starts_dynamic_element(&self, cursor: &Cursor) -> bool {
        matches!(
            cursor.peek_n(1),
            Some(TokenTree::Group(group))
                if matches!(group.delimiter(), Delimiter::Brace | Delimiter::Parenthesis | Delimiter::Bracket)
        ) || matches!(
            cursor.peek_n(1),
            Some(TokenTree::Ident(_)) | Some(TokenTree::Literal(_)) | Some(TokenTree::Punct(_))
        )
    }

    fn consume_markup_slot_call(&self, cursor: &mut Cursor) -> bool {
        let mut lookahead = cursor.clone();
        if lookahead.consume_ident("Markup").is_none() {
            return false;
        }
        if !lookahead.consume_double_punct(':', ':') {
            return false;
        }
        if lookahead.consume_ident("slot").is_none() {
            return false;
        }
        let Some(group) = lookahead.consume_group(Delimiter::Parenthesis) else {
            return false;
        };
        if !group.stream().is_empty() {
            return false;
        }

        *cursor = lookahead;
        true
    }
}

fn fold_static_class_literals(classes: &[TokenStream]) -> Option<String> {
    let mut result = String::new();
    for class in classes {
        let lit = extract_str_literal(class)?;
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(&lit);
    }
    Some(result)
}

fn is_str_literal(ts: &TokenStream) -> bool {
    extract_str_literal(ts).is_some()
}

fn extract_str_literal(ts: &TokenStream) -> Option<String> {
    let mut iter = ts.clone().into_iter();
    let Some(TokenTree::Literal(_)) = iter.next() else {
        return None;
    };
    if iter.next().is_some() {
        return None;
    }
    let lit: LitStr = syn::parse2(ts.clone()).ok()?;
    Some(lit.value())
}

enum SelectorStart {
    Class,
    Id,
}

fn group_looks_struct_literal(group: &Group) -> bool {
    let mut cursor = Cursor::new(group.stream());

    if cursor.consume_double_punct('.', '.') {
        return true;
    }

    if !matches!(cursor.next(), Some(TokenTree::Ident(_))) {
        return false;
    }

    cursor.is_empty()
        || cursor.peek_punct(':')
        || cursor.peek_punct(',')
        || (cursor.peek_punct('.') && cursor.peek_punct_n(1, '.'))
}
