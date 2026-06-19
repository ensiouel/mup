use crate::Markup;
use crate::html::{assert_valid_attr_name, escape_attr_value_into, push_display};
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::hash::BuildHasher;

/// Provides one or more HTML attribute names.
///
/// `Option<T>` skips the attribute name when it is `None`.
pub trait AttributeName {
    /// Calls `f` for every attribute name represented by this value.
    fn with_attr_name(&self, f: &mut dyn FnMut(&str));
}

impl<T: AttributeName + ?Sized> AttributeName for &T {
    fn with_attr_name(&self, f: &mut dyn FnMut(&str)) {
        (*self).with_attr_name(f);
    }
}

impl AttributeName for str {
    fn with_attr_name(&self, f: &mut dyn FnMut(&str)) {
        f(self);
    }
}

impl AttributeName for String {
    fn with_attr_name(&self, f: &mut dyn FnMut(&str)) {
        self.as_str().with_attr_name(f);
    }
}

impl AttributeName for Cow<'_, str> {
    fn with_attr_name(&self, f: &mut dyn FnMut(&str)) {
        self.as_ref().with_attr_name(f);
    }
}

impl<T: AttributeName> AttributeName for Option<T> {
    fn with_attr_name(&self, f: &mut dyn FnMut(&str)) {
        if let Some(value) = self.as_ref() {
            value.with_attr_name(f);
        }
    }
}

/// Renders a Rust value as an HTML attribute value.
///
/// Strings and `Markup` are escaped for attribute context, booleans render as
/// boolean attributes, and `Option<T>` skips the attribute when it is `None`.
pub trait AttributeValue {
    /// Appends this value as attribute `name` into `out`.
    fn render_attr_into(&self, out: &mut String, name: &str);

    #[doc(hidden)]
    fn render_static_attr_into(&self, out: &mut String, name: &str) {
        self.render_attr_into(out, name);
    }
}

impl<T: AttributeValue + ?Sized> AttributeValue for &T {
    fn render_attr_into(&self, out: &mut String, name: &str) {
        (*self).render_attr_into(out, name);
    }
    fn render_static_attr_into(&self, out: &mut String, name: &str) {
        (*self).render_static_attr_into(out, name);
    }
}

impl AttributeValue for bool {
    fn render_attr_into(&self, out: &mut String, name: &str) {
        if *self {
            push_boolean_attr(out, name);
        }
    }
    fn render_static_attr_into(&self, out: &mut String, name: &str) {
        if *self {
            out.push(' ');
            out.push_str(name);
        }
    }
}

impl<T: AttributeValue> AttributeValue for Option<T> {
    fn render_attr_into(&self, out: &mut String, name: &str) {
        if let Some(value) = self.as_ref() {
            value.render_attr_into(out, name);
        }
    }
    fn render_static_attr_into(&self, out: &mut String, name: &str) {
        if let Some(value) = self.as_ref() {
            value.render_static_attr_into(out, name);
        }
    }
}

impl AttributeValue for Markup {
    fn render_attr_into(&self, out: &mut String, name: &str) {
        push_str_attr(out, name, self.as_str());
    }
    fn render_static_attr_into(&self, out: &mut String, name: &str) {
        push_str_attr_unchecked(out, name, self.as_str());
    }
}

impl AttributeValue for str {
    fn render_attr_into(&self, out: &mut String, name: &str) {
        push_str_attr(out, name, self);
    }
    #[inline]
    fn render_static_attr_into(&self, out: &mut String, name: &str) {
        push_str_attr_unchecked(out, name, self);
    }
}

impl AttributeValue for String {
    fn render_attr_into(&self, out: &mut String, name: &str) {
        self.as_str().render_attr_into(out, name);
    }
    fn render_static_attr_into(&self, out: &mut String, name: &str) {
        push_str_attr_unchecked(out, name, self);
    }
}

impl AttributeValue for Cow<'_, str> {
    fn render_attr_into(&self, out: &mut String, name: &str) {
        self.as_ref().render_attr_into(out, name);
    }
    fn render_static_attr_into(&self, out: &mut String, name: &str) {
        push_str_attr_unchecked(out, name, self.as_ref());
    }
}

impl AttributeValue for char {
    fn render_attr_into(&self, out: &mut String, name: &str) {
        let mut buffer = [0; 4];
        push_str_attr(out, name, self.encode_utf8(&mut buffer));
    }
    fn render_static_attr_into(&self, out: &mut String, name: &str) {
        let mut buffer = [0; 4];
        push_str_attr_unchecked(out, name, self.encode_utf8(&mut buffer));
    }
}

macro_rules! impl_display_attr_value {
    ($($ty:ty),* $(,)?) => {
        $(
            impl AttributeValue for $ty {
                fn render_attr_into(&self, out: &mut String, name: &str) {
                    push_display_attr(out, name, self);
                }
                fn render_static_attr_into(&self, out: &mut String, name: &str) {
                    push_display_attr_unchecked(out, name, self);
                }
            }
        )*
    };
}

impl_display_attr_value!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);

/// Renders a Rust value into a `class` attribute segment.
///
/// `Option<T>` skips missing classes. Empty rendered segments are omitted.
pub trait ClassValue {
    /// Appends this class segment into `out`.
    fn render_class_into(&self, out: &mut String);

    /// Appends this class segment into `out` with HTML attribute escaping.
    ///
    /// Used when writing directly into the output buffer to skip the intermediate class String.
    #[doc(hidden)]
    fn render_class_attr_into(&self, out: &mut String) {
        let mut buf = String::new();
        self.render_class_into(&mut buf);
        escape_attr_value_into(&buf, out);
    }
}

impl<T: ClassValue + ?Sized> ClassValue for &T {
    fn render_class_into(&self, out: &mut String) {
        (*self).render_class_into(out);
    }
    fn render_class_attr_into(&self, out: &mut String) {
        (*self).render_class_attr_into(out);
    }
}

impl<T: ClassValue> ClassValue for Option<T> {
    fn render_class_into(&self, out: &mut String) {
        if let Some(value) = self.as_ref() {
            value.render_class_into(out);
        }
    }
    fn render_class_attr_into(&self, out: &mut String) {
        if let Some(value) = self.as_ref() {
            value.render_class_attr_into(out);
        }
    }
}

impl ClassValue for Markup {
    fn render_class_into(&self, out: &mut String) {
        out.push_str(self.as_str());
    }
    fn render_class_attr_into(&self, out: &mut String) {
        escape_attr_value_into(self.as_str(), out);
    }
}

impl ClassValue for str {
    fn render_class_into(&self, out: &mut String) {
        out.push_str(self);
    }
    #[inline]
    fn render_class_attr_into(&self, out: &mut String) {
        escape_attr_value_into(self, out);
    }
}

impl ClassValue for String {
    fn render_class_into(&self, out: &mut String) {
        self.as_str().render_class_into(out);
    }
    fn render_class_attr_into(&self, out: &mut String) {
        escape_attr_value_into(self, out);
    }
}

impl ClassValue for Cow<'_, str> {
    fn render_class_into(&self, out: &mut String) {
        self.as_ref().render_class_into(out);
    }
    fn render_class_attr_into(&self, out: &mut String) {
        escape_attr_value_into(self.as_ref(), out);
    }
}

impl ClassValue for char {
    fn render_class_into(&self, out: &mut String) {
        out.push(*self);
    }
    fn render_class_attr_into(&self, out: &mut String) {
        let mut buf = [0; 4];
        escape_attr_value_into(self.encode_utf8(&mut buf), out);
    }
}

impl ClassValue for bool {
    fn render_class_into(&self, out: &mut String) {
        push_display(out, self);
    }
}

macro_rules! impl_display_class_value {
    ($($ty:ty),* $(,)?) => {
        $(
            impl ClassValue for $ty {
                fn render_class_into(&self, out: &mut String) {
                    push_display(out, self);
                }
            }
        )*
    };
}

impl_display_class_value!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);

/// Renders a collection of HTML attributes.
///
/// Implemented for maps, arrays/slices of pairs, and arrays/slices of boolean
/// attribute names.
pub trait Attributes {
    /// Appends all represented attributes into `out`.
    fn render_attrs_into(&self, out: &mut String);
}

impl<T: Attributes + ?Sized> Attributes for &T {
    fn render_attrs_into(&self, out: &mut String) {
        (*self).render_attrs_into(out);
    }
}

macro_rules! impl_boolean_attrs {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Attributes for [$ty] {
                fn render_attrs_into(&self, out: &mut String) {
                    render_boolean_attrs(out, self);
                }
            }

            impl<const N: usize> Attributes for [$ty; N] {
                fn render_attrs_into(&self, out: &mut String) {
                    self.as_slice().render_attrs_into(out);
                }
            }

            impl Attributes for Vec<$ty> {
                fn render_attrs_into(&self, out: &mut String) {
                    self.as_slice().render_attrs_into(out);
                }
            }
        )*
    };
}

impl_boolean_attrs!(&str, String, Cow<'_, str>);

impl<K, V, S> Attributes for HashMap<K, V, S>
where
    K: AttributeName,
    V: AttributeValue,
    S: BuildHasher,
{
    fn render_attrs_into(&self, out: &mut String) {
        render_attr_pairs(out, self);
    }
}

impl<K, V> Attributes for BTreeMap<K, V>
where
    K: AttributeName + Ord,
    V: AttributeValue,
{
    fn render_attrs_into(&self, out: &mut String) {
        render_attr_pairs(out, self);
    }
}

impl<K, V> Attributes for (K, V)
where
    K: AttributeName,
    V: AttributeValue,
{
    fn render_attrs_into(&self, out: &mut String) {
        crate::template::push_attr(out, &self.0, &self.1);
    }
}

impl<K, V> Attributes for [(K, V)]
where
    K: AttributeName,
    V: AttributeValue,
{
    fn render_attrs_into(&self, out: &mut String) {
        render_attr_pairs(out, self.iter().map(|(name, value)| (name, value)));
    }
}

impl<K, V, const N: usize> Attributes for [(K, V); N]
where
    K: AttributeName,
    V: AttributeValue,
{
    fn render_attrs_into(&self, out: &mut String) {
        self.as_slice().render_attrs_into(out);
    }
}

impl<K, V> Attributes for Vec<(K, V)>
where
    K: AttributeName,
    V: AttributeValue,
{
    fn render_attrs_into(&self, out: &mut String) {
        self.as_slice().render_attrs_into(out);
    }
}

fn render_attr_pairs<'a, K, V>(out: &mut String, attrs: impl IntoIterator<Item = (&'a K, &'a V)>)
where
    K: AttributeName + 'a,
    V: AttributeValue + 'a,
{
    for (name, value) in attrs {
        crate::template::push_attr(out, name, value);
    }
}

fn render_boolean_attrs<'a, N>(out: &mut String, names: impl IntoIterator<Item = &'a N>)
where
    N: AttributeName + 'a,
{
    for name in names {
        crate::template::push_bool_attr(out, name);
    }
}

fn push_attr_prefix(out: &mut String, name: &str) {
    assert_valid_attr_name(name);
    out.push(' ');
    out.push_str(name);
    out.push_str("=\"");
}

pub(crate) fn push_str_attr(out: &mut String, name: &str, value: &str) {
    push_attr_prefix(out, name);
    escape_attr_value_into(value, out);
    out.push('"');
}

// ponytail: macro-validated static attr name, skip assert_valid_attr_name
#[inline]
pub(crate) fn push_str_attr_unchecked(out: &mut String, name: &str, value: &str) {
    out.push(' ');
    out.push_str(name);
    out.push_str("=\"");
    escape_attr_value_into(value, out);
    out.push('"');
}

// Numeric Display output cannot contain HTML special chars, so no escaping is needed here.
fn push_display_attr(out: &mut String, name: &str, value: &impl fmt::Display) {
    push_attr_prefix(out, name);
    push_display(out, value);
    out.push('"');
}

// ponytail: macro-validated static attr name, skip assert_valid_attr_name
#[inline]
fn push_display_attr_unchecked(out: &mut String, name: &str, value: &impl fmt::Display) {
    out.push(' ');
    out.push_str(name);
    out.push_str("=\"");
    push_display(out, value);
    out.push('"');
}

pub(crate) fn push_boolean_attr(out: &mut String, name: &str) {
    assert_valid_attr_name(name);
    out.push(' ');
    out.push_str(name);
}
