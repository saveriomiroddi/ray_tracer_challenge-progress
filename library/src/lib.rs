mod color;
mod sdl2_interface;
mod tuple;

pub use color::Color;
pub use sdl2_interface::Sdl2Interface;
pub use tuple::Tuple;

#[cfg(test)]
#[macro_use]
extern crate assert_float_eq;

#[cfg(test)]
mod tuple_test;

#[cfg(test)]
mod color_test;
