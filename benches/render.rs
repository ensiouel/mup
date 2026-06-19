use std::fmt::Write as _;

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use horrorshow::prelude::Template as _;
use mup::{Markup, component, markup};

#[derive(Clone, Copy)]
pub struct NavItem {
    label: &'static str,
    href: &'static str,
}

#[derive(Clone, Copy)]
pub struct Product {
    id: u32,
    name: &'static str,
    slug: &'static str,
    description: &'static str,
    price: &'static str,
    in_stock: bool,
    featured: bool,
}

struct PageInput<'a> {
    title: &'a str,
    user: &'a str,
    nav: &'a [NavItem],
    products: &'a [Product],
}

static NAV_ITEMS: [NavItem; 4] = [
    NavItem {
        label: "Overview",
        href: "/overview",
    },
    NavItem {
        label: "Products",
        href: "/products",
    },
    NavItem {
        label: "Pricing",
        href: "/pricing",
    },
    NavItem {
        label: "Docs",
        href: "/docs",
    },
];

static PRODUCTS: [Product; 12] = [
    Product {
        id: 1001,
        name: "Rust Hoodie",
        slug: "rust-hoodie",
        description: "Warm, safe & fearless.",
        price: "$59.00",
        in_stock: true,
        featured: true,
    },
    Product {
        id: 1002,
        name: "Macro Mug",
        slug: "macro-mug",
        description: "Expands coffee before compile time.",
        price: "$18.50",
        in_stock: true,
        featured: false,
    },
    Product {
        id: 1003,
        name: "Borrow Checker Poster",
        slug: "borrow-checker-poster",
        description: "A reminder that lifetimes are design.",
        price: "$12.00",
        in_stock: false,
        featured: false,
    },
    Product {
        id: 1004,
        name: "Ferris Sticker Pack",
        slug: "ferris-stickers",
        description: "Twelve tiny crabs for laptops.",
        price: "$7.95",
        in_stock: true,
        featured: true,
    },
    Product {
        id: 1005,
        name: "Zero-Cost Notebook",
        slug: "zero-cost-notebook",
        description: "Abstractions on every page.",
        price: "$14.25",
        in_stock: true,
        featured: false,
    },
    Product {
        id: 1006,
        name: "Unsafe-Free Tote",
        slug: "unsafe-free-tote",
        description: "Carries groceries, not UB.",
        price: "$22.00",
        in_stock: false,
        featured: true,
    },
    Product {
        id: 1007,
        name: "Trait Object Pin",
        slug: "trait-object-pin",
        description: "Dynamic dispatch for denim jackets.",
        price: "$5.00",
        in_stock: true,
        featured: false,
    },
    Product {
        id: 1008,
        name: "Cargo Blanket",
        slug: "cargo-blanket",
        description: "Keeps builds and people warm.",
        price: "$44.00",
        in_stock: true,
        featured: false,
    },
    Product {
        id: 1009,
        name: "Eco <Lamp>",
        slug: "eco-lamp",
        description: "Escapes names with < and > correctly.",
        price: "$35.75",
        in_stock: false,
        featured: false,
    },
    Product {
        id: 1010,
        name: "Iterator Socks",
        slug: "iterator-socks",
        description: "One pair, many adapters.",
        price: "$9.99",
        in_stock: true,
        featured: true,
    },
    Product {
        id: 1011,
        name: "Enum Enamel Set",
        slug: "enum-enamel-set",
        description: "Variants for every mood.",
        price: "$16.40",
        in_stock: true,
        featured: false,
    },
    Product {
        id: 1012,
        name: "HTMX Fragment Cards",
        slug: "htmx-fragment-cards",
        description: "Partial updates without ceremony.",
        price: "$11.10",
        in_stock: true,
        featured: false,
    },
];

static INPUT: PageInput<'static> = PageInput {
    title: "Server-rendered catalog",
    user: "Ada <admin>",
    nav: &NAV_ITEMS,
    products: &PRODUCTS,
};

component! {
    struct MupComponentProductCard<'a> {
        product: &'a Product,
    } {
        article.card.(if product.in_stock { "in-stock" } else { "sold-out" }) data-id=product.id {
            h2 { @product.name }
            @if product.featured {
                span.badge { "Featured" }
            }
            p { @product.description }
            footer {
                a href=product.slug { "Details" }
                span.price { @product.price }
            }
        }
    }

    struct MupComponentPage<'a> {
        title: &'a str,
        user: &'a str,
        nav: &'a [NavItem],
        products: &'a [Product],
    } {
        @Markup::doctype()
        html lang="en" {
            head {
                meta charset="utf-8";
                title { @title }
            }
            body.app data-user=user {
                header.site-header {
                    a.logo href="/" { "mup" }
                    nav aria-label="Primary" {
                        @for item in nav.iter() {
                            a href=item.href { @item.label }
                        }
                    }
                }
                main #products {
                    h1 { @title }
                    p.lead { "Rendered for " @user }
                    section.grid {
                        @for product in products.iter() {
                            @MupComponentProductCard { product }
                        }
                    }
                }
            }
        }
    }
}

markup::define! {
    MarkupRsPage<'a>(
        title: &'a str,
        user: &'a str,
        nav: &'a [NavItem],
        products: &'a [Product],
    ) {
        @markup::doctype()
        html[lang = "en"] {
            head {
                meta[charset = "utf-8"];
                title { @title }
            }
            body.app["data-user" = user] {
                header[class = "site-header"] {
                    a.logo[href = "/"] { "mup" }
                    nav["aria-label" = "Primary"] {
                        @for item in *nav {
                            a[href = item.href] { @item.label }
                        }
                    }
                }
                main #products {
                    h1 { @title }
                    p.lead { "Rendered for " @user }
                    section.grid {
                        @for product in *products {
                            article.card.{if product.in_stock { "in-stock" } else { "sold-out" }}["data-id" = product.id] {
                                h2 { @product.name }
                                @if product.featured {
                                    span.badge { "Featured" }
                                }
                                p { @product.description }
                                footer {
                                    a[href = product.slug] { "Details" }
                                    span.price { @product.price }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_mup(input: &PageInput<'_>) -> String {
    markup! {
        @Markup::doctype()
        html lang="en" {
            head {
                meta charset="utf-8";
                title { @input.title }
            }
            body.app data-user=input.user {
                header.site-header {
                    a.logo href="/" { "mup" }
                    nav aria-label="Primary" {
                        @for item in input.nav {
                            a href=item.href { @item.label }
                        }
                    }
                }
                main #products {
                    h1 { @input.title }
                    p.lead { "Rendered for " @input.user }
                    section.grid {
                        @for product in input.products {
                            article.card.(if product.in_stock { "in-stock" } else { "sold-out" }) data-id=product.id {
                                h2 { @product.name }
                                @if product.featured {
                                    span.badge { "Featured" }
                                }
                                p { @product.description }
                                footer {
                                    a href=product.slug { "Details" }
                                    span.price { @product.price }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    .into_string()
}

fn render_mup_component(input: &PageInput<'_>) -> String {
    markup! {
        @MupComponentPage {
            title: input.title,
            user: input.user,
            nav: input.nav,
            products: input.products,
        }
    }
    .into_string()
}

fn render_maud(input: &PageInput<'_>) -> String {
    maud::html! {
        (maud::DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                title { (input.title) }
            }
            body.app data-user=(input.user) {
                header.site-header {
                    a.logo href="/" { "mup" }
                    nav aria-label="Primary" {
                        @for item in input.nav {
                            a href=(item.href) { (item.label) }
                        }
                    }
                }
                main #products {
                    h1 { (input.title) }
                    p.lead { "Rendered for " (input.user) }
                    section.grid {
                        @for product in input.products {
                            article.card.(if product.in_stock { "in-stock" } else { "sold-out" }) data-id=(product.id) {
                                h2 { (product.name) }
                                @if product.featured {
                                    span.badge { "Featured" }
                                }
                                p { (product.description) }
                                footer {
                                    a href=(product.slug) { "Details" }
                                    span.price { (product.price) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    .into_string()
}

fn render_markup_rs(input: &PageInput<'_>) -> String {
    MarkupRsPage {
        title: input.title,
        user: input.user,
        nav: input.nav,
        products: input.products,
    }
    .to_string()
}

fn render_horrorshow(input: &PageInput<'_>) -> String {
    horrorshow::html! {
        : horrorshow::helper::doctype::HTML;
        html(lang = "en") {
            head {
                meta(charset = "utf-8");
                title { : input.title; }
            }
            body(class = "app", data-user = input.user) {
                header(class = "site-header") {
                    a(class = "logo", href = "/") { : "mup"; }
                    nav(aria-label = "Primary") {
                        @ for item in input.nav {
                            a(href = item.href) { : item.label; }
                        }
                    }
                }
                main(id = "products") {
                    h1 { : input.title; }
                    p(class = "lead") {
                        : "Rendered for ";
                        : input.user;
                    }
                    section(class = "grid") {
                        @ for product in input.products {
                            article(class = if product.in_stock { "card in-stock" } else { "card sold-out" }, data-id = product.id) {
                                h2 { : product.name; }
                                @ if product.featured {
                                    span(class = "badge") { : "Featured"; }
                                }
                                p { : product.description; }
                                footer {
                                    a(href = product.slug) { : "Details"; }
                                    span(class = "price") { : product.price; }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    .into_string()
    .expect("horrorshow render should not fail")
}

fn render_manual(input: &PageInput<'_>) -> String {
    let mut out = String::with_capacity(5_500);

    out.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><title>");
    push_text(&mut out, input.title);
    out.push_str("</title></head><body class=\"app\"");
    push_attr(&mut out, "data-user", input.user);
    out.push_str("><header class=\"site-header\"><a class=\"logo\" href=\"/\">mup</a>");
    out.push_str("<nav aria-label=\"Primary\">");

    for item in input.nav {
        out.push_str("<a");
        push_attr(&mut out, "href", item.href);
        out.push('>');
        push_text(&mut out, item.label);
        out.push_str("</a>");
    }

    out.push_str("</nav></header><main id=\"products\"><h1>");
    push_text(&mut out, input.title);
    out.push_str("</h1><p class=\"lead\">Rendered for ");
    push_text(&mut out, input.user);
    out.push_str("</p><section class=\"grid\">");

    for product in input.products {
        out.push_str("<article class=\"card ");
        out.push_str(if product.in_stock {
            "in-stock"
        } else {
            "sold-out"
        });
        out.push('"');
        write!(out, " data-id=\"{}\">", product.id).expect("writing to String should not fail");
        out.push_str("<h2>");
        push_text(&mut out, product.name);
        out.push_str("</h2>");

        if product.featured {
            out.push_str("<span class=\"badge\">Featured</span>");
        }

        out.push_str("<p>");
        push_text(&mut out, product.description);
        out.push_str("</p><footer><a");
        push_attr(&mut out, "href", product.slug);
        out.push_str(">Details</a><span class=\"price\">");
        push_text(&mut out, product.price);
        out.push_str("</span></footer></article>");
    }

    out.push_str("</section></main></body></html>");
    out
}

fn push_text(out: &mut String, value: &str) {
    for ch in value.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(ch),
        }
    }
}

fn push_attr(out: &mut String, name: &str, value: &str) {
    out.push(' ');
    out.push_str(name);
    out.push_str("=\"");
    push_attr_value(out, value);
    out.push('"');
}

fn push_attr_value(out: &mut String, value: &str) {
    for ch in value.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(ch),
        }
    }
}

fn assert_same_output() {
    let expected = render_mup(&INPUT);

    assert_eq!(render_mup_component(&INPUT), expected, "mup component!");
    assert_eq!(render_maud(&INPUT), expected, "maud");
    assert_eq!(render_markup_rs(&INPUT), expected, "markup.rs");
    assert_eq!(render_horrorshow(&INPUT), expected, "horrorshow");
    assert_eq!(render_manual(&INPUT), expected, "manual String");
}

fn bench_render(c: &mut Criterion) {
    assert_same_output();

    let mut group = c.benchmark_group("render/catalog_page");
    group.bench_function("mup/markup", |b| {
        b.iter(|| black_box(render_mup(black_box(&INPUT))));
    });
    group.bench_function("mup/component", |b| {
        b.iter(|| black_box(render_mup_component(black_box(&INPUT))));
    });
    group.bench_function("maud", |b| {
        b.iter(|| black_box(render_maud(black_box(&INPUT))));
    });
    group.bench_function("markup.rs", |b| {
        b.iter(|| black_box(render_markup_rs(black_box(&INPUT))));
    });
    group.bench_function("horrorshow", |b| {
        b.iter(|| black_box(render_horrorshow(black_box(&INPUT))));
    });
    group.bench_function("manual_string", |b| {
        b.iter(|| black_box(render_manual(black_box(&INPUT))));
    });
    group.finish();
}

criterion_group!(benches, bench_render);
criterion_main!(benches);
