#![feature(crate_in_paths)]
#![feature(repr_transparent)]

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::mem;
use std::os::raw::c_void;

pub unsafe trait Subtype<T> {
  fn upcast(&self) -> T;
}

#[derive(Debug, Eq, PartialEq)]
#[repr(u8)] pub enum Boolean {
  TRUE = 1,
  FALSE = 0
}

pub type Byte = u8;
pub type SignedByte = i8;

pub type UInt8 = u8;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub type SInt8 = i8;
pub type SInt16 = i16;
pub type SInt32 = i32;
pub type SInt64 = i64;

pub type Float32 = f32;
pub type Float64 = f64;

pub type UniChar = u16;
pub type UniCharCount = u64;
pub type StringPtr = *mut u8;
pub type ConstStringPtr = *const u8;
pub type Str255 = [u8; 256];
pub type ConstStr255Param = *const u8;

pub type UTF8Char = u8;
pub type UTF16Char = u16;
pub type UTF32Char = u32;

pub type CFIndex = i64;

#[repr(transparent)] pub struct OSStatus(i32);
#[repr(transparent)] pub struct OSErr(i16);
#[repr(transparent)] pub struct RegionCode(i16);
#[repr(transparent)] pub struct LangCode(i16);
#[repr(transparent)] pub struct ScriptCode(i16);
#[repr(transparent)] pub struct FourCharCode(u32);
#[repr(transparent)] pub struct OSType(u32);

#[repr(transparent)] #[derive(PartialEq, Eq, Debug)] pub struct CFTypeID(u64);
#[repr(transparent)] pub struct CFOptionFlags(u64);
#[repr(transparent)] pub struct CFHashCode(u64);

#[repr(transparent)] pub struct CFTypeRef(usize);

unsafe impl Subtype<CFTypeRef> for CFTypeRef {
  fn upcast(&self) -> CFTypeRef {
    return CFTypeRef(self.0);
  }
}

#[repr(transparent)] pub struct CFStringRef(usize);

pub type CFMutableStringRef = CFStringRef;

#[repr(transparent)] pub struct CFPropertyListRef(usize);

#[repr(i64)]
pub enum CFComparisonResult {
  kCFCompareLessThan = -1,
  kCFCompareEqualTo = 0,
  kCFCompareGreaterThan = 1
}

pub type CFComparatorFunction = unsafe extern fn(val1: *const c_void, val2: *const c_void, context: *mut c_void) -> CFComparisonResult;

pub const kCFNotFound: CFIndex = -1;

pub struct CFRange {
  location: CFIndex,
  length: CFIndex
}

pub fn CFRangeMake(loc: CFIndex, len: CFIndex) -> CFRange {
  return CFRange { location: loc, length: len };
}

#[repr(transparent)] pub struct CFNullRef(usize);

unsafe impl Subtype<CFTypeRef> for CFNullRef {
  fn upcast(&self) -> CFTypeRef {
    return CFTypeRef(self.0);
  }
}

unsafe impl Subtype<CFNullRef> for CFNullRef {
  fn upcast(&self) -> CFNullRef {
    return CFNullRef(self.0);
  }
}

#[repr(transparent)] pub struct CFAllocatorRef(usize);

unsafe impl Subtype<CFTypeRef> for CFAllocatorRef {
  fn upcast(&self) -> CFTypeRef {
    return CFTypeRef(self.0);
  }
}

unsafe impl Subtype<CFAllocatorRef> for CFAllocatorRef {
  fn upcast(&self) -> CFAllocatorRef {
    return CFAllocatorRef(self.0);
  }
}
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
  
  #[link(name = "CoreFoundation", kind = "framework")]
  extern {
    pub fn CFNullGetTypeID() -> CFTypeID;

    pub static kCFNull: CFNullRef;

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

    pub fn CFGetTypeID(cf: CFTypeRef) -> CFTypeID;
    pub fn CFCopyTypeIDDescription(type_id: CFTypeID) -> CFStringRef;
    pub fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    pub fn CFRelease(cf: CFTypeRef);
    pub fn CFAutorelease(arg: CFTypeRef) -> CFTypeRef;
    pub fn CFGetRetainCount(cf: CFTypeRef) -> CFIndex;
    pub fn CFEqual(cf1: CFTypeRef, cf2: CFTypeRef) -> Boolean;
    pub fn CFHash(cf: CFTypeRef) -> CFHashCode;
    pub fn CFCopyDescription(cf: CFTypeRef) -> CFStringRef;
    pub fn CFGetAllocator(cf: CFTypeRef) -> CFAllocatorRef;
  }
}

pub fn CFNullGetTypeID() -> CFTypeID {
  return unsafe { ext::CFNullGetTypeID() };
}

pub static kCFNull: &'static CFNullRef = unsafe { &ext::kCFNull };

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

pub fn CFGetTypeID<T: Subtype<CFTypeRef>>(cf: &T) -> CFTypeID {
  return unsafe { ext::CFGetTypeID(cf.upcast()) };
}

pub fn CFCopyTypeIDDescription(type_id: CFTypeID) -> CFStringRef {
  return unsafe { ext::CFCopyTypeIDDescription(type_id) };
}

pub unsafe fn CFRetain<T: Subtype<CFTypeRef>>(cf: T) -> T {
  mem::forget(ext::CFRetain(cf.upcast()));

  return cf;
}

pub unsafe fn CFRelease<T: Subtype<CFTypeRef>>(cf: T) {
  ext::CFRelease(cf.upcast());
}

pub unsafe fn CFAutorelease<T: Subtype<CFTypeRef>>(cf: T) {
  ext::CFAutorelease(cf.upcast());
}

pub fn CFGetRetainCount<T: Subtype<CFTypeRef>>(cf: &T) -> CFIndex {
  return unsafe { ext::CFGetRetainCount(cf.upcast()) };
}

pub fn CFEqual<T1: Subtype<CFTypeRef>, T2: Subtype<CFTypeRef>>(cf1: &T1, cf2: &T2) -> Boolean {
  return unsafe { ext::CFEqual(cf1.upcast(), cf2.upcast()) };
}

pub fn CFHash<T: Subtype<CFTypeRef>>(cf: &T) -> CFHashCode {
  return unsafe { ext::CFHash(cf.upcast()) };
}

pub fn CFCopyDescription<T: Subtype<CFTypeRef>>(cf: &T) -> CFStringRef {
  return unsafe { ext::CFCopyDescription(cf.upcast()) };
}

pub fn CFGetAllocator<T: Subtype<CFTypeRef>>(cf: &T) -> CFAllocatorRef {
  return unsafe { CFRetain(ext::CFGetAllocator(cf.upcast())) };
}

pub trait CFTypeClass : Subtype<CFTypeRef> + Sized {
  fn get_type_id(&self) -> CFTypeID {
    return CFGetTypeID(self);
  }
}

impl<T> CFTypeClass for T where T: Subtype<CFTypeRef> { }

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn it_nulls() {
    assert_eq!(CFNullGetTypeID(), CFGetTypeID(kCFNull));
    assert_eq!(CFNullGetTypeID(), kCFNull.get_type_id());
  }

  #[test]
  fn it_compares() {
    assert_eq!(CFEqual(kCFNull, kCFAllocatorSystemDefault), Boolean::FALSE);
  }
}

