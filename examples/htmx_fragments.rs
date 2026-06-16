use mup::{Markup, markup};

fn page() -> Markup {
    markup! {
        div {
            header { "Todos" }
            @Markup::fragment("todos") {
                ul {
                    li { "Ship mup" }
                }
            }
        }
    }
}

fn main() {
    let page = page();
    let fragment = page.render_fragment("todos");

    assert_eq!(fragment.as_str(), "<ul><li>Ship mup</li></ul>");
}
