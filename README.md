# mup

> **Note:** this codebase is fully AI-generated. `mup` is primarily a project
> for my personal use, but it exists because, in my opinion, it solves several
> practical problems I ran into while using similar Rust HTML DSLs.

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
- A compact syntax where arbitrary Rust expressions are wrapped in `(...)`, with
  a short form for attribute values produced by function calls.

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

## Basic Usage

<table>
<thead>
<tr>
<th>Rust</th>
<th>HTML result</th>
</tr>
</thead>
<tbody>
<tr>
<td valign="top">

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

</td>
<td valign="top">

```html
<!DOCTYPE html>
<html lang="en">
<body>
<h1>mup</h1>
</body>
</html>
```

</td>
</tr>
</tbody>
</table>

## Text, Values, Raw HTML

<table>
<thead>
<tr>
<th>Rust</th>
<th>HTML result</th>
</tr>
</thead>
<tbody>
<tr>
<td valign="top">

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
            (
                let a = 2;
                let b = 3;
                a + b
            )
        }
    }
};
```

</td>
<td valign="top">

```html

<div>
    <p>literal text: &lt;tag&gt; &amp; value</p>
    <p>rust value: &lt;escaped &amp; safe&gt;</p>
    <p>raw markup: <strong>raw html</strong></p>
    <p>expression: 3</p>
    <p>block result: 5</p>
</div>
```

</td>
</tr>
</tbody>
</table>

## Tags And Selectors

<table>
<thead>
<tr>
<th>Rust</th>
<th>HTML result</th>
</tr>
</thead>
<tbody>
<tr>
<td valign="top">

```rust
use mup::markup;

let class = Some("dynamic-class");
let no_class: Option< & str> = None;
let id = "hero";

let html = markup! {
    div.root.(class).(no_class) #(id) {
        span.item { "selectors" }
    }
};
```

</td>
<td valign="top">

```html

<div class="root dynamic-class" id="hero">
    <span class="item">selectors</span>
</div>
```

</td>
</tr>
</tbody>
</table>

## Div Shorthand

Starting a node with `.class` or `#id` creates a `div`.

<table>
<thead>
<tr>
<th>Rust</th>
<th>HTML result</th>
</tr>
</thead>
<tbody>
<tr>
<td valign="top">

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

</td>
<td valign="top">

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

</td>
</tr>
</tbody>
</table>

## Custom And Dynamic Tags

<table>
<thead>
<tr>
<th>Rust</th>
<th>HTML result</th>
</tr>
</thead>
<tbody>
<tr>
<td valign="top">

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

</td>
<td valign="top">

```html

<my-block class="foo">
    <my-block-item>static custom tag</my-block-item>
</my-block>

<my-dynamic class="generated" data-source="expression">
    dynamic tag
</my-dynamic>
```

</td>
</tr>
</tbody>
</table>

## Void Elements

Use `;` for HTML void elements. Non-void tags still need `{ ... }`.

<table>
<thead>
<tr>
<th>Rust</th>
<th>HTML result</th>
</tr>
</thead>
<tbody>
<tr>
<td valign="top">

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

</td>
<td valign="top">

```html

<div>
    <br>
    <input type="text" disabled>
    <img class="logo" src="/logo.svg" alt="">
</div>
```

</td>
</tr>
</tbody>
</table>

## Attributes

<table>
<thead>
<tr>
<th>Rust</th>
<th>HTML result</th>
</tr>
</thead>
<tbody>
<tr>
<td valign="top">

```rust
use mup::markup;

let html = markup! {
    button
        type="button"
        disabled
        checked=(true)
        hidden=(false) // False boolean attrs are skipped.
    {
        "attributes"
    }
};
```

</td>
<td valign="top">

```html

<button type="button" disabled checked>attributes</button>
```

</td>
</tr>
</tbody>
</table>

## Dynamic Attribute Values

<table>
<thead>
<tr>
<th>Rust</th>
<th>HTML result</th>
</tr>
</thead>
<tbody>
<tr>
<td valign="top">

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
        title=(title)
        aria-describedby=(empty) // None is skipped.
        aria-label=(cow_label)
    {
        "dynamic values"
    }
};
```

</td>
<td valign="top">

```html

<div data-label="function attr value" title="optional title" aria-label="cow attr value">
    dynamic values
</div>
```

</td>
</tr>
</tbody>
</table>

## Dynamic Attribute Names

<table>
<thead>
<tr>
<th>Rust</th>
<th>HTML result</th>
</tr>
</thead>
<tbody>
<tr>
<td valign="top">

```rust
use mup::markup;

let attr_name = "data-dynamic";

let html = markup! {
    div (attr_name)=("dynamic attribute name") {
        "attribute name from expression"
    }
};
```

</td>
<td valign="top">

```html

<div data-dynamic="dynamic attribute name">
    attribute name from expression
</div>
```

</td>
</tr>
</tbody>
</table>

## Attribute Spreads

Use `..attrs` to spread maps, pair arrays, or arrays of boolean attribute
names.

<table>
<thead>
<tr>
<th>Rust</th>
<th>HTML result</th>
</tr>
</thead>
<tbody>
<tr>
<td valign="top">

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

</td>
<td valign="top">

```html

<div>
    <div data-map="yes">HashMap spread</div>
    <div aria-live="polite" data-tree="yes">BTreeMap spread</div>
    <div data-kind="pair-array" role="note">array of pairs spread</div>
    <button a b>boolean attr names</button>
    <button autofocus formnovalidate>array literal</button>
</div>
```

</td>
</tr>
</tbody>
</table>

## Function And Macro Calls

<table>
<thead>
<tr>
<th>Rust</th>
<th>HTML result</th>
</tr>
</thead>
<tbody>
<tr>
<td valign="top">

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

</td>
<td valign="top">

```html

<div>
    <p>function call: 42</p>
    <p>local function: 42</p>
    <p>macro call: foo bar</p>
</div>
```

</td>
</tr>
</tbody>
</table>

## Slots

`Markup::slot()` marks where children are inserted when reusable markup is
rendered as `@shell { ... }`.

<table>
<thead>
<tr>
<th>Rust</th>
<th>HTML result</th>
</tr>
</thead>
<tbody>
<tr>
<td valign="top">

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

</td>
<td valign="top">

```html

<div class="shell">
    <header>before slot</header>
    <p>inserted children</p>
    <footer>after slot</footer>
</div>
```

</td>
</tr>
</tbody>
</table>

## Fragments

`Markup::fragment(name)` marks a named part of markup. The full markup renders
normally, and `render_fragment(name)` renders only that fragment body.
Use `try_render_fragment(name)` when a missing fragment should return `None`
instead of panicking.

<table>
<thead>
<tr>
<th>Rust</th>
<th>HTML result</th>
</tr>
</thead>
<tbody>
<tr>
<td valign="top">

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

</td>
<td valign="top">

```html

<div>content</div>
```

</td>
</tr>
</tbody>
</table>

Nested fragments are rendered as normal content when their parent fragment is
rendered.

<table>
<thead>
<tr>
<th>Rust</th>
<th>HTML result</th>
</tr>
</thead>
<tbody>
<tr>
<td valign="top">

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

</td>
<td valign="top">

```html
<!-- outer -->
inner
<div>content</div>

<!-- inner -->
<div>content</div>
```

</td>
</tr>
</tbody>
</table>

Fragments can contain slots, so children passed to reusable markup are also
available through `render_fragment`.

<table>
<thead>
<tr>
<th>Rust</th>
<th>HTML result</th>
</tr>
</thead>
<tbody>
<tr>
<td valign="top">

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

</td>
<td valign="top">

```html
<p>inserted children</p>
```

</td>
</tr>
</tbody>
</table>

## Components

Use `@children` inside a component body to render children passed at call site.

<table>
<thead>
<tr>
<th>Rust</th>
<th>HTML result</th>
</tr>
</thead>
<tbody>
<tr>
<td valign="top">

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

</td>
<td valign="top">

```html

<section class="layout">
    <h1>Layout title</h1>
    <p>layout body</p>
</section>
```

</td>
</tr>
</tbody>
</table>

## Component Slot Alias

Inside components, `@Markup::slot()` is also accepted as an alias for
`@children`.

<table>
<thead>
<tr>
<th>Rust</th>
<th>HTML result</th>
</tr>
</thead>
<tbody>
<tr>
<td valign="top">

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

</td>
<td valign="top">

```html

<article class="card">
    <h2>Card title</h2>
    <p>card body</p>
</article>
```

</td>
</tr>
</tbody>
</table>

## Struct Literal Components

<table>
<thead>
<tr>
<th>Rust</th>
<th>HTML result</th>
</tr>
</thead>
<tbody>
<tr>
<td valign="top">

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

</td>
<td valign="top">

```html
<span class="badge info" id="info">
  badge body
</span>
```

</td>
</tr>
</tbody>
</table>

## Custom Render

<table>
<thead>
<tr>
<th>Rust</th>
<th>HTML result</th>
</tr>
</thead>
<tbody>
<tr>
<td valign="top">

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

</td>
<td valign="top">

```html
<a href="/docs">custom Render implementation</a>
```

</td>
</tr>
</tbody>
</table>

## Enum Components

<table>
<thead>
<tr>
<th>Rust</th>
<th>HTML result</th>
</tr>
</thead>
<tbody>
<tr>
<td valign="top">

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

</td>
<td valign="top">

```html
<p>
    statuses:
    <span class="status ok">Ok</span>
    /
    <span class="status warning">Warning</span>
</p>
```

</td>
</tr>
</tbody>
</table>

## Control Flow

<table>
<thead>
<tr>
<th>Rust</th>
<th>HTML result</th>
</tr>
</thead>
<tbody>
<tr>
<td valign="top">

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

</td>
<td valign="top">

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

</td>
</tr>
</tbody>
</table>
