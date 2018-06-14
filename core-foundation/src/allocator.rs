use crate::*;

pub type CFAllocatorRetainCallBack = unsafe extern fn(info: *const c_void) -> *const c_void;
pub type CFAllocatorReleaseCallBack = unsafe extern fn(info: *const c_void);
pub type CFAllocatorCopyDescriptionCallBack = unsafe extern fn(info: *const c_void) -> CFStringRef;
pub type CFAllocatorAllocateCallBack = unsafe extern fn(allocSize: CFIndex, hint: CFOptionFlags, info: *const c_void) -> *mut c_void;
pub type CFAllocatorReallocateCallBack = unsafe extern fn(ptr: *mut c_void, newsize: CFIndex, hint: CFOptionFlags, info: *mut c_void) -> *mut c_void;
pub type CFAllocatorDeallocateCallBack = unsafe extern fn(ptr: *mut c_void, info: *const c_void);
pub type CFAllocatorPreferredSizeCallBack = unsafe extern fn(size: CFIndex, hint: CFOptionFlags, info: *const c_void) -> CFIndex;

#[repr(C)] pub struct CFAllocatorContext {
  version: CFIndex,
  info: *mut c_void,
  retain: CFAllocatorRetainCallBack,
  release: CFAllocatorReleaseCallBack,
  copyDescription: CFAllocatorCopyDescriptionCallBack,
  allocate: CFAllocatorAllocateCallBack,
  reallocate: CFAllocatorReallocateCallBack,
  deallocate: CFAllocatorDeallocateCallBack,
  preferredSize: CFAllocatorPreferredSizeCallBack
}

mod ext {
  use crate::*;

  extern {
    pub fn CFAllocatorGetTypeID() -> CFTypeID;
    pub fn CFAllocatorSetDefault(allocator: CFAllocatorRef);
    pub fn CFAllocatorGetDefault() -> CFAllocatorRef;
    pub fn CFAllocatorCreate(allocator: CFAllocatorRef, context: *mut CFAllocatorContext) -> CFAllocatorRef;
    pub fn CFAllocatorAllocate(allocator: CFAllocatorRef, size: CFIndex, hint: CFOptionFlags) -> *mut c_void;
    pub fn CFAllocatorReallocate(allocator: CFAllocatorRef, ptr: *mut c_void, newsize: CFIndex, hint: CFOptionFlags) -> *mut c_void;
    pub fn CFAllocatorDeallocate(allocator: CFAllocatorRef, ptr: *mut c_void);
    pub fn CFAllocatorGetPreferredSizeForSize(allocator: CFAllocatorRef, size: CFIndex, hint: CFOptionFlags) -> CFIndex;
    pub fn CFAllocatorGetContext(allocator: CFAllocatorRef, context: *mut CFAllocatorContext);

    pub static kCFAllocatorDefault: CFAllocatorRef;
    pub static kCFAllocatorSystemDefault: CFAllocatorRef;
    pub static kCFAllocatorMalloc: CFAllocatorRef;
    pub static kCFAllocatorMallocZone: CFAllocatorRef;
    pub static kCFAllocatorNull: CFAllocatorRef;
    pub static kCFAllocatorUseContext: CFAllocatorRef;
  }
}

#[repr(transparent)] pub struct CFAllocatorRef(usize);

unsafe impl Subtype<CFTypeRef> for CFAllocatorRef {
  unsafe fn upcast(&self) -> CFTypeRef {
    return CFTypeRef(self.0);
  }
}

unsafe impl Subtype<CFAllocatorRef> for CFAllocatorRef {
  unsafe fn upcast(&self) -> CFAllocatorRef {
    return CFAllocatorRef(self.0);
  }
}

pub fn CFAllocatorGetTypeID() -> CFTypeID {
  return unsafe { ext::CFAllocatorGetTypeID() };
}

pub unsafe fn CFAllocatorSetDefault<T: Subtype<CFAllocatorRef>>(allocator: T) {
  return ext::CFAllocatorSetDefault(allocator.upcast());
}

pub fn CFAllocatorGetDefault() -> CFAllocatorRef {
  return unsafe { ext::CFAllocatorGetDefault() };
}

pub unsafe fn CFAllocatorCreate<T: Subtype<CFAllocatorRef>>(allocator: T, context: *mut CFAllocatorContext) -> CFAllocatorRef {
  return ext::CFAllocatorCreate(allocator.upcast(), context);
}

pub unsafe fn CFAllocatorAllocate<T: Subtype<CFAllocatorRef>>(allocator: T, size: CFIndex, hint: CFOptionFlags) -> *mut c_void {
  return ext::CFAllocatorAllocate(allocator.upcast(), size, hint);
}

pub unsafe fn CFAllocatorReallocate<T: Subtype<CFAllocatorRef>>(allocator: T, ptr: *mut c_void, newsize: CFIndex, hint: CFOptionFlags) -> *mut c_void {
  return ext::CFAllocatorReallocate(allocator.upcast(), ptr, newsize, hint);
}

pub unsafe fn CFAllocatorDeallocate<T: Subtype<CFAllocatorRef>>(allocator: T, ptr: *mut c_void) {
  return ext::CFAllocatorDeallocate(allocator.upcast(), ptr);
}

pub fn CFAllocatorGetPreferredSizeForSize<T: Subtype<CFAllocatorRef>>(allocator: T, size: CFIndex, hint: CFOptionFlags) -> CFIndex {
  return unsafe { ext::CFAllocatorGetPreferredSizeForSize(allocator.upcast(), size, hint) };
}

pub unsafe fn CFAllocatorGetContext<T: Subtype<CFAllocatorRef>>(allocator: T, context: *mut CFAllocatorContext) {
  return ext::CFAllocatorGetContext(allocator.upcast(), context);
}

pub static kCFAllocatorDefault: &'static CFAllocatorRef = unsafe { &ext::kCFAllocatorDefault };
pub static kCFAllocatorSystemDefault: &'static CFAllocatorRef = unsafe { &ext::kCFAllocatorSystemDefault };
pub static kCFAllocatorMalloc: &'static CFAllocatorRef = unsafe { &ext::kCFAllocatorMalloc };
pub static kCFAllocatorMallocZone: &'static CFAllocatorRef = unsafe { &ext::kCFAllocatorMallocZone };
pub static kCFAllocatorNull: &'static CFAllocatorRef = unsafe { &ext::kCFAllocatorNull };
pub static kCFAllocatorUseContext: &'static CFAllocatorRef = unsafe { &ext::kCFAllocatorUseContext };
