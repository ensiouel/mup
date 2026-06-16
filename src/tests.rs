use super::*;
use std::collections::HashMap;

#[test]
fn renders_tags_text_and_selectors() {
    let actual = markup! {
        div.foo.bar #main {
            "hello"
            span { "world" }
        }
    };

    assert_eq!(
        actual.as_str(),
        r#"<div class="foo bar" id="main">hello<span>world</span></div>"#
    );
}

#[test]
fn renders_div_shorthand_selectors() {
    let actual = markup! {
        .panel {
            "class shorthand"
        }

        #main {
            "id shorthand"
        }

        .panel #main data-state="open" {
            "combined shorthand"
        }
    };

    assert_eq!(
        actual.as_str(),
        r#"<div class="panel">class shorthand</div><div id="main">id shorthand</div><div class="panel" id="main" data-state="open">combined shorthand</div>"#
    );
}

#[test]
fn renders_hyphenated_custom_tags() {
    let actual = markup! {
        my-block.foo {
            my-block-item { "content" }
        }
    };

    assert_eq!(
        actual.as_str(),
        r#"<my-block class="foo"><my-block-item>content</my-block-item></my-block>"#
    );
}

#[test]
fn renders_dynamic_tags() {
    let tag = format!("my-{}", "block");
    let child_tag = "my-child";

    let actual = markup! {
        (tag).dynamic data-id="root" {
            (child_tag) { "content" }
        }
    };

    assert_eq!(
        actual.as_str(),
        r#"<my-block class="dynamic" data-id="root"><my-child>content</my-child></my-block>"#
    );
}

#[test]
fn renders_semicolon_terminated_void_elements() {
    let actual = markup! {
        div {
            br;
            input type="text" disabled;
            img.logo src="/logo.svg" alt="";
        }
    };

    assert_eq!(
        actual.as_str(),
        r#"<div><br><input type="text" disabled><img class="logo" src="/logo.svg" alt=""></div>"#
    );
}

#[test]
fn renders_dynamic_semicolon_terminated_elements() {
    let tag = "meta";

    let actual = markup! {
        (tag) charset="utf-8";
    };

    assert_eq!(actual.as_str(), r#"<meta charset="utf-8">"#);
}

#[test]
#[should_panic(expected = "not an HTML void element")]
fn rejects_semicolon_terminated_non_void_elements() {
    let _ = markup! {
        div;
    };
}

#[test]
fn renders_dynamic_class_and_id_selectors() {
    let class_value = "primary";
    let extra = Some("wide");
    let none = None::<String>;
    let id_value = "hero";

    let actual = markup! {
        div.foo.(class_value).(extra).(none) #(id_value) {}
    };

    assert_eq!(
        actual.as_str(),
        r#"<div class="foo primary wide" id="hero"></div>"#
    );
}

#[test]
fn escapes_dynamic_class_and_id_selectors() {
    let class_value = r#"a"b&c<d"#;
    let id_value = r#"x"y&z<q"#;

    let actual = markup! {
        div.(class_value) #(id_value) {}
    };

    assert_eq!(
        actual.as_str(),
        r#"<div class="a&quot;b&amp;c&lt;d" id="x&quot;y&amp;z&lt;q"></div>"#
    );
}

#[test]
fn escapes_text_and_attribute_values() {
    struct View {
        title: String,
    }

    let view = View {
        title: "<Title & \"quote\">".to_owned(),
    };

    let actual = markup! {
        div title=(view.title) {
            @view.title
        }
    };

    assert_eq!(
        actual.as_str(),
        r#"<div title="&lt;Title &amp; &quot;quote&quot;&gt;">&lt;Title &amp; "quote"&gt;</div>"#
    );
}

#[test]
fn supports_dynamic_attributes() {
    let id = "foo";
    let attr = "hx-get";
    let url = "/home";

    let actual = markup! {
        div id=(id) (attr)=(url) {}
    };

    assert_eq!(actual.as_str(), r#"<div id="foo" hx-get="/home"></div>"#);
}

#[test]
fn supports_function_call_attribute_values_without_outer_parentheses() {
    fn foo() -> String {
        "foo".to_owned()
    }

    fn bar(value: &str) -> String {
        format!("bar-{value}")
    }

    let attr = "data-dynamic";

    let actual = markup! {
        div
            foo=foo()
            data-bar=bar("baz")
            data-path=String::from("path")
            "data-literal-call"=bar("literal")
            "data-literal-expr"=(String::from("expr"))
            "data-literal-value"="value"
            (attr)=foo()
        {}
    };

    assert_eq!(
        actual.as_str(),
        r#"<div foo="foo" data-bar="bar-baz" data-path="path" data-literal-call="bar-literal" data-literal-expr="expr" data-literal-value="value" data-dynamic="foo"></div>"#
    );
}

#[test]
fn supports_explicit_attribute_spread() {
    let map = HashMap::from([("id", "foo"), ("hx-get", "/home")]);

    let actual = markup! {
        div ..map {}
    }
    .into_string();

    assert!(actual.starts_with("<div "));
    assert!(actual.ends_with("></div>"));
    assert!(actual.contains(r#" id="foo""#));
    assert!(actual.contains(r#" hx-get="/home""#));
}

#[test]
fn supports_array_pair_attribute_spread() {
    let attrs = [("id", "foo"), ("data-kind", "array")];

    let actual = markup! {
        div ..attrs {}
    };

    assert_eq!(actual.as_str(), r#"<div id="foo" data-kind="array"></div>"#);
}

#[test]
fn supports_array_boolean_attribute_spread() {
    let attrs = ["a", "b"];

    let actual = markup! {
        div ..attrs {}
    };

    assert_eq!(actual.as_str(), "<div a b></div>");
}

#[test]
fn supports_array_literal_boolean_attribute_spread() {
    let actual = markup! {
        div ..["a", "b"] {}
    };

    assert_eq!(actual.as_str(), "<div a b></div>");
}

#[test]
fn supports_parenthesized_attribute_spread_expression() {
    let actual = markup! {
        div ..([("id", "foo"), ("data-kind", "expr")]) {}
    };

    assert_eq!(actual.as_str(), r#"<div id="foo" data-kind="expr"></div>"#);
}

#[test]
fn supports_dynamic_tag_attribute_spread() {
    let tag = "section";
    let attrs = [("id", "hero"), ("data-kind", "dynamic")];

    let actual = markup! {
        (tag) ..attrs {}
    };

    assert_eq!(
        actual.as_str(),
        r#"<section id="hero" data-kind="dynamic"></section>"#
    );
}

#[test]
fn skips_none_attributes_and_renders_bool_attributes() {
    let none = None::<String>;
    let missing_name = None::<String>;
    let enabled = true;
    let disabled = false;

    let actual = markup! {
        div data-value=(none) (missing_name)=("x") hidden=(enabled) inert=(disabled) {}
    };

    assert_eq!(actual.as_str(), "<div hidden></div>");
}

#[test]
fn renders_components_with_children_slot() {
    let children = Markup::slot();
    let layout: Markup = markup! {
        html {
            head {}
            body {
                @children
            }
        }
    };
    let title = "title";

    assert!(layout.accepts_children());
    assert_eq!(layout.as_str(), "<html><head></head><body></body></html>");

    let actual = markup! {
        @layout {
            span { "foo" }
            @title
        }
    };

    assert_eq!(
        actual.as_str(),
        "<html><head></head><body><span>foo</span>title</body></html>"
    );
}

#[test]
fn preserves_nested_children_slots() {
    let children = Markup::slot();
    let layout: Markup = markup! {
        main {
            @children
        }
    };

    let children = Markup::slot();
    let wrapped: Markup = markup! {
        @layout {
            section {
                @children
            }
        }
    };

    let actual = markup! {
        @wrapped {
            "nested"
        }
    };

    assert_eq!(actual.as_str(), "<main><section>nested</section></main>");
}

#[test]
fn inserts_slot_from_path_call() {
    let layout: Markup = markup! {
        main {
            @Markup::slot()
        }
    };

    assert!(layout.accepts_children());

    let actual = markup! {
        @layout {
            "content"
        }
    };

    assert_eq!(actual.as_str(), "<main>content</main>");
}

#[test]
fn renders_named_fragment() {
    let page = markup! {
        div {
            "header"
            @Markup::fragment("name") {
                div { "content" }
            }
        }
    };

    assert_eq!(page.as_str(), "<div>header<div>content</div></div>");
    assert_eq!(page.render_fragment("name").as_str(), "<div>content</div>");
    assert_eq!(
        page.try_render_fragment("name")
            .as_ref()
            .map(Markup::as_str),
        Some("<div>content</div>")
    );
    assert!(page.try_render_fragment("missing").is_none());
}

#[test]
fn renders_nested_fragment_as_part_of_parent_fragment() {
    let page = markup! {
        div {
            "outer"
            @Markup::fragment("outer") {
                "inner"
                @Markup::fragment("inner") {
                    div { "content" }
                }
            }
        }
    };

    assert_eq!(page.as_str(), "<div>outerinner<div>content</div></div>");
    assert_eq!(
        page.render_fragment("outer").as_str(),
        "inner<div>content</div>"
    );
    assert_eq!(page.render_fragment("inner").as_str(), "<div>content</div>");
}

#[test]
fn renders_fragment_with_slot_children() {
    let shell = markup! {
        section {
            @Markup::fragment("body") {
                @Markup::slot()
            }
        }
    };

    let page = markup! {
        @shell {
            div { "content" }
        }
    };

    assert!(shell.accepts_children());
    assert_eq!(shell.as_str(), "<section></section>");
    assert_eq!(page.as_str(), "<section><div>content</div></section>");
    assert_eq!(page.render_fragment("body").as_str(), "<div>content</div>");
}

#[test]
fn renders_component_children_inside_fragment() {
    component! {
        struct Layout {} {
            main {
                @Markup::fragment("body") {
                    @children
                }
            }
        }
    }

    let layout = Layout {};
    let page = markup! {
        @layout {
            p { "Body" }
        }
    };

    assert_eq!(page.as_str(), "<main><p>Body</p></main>");
    assert_eq!(page.render_fragment("body").as_str(), "<p>Body</p>");
}

#[test]
fn renders_option_children_inside_fragment() {
    struct Layout;

    impl Render for Layout {
        fn render(&self, children: Option<Markup>) -> Markup {
            markup! {
                main {
                    @Markup::fragment("body") {
                        @children
                    }
                }
            }
        }
    }

    let page = markup! {
        @Layout {
            p { "Body" }
        }
    };

    assert_eq!(page.as_str(), "<main><p>Body</p></main>");
    assert_eq!(page.render_fragment("body").as_str(), "<p>Body</p>");
}

#[test]
fn renders_doctype_from_path_call() {
    let actual = markup! {
        @Markup::doctype()
        html {
            body {}
        }
    };

    assert_eq!(actual.as_str(), "<!DOCTYPE html><html><body></body></html>");
}

#[test]
fn renders_custom_struct_with_optional_children_render() {
    struct Card {
        title: String,
    }

    impl Render for Card {
        fn render(&self, _children: Option<Markup>) -> Markup {
            let title = &self.title;

            markup! {
                article.card {
                    h2 { @title }
                }
            }
        }
    }

    let card = Card {
        title: "Hello <world>".to_owned(),
    };
    let actual = markup! {
        main { @card }
    };

    assert_eq!(
        actual.as_str(),
        r#"<main><article class="card"><h2>Hello &lt;world&gt;</h2></article></main>"#
    );
}

#[test]
fn renders_custom_component_with_optional_children_render() {
    struct Layout {
        title: String,
    }

    impl Render for Layout {
        fn render(&self, children: Option<Markup>) -> Markup {
            let title = &self.title;

            markup! {
                section.layout {
                    h1 { @title }
                    @children
                }
            }
        }
    }

    let layout = Layout {
        title: "Title".to_owned(),
    };
    let actual = markup! {
        @layout {
            p { "Body" }
        }
    };

    assert_eq!(
        actual.as_str(),
        r#"<section class="layout"><h1>Title</h1><p>Body</p></section>"#
    );
}

#[test]
fn component_macro_declares_struct_and_render_impl() {
    component! {
        struct Layout {
            pub title: String,
        } {
            section.layout {
                h1 { @title }
                @children
            }
        }

        struct Badge {
            pub text: String,
        } {
            span.badge { @text }
        }
    }

    let actual = markup! {
        @Layout {
            title: "Macro".to_owned()
        } {
            @Badge {
                text: "New".to_owned()
            }
            p { "Body" }
        }
    };

    assert_eq!(
        actual.as_str(),
        r#"<section class="layout"><h1>Macro</h1><span class="badge">New</span><p>Body</p></section>"#
    );
}

#[test]
fn component_macro_slot_alias_does_not_conflict_with_children_field() {
    component! {
        struct Panel {
            pub children: String,
        } {
            section {
                h1 { (children) }
                @Markup::slot()
            }
        }
    }

    let actual = markup! {
        @Panel {
            children: "Field".to_owned()
        } {
            p { "Slot" }
        }
    };

    assert_eq!(
        actual.as_str(),
        "<section><h1>Field</h1><p>Slot</p></section>"
    );
}

#[test]
fn component_macro_supports_dynamic_class_and_id_selectors() {
    component! {
        struct Pill {
            pub class_name: String,
            pub id: String,
        } {
            span.badge.(class_name) #(id) { "New" }
        }
    }

    let actual = markup! {
        @Pill {
            class_name: "active".to_owned(),
            id: "pill".to_owned()
        }
    };

    assert_eq!(
        actual.as_str(),
        r#"<span class="badge active" id="pill">New</span>"#
    );
}

#[test]
fn renders_struct_literal_props() {
    struct Layout {
        title: String,
    }

    impl Render for Layout {
        fn render(&self, _children: Option<Markup>) -> Markup {
            let title = &self.title;

            markup! {
                section {
                    h1 { @title }
                }
            }
        }
    }

    let actual = markup! {
        @Layout {
            title: "Props".to_owned()
        }
    };

    assert_eq!(actual.as_str(), "<section><h1>Props</h1></section>");
}

#[test]
fn renders_struct_literal_props_with_children() {
    struct Layout {
        title: String,
    }

    impl Render for Layout {
        fn render(&self, children: Option<Markup>) -> Markup {
            let title = &self.title;

            markup! {
                section {
                    h1 { @title }
                    @children
                }
            }
        }
    }

    let actual = markup! {
        @Layout {
            title: "Props".to_owned()
        } {
            p { "Child" }
        }
    };

    assert_eq!(
        actual.as_str(),
        "<section><h1>Props</h1><p>Child</p></section>"
    );
}

#[test]
fn renders_raw_markup_without_escaping() {
    let child = Markup::raw("<span>raw</span>");

    let actual = markup! {
        div { @child }
    };

    assert_eq!(actual.as_str(), "<div><span>raw</span></div>");
}

#[test]
fn renders_macro_invocation_nodes() {
    let actual = markup! {
        p {
            @format!("{} {}", "foo", "bar")
        }
    };

    assert_eq!(actual.as_str(), "<p>foo bar</p>");
}

#[test]
fn renders_function_call_nodes() {
    fn foo() -> String {
        "foo".to_owned()
    }

    let actual = markup! {
        p { @foo() }
    };

    assert_eq!(actual.as_str(), "<p>foo</p>");
}

#[test]
fn supports_markup_function_declarations() {
    let actual = markup! {
        @fn double(x: i32) -> i32 {
            x * 2
        }

        p { @double(21) }
    };

    assert_eq!(actual.as_str(), "<p>42</p>");
}

#[test]
fn supports_parenthesized_statement_blocks() {
    let actual = markup! {
        p {
            (
                let a = 2;
                let b = 3;
                a + b
            )
        }

        p {
            (
                let name = "John".to_string();
                name + " Snow"
            )
        }
    };

    assert_eq!(actual.as_str(), "<p>5</p><p>John Snow</p>");
}

#[test]
fn supports_markup_control_flow_and_expr_nodes() {
    struct Item {
        name: String,
    }

    let items = vec![
        Item {
            name: "one".to_owned(),
        },
        Item {
            name: "two".to_owned(),
        },
    ];
    let user = Some("Ada");

    let actual = markup! {
        @let foo = "bar";
        @let cond = true;
        @if cond {
            @foo
        } @else {
            "fallback"
        }
        @for item in &items {
            div {
                (format!("item_{}", item.name))
            }
        }
        @match user {
            Some(name) if name.starts_with('A') => {
                span { @name }
            }
            _ => {
                "matched"
            }
        }
    };

    assert_eq!(
        actual.as_str(),
        "bar<div>item_one</div><div>item_two</div><span>Ada</span>"
    );
}

#[test]
fn supports_if_let_else_if_and_match_patterns() {
    let maybe = Some("hit");
    let score = 2;

    let actual = markup! {
        @if let Some(value) = maybe {
            @value
        }
        @if score == 1 {
            "one"
        } @else if score == 2 {
            "two"
        } @else {
            "other"
        }
        @match score {
            0 | 1 => {
                "small"
            }
            value if value > 1 => {
                (format!("big{value}"))
            }
            _ => {
                "fallback"
            }
        }
    };

    assert_eq!(actual.as_str(), "hittwobig2");
}

#[test]
fn component_macro_supports_control_flow_nodes() {
    component! {
        struct List {
            pub items: Vec<&'static str>,
        } {
            @let show = true;
            @if show {
                @for item in items {
                    @match *item {
                        "primary" => {
                            span.primary { @item }
                        }
                        _ => {
                            span { @item }
                        }
                    }
                }
            }
        }
    }

    let actual = markup! {
        @List {
            items: vec!["primary", "secondary"]
        }
    };

    assert_eq!(
        actual.as_str(),
        r#"<span class="primary">primary</span><span>secondary</span>"#
    );
}
