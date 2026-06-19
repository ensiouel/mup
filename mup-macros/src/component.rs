use crate::cursor::Cursor;
use crate::markup;
use crate::mup_path;
use proc_macro2::{Delimiter, Ident, TokenStream, TokenTree};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Fields, ItemEnum, ItemImpl, ItemStruct, Result, Token, Visibility, braced};

pub fn expand(input: TokenStream) -> TokenStream {
    syn::parse2::<ComponentInput>(input)
        .and_then(ComponentInput::expand)
        .unwrap_or_else(|error| error.to_compile_error())
}

struct ComponentInput {
    items: Vec<ComponentItem>,
}

enum ComponentItem {
    Struct {
        item: ItemStruct,
        body: TokenStream,
    },
    Enum {
        item: ItemEnum,
        arms: EnumRenderArms,
    },
    Impl(ItemImpl),
}

enum ComponentItemKind {
    Struct,
    Enum,
    Impl,
}

struct RenderContext {
    crate_path: TokenStream,
    builder: Ident,
    children: Ident,
}

struct EnumRenderArms {
    arms: Vec<EnumRenderArm>,
}

struct EnumRenderArm {
    pattern: TokenStream,
    body: TokenStream,
}

impl Parse for ComponentInput {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut items = Vec::new();

        while !input.is_empty() {
            items.push(input.parse()?);
        }

        Ok(Self { items })
    }
}

impl Parse for ComponentItem {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        match peek_component_item_kind(input)? {
            ComponentItemKind::Struct => {
                let (item, body) = parse_item_with_markup_body(input)?;
                Ok(Self::Struct { item, body })
            }
            ComponentItemKind::Enum => {
                let (item, body) = parse_item_with_markup_body(input)?;
                Ok(Self::Enum {
                    item,
                    arms: EnumRenderArms::parse(body)?,
                })
            }
            ComponentItemKind::Impl => Ok(Self::Impl(input.parse()?)),
        }
    }
}

impl ComponentInput {
    fn expand(self) -> Result<TokenStream> {
        let context = RenderContext::new(mup_path::get());
        let mut out = TokenStream::new();

        for item in self.items {
            out.extend(item.expand(&context)?);
        }

        Ok(out)
    }
}

impl ComponentItem {
    fn expand(self, context: &RenderContext) -> Result<TokenStream> {
        match self {
            Self::Struct { item, body } => expand_struct(context, item, body),
            Self::Enum { item, arms } => expand_enum(context, item, arms),
            Self::Impl(item) => Ok(quote!(#item)),
        }
    }
}

impl RenderContext {
    fn new(crate_path: TokenStream) -> Self {
        Self {
            crate_path,
            builder: format_ident!("__markup_template"),
            children: format_ident!("__markup_children"),
        }
    }

    fn expand_markup_body(&self, body: TokenStream) -> Result<TokenStream> {
        markup::expand_body_with(&self.crate_path, &self.builder, Some(&self.children), body)
    }

    fn render_body_expr(&self, body: TokenStream) -> TokenStream {
        let crate_path = &self.crate_path;
        let builder = &self.builder;

        quote! {
            let mut #builder = #crate_path::template::TemplateBuilder::new();
            #body
            #builder.finish()
        }
    }
}

impl EnumRenderArms {
    fn parse(input: TokenStream) -> Result<Self> {
        let mut cursor = Cursor::new(input);
        let mut arms = Vec::new();

        while !cursor.is_empty() {
            let pattern = qualify_enum_variant_pattern(cursor.collect_until_arrow()?);
            let Some(body) = cursor.consume_group(Delimiter::Brace) else {
                return Err(cursor.unexpected("expected `{ ... }` after enum render arm"));
            };
            cursor.skip_optional_comma();

            arms.push(EnumRenderArm {
                pattern,
                body: body.stream(),
            });
        }

        Ok(Self { arms })
    }

    fn expand(self, context: &RenderContext) -> Result<TokenStream> {
        let mut out = TokenStream::new();

        for arm in self.arms {
            out.extend(arm.expand(context)?);
        }

        Ok(out)
    }
}

impl EnumRenderArm {
    fn expand(self, context: &RenderContext) -> Result<TokenStream> {
        let pattern = self.pattern;
        let body = context.expand_markup_body(self.body)?;
        let body = context.render_body_expr(body);

        Ok(quote! {
            #pattern => {
                #body
            },
        })
    }
}

fn peek_component_item_kind(input: ParseStream<'_>) -> Result<ComponentItemKind> {
    let ahead = input.fork();
    let _attrs = Attribute::parse_outer(&ahead)?;
    let _vis: Visibility = ahead.parse()?;

    if ahead.peek(Token![struct]) {
        return Ok(ComponentItemKind::Struct);
    }

    if ahead.peek(Token![enum]) {
        return Ok(ComponentItemKind::Enum);
    }

    if ahead.peek(Token![impl]) {
        return Ok(ComponentItemKind::Impl);
    }

    Err(input.error("expected `struct`, `enum`, or `impl` in component!"))
}

fn parse_item_with_markup_body<T>(input: ParseStream<'_>) -> Result<(T, TokenStream)>
where
    T: Parse,
{
    let item = input.parse()?;
    let content;
    braced!(content in input);
    let body = content.parse()?;

    Ok((item, body))
}

fn expand_struct(
    context: &RenderContext,
    item: ItemStruct,
    body: TokenStream,
) -> Result<TokenStream> {
    let fields = named_struct_fields(&item)?;
    let field_idents: Vec<&Ident> = fields
        .named
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect();

    let crate_path = &context.crate_path;
    let builder = &context.builder;
    let children = &context.children;
    let name = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    let render_body = context.expand_markup_body(body)?;

    Ok(quote! {
        #item

        impl #impl_generics #crate_path::Render for #name #ty_generics #where_clause {
            fn render(
                &self,
                #children: ::std::option::Option<#crate_path::Markup>,
            ) -> #crate_path::Markup {
                #[allow(unused_variables)]
                let Self {
                    #(#field_idents,)*
                    ..
                } = self;
                let mut #builder = #crate_path::template::TemplateBuilder::new();
                #render_body
                #builder.finish()
            }

            // ponytail: write directly into caller's builder — eliminates per-component
            // TemplateBuilder::new() + finish() + Markup allocation on the hot path
            fn render_into_builder(
                &self,
                #builder: &mut #crate_path::template::TemplateBuilder,
            ) {
                let #children: ::std::option::Option<#crate_path::Markup> =
                    ::std::option::Option::None;
                #[allow(unused_variables)]
                let Self {
                    #(#field_idents,)*
                    ..
                } = self;
                #render_body
            }
        }
    })
}

fn expand_enum(
    context: &RenderContext,
    item: ItemEnum,
    arms: EnumRenderArms,
) -> Result<TokenStream> {
    let crate_path = &context.crate_path;
    let children = &context.children;
    let name = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    let arms = arms.expand(context)?;

    Ok(quote! {
        #item

        impl #impl_generics #crate_path::Render for #name #ty_generics #where_clause {
            fn render(
                &self,
                #children: ::std::option::Option<#crate_path::Markup>,
            ) -> #crate_path::Markup {
                match self {
                    #arms
                }
            }
        }
    })
}

fn named_struct_fields(item: &ItemStruct) -> Result<&syn::FieldsNamed> {
    match &item.fields {
        Fields::Named(fields) => Ok(fields),
        _ => Err(syn::Error::new_spanned(
            &item.ident,
            "component! currently expects structs with named fields",
        )),
    }
}

fn qualify_enum_variant_pattern(pattern: TokenStream) -> TokenStream {
    let mut tokens = pattern.clone().into_iter();
    let Some(TokenTree::Ident(first)) = tokens.next() else {
        return pattern;
    };

    if first == "Self" || first == "_" {
        return pattern;
    }

    let rest: TokenStream = tokens.collect();
    quote!(Self::#first #rest)
}
