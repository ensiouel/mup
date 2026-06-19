use crate::Markup;
use crate::html::{escape_text_into, push_display};
use crate::template::TemplateBuilder;
use std::borrow::Cow;

/// Converts Rust values into [`Markup`].
///
/// Implement this trait for application types that should be renderable with
/// `@value` inside `markup!` or as components with children.
pub trait Render {
    /// Renders this value.
    ///
    /// `children` is `Some` when the value is rendered as `@value { ... }`.
    fn render(&self, children: Option<Markup>) -> Markup;

    #[doc(hidden)]
    #[inline]
    fn render_into_builder(&self, builder: &mut TemplateBuilder) {
        let markup = self.render(None);
        builder.push_markup(&markup);
    }
}

impl<T: Render + ?Sized> Render for &T {
    fn render(&self, children: Option<Markup>) -> Markup {
        (*self).render(children)
    }
    #[inline]
    fn render_into_builder(&self, builder: &mut TemplateBuilder) {
        (*self).render_into_builder(builder);
    }
}

impl Render for Markup {
    fn render(&self, children: Option<Markup>) -> Markup {
        if let Some(children) = children {
            self.with_children(children)
        } else {
            self.clone()
        }
    }
    #[inline]
    fn render_into_builder(&self, builder: &mut TemplateBuilder) {
        builder.push_markup(self);
    }
}

impl Render for str {
    fn render(&self, _children: Option<Markup>) -> Markup {
        render_escaped(self)
    }
    #[inline]
    fn render_into_builder(&self, builder: &mut TemplateBuilder) {
        escape_text_into(self, &mut builder.current);
    }
}

impl Render for String {
    fn render(&self, children: Option<Markup>) -> Markup {
        self.as_str().render(children)
    }
    #[inline]
    fn render_into_builder(&self, builder: &mut TemplateBuilder) {
        escape_text_into(self, &mut builder.current);
    }
}

impl Render for Cow<'_, str> {
    fn render(&self, children: Option<Markup>) -> Markup {
        self.as_ref().render(children)
    }
    #[inline]
    fn render_into_builder(&self, builder: &mut TemplateBuilder) {
        escape_text_into(self.as_ref(), &mut builder.current);
    }
}

impl<T: Render> Render for Option<T> {
    fn render(&self, children: Option<Markup>) -> Markup {
        match self {
            Some(value) => value.render(children),
            None => Markup::new(),
        }
    }
    #[inline]
    fn render_into_builder(&self, builder: &mut TemplateBuilder) {
        if let Some(value) = self {
            value.render_into_builder(builder);
        }
    }
}

impl Render for char {
    fn render(&self, _children: Option<Markup>) -> Markup {
        let mut buffer = [0; 4];
        render_escaped(self.encode_utf8(&mut buffer))
    }
    #[inline]
    fn render_into_builder(&self, builder: &mut TemplateBuilder) {
        let mut buffer = [0; 4];
        escape_text_into(self.encode_utf8(&mut buffer), &mut builder.current);
    }
}

macro_rules! impl_display_render {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Render for $ty {
                fn render(&self, _children: Option<Markup>) -> Markup {
                    render_display(self)
                }
                #[inline]
                fn render_into_builder(&self, builder: &mut TemplateBuilder) {
                    push_display(&mut builder.current, self);
                }
            }
        )*
    };
}

impl_display_render!(
    bool, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);

fn render_escaped(text: &str) -> Markup {
    let mut out = String::new();
    escape_text_into(text, &mut out);
    Markup::raw(out)
}

fn render_display(value: &impl std::fmt::Display) -> Markup {
    let mut out = String::new();
    push_display(&mut out, value);
    Markup::raw(out)
}
