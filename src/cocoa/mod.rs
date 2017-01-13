#![allow(non_upper_case_globals)]
use std;
use objc;
use super::ObjectiveC;

use core_graphics::*;
use foundation::*;

pub type NSRect = CGRect;

#[link(name = "Cocoa", kind = "framework")]
extern {}

pub trait NSApplication : NSObject {
  fn run(&self) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(run), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn terminate<T0: 'static + ObjectiveC>(&self, sender: T0) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(terminate:), (sender.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }
}

pub struct NSApplicationID(*mut std::os::raw::c_void);

impl NSApplicationID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return NSApplicationID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return NSApplicationID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return NSApplicationID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("NSApplication").unwrap();
  }

  pub fn shared_application() -> Self {
    unsafe {
      return msg_send![NSApplicationID::class(), sharedApplication];
    }
  }
}

impl NSObject for NSApplicationID {}
impl NSApplication for NSApplicationID {}

impl Clone for NSApplicationID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for NSApplicationID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for NSApplicationID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return NSApplicationID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }

  fn as_mut_ptr(&mut self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for NSApplicationID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for NSApplicationID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait NSView : NSObject {
}

pub struct NSViewID(*mut std::os::raw::c_void);

impl NSViewID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return NSViewID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return NSViewID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return NSViewID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("NSView").unwrap();
  }
}

impl NSObject for NSViewID {}
impl NSView for NSViewID {}

impl Clone for NSViewID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for NSViewID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for NSViewID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return NSViewID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }

  fn as_mut_ptr(&mut self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for NSViewID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for NSViewID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait NSWindow : NSObject {
  fn init_with_content_rect_style_mask_backing_defer(self, content_rect: NSRect, style: usize, buffering_type: usize, flag: bool) -> Self where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(initWithContentRect:styleMask:backing:defer:), (content_rect, style, buffering_type, flag)) {
        Err(s) => panic!("{}", s),
        Ok(result) => {
          std::mem::forget(self);

          return result;
        }
      }
    }
  }

  fn make_key_and_order_front<T0: 'static + ObjectiveC>(&self, sender: T0) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(makeKeyAndOrderFront:), (sender.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn content_view(&self) -> NSViewID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(contentView), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: NSViewID = r;

          return r.retain();
        }
      }
    }
  }

  fn set_content_view<T: 'static + ObjectiveC + NSView>(&self, content_view: T) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setContentView:), (content_view.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn delegate(&self) -> NSWindowDelegateID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(delegate), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: NSWindowDelegateID = r;

          return r.retain();
        }
      }
    }
  }

  fn set_delegate<T: 'static + ObjectiveC + NSWindowDelegate>(&self, delegate: T) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setDelegate:), (delegate.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn title(&self) -> NSStringID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(title), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: NSStringID = r;

          return r.retain();
        }
      }
    }
  }

  fn set_title<T: 'static + ObjectiveC + NSString>(&self, title: T) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setTitle:), (title.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }
}

pub struct NSWindowID(*mut std::os::raw::c_void);

impl NSWindowID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return NSWindowID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return NSWindowID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return NSWindowID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("NSWindow").unwrap();
  }
}

impl NSObject for NSWindowID {}
impl NSWindow for NSWindowID {}

impl Clone for NSWindowID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for NSWindowID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for NSWindowID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return NSWindowID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }

  fn as_mut_ptr(&mut self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for NSWindowID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for NSWindowID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait NSWindowDelegate : NSObject {
}

pub struct NSWindowDelegateID(*mut std::os::raw::c_void);

impl NSWindowDelegateID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return NSWindowDelegateID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return NSWindowDelegateID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return NSWindowDelegateID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("NSWindowDelegate").unwrap();
  }
}

impl NSObject for NSWindowDelegateID {}
impl NSWindowDelegate for NSWindowDelegateID {}

impl Clone for NSWindowDelegateID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for NSWindowDelegateID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for NSWindowDelegateID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return NSWindowDelegateID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }

  fn as_mut_ptr(&mut self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for NSWindowDelegateID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for NSWindowDelegateID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}
