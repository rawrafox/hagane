#![allow(unknown_lints, needless_return, zero_prefixed_literal, enum_clike_unportable_variant, len_without_is_empty, expl_impl_clone_on_copy)]

#[macro_use] extern crate objc;

#[macro_use] pub mod macros;

pub mod cocoa;
pub use cocoa::*;

pub mod core_animation;
pub use core_animation::*;

pub mod core_graphics;
pub use core_graphics::*;

pub mod metal;
pub use metal::*;

pub mod metal_kit;
pub use metal_kit::*;

pub trait ObjectiveC {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self;

  #[inline] fn ptr_to_self(&self) -> *mut std::os::raw::c_void;
}
