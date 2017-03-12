#![allow(non_upper_case_globals)]
use std;
use objc;
use hagane_objc::ObjectiveC;

use foundation::*;
use metal::*;

pub trait CAMetalDrawable : MTLDrawable + NSObject {
}

#[repr(C)] pub struct CAMetalDrawableID(*mut std::os::raw::c_void);

impl CAMetalDrawableID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return CAMetalDrawableID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return CAMetalDrawableID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return CAMetalDrawableID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl MTLDrawable for CAMetalDrawableID {}
impl NSObject for CAMetalDrawableID {}
impl CAMetalDrawable for CAMetalDrawableID {}

impl Clone for CAMetalDrawableID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Default for CAMetalDrawableID {
  fn default() -> Self {
    return Self::nil();
  }
}

impl Drop for CAMetalDrawableID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for CAMetalDrawableID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for CAMetalDrawableID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for CAMetalDrawableID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}
