#[macro_export]
macro_rules! id {
  ($id_name:ident, $trait_name:ident) => (
    pub struct $id_name(*mut std::os::raw::c_void);

    impl $id_name {
      pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
        return $id_name(ptr);
      }

      pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
        return $id_name(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
      }

      pub fn nil() -> Self {
        return $id_name(0 as *mut std::os::raw::c_void);
      }

      pub fn is_nil(&self) -> bool {
        return self.0 as usize == 0;
      }
    }

    impl $crate::ObjectiveC for $id_name {
      fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
        return $id_name::from_ptr(ptr);
      }

      fn as_ptr(&self) -> *mut std::os::raw::c_void {
        return self.0;
      }
    }

    impl Clone for $id_name {
      fn clone(&self) -> Self {
        use ObjectiveC;

        let ptr = self.as_ptr();
        
        return Self::from_ptr(ptr).retain();
      }
    }

    impl Drop for $id_name {
      fn drop(&mut self) {
        use ObjectiveC;

        if !self.is_nil() {
          // println!("Releasing {}: current retain count is {}", stringify!($id_name), self.retain_count());
          self.release();
        }
      }
    }

    unsafe impl objc::Encode for $id_name {
      fn encode() -> objc::Encoding {
        return unsafe { objc::Encoding::from_str("@") };
      }
    }

    impl $trait_name for $id_name {}
  );
  ($id_name:ident, $trait_name:ident, $class:expr) => (
    id!($id_name, $trait_name);

    impl $id_name {
      pub fn alloc() -> Self {
        return unsafe { msg_send![Self::class(), alloc] };
      }

      pub fn class() -> &'static objc::runtime::Class {
        return objc::runtime::Class::get($class).unwrap();
      }
    }
  )
}

#[macro_export]
macro_rules! initializer {
  ($fn_name:ident, $sel:expr, ($($name:ident : $arg_type:ty),*), <$($bound_name:ident : $bound_type:ident),*>) => (
    fn $fn_name<$($bound_name : 'static + $bound_type),*>(self, $($name: $arg_type),*) -> Self where Self: 'static + Sized {
      use objc;

      unsafe {
        match objc::__send_message(self.as_object(), $sel, ($($name,)*)) {
          Err(s) => panic!("{}", s),
          Ok(result) => {
            std::mem::forget(self);

            return result;
          }
        };
      }
    }
  );
  ($fn_name:ident, $sel:expr, ($($name:ident : $arg_type:ty),*)) => (
    fn $fn_name(self, $($name: $arg_type),*) -> Self where Self: 'static + Sized {
      use objc;

      unsafe {
        match objc::__send_message(self.as_object(), $sel, ($($name,)*)) {
          Err(s) => panic!("{}", s),
          Ok(result) => {
            std::mem::forget(self);

            return result;
          }
        };
      }
    }
  )
}

#[macro_export]
macro_rules! forward {
  ($fn_name:ident, $sel:expr, ($($name:ident : $arg_type:ty),*) -> $return_type:ty, <$($bound_name:ident : $bound_type:ident),*>) => (
    fn $fn_name<$($bound_name : 'static + $bound_type),*>(&self, $($name: $arg_type),*) -> $return_type where Self: 'static + Sized {
      use objc;

      unsafe {
        let target = self.as_object();

        return match objc::__send_message(target, $sel, ($($name,)*)) {
          Err(s) => panic!("{}", s),
          Ok(r) => r,
        };
      }
    }
  );
  ($fn_name:ident, $sel:expr, ($($name:ident : $arg_type:ty),*) -> $return_type:ty, <$($bound_name:ident : $bound_type:ident),*>, retain) => (
    #[allow(unused_imports)]
    fn $fn_name<$($bound_name : 'static + $crate::ObjectiveC + $bound_type),*>(&self, $($name: $arg_type),*) -> $return_type where Self: 'static + Sized {
      use objc;
      use $crate::ObjectiveC;

      unsafe {
        let target = self.as_object();

        match objc::__send_message(target, $sel, ($($name,)*)) {
          Err(s) => panic!("{}", s),
          Ok(r) => {
            let r: $return_type = r;

            return r.retain();
          }
        };
      }
    }
  );
  ($fn_name:ident, $sel:expr, ($($name:ident : $arg_type:ty),*) -> $return_type:ty) => (
    fn $fn_name(&self, $($name: $arg_type),*) -> $return_type where Self: 'static + Sized {
      use objc;

      unsafe {
        let target = self.as_object();

        return match objc::__send_message(target, $sel, ($($name,)*)) {
          Err(s) => panic!("{}", s),
          Ok(r) => r,
        };
      }
    }
  );
  ($fn_name:ident, $sel:expr, ($($name:ident : $arg_type:ty),*) -> $return_type:ty, retain) => (
    #[allow(unused_imports)]
    fn $fn_name(&self, $($name: $arg_type),*) -> $return_type where Self: 'static + Sized {
      use objc;
      use $crate::ObjectiveC;

      unsafe {
        let target = self.as_object();

        match objc::__send_message(target, $sel, ($($name,)*)) {
          Err(s) => panic!("{}", s),
          Ok(r) => {
            let r: $return_type = r;

            return r.retain();
          }
        }
      }
    }
  )
}
