#![allow(unknown_lints, needless_return, zero_prefixed_literal, enum_clike_unportable_variant, len_without_is_empty, expl_impl_clone_on_copy, non_upper_case_globals)]
#![feature(repr_simd, platform_intrinsics)]

#[macro_use] extern crate bitflags;
#[macro_use] extern crate objc;

extern crate byteorder;
extern crate hagane_objc;
extern crate hagane_simd;

pub mod cocoa;
pub use cocoa::*;

pub mod core_animation;
pub use core_animation::*;

pub mod core_foundation;
pub use core_foundation::*;

pub mod core_graphics;
pub use core_graphics::*;

pub mod foundation;
pub use foundation::*;

pub mod metal;
pub use metal::*;

pub mod metal_kit;
pub use metal_kit::*;

pub mod model_io;
pub use model_io::*;

pub mod dds;
pub mod eve_rust;
pub mod rust_metal;
