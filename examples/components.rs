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

fn main() {
    let html = markup! {
        @Layout::new("Components", "generic badge") {
            p { "children" }
        }
    };

    assert_eq!(
        html.as_str(),
        "<main><h1>Components</h1><p>generic badge</p><p>children</p></main>"
    );
}
