#[macro_use] extern crate objc;
#[macro_use] extern crate objc_id;
#[macro_use] extern crate objc_foundation;

use objc_id::Id;

use objc_foundation::{NSArray, NSString};
use objc_foundation::{INSObject};

object_struct!(Device);

#[allow(non_camel_case_types)]
pub enum FeatureSet {
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
pub struct Size {
    pub width: usize,
    pub height: usize,
    pub depth: usize
}

#[link(name = "Metal", kind = "framework")]
extern {
  fn MTLCopyAllDevices() -> *mut u8;
  fn MTLCreateSystemDefaultDevice() -> *mut u8;
}

impl Device {
  pub fn all() -> Id<NSArray<Device>> {
    unsafe {
      let ptr = MTLCopyAllDevices();

      return Id::from_retained_ptr(ptr as *mut NSArray<Device>);
    }
  }

  pub fn system_default() -> Id<Self> {
    unsafe {
      let ptr = MTLCreateSystemDefaultDevice();

      return Id::from_retained_ptr(ptr as *mut Device);
    }
  }
}

pub trait IDevice : INSObject {
  fn is_depth24_stencil8_pixel_format_supported(&self) -> bool {
    return unsafe { msg_send![self, isDepth24Stencil8PixelFormatSupported] };
  }

  fn is_headless(&self) -> bool {
    return unsafe { msg_send![self, isHeadless] };
  }

  fn is_low_power(&self) -> bool {
    return unsafe { msg_send![self, isLowPower] };
  }

  fn max_threads_per_threadgroup(&self) -> Size {
    return unsafe { msg_send![self, maxThreadsPerThreadgroup] };
  }

  fn name(&self) -> Id<NSString> {
    return unsafe { Id::from_ptr(msg_send![self, name]) };
  }

  fn recommended_max_working_set_size(&self) -> u64 {
    return unsafe { msg_send![self, recommendedMaxWorkingSetSize] };
  }

  fn supports_feature_set(&self, feature_set: FeatureSet) -> bool {
    return unsafe { msg_send![self, supportsFeatureSet: feature_set] };
  }

  fn supports_texture_sample_count(&self, n: usize) -> bool {
    return unsafe { msg_send![self, supportsTextureSampleCount: n] };
  }

  // Support methods

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

impl IDevice for Device { }

#[cfg(test)]
mod tests {
  use super::Device;
  use super::IDevice;

  #[test]
  fn it_finds_default_device() {
    assert_eq!(Device::system_default().is_headless(), false);
  }
}
