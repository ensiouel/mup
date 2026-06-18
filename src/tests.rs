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
#[should_panic(expected = "invalid HTML tag name")]
fn rejects_invalid_dynamic_tag_names() {
    let tag = "bad tag";

    let _ = markup! {
        (tag) {}
    };
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
        title: Title,
    }
    struct Title {
        inner: String,
    }

    let view = View {
        title: Title {
            inner: "<Title & \"quote\">".to_owned(),
        },
    };

    let actual = markup! {
        div title=view.title.inner {
            ({
                let inner = &view.title.inner;
                inner.to_string()
            })
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
            (attr)=foo()
        {}
    };

    assert_eq!(
        actual.as_str(),
        r#"<div foo="foo" data-bar="bar-baz" data-path="path" data-dynamic="foo"></div>"#
    );
}

#[test]
fn supports_literal_attribute_names() {
    let expr = String::from("expr");

    let actual = markup! {
        div
            "data-call"=String::from("call")
            "data-expr"=(expr)
            "data-value"="value"
        {}
    };

    assert_eq!(
        actual.as_str(),
        r#"<div data-call="call" data-expr="expr" data-value="value"></div>"#
    );
}

#[test]
fn supports_field_chain_attribute_values_without_outer_parentheses() {
    struct View {
        title: Title,
    }
    struct Title {
        inner: String,
    }

    let view = View {
        title: Title {
            inner: "field".to_owned(),
        },
    };
    let attr = "data-dynamic-field";

    let actual = markup! {
        div.featured
            title=view.title.inner
            "data-literal-field"=view.title.inner
            (attr)=view.title.inner
            data-next="next"
        {}
    };

    assert_eq!(
        actual.as_str(),
        r#"<div class="featured" title="field" data-literal-field="field" data-dynamic-field="field" data-next="next"></div>"#
    );
}

#[test]
fn supports_method_chain_attribute_values_without_outer_parentheses() {
    struct View {
        title: String,
    }

    impl View {
        fn slug(&self, suffix: &str) -> String {
            format!("{}-{suffix}", self.title.to_lowercase())
        }
    }

    let view = View {
        title: "Profile".to_owned(),
    };
    let attr = "data-dynamic-slug";
    let label = "method attr";

    let actual = markup! {
        div
            title=label
            data-title=view.title
            data-slug=view.slug("card").to_uppercase()
            "data-literal-slug"=view.slug("literal")
            (attr)=view.slug("dynamic")
        {}
    };

    assert_eq!(
        actual.as_str(),
        r#"<div title="method attr" data-title="Profile" data-slug="PROFILE-CARD" data-literal-slug="profile-literal" data-dynamic-slug="profile-dynamic"></div>"#
    );
}

#[test]
fn supports_block_expression_attribute_values() {
    let attr = "data-dynamic-block";

    let actual = markup! {
        div
            title=({
                let value = "block";
                format!("{value}-title")
            })
            "data-literal-block"=({
                let value = "literal";
                format!("{value}-block")
            })
            (attr)=({
                let value = "dynamic";
                format!("{value}-block")
            })
        {}
    };

    assert_eq!(
        actual.as_str(),
        r#"<div title="block-title" data-literal-block="literal-block" data-dynamic-block="dynamic-block"></div>"#
    );
}

#[test]
#[should_panic(expected = "invalid HTML attribute name")]
fn rejects_invalid_dynamic_attribute_names() {
    let attr = "bad attr";

    let _ = markup! {
        div (attr)=("value") {}
    };
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
fn supports_tuple_pair_attribute_spread() {
    let attr = ("data-value", "hello");

    let actual = markup! {
        div ..attr {}
    };

    assert_eq!(actual.as_str(), r#"<div data-value="hello"></div>"#);
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
#[should_panic(expected = "fragment not found: missing")]
fn panics_when_rendering_missing_fragment() {
    let page = markup! {
        div {
            @Markup::fragment("name") {
                "content"
            }
        }
    };

    let _ = page.render_fragment("missing");
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
fn component_macro_supports_lifetimes_and_generics() {
    component! {
        struct Field<'a, T: Render + Clone = String, const N: usize = 1>
        where
            T: Render + Clone,
        {
            pub label: &'a str,
            pub value: T,
            pub _slots: [(); N],
        } {
            span.field {
                @label
                ": "
                @value
                " / "
                @N
            }
        }

        impl<'a, T: Render + Clone, const N: usize> Field<'a, T, N>
        where
            T: Render + Clone,
        {
            fn new(label: &'a str, value: T, _slots: [(); N]) -> Self {
                Self {
                    label,
                    value,
                    _slots,
                }
            }
        }

        impl<'a, T> Field<'a, Option<T>, 1>
        where
            Option<T>: Render + Clone,
        {
            fn from_option(label: &'a str, value: Option<T>) -> Self {
                Self {
                    label,
                    value,
                    _slots: [(); 1],
                }
            }
        }
    }

    let actual = markup! {
        @Field::new("name", "Mup".to_owned(), [(); 2])
        @Field::from_option("maybe", Some("optional".to_owned()))
    };

    assert_eq!(
        actual.as_str(),
        r#"<span class="field">name: Mup / 2</span><span class="field">maybe: optional / 1</span>"#
    );
}

#[test]
fn component_macro_supports_struct_self_method_calls() {
    component! {
        struct Profile {
            first: String,
            last: String,
        } {
            article {
                h1 { @self.display_name(" / ") }
                p { @self.initials().to_uppercase() }
            }
        }

        impl Profile {
            fn new(first: impl Into<String>, last: impl Into<String>) -> Self {
                Self {
                    first: first.into(),
                    last: last.into(),
                }
            }

            fn display_name(&self, separator: &str) -> String {
                format!("{}{}{}", self.last, separator, self.first)
            }

            fn initials(&self) -> String {
                format!(
                    "{}{}",
                    self.first.chars().next().unwrap_or_default(),
                    self.last.chars().next().unwrap_or_default()
                )
            }
        }
    }

    let actual = markup! {
        @Profile::new("Ada", "Lovelace")
    };

    assert_eq!(
        actual.as_str(),
        "<article><h1>Lovelace / Ada</h1><p>AL</p></article>"
    );
}

#[test]
fn component_macro_supports_struct_self_attribute_values() {
    component! {
        struct Link {
            label: String,
            route: String,
        } {
            @let tracking_attr = "data-track";
            a
                href=self.href()
                title=self.title("Open").to_uppercase()
                data-route=self.route
                data-kind=Self::kind()
                "data-label"=label
                (tracking_attr)=self.tracking_id()
            {
                @label
            }
        }

        impl Link {
            fn new(label: impl Into<String>, route: impl Into<String>) -> Self {
                Self {
                    label: label.into(),
                    route: route.into(),
                }
            }

            fn href(&self) -> String {
                format!("/{}", self.route.trim_start_matches('/'))
            }

            fn title(&self, prefix: &str) -> String {
                format!("{prefix} {}", self.label)
            }

            fn tracking_id(&self) -> String {
                format!("link-{}", self.route.trim_matches('/'))
            }

            fn kind() -> &'static str {
                "nav"
            }
        }
    }

    let actual = markup! {
        @Link::new("Docs", "/docs")
    };

    assert_eq!(
        actual.as_str(),
        r#"<a href="/docs" title="OPEN DOCS" data-route="/docs" data-kind="nav" data-label="Docs" data-track="link-docs">Docs</a>"#
    );
}

#[test]
fn component_macro_supports_struct_associated_function_calls() {
    component! {
        struct Meter {
            value: i32,
        } {
            div {
                span { @Self::format_value(*value) }
                small { @Meter::unit_label() }
            }
        }

        impl Meter {
            fn new(value: i32) -> Self {
                Self { value }
            }

            fn format_value(value: i32) -> String {
                format!("{value:03}")
            }

            fn unit_label() -> &'static str {
                "rpm"
            }
        }
    }

    let actual = markup! {
        @Meter::new(7)
    };

    assert_eq!(
        actual.as_str(),
        "<div><span>007</span><small>rpm</small></div>"
    );
}

#[test]
fn component_macro_supports_generic_enums() {
    component! {
        enum Marker<'a, T: Render> {
            Borrowed(&'a T),
            Owned { value: T },
        } {
            Borrowed(value) => {
                span.borrowed { @value }
            },
            Owned { value } => {
                span.owned { @value }
            },
        }
    }

    let borrowed_text = "borrowed".to_owned();
    let borrowed = Marker::Borrowed(&borrowed_text);
    let owned = Marker::Owned {
        value: "owned".to_owned(),
    };
    let actual = markup! {
        @borrowed
        @owned
    };

    assert_eq!(
        actual.as_str(),
        r#"<span class="borrowed">borrowed</span><span class="owned">owned</span>"#
    );
}

#[test]
fn component_macro_supports_enum_self_method_call_nodes() {
    component! {
        enum Status {
            Ready,
        } {
            Ready => {
                span { @self.label() }
            }
        }

        impl Status {
            fn label(&self) -> &'static str {
                "ready"
            }
        }
    }

    let status = Status::Ready;
    let actual = markup! {
        @status
    };

    assert_eq!(actual.as_str(), "<span>ready</span>");
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
fn supports_parenthesized_block_expressions() {
    let actual = markup! {
        p {
            ({
                let a = 2;
                let b = 3;
                a + b
            })
        }

        p {
            ({
                let name = "John".to_string();
                name + " Snow"
            })
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

#[test]
fn supports_dashed_class_and_id_shorthand() {
    let actual = markup! {
        div.foo-bar #main-nav {
            span.badge-foo_bar { "content" }
        }
    };

    assert_eq!(
        actual.as_str(),
        r#"<div class="foo-bar" id="main-nav"><span class="badge-foo_bar">content</span></div>"#
    );
}

#[test]
fn supports_colon_prefixed_attribute_names() {
    let is_disabled = true;

    let actual = markup! {
        input :type="password" :disabled=is_disabled {}
    };

    assert_eq!(
        actual.as_str(),
        r#"<input :type="password" :disabled></input>"#
    );
}

#[test]
fn supports_at_prefixed_attribute_names() {
    let actual = markup! {
        button @click="handler" @focus="true" {}
    };

    assert_eq!(
        actual.as_str(),
        r#"<button @click="handler" @focus="true"></button>"#
    );
}

#[test]
fn supports_dashed_colon_and_at_prefixed_attribute_names() {
    let actual = markup! {
        div :data-value="foo" @event-name="bar" {}
    };

    assert_eq!(
        actual.as_str(),
        r#"<div :data-value="foo" @event-name="bar"></div>"#
    );
}
