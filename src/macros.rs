#[macro_export]
macro_rules! id {
  ($id_name:ident, $trait_name:ident) => (
    #[derive(Copy, Clone)]
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

      fn ptr_to_self(&self) -> *mut std::os::raw::c_void {
        return self.0;
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
macro_rules! forward {
  ($fn_name:ident, $sel:expr, ($($name:ident : $arg_type:ty),*) -> $return_type:ty, <$($bound_name:ident : $bound_type:ident),*>) => (
    fn $fn_name<$($bound_name : $bound_type + 'static),*>(&self, $($name: $arg_type),*) -> $return_type where Self: Sized {
      unsafe {
        let target = self.ptr_to_self() as *mut objc::runtime::Object;

        return match objc::__send_message(target, $sel, ($($name,)*)) {
          Err(s) => panic!("{}", s),
          Ok(r) => r,
        };
      }
    }
  );
  ($fn_name:ident, $sel:expr, ($($name:ident : $arg_type:ty),*) -> $return_type:ty) => (
    fn $fn_name(&self, $($name: $arg_type),*) -> $return_type where Self: Sized {
      unsafe {
        let target = self.ptr_to_self() as *mut objc::runtime::Object;

        return match objc::__send_message(target, $sel, ($($name,)*)) {
          Err(s) => panic!("{}", s),
          Ok(r) => r,
        };
      }
    }
  )
}
