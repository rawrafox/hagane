#[macro_use] extern crate objc;
// #[macro_use] extern crate objc_id;
// #[macro_use] extern crate objc_foundation;

use std::marker::PhantomData;
use std::os::raw::c_void;
use std::slice;
use std::str;

use objc::runtime::Object;

// pub mod core_graphics;
// pub mod kit;

#[export_macro]
macro_rules! forward {
  ($fn_name:ident, $sel:expr, ($($name:ident : $arg_type:ty),*) -> $return_type:ty) => (
    fn $fn_name(&self, $($name: $arg_type),*) -> $return_type {
      unsafe {
        let target = self.ptr_to_self() as *mut Object;

        return match objc::__send_message(target, $sel, ($($name,)*)) {
          Err(s) => panic!("{}", s),
          Ok(r) => r,
        };
      }
    }
  )
}

#[allow(non_camel_case_types)]
pub enum MTLFeatureSet {
  iOS_GPUFamily1_v1           = 00000,
  iOS_GPUFamily1_v2           = 00002,
  iOS_GPUFamily1_v3           = 00005,
  iOS_GPUFamily2_v1           = 00001,
  iOS_GPUFamily2_v2           = 00003,
  iOS_GPUFamily2_v3           = 00006,
  iOS_GPUFamily3_v1           = 00004,
  iOS_GPUFamily3_v2           = 00007,
  OSX_GPUFamily1_v1           = 10000,
  OSX_GPUFamily1_v2           = 10001,
  OSX_ReadWriteTextureTier2   = 10002,
  tvOS_GPUFamily1_v1          = 30000,
  tvOS_GPUFamily1_v2          = 30001
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MTLSize {
    pub width: usize,
    pub height: usize,
    pub depth: usize
}

#[repr(C)]
pub struct Id<T: ?Sized + 'static>(*mut c_void, PhantomData<&'static T>);

impl<T: ?Sized> Id<T> {
  pub fn new(ptr: *mut c_void) -> Self {
    return Id(ptr, PhantomData);
  }
}

pub trait NSObject {
  #[inline(always)]
  fn ptr_to_self(&self) -> *mut c_void;

  fn is_equal_to<T: NSObject>(&self, object: &T) -> bool where Self: Sized {
    unsafe {
      return msg_send!(self.ptr_to_self() as *mut Object, isEqualTo: object.ptr_to_self() as *mut Object);
    }
  }
}

impl NSObject for Id<NSObject> {
  fn ptr_to_self(&self) -> *mut c_void {
    return self.0;
  }
}

pub trait NSArray<T: NSObject + ?Sized> : NSObject {
  forward!(count, sel!(count), () -> usize);
  forward!(object_at_index, sel!(objectAtIndex:), (i: usize) -> Id<T>);

  // Rust Helpers

  forward!(len, sel!(count), () -> usize);

  fn to_vec(&self) -> Vec<Id<T>> {
    let n = self.count();
    let mut result = Vec::with_capacity(n);

    for i in 0 .. n {
      result.push(self.object_at_index(i) as Id<T>);
    }

    return result;
  }
}

impl<T: NSObject + ?Sized> NSObject for Id<NSArray<T>> {
  fn ptr_to_self(&self) -> *mut c_void {
    return self.0;
  }
}

impl<T: NSObject + ?Sized> NSArray<T> for Id<NSArray<T>> { }

pub enum NSStringEncoding {
  NSASCIIStringEncoding = 1,
  NSNEXTSTEPStringEncoding = 2,
  NSJapaneseEUCStringEncoding = 3,
  NSUTF8StringEncoding = 4,
  NSISOLatin1StringEncoding = 5,
  NSSymbolStringEncoding = 6,
  NSNonLossyASCIIStringEncoding = 7,
  NSShiftJISStringEncoding = 8,
  NSISOLatin2StringEncoding = 9,
  NSUnicodeStringEncoding = 10,
  NSWindowsCP1251StringEncoding = 11,
  NSWindowsCP1252StringEncoding = 12,
  NSWindowsCP1253StringEncoding = 13,
  NSWindowsCP1254StringEncoding = 14,
  NSWindowsCP1250StringEncoding = 15,
  NSISO2022JPStringEncoding = 21,
  NSMacOSRomanStringEncoding = 30,

  NSUTF16BigEndianStringEncoding = 0x90000100,
  NSUTF16LittleEndianStringEncoding = 0x94000100,

  NSUTF32StringEncoding = 0x8c000100,
  NSUTF32BigEndianStringEncoding = 0x98000100,
  NSUTF32LittleEndianStringEncoding = 0x9c000100
}

pub trait NSString : NSObject {
  forward!(length_of_bytes_using_encoding, sel!(lengthOfBytesUsingEncoding:), (encoding: NSStringEncoding) -> usize);
  forward!(utf8_string, sel!(UTF8String), () -> *const u8);
  
  // Rust Helpers

  fn len(&self) -> usize {
    return self.length_of_bytes_using_encoding(NSStringEncoding::NSUTF8StringEncoding);
  }

  fn as_str(&self) -> &str {
    let bytes = self.utf8_string();
    let len = self.len();

    unsafe {
      let bytes = slice::from_raw_parts(bytes, len);

      str::from_utf8(bytes).unwrap()
    }
  }
}

impl NSObject for Id<NSString> {
  fn ptr_to_self(&self) -> *mut c_void {
    return self.0;
  }
}

impl NSString for Id<NSString> { }

pub trait MTLDevice : NSObject {
  forward!(is_depth24_stencil8_pixel_format_supported, sel!(isDepth24Stencil8PixelFormatSupported), () -> bool);
  forward!(is_headless, sel!(isHeadless), () -> bool);
  forward!(is_low_power, sel!(isLowPower), () -> bool);
  forward!(max_threads_per_threadgroup, sel!(maxThreadsPerThreadgroup), () -> MTLSize);
  forward!(name, sel!(name), () -> Id<NSString>);
  forward!(recommended_max_working_set_size, sel!(recommendedMaxWorkingSetSize), () -> u64);
  forward!(supports_feature_set, sel!(supportsFeatureSet:), (feature_set: MTLFeatureSet) -> bool);
  forward!(supports_texture_sample_count, sel!(supportsTextureSampleCount:), (i: usize) -> bool);

  // Rust Helpers

  fn texture_sample_counts(&self) -> Vec<usize> {
    let mut result = Vec::new();

    for i in 1 .. 128 {
      if self.supports_texture_sample_count(i) {
        result.push(i);
      }
    }

    return result;
  }
}

impl NSObject for Id<MTLDevice> {
  fn ptr_to_self(&self) -> *mut c_void {
    return self.0;
  }
}

impl MTLDevice for Id<MTLDevice> {}

#[link(name = "Metal", kind = "framework")]
extern {
  fn MTLCopyAllDevices() -> *mut c_void;
  fn MTLCreateSystemDefaultDevice() -> *mut c_void;
}

pub fn all_devices() -> Id<NSArray<MTLDevice>> {
  unsafe {
    return Id::new(MTLCopyAllDevices());
  }
}

pub fn system_default_device() -> Id<MTLDevice> {
  unsafe {
    return Id::new(MTLCreateSystemDefaultDevice());
  }
}
