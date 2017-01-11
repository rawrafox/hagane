use std;
use objc;

use std::os::raw::c_void;

use metal::*;

use super::{ObjectiveC, NSArrayID, NSObject, NSErrorID, NSString, NSStringID};

#[link(name = "Metal", kind = "framework")]
extern {
  fn MTLCopyAllDevices() -> *mut c_void;
  fn MTLCreateSystemDefaultDevice() -> *mut c_void;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MTLClearColor {
  pub red: f64,
  pub green: f64,
  pub blue: f64,
  pub alpha: f64
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MTLSize {
  pub width: usize,
  pub height: usize,
  pub depth: usize
}

pub trait MTLDevice : NSObject {
  forward!(is_depth24_stencil8_pixel_format_supported, sel!(isDepth24Stencil8PixelFormatSupported), () -> bool);
  forward!(is_headless, sel!(isHeadless), () -> bool);
  forward!(is_low_power, sel!(isLowPower), () -> bool);
  forward!(max_threads_per_threadgroup, sel!(maxThreadsPerThreadgroup), () -> MTLSize);
  forward!(name, sel!(name), () -> NSStringID, retain);
  forward!(new_buffer_with_length_options, sel!(newBufferWithLength:options:), (length: usize, options: usize) -> MTLBufferID);
  forward!(new_buffer_with_bytes_length_options, sel!(newBufferWithBytes:length:options:), (bytes: *const c_void, length: usize, options: usize) -> MTLBufferID);
  forward!(new_command_queue, sel!(newCommandQueue), () -> MTLCommandQueueID);
  forward!(new_depth_stencil_state_with_descriptor, sel!(newDepthStencilStateWithDescriptor:), (descriptor: T) -> MTLDepthStencilStateID, <T: MTLDepthStencilDescriptor>);
  forward!(recommended_max_working_set_size, sel!(recommendedMaxWorkingSetSize), () -> u64);
  forward!(supports_feature_set, sel!(supportsFeatureSet:), (feature_set: MTLFeatureSet) -> bool);
  forward!(supports_texture_sample_count, sel!(supportsTextureSampleCount:), (i: usize) -> bool);

  fn new_library_with_file<T: NSString>(&self, filepath: T) -> Result<MTLLibraryID, NSErrorID> {
    let mut error = NSErrorID::nil();

    unsafe {
      let lib = msg_send![self.as_object(), newLibraryWithFile: filepath error: &mut error];

      if error.is_nil() {
        return Ok(lib);
      } else {
        return Err(error);
      }
    }
  }

  fn new_render_pipeline_state_with_descriptor<T: MTLRenderPipelineDescriptor>(&self, descriptor: T) -> Result<MTLRenderPipelineStateID, NSErrorID> {
    let mut error = NSErrorID::nil();

    unsafe {
      let lib = msg_send![self.as_object(), newRenderPipelineStateWithDescriptor: descriptor error: &mut error];

      if error.is_nil() {
        return Ok(lib);
      } else {
        return Err(error);
      }
    }
  }

  // Rust Helpers

  fn texture_sample_counts(&self) -> Vec<usize> where Self: 'static + Sized {
    let mut result = Vec::new();

    for i in 1 .. 128 {
      if self.supports_texture_sample_count(i) {
        result.push(i);
      }
    }

    return result;
  }
}

id!(MTLDeviceID, MTLDevice);

impl NSObject for MTLDeviceID {}

pub fn all_devices() -> NSArrayID {
  unsafe {
    return NSArrayID::from_ptr(MTLCopyAllDevices());
  }
}

pub fn system_default_device() -> MTLDeviceID {
  unsafe {
    let device = MTLDeviceID::from_ptr(MTLCreateSystemDefaultDevice());

    return device.retain();
  }
}
