#![allow(non_upper_case_globals)]
use std;
use objc;
use super::ObjectiveC;
mod initializer;

use foundation::*;
use cocoa::*;
use metal_kit::*;
pub use self::initializer::*;

pub trait RSMView : MTKView + NSView + NSObject {
}

#[repr(C)] pub struct RSMViewID(*mut std::os::raw::c_void);

impl RSMViewID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return RSMViewID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return RSMViewID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return RSMViewID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("RSMView").unwrap();
  }
}

impl MTKView for RSMViewID {}
impl NSView for RSMViewID {}
impl NSObject for RSMViewID {}
impl RSMView for RSMViewID {}

impl Clone for RSMViewID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for RSMViewID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for RSMViewID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for RSMViewID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for RSMViewID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait RSMWindowDelegate : NSWindowDelegate + NSObject {
  fn init(self) -> Self where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(init), ()) {
        Err(s) => panic!("{}", s),
        Ok(result) => {
          std::mem::forget(self);

          return result;
        }
      }
    }
  }
}

#[repr(C)] pub struct RSMWindowDelegateID(*mut std::os::raw::c_void);

impl RSMWindowDelegateID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return RSMWindowDelegateID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return RSMWindowDelegateID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return RSMWindowDelegateID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("RSMWindowDelegate").unwrap();
  }

  pub fn new() -> Self where Self: 'static + Sized {
    return RSMWindowDelegateID::alloc().init();
  }
}

impl NSWindowDelegate for RSMWindowDelegateID {}
impl NSObject for RSMWindowDelegateID {}
impl RSMWindowDelegate for RSMWindowDelegateID {}

impl Clone for RSMWindowDelegateID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for RSMWindowDelegateID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for RSMWindowDelegateID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for RSMWindowDelegateID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for RSMWindowDelegateID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}
