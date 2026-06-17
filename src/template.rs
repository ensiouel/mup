use crate::attrs::{push_boolean_attr, push_str_attr};
use crate::html::{assert_valid_tag_name, assert_valid_void_tag_name};
use crate::markup::{MarkupPart, push_markup_part};
use crate::{AttributeName, AttributeValue, Attributes, ClassValue, Markup, Render};

#[doc(hidden)]
pub struct TemplateBuilder {
    pub current: String,
    parts: Vec<MarkupPart>,
}

impl Default for TemplateBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateBuilder {
    pub fn new() -> Self {
        Self {
            current: String::new(),
            parts: Vec::new(),
        }
    }

    pub fn push_markup(&mut self, markup: &Markup) {
        self.flush_current();
        markup.append_parts_to(&mut self.parts);
    }

    pub fn finish(mut self) -> Markup {
        self.flush_current();
        Markup::from_parts(self.parts)
    }

    fn flush_current(&mut self) {
        if !self.current.is_empty() {
            push_markup_part(
                &mut self.parts,
                MarkupPart::Html(std::mem::take(&mut self.current)),
            );
        }
    }
}

pub fn render<T>(value: &T, children: Option<Markup>) -> Markup
where
    T: Render + ?Sized,
{
    value.render(children)
}

pub fn push_start_tag(out: &mut String, tag: &str) {
    assert_valid_tag_name(tag);
    out.push('<');
    out.push_str(tag);
}

pub fn finish_start_tag(out: &mut String) {
    out.push('>');
}

pub fn finish_void_tag(out: &mut String, tag: &str) {
    assert_valid_void_tag_name(tag);
    out.push('>');
}

pub fn push_end_tag(out: &mut String, tag: &str) {
    assert_valid_tag_name(tag);
    out.push_str("</");
    out.push_str(tag);
    out.push('>');
}

pub fn push_attr<N, V>(out: &mut String, name: &N, value: &V)
where
    N: AttributeName + ?Sized,
    V: AttributeValue + ?Sized,
{
    name.with_attr_name(&mut |name| value.render_attr_into(out, name));
}

pub fn push_bool_attr<N>(out: &mut String, name: &N)
where
    N: AttributeName + ?Sized,
{
    name.with_attr_name(&mut |name| push_boolean_attr(out, name));
}

pub fn push_attrs<A>(out: &mut String, attrs: &A)
where
    A: Attributes + ?Sized,
{
    attrs.render_attrs_into(out);
}

pub fn push_attr_segments<V>(out: &mut String, segments: &[&str], value: &V)
where
    V: AttributeValue + ?Sized,
{
    match segments {
        [name] => value.render_attr_into(out, name),
        _ => value.render_attr_into(out, &join_segments(segments)),
    }
}

pub fn push_bool_attr_segments(out: &mut String, segments: &[&str]) {
    match segments {
        [name] => push_boolean_attr(out, name),
        _ => push_boolean_attr(out, &join_segments(segments)),
    }
}

pub fn push_class_value<V>(classes: &mut String, value: &V)
where
    V: ClassValue + ?Sized,
{
    let original_len = classes.len();

    if original_len > 0 {
        classes.push(' ');
    }

    let value_start = classes.len();
    value.render_class_into(classes);

    if classes.len() == value_start {
        classes.truncate(original_len);
    }
}

pub fn push_class_attr(out: &mut String, classes: &str) {
    if !classes.is_empty() {
        push_str_attr(out, "class", classes);
    }
}

pub fn push_prefixed_attr_segments<V>(out: &mut String, prefix: &str, segments: &[&str], value: &V)
where
    V: AttributeValue + ?Sized,
{
    value.render_attr_into(out, &build_prefixed_name(prefix, segments));
}

pub fn push_bool_prefixed_attr_segments(out: &mut String, prefix: &str, segments: &[&str]) {
    push_boolean_attr(out, &build_prefixed_name(prefix, segments));
}

fn build_prefixed_name(prefix: &str, segments: &[&str]) -> String {
    let size = prefix.len()
        + segments.iter().map(|s| s.len()).sum::<usize>()
        + segments.len().saturating_sub(1);
    let mut name = String::with_capacity(size);
    name.push_str(prefix);
    for (i, seg) in segments.iter().enumerate() {
        if i > 0 {
            name.push('-');
        }
        name.push_str(seg);
    }
    name
}

fn join_segments(segments: &[&str]) -> String {
    build_prefixed_name("", segments)
}
