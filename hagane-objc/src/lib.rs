#[macro_use] extern crate objc;

pub trait ObjectiveC {
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

  #[inline] fn retain_count(&self) -> usize where Self: 'static + Sized {
    return unsafe { msg_send![self.as_object(), retainCount] };
  }

  #[inline] unsafe fn release(&self) where Self: 'static + Sized {
    msg_send![self.as_object(), release];
  }
}
