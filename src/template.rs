use crate::attrs::push_boolean_attr;
use crate::html::{assert_valid_tag_name, assert_valid_void_tag_name, escape_attr_value_into};
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
    #[inline]
    pub fn new() -> Self {
        // ponytail: 512 covers most templates with at most 2 reallocations instead of 10+
        Self {
            current: String::with_capacity(512),
            parts: Vec::new(),
        }
    }

    /// Renders `value` into this builder directly, bypassing intermediate Markup allocation.
    ///
    /// Method form so it auto-derefs for both owned `TemplateBuilder` and `&mut TemplateBuilder`
    /// call sites in macro-generated code.
    #[doc(hidden)]
    #[inline]
    pub fn push_render<T>(&mut self, value: &T)
    where
        T: Render + ?Sized,
    {
        value.render_into_builder(self);
    }

    #[inline]
    pub fn push_markup(&mut self, markup: &Markup) {
        if !markup.has_template() {
            // ponytail: fast path — plain Markup has no slots/fragments, skip flush+parts
            self.current.push_str(markup.as_str());
        } else {
            self.flush_current();
            markup.append_parts_to(&mut self.parts);
        }
    }

    #[inline]
    pub fn finish(mut self) -> Markup {
        if self.parts.is_empty() {
            // ponytail: fast path — no slots/fragments, current IS the final output
            return Markup::raw(self.current);
        }
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

// ponytail: static tag — macro already validated name at compile time, skip runtime assert
#[inline]
pub fn push_start_tag_static(out: &mut String, tag: &str) {
    out.push('<');
    out.push_str(tag);
}

#[inline]
pub fn finish_start_tag(out: &mut String) {
    out.push('>');
}

pub fn finish_void_tag(out: &mut String, tag: &str) {
    assert_valid_void_tag_name(tag);
    out.push('>');
}

// ponytail: static tag — skip assert_valid_tag_name (already checked by push_start_tag_static),
//           but still assert_valid_void_tag_name: the macro doesn't know which tags are void
pub fn finish_void_tag_static(out: &mut String, tag: &str) {
    assert_valid_void_tag_name(tag);
    out.push('>');
}

pub fn push_end_tag(out: &mut String, tag: &str) {
    assert_valid_tag_name(tag);
    out.push_str("</");
    out.push_str(tag);
    out.push('>');
}

// ponytail: static tag — macro already validated name at compile time, skip runtime assert
#[inline]
pub fn push_end_tag_static(out: &mut String, tag: &str) {
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

// ponytail: static attr name validated by macro, skip assert_valid_attr_name
#[inline]
pub fn push_static_attr<V>(out: &mut String, name: &str, value: &V)
where
    V: AttributeValue + ?Sized,
{
    value.render_static_attr_into(out, name);
}

pub fn push_bool_attr<N>(out: &mut String, name: &N)
where
    N: AttributeName + ?Sized,
{
    name.with_attr_name(&mut |name| push_boolean_attr(out, name));
}

// ponytail: static attr name validated by macro, skip assert_valid_attr_name
#[inline]
pub fn push_static_bool_attr(out: &mut String, name: &str) {
    out.push(' ');
    out.push_str(name);
}

pub fn push_attrs<A>(out: &mut String, attrs: &A)
where
    A: Attributes + ?Sized,
{
    attrs.render_attrs_into(out);
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

    // If the value rendered nothing, roll back the space separator that was added above.
    if classes.len() == value_start {
        classes.truncate(original_len);
    }
}

#[inline]
pub fn push_class_attr(out: &mut String, classes: &str) {
    if !classes.is_empty() {
        // ponytail: "class" is always a valid attr name, skip assert_valid_attr_name
        out.push_str(" class=\"");
        escape_attr_value_into(classes, out);
        out.push('"');
    }
}

/// Writes one class value with attribute escaping directly into `out`.
///
/// Returns `true` when something was written (for the caller to decide whether
/// to write a space separator before the next value).
#[doc(hidden)]
#[inline]
pub fn push_class_direct<V>(out: &mut String, value: &V) -> bool
where
    V: ClassValue + ?Sized,
{
    let start = out.len();
    value.render_class_attr_into(out);
    out.len() != start
}

