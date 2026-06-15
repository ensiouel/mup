use crate::Markup;
use crate::html::{escape_text_into, push_display};
use std::borrow::Cow;

pub trait Render {
    fn render(&self, children: Option<Markup>) -> Markup;
}

impl<T: Render + ?Sized> Render for &T {
    fn render(&self, children: Option<Markup>) -> Markup {
        (*self).render(children)
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
}

impl Render for str {
    fn render(&self, _children: Option<Markup>) -> Markup {
        let mut out = String::new();
        escape_text_into(self, &mut out);
        Markup::raw(out)
    }
}

impl Render for String {
    fn render(&self, children: Option<Markup>) -> Markup {
        self.as_str().render(children)
    }
}

impl Render for Cow<'_, str> {
    fn render(&self, children: Option<Markup>) -> Markup {
        self.as_ref().render(children)
    }
}

impl<T: Render> Render for Option<T> {
    fn render(&self, children: Option<Markup>) -> Markup {
        if let Some(value) = self.as_ref() {
            value.render(children)
        } else {
            Markup::new()
        }
    }
}

impl Render for char {
    fn render(&self, _children: Option<Markup>) -> Markup {
        let mut buffer = [0; 4];
        let mut out = String::new();
        escape_text_into(self.encode_utf8(&mut buffer), &mut out);
        Markup::raw(out)
    }
}

macro_rules! impl_display_render {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Render for $ty {
                fn render(&self, _children: Option<Markup>) -> Markup {
                    render_display(self)
                }
            }
        )*
    };
}

impl_display_render!(
    bool, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);

fn render_display(value: &impl std::fmt::Display) -> Markup {
    let mut out = String::new();
    push_display(&mut out, value);
    Markup::raw(out)
}
