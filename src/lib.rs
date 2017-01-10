#![allow(unknown_lints, needless_return, zero_prefixed_literal, enum_clike_unportable_variant, len_without_is_empty, expl_impl_clone_on_copy)]

#[macro_use] extern crate bitflags;
#[macro_use] extern crate objc;

#[macro_use] mod macros;

pub mod cocoa;
pub use cocoa::*;

pub mod core_animation;
pub use core_animation::*;

pub mod core_graphics;
pub use core_graphics::*;

pub mod foundation;
pub use foundation::*;

pub mod metal;
pub use metal::*;

pub mod legacy_metal;
pub use legacy_metal::*;

pub mod metal_kit;
pub use metal_kit::*;

pub mod model_io;
pub use model_io::*;

pub mod rust_metal;
pub use rust_metal::*;

pub trait ObjectiveC {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self;

  #[inline] fn as_ptr(&self) -> *mut std::os::raw::c_void;
  

  fn as_object(&self) -> &mut objc::runtime::Object {
    let ptr = self.as_ptr() as *mut objc::runtime::Object;
    
    if ptr as usize != 0 {
      return unsafe { &mut *ptr };
    } else {
      panic!("Trying to convert nil into object")
    }
  }

  #[inline] fn retain(self) -> Self where Self: 'static + Sized {
    unsafe { msg_send![self.as_object(), retain] };

    return self;
  }

  forward!(retain_count, sel!(retainCount), () -> usize);
  forward!(release, sel!(release), () -> ());
}
