# mup

[![CI](https://github.com/ensiouel/mup/actions/workflows/ci.yml/badge.svg)](https://github.com/ensiouel/mup/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/mup.svg)](https://crates.io/crates/mup)
[![Docs.rs](https://docs.rs/mup/badge.svg)](https://docs.rs/mup)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ensiouel/mup/blob/main/LICENSE)

`mup` is a small Rust HTML DSL built around `markup!`, `component!`, `Markup`,
and the `Render` trait.

It is designed for server-rendered Rust applications that want templates to
stay in Rust code: type-checked, composable, easy to refactor, and convenient
for partial HTML responses.

The generated HTML is compact. In this reference, HTML output is formatted only
for readability.

## What Is mup For?

Use `mup` when you want:

- HTML templates without separate template files or a separate template
  language.
- Dynamic tags, dynamic attribute names, dynamic attribute values, and attribute
  spreads from maps or arrays.
- Custom elements like `my-block` and `my-block-item` without stringly tag
  syntax for the common case.
- Components that can receive children directly at the call site.
- Named fragments for rendering partial responses, especially for HTMX-style
  endpoints.
- A compact syntax where arbitrary Rust expressions are wrapped in `(...)`,
  block expressions use `({ ... })`, and common attribute values have short
  forms like `foo=foo()` and `title=view.title`.

The core idea is to keep static HTML visually close to HTML, while making the
dynamic parts explicit and local.

## Why Another Rust HTML DSL?

`mup` is in the same family as
[Maud](https://maud.lambda.xyz/) and
[markup.rs](https://github.com/utkarshkukreti/markup.rs): Rust-native HTML
DSLs that render escaped HTML and let templates use Rust values directly.

The difference is the shape of the API:

- Maud is mature, fast, and small. Its documented composition model is ordinary
  Rust functions returning markup. `mup` keeps that option, but also adds
  first-class component calls with embedded children: `@Layout { ... }`.
- `markup.rs` is a high-performance proc-macro template engine with defined
  templates, dynamic attributes, and attribute spreads. `mup` focuses on one
  inline `markup!` syntax with plain custom tags like `my-block`, dynamic tags
  as `(tag)`, dynamic attribute names as `(attr)=...`, and attributes spread as
  `..attrs`.
- `mup` has named fragments: define `@Markup::fragment("name") { ... }` inside
  a full page and later render only `page.render_fragment("name")`. This is
  useful for HTMX and similar partial-update flows.

In short, `mup` tries to be as capable as the established Rust HTML DSLs while
optimizing for dynamic markup ergonomics, component children, and partial
responses.

## Framework Integrations

`mup` ships optional `IntoResponse` / `Responder` implementations for the most
common Rust web frameworks. Enable the feature that matches your stack:

```toml
# Axum
mup = { version = "0.7", features = ["axum"] }

# Actix-web
mup = { version = "0.7", features = ["actix-web"] }

# Rocket
mup = { version = "0.7", features = ["rocket"] }
```

Once enabled, return `Markup` directly from any handler — the response is sent
with `Content-Type: text/html; charset=utf-8` automatically:

```rust,ignore
// Axum
async fn page() -> Markup {
    markup! { html { body { h1 { "Hello" } } } }
}

// Actix-web
#[get("/")]
async fn page() -> Markup {
    markup! { html { body { h1 { "Hello" } } } }
}

// Rocket
#[get("/")]
fn page() -> Markup {
    markup! { html { body { h1 { "Hello" } } } }
}
```

Runnable server examples are in the `examples/` directory:

```sh
cargo run --example axum    --features axum
cargo run --example actix   --features actix-web
cargo run --example rocket  --features rocket
```

## Feature Snapshot

Dynamic tags, dynamic attribute names, and attribute spreads:

```rust,ignore
let html = markup! {
    (tag) (attr)=("/todos") ..attrs {
        "body"
    }
};
```

Custom elements with hyphenated names use the same tag syntax:

```rust,ignore
let html = markup! {
    my-block.active {
        my-block-item { "item" }
    }
};
```

Components receive children directly where they are used:

```rust,ignore
let html = markup! {
    @layout {
        p { "Body" }
    }
};
```

Named fragments make partial responses explicit:

```rust,ignore
let page = markup! {
    div {
        "header"
        @Markup::fragment("body") {
            p { "Body" }
        }
    }
};

let body = page.render_fragment("body");
```

## Syntax Reference

- Tags: `div { ... }`, `my-block { ... }`, and dynamic tags as `(tag) { ... }`.
- Classes and ids: `div.foo-bar #main-nav { ... }`, simple or dashed names;
  dynamic values as `.(class)` and `#(id)`.
- Static attributes: `type="button"`, `disabled`, and hyphenated names like
  `data-state="open"`.
- Prefixed attribute names: `:type="text"` and `@click="handler"` for
  Vue/Alpine.js-style attribute names.
- Dynamic attribute names: `(attr)=("value")`.
- Dynamic attribute values: `title=(expr)` for arbitrary Rust expressions.
- Attribute short forms: `title=view.title`, `title=label()`, and
  `"data-label"=label()`.
- Block expressions: use `({ ... })` in body nodes and `attr=({ ... })` in
  attributes when local statements are needed.
- Attribute spreads: `..attrs`, `..[("id", "hero")]`, or `..(expr)`.
- Void elements: use `;`, for example `br;` or `input type="text";`.
- Components and values: `@value`, `@component { ... }`, and
  `@Component { field: value } { children }`.
- Control flow: `@let`, `@if`, `@for`, and `@match`.

## Security And Escaping

`mup` is escaped by default:

- Text rendered from string values escapes `&`, `<`, and `>`.
- Attribute values escape `&`, `<`, `>`, and `"`.
- Dynamic tag names and attribute names are validated before rendering.
- Boolean attributes render only when their value is `true`.
- `Option<T>` values render nothing when they are `None`.
- The crate forbids unsafe Rust with `#![forbid(unsafe_code)]`.

Raw HTML is explicit. `Markup::raw(...)` inserts HTML as-is and should only be
used with trusted content that has already been sanitized or generated by your
application.

## Basic Usage

**Rust**

```rust
use mup::{Markup, markup};

let html = markup! {
    @Markup::doctype()
    html lang="en" {
        body {
            h1 { "mup" }
        }
    }
};
```

**HTML result**

```html
<!DOCTYPE html>
<html lang="en">
<body>
<h1>mup</h1>
</body>
</html>
```

## Text, Values, Raw HTML

**Rust**

```rust
use mup::{Markup, markup};

let escaped = "<escaped & safe>";
let raw = Markup::raw("<strong>raw html</strong>");

let html = markup! {
    div {
        p { "literal text: " "<tag> & value" }
        p { "rust value: " @escaped } // `@value` renders and escapes.
        p { "raw markup: " @raw } // `raw` is inserted as-is.
        p { "expression: " (1 + 2) } // Parentheses render a Rust expression.
        p {
            "block result: "
            ({
                let a = 2;
                let b = 3;
                a + b
            })
        }
    }
};
```

**HTML result**

```html

<div>
    <p>literal text: &lt;tag&gt; &amp; value</p>
    <p>rust value: &lt;escaped &amp; safe&gt;</p>
    <p>raw markup: <strong>raw html</strong></p>
    <p>expression: 3</p>
    <p>block result: 5</p>
</div>
```

Parenthesized body nodes must contain Rust expressions. Use a block expression
when local statements are needed.

```rust
use mup::markup;

let html = markup! {
    p {
        ({
            let a = 2;
            a
        })
    }
};
```

## Tags And Selectors

**Rust**

```rust
use mup::markup;

let class = Some("dynamic-class");
let no_class: Option< & str> = None;
let id = "hero";

let html = markup! {
    div.root.(class).(no_class) #(id) {
        span.item { "selectors" }
        span.badge-label { "dashed class" }
        nav #main-nav { "dashed id" }
    }
};
```

**HTML result**

```html

<div class="root dynamic-class" id="hero">
    <span class="item">selectors</span>
    <span class="badge-label">dashed class</span>
    <nav id="main-nav">dashed id</nav>
</div>
```

Class and id shorthand names can contain dashes and underscores:
`.badge-foo_bar` produces `class="badge-foo_bar"`.
For dynamic values, use `.( expr )` and `#( expr )`.

## Div Shorthand

Starting a node with `.class` or `#id` creates a `div`.

**Rust**

```rust
use mup::markup;

let html = markup! {
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
```

**HTML result**

```html

<div class="panel">
    class shorthand
</div>

<div id="main">
    id shorthand
</div>

<div class="panel" id="main" data-state="open">
    combined shorthand
</div>
```

## Custom And Dynamic Tags

**Rust**

```rust
use mup::markup;

let tag = format!("my-{}", "dynamic");

let html = markup! {
    my-block.foo {
        my-block-item { "static custom tag" }
    }

    (tag).generated data-source="expression" {
        "dynamic tag"
    }
};
```

**HTML result**

```html

<my-block class="foo">
    <my-block-item>static custom tag</my-block-item>
</my-block>

<my-dynamic class="generated" data-source="expression">
    dynamic tag
</my-dynamic>
```

## Void Elements

Use `;` for HTML void elements. Non-void tags still need `{ ... }`.

**Rust**

```rust
use mup::markup;

let html = markup! {
    div {
        br;
        input type="text" disabled;
        img.logo src="/logo.svg" alt="";
    }
};
```

**HTML result**

```html

<div>
    <br>
    <input type="text" disabled>
    <img class="logo" src="/logo.svg" alt="">
</div>
```

## Attributes

**Rust**

```rust
use mup::markup;

let html = markup! {
    button
        type="button"
        disabled
        checked=true
        hidden=false // False boolean attrs are skipped.
    {
        "attributes"
    }
};
```

**HTML result**

```html

<button type="button" disabled checked>attributes</button>
```

## Dynamic Attribute Values

**Rust**

```rust
use mup::markup;
use std::borrow::Cow;

fn label() -> String {
    "function attr value".to_owned()
}

let title = Some("optional title");
let empty: Option< & str> = None;
let cow_label: Cow<'static, str> = Cow::Borrowed("cow attr value");

let html = markup! {
    div
        "data-label"=label()
        data-block=({
            let prefix = "block";
            format!("{prefix} attr value")
        })
        title=(title)
        aria-describedby=(empty) // None is skipped.
        aria-label=(cow_label)
    {
        "dynamic values"
    }
};
```

**HTML result**

```html

<div data-label="function attr value" data-block="block attr value" title="optional title" aria-label="cow attr value">
    dynamic values
</div>
```

Attribute values use the same expression rule. Use `=({ ... })` for local
statements, or short forms for function calls, field chains, and method chains.

```rust
use mup::markup;

let html = markup! {
    div title=({
        let value = "title";
        value
    }) {}
};
```

## Dynamic Attribute Names

**Rust**

```rust
use mup::markup;

let attr_name = "data-dynamic";

let html = markup! {
    div (attr_name)=("dynamic attribute name") {
        "attribute name from expression"
    }
};
```

**HTML result**

```html

<div data-dynamic="dynamic attribute name">
    attribute name from expression
</div>
```

## Prefixed Attribute Names

Attribute names starting with `:` or `@` are written directly without quotes.
This is the conventional syntax used by Vue.js, Alpine.js, and similar
frameworks.

**Rust**

```rust
use mup::markup;

let is_required = true;

let html = markup! {
    form {
        input :type="email" :required=(is_required) {}
        button @click="submitForm" { "Submit" }
    }
};
```

**HTML result**

```html

<form>
    <input :type="email" :required>
    <button @click="submitForm">Submit</button>
</form>
```

Dashed names work the same way: `:data-value="x"` and `@event-name="handler"`
are both valid.

For names that contain other special characters such as an Alpine.js modifier
(`:click.prevent`), use a string literal:

```rust,ignore
let html = markup! {
    button "@click.prevent"="handler" { "click" }
};
```

## Attribute Spreads

Use `..attrs` to spread maps, pair arrays, or arrays of boolean attribute
names.

**Rust**

```rust
use mup::markup;
use std::collections::{BTreeMap, HashMap};

let map_attrs = HashMap::from([("data-map", "yes")]);

let mut tree_attrs = BTreeMap::new();
tree_attrs.insert("aria-live", "polite");
tree_attrs.insert("data-tree", "yes");

let pair_attrs = [("data-kind", "pair-array"), ("role", "note")];
let boolean_attr_names = ["a", "b"];

let html = markup! {
    div {
        div ..map_attrs { "HashMap spread" }
        div ..tree_attrs { "BTreeMap spread" }
        div ..pair_attrs { "array of pairs spread" }
        button ..boolean_attr_names { "boolean attr names" }
        button ..["autofocus", "formnovalidate"] { "array literal" }
    }
};
```

**HTML result**

```html

<div>
    <div data-map="yes">HashMap spread</div>
    <div aria-live="polite" data-tree="yes">BTreeMap spread</div>
    <div data-kind="pair-array" role="note">array of pairs spread</div>
    <button a b>boolean attr names</button>
    <button autofocus formnovalidate>array literal</button>
</div>
```

## Function And Macro Calls

**Rust**

```rust
use mup::markup;

fn answer() -> i32 {
    42
}

let html = markup! {
    div {
        @fn double(x: i32) -> i32 {
            x * 2
        }

        p { "function call: " @answer() }
        p { "local function: " @double(21) }
        p { "macro call: " @format!("{} {}", "foo", "bar") }
    }
};
```

**HTML result**

```html

<div>
    <p>function call: 42</p>
    <p>local function: 42</p>
    <p>macro call: foo bar</p>
</div>
```

## Slots

`Markup::slot()` marks where children are inserted when reusable markup is
rendered as `@shell { ... }`.

**Rust**

```rust
use mup::{Markup, markup};

let shell = markup! {
    div.shell {
        header { "before slot" }
        @Markup::slot()
        footer { "after slot" }
    }
};

let html = markup! {
    @shell {
        p { "inserted children" }
    }
};
```

**HTML result**

```html

<div class="shell">
    <header>before slot</header>
    <p>inserted children</p>
    <footer>after slot</footer>
</div>
```

## Fragments

`Markup::fragment(name)` marks a named part of markup. The full markup renders
normally, and `render_fragment(name)` renders only that fragment body.
Use `try_render_fragment(name)` when a missing fragment should return `None`
instead of panicking.

**Rust**

```rust
use mup::{Markup, markup};

let page = markup! {
    div {
        "header"
        @Markup::fragment("body") {
            div { "content" }
        }
    }
};

let fragment = page.render_fragment("body");
```

**HTML result**

```html

<div>content</div>
```

Nested fragments are rendered as normal content when their parent fragment is
rendered.

**Rust**

```rust
use mup::{Markup, markup};

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

let outer = page.render_fragment("outer");
let inner = page.render_fragment("inner");
```

**HTML result**

```html
<!-- outer -->
inner
<div>content</div>

<!-- inner -->
<div>content</div>
```

Fragments can contain slots, so children passed to reusable markup are also
available through `render_fragment`.

**Rust**

```rust
use mup::{Markup, markup};

let shell = markup! {
    section {
        @Markup::fragment("body") {
            @Markup::slot()
        }
    }
};

let page = markup! {
    @shell {
        p { "inserted children" }
    }
};

let body = page.render_fragment("body");
```

**HTML result**

```html
<p>inserted children</p>
```

## Components

Use `@children` inside a component body to render children passed at call site.

**Rust**

```rust
use mup::{component, markup};

component! {
    struct Layout {
        title: String,
    } {
        section.layout {
            h1 { @title }
            @children
        }
    }

    impl Layout {
        fn new(title: impl Into<String>) -> Self {
            Self {
                title: title.into(),
            }
        }
    }
}

let html = markup! {
    @Layout::new("Layout title") {
        p { "layout body" }
    }
};
```

**HTML result**

```html

<section class="layout">
    <h1>Layout title</h1>
    <p>layout body</p>
</section>
```

## Generic Components

`component!` accepts Rust generics on components and matching inherent `impl`
blocks, including lifetimes, type parameters, const parameters, inline bounds,
defaults, and `where` clauses.

**Rust**

```rust
use mup::{Render, component, markup};

component! {
    struct Layout<'a, T>
    where
        T: Render,
    {
        title: &'a str,
        badge: T,
    } {
        main {
            h1 { @title }
            p { @badge }
            @children
        }
    }

    impl<'a, T> Layout<'a, T>
    where
        T: Render,
    {
        fn new(title: &'a str, badge: T) -> Self {
            Self { title, badge }
        }
    }
}

let html = markup! {
    @Layout::new("Components", "generic badge") {
        p { "children" }
    }
};
```

**HTML result**

```html
<main>
    <h1>Components</h1>
    <p>generic badge</p>
    <p>children</p>
</main>
```

## Component Methods

Inside component bodies, `@self.method(...)` calls methods on the current
component instance. Associated functions that do not take `self` can be called
with `@Self::function(...)`. The same forms work in attribute values.

**Rust**

```rust
use mup::{component, markup};

component! {
    struct Profile {
        first: String,
        last: String,
    } {
        article data-profile=self.initials().to_lowercase() data-kind=Self::kind() {
            h1 { @self.display_name(" ") }
            p { @self.initials().to_uppercase() }
            small { @Self::kind() }
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
            format!("{}{}{}", self.first, separator, self.last)
        }

        fn initials(&self) -> String {
            format!(
                "{}{}",
                self.first.chars().next().unwrap_or_default(),
                self.last.chars().next().unwrap_or_default()
            )
        }

        fn kind() -> &'static str {
            "profile"
        }
    }
}

let html = markup! {
    @Profile::new("Ada", "Lovelace")
};
```

**HTML result**

```html
<article data-profile="al" data-kind="profile">
    <h1>Ada Lovelace</h1>
    <p>AL</p>
    <small>profile</small>
</article>
```

## Component Slot Alias

Inside components, `@Markup::slot()` is also accepted as an alias for
`@children`.

**Rust**

```rust
use mup::{Markup, component, markup};

component! {
    struct Card {
        title: String,
    } {
        article.card {
            h2 { @title }
            @Markup::slot()
        }
    }

    impl Card {
        fn new(title: impl Into<String>) -> Self {
            Self {
                title: title.into(),
            }
        }
    }
}

let html = markup! {
    @Card::new("Card title") {
        p { "card body" }
    }
};
```

**HTML result**

```html

<article class="card">
    <h2>Card title</h2>
    <p>card body</p>
</article>
```

## Struct Literal Components

**Rust**

```rust
use mup::{component, markup};

component! {
    struct Badge {
        id: String,
        tone: Option<&'static str>,
    } {
        span.badge.(tone) #(id) {
            @children
        }
    }
}

let html = markup! {
    @Badge {
        id: "info".to_owned(),
        tone: Some("info")
    } {
        "badge body"
    }
};
```

**HTML result**

```html
<span class="badge info" id="info">
  badge body
</span>
```

## Custom Render

**Rust**

```rust
use mup::{Markup, Render, markup};

struct Link {
    href: String,
    label: String,
}

impl Render for Link {
    fn render(&self, _children: Option<Markup>) -> Markup {
        let href = &self.href;
        let label = &self.label;

        markup! {
            a href=(href) { @label }
        }
    }
}

let html = markup! {
    @Link {
        href: "/docs".to_owned(),
        label: "custom Render implementation".to_owned()
    }
};
```

**HTML result**

```html
<a href="/docs">custom Render implementation</a>
```

## Enum Components

**Rust**

```rust
use mup::{component, markup};

component! {
    enum Status {
        Ok,
        Warning,
    } {
        Ok => {
            span.status.ok { "Ok" }
        },
        Warning => {
            span.status.warning { "Warning" }
        },
    }

    impl Status {
        fn ok() -> Self {
            Self::Ok
        }

        fn warning() -> Self {
            Self::Warning
        }
    }
}

let html = markup! {
    p {
        "statuses: "
        @Status::ok()
        " / "
        @Status::warning()
    }
};
```

**HTML result**

```html
<p>
    statuses:
    <span class="status ok">Ok</span>
    /
    <span class="status warning">Warning</span>
</p>
```

## Control Flow

**Rust**

```rust
use mup::markup;

let show_details = true;
let items = ["one", "two", "three"];
let maybe_user = Some("Ada");

let html = markup! {
    @let local = "local binding";

    @if show_details {
        p { "if branch with " @local }
    } @else {
        p { "else branch" }
    }

    ul {
        @for item in &items {
            li { @item }
        }
    }

    @match maybe_user {
        Some(name) if name.starts_with('A') => {
            p { "matched user: " @name }
        }
        Some(name) => {
            p { "other user: " @name }
        }
        None => {
            p { "no user" }
        }
    }
};
```

**HTML result**

```html
<p>
    if branch with local binding
</p>

<ul>
    <li>one</li>
    <li>two</li>
    <li>three</li>
</ul>

<p>
    matched user: Ada
</p>
```

## Notes

`mup` is primarily a personal project, started to solve practical problems I
ran into while using similar Rust HTML DSLs. Contributions and feedback are
welcome via GitHub issues.
