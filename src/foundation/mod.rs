use std;
use objc;
use super::ObjectiveC;
use cocoa::*;

#[link(name = "Foundation", kind = "framework")]
extern {}

pub trait NSObject : ObjectiveC {
  fn is_equal<T5: 'static + NSObject>(self, view: T5) -> bool where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(isEqual:), (view.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: bool = r;

          return result;
        }
      }
    }
  }

  fn hash(&self) -> NSUInteger where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(hash), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn description(&self) -> NSStringID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(description), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: NSStringID = r;

          return r.retain();
        }
      }
    }
  }

  fn debug_description(&self) -> NSStringID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(debugDescription), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: NSStringID = r;

          return r.retain();
        }
      }
    }
  }

  fn is_proxy(self) -> bool where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(isProxy), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: bool = r;

          return result;
        }
      }
    }
  }
}

pub struct NSObjectID(*mut std::os::raw::c_void);

impl NSObjectID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return NSObjectID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return NSObjectID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return NSObjectID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for NSObjectID {}

impl Clone for NSObjectID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for NSObjectID {
  fn drop(&mut self) {
    if !self.is_nil() {
      self.release();
    }
  }
}

impl ObjectiveC for NSObjectID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return NSObjectID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for NSObjectID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}
