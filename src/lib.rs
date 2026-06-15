mod attrs;
mod html;
mod macros;
mod markup;
mod render;

#[doc(hidden)]
pub mod template;

pub use attrs::{AttributeName, AttributeValue, Attributes, ClassValue};
pub use markup::Markup;
pub use render::Render;

#[cfg(test)]
mod tests;
