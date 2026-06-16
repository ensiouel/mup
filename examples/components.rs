use mup::{component, markup};

component! {
    struct Layout {
        title: String,
    } {
        main {
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

fn main() {
    let html = markup! {
        @Layout::new("Components") {
            p { "children" }
        }
    };

    assert_eq!(
        html.as_str(),
        "<main><h1>Components</h1><p>children</p></main>"
    );
}
