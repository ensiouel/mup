use std::fmt::{self, Write as _};

pub(crate) fn escape_text_into(value: &str, out: &mut String) {
    escape_with(value, out, false);
}

pub(crate) fn escape_attr_value_into(value: &str, out: &mut String) {
    escape_with(value, out, true);
}

fn escape_with(value: &str, out: &mut String, attr: bool) {
    let mut start = 0;

    for (index, ch) in value.char_indices() {
        let replacement = match ch {
            '&' => Some("&amp;"),
            '<' => Some("&lt;"),
            '>' => Some("&gt;"),
            '"' if attr => Some("&quot;"),
            _ => None,
        };

        if let Some(replacement) = replacement {
            out.push_str(&value[start..index]);
            out.push_str(replacement);
            start = index + ch.len_utf8();
        }
    }

    out.push_str(&value[start..]);
}

pub(crate) fn push_display(out: &mut String, value: &impl fmt::Display) {
    let _ = write!(out, "{value}");
}

pub(crate) fn assert_valid_tag_name(name: &str) {
    let valid = !name.is_empty()
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'));

    assert!(valid, "invalid HTML tag name: {name:?}");
}

pub(crate) fn assert_valid_void_tag_name(name: &str) {
    let valid = [
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
        "source", "track", "wbr",
    ]
    .iter()
    .any(|tag| tag.eq_ignore_ascii_case(name));

    assert!(valid, "not an HTML void element: {name:?}");
}

pub(crate) fn assert_valid_attr_name(name: &str) {
    let valid = !name.is_empty()
        && name.chars().all(|ch| {
            !ch.is_whitespace() && !matches!(ch, '"' | '\'' | '>' | '<' | '=' | '/' | '`')
        });

    assert!(valid, "invalid HTML attribute name: {name:?}");
}
