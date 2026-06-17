use std::collections::HashMap;

use mup::{Markup, markup};

struct NavItem {
    label: &'static str,
    href: &'static str,
}

fn main() {
    let app_name = "mup";
    let user = Some("Ada");
    let unsafe_text = "<script>alert('xss')</script>";
    let nav = [
        NavItem {
            label: "Docs",
            href: "/docs",
        },
        NavItem {
            label: "Examples",
            href: "/examples",
        },
    ];

    let mut attrs = HashMap::new();
    attrs.insert("data-app", app_name);
    attrs.insert("hx-boost", "true");

    let html = markup! {
        @Markup::doctype()
        html lang="en" {
            head {
                meta charset="utf-8";
                title { @app_name " basic example" }
            }
            body .app ..attrs {
                header {
                    h1 { @app_name }

                    @if let Some(name) = user {
                        p { "Signed in as " @name }
                    } @else {
                        a href="/login" { "Sign in" }
                    }
                }

                nav aria-label="Primary" {
                    @for item in &nav {
                        a href=item.href { @item.label }
                    }
                }

                main {
                    section.card data-state="escaped" {
                        h2 { "Escaping" }
                        p title=app_name {
                            "User input is escaped by default: "
                            @unsafe_text
                        }
                    }
                }
            }
        }
    };

    println!("{}", html.as_str());
}
