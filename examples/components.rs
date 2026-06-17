use mup::{Render, component, markup};

component! {
    struct Badge {
        label: String,
    } {
        span.badge data-kind=Self::kind() {
            @label
        }
    }

    impl Badge {
        fn new(label: impl Into<String>) -> Self {
            Self {
                label: label.into(),
            }
        }

        fn kind() -> &'static str {
            "status"
        }
    }

    struct Layout<'a, T>
    where
        T: Render,
    {
        title: &'a str,
        badge: T,
    } {
        main.layout data-title=self.slug() data-score=Self::score(3) {
            header {
                h1 { @self.heading() }
                @badge
            }
            section.content {
                @children
            }
        }
    }

    impl<'a, T> Layout<'a, T>
    where
        T: Render,
    {
        fn new(title: &'a str, badge: T) -> Self {
            Self { title, badge }
        }

        fn heading(&self) -> String {
            format!("{} example", self.title)
        }

        fn slug(&self) -> String {
            self.title.to_lowercase().replace(' ', "-")
        }

        fn score(value: i32) -> i32 {
            value * 2
        }
    }
}

fn main() {
    let html = markup! {
        @Layout::new("Components", Badge::new("stable")) {
            p { "Components can render fields, methods, associated functions, and children." }
        }
    };

    println!("{}", html.as_str());
}
