use mup::{Markup, markup};

fn main() {
    let title = "mup";
    let escaped = "<safe>";

    let html = markup! {
        @Markup::doctype()
        html lang="en" {
            body {
                h1 title=(title) { @title }
                p { "escaped: " @escaped }
            }
        }
    };

    assert_eq!(
        html.as_str(),
        r#"<!DOCTYPE html><html lang="en"><body><h1 title="mup">mup</h1><p>escaped: &lt;safe&gt;</p></body></html>"#
    );
}
