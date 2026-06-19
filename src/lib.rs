#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

extern crate self as mup;

mod attrs;
mod html;
mod macros;
mod markup;
mod render;

#[doc(hidden)]
pub mod template;

pub use attrs::{AttributeName, AttributeValue, Attributes, ClassValue};
pub use macros::{component, markup};
pub use markup::Markup;
pub use render::Render;

#[cfg(test)]
mod tests;
