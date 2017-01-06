use std;
use objc;

use std::os::raw::c_void;

use super::{CAMetalDrawable, NSArrayID, NSObject, NSStringID};

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

pub trait MTLCommandBuffer : NSObject {
  forward!(commit, sel!(commit), () -> ());
  forward!(present_drawable, sel!(presentDrawable:), (drawable: T) -> (), <T: CAMetalDrawable>);
  forward!(render_command_encoder_with_descriptor, sel!(renderCommandEncoderWithDescriptor:), (render_pass_descriptor: T) -> MTLCommandEncoderID, <T: MTLRenderPassDescriptor>);
}

id!(MTLCommandBufferID, MTLCommandBuffer);

impl NSObject for MTLCommandBufferID {}

pub trait MTLCommandEncoder : NSObject {
  forward!(end_encoding, sel!(endEncoding), () -> ());
}

id!(MTLCommandEncoderID, MTLCommandEncoder);

impl NSObject for MTLCommandEncoderID {}

pub trait MTLCommandQueue : NSObject {
  forward!(command_buffer, sel!(commandBuffer), () -> MTLCommandBufferID);
}

id!(MTLCommandQueueID, MTLCommandQueue);

impl NSObject for MTLCommandQueueID {}

pub trait MTLDevice : NSObject {
  forward!(is_depth24_stencil8_pixel_format_supported, sel!(isDepth24Stencil8PixelFormatSupported), () -> bool);
  forward!(is_headless, sel!(isHeadless), () -> bool);
  forward!(is_low_power, sel!(isLowPower), () -> bool);
  forward!(max_threads_per_threadgroup, sel!(maxThreadsPerThreadgroup), () -> MTLSize);
  forward!(name, sel!(name), () -> NSStringID);
  forward!(new_command_queue, sel!(newCommandQueue), () -> MTLCommandQueueID);
  forward!(recommended_max_working_set_size, sel!(recommendedMaxWorkingSetSize), () -> u64);
  forward!(supports_feature_set, sel!(supportsFeatureSet:), (feature_set: MTLFeatureSet) -> bool);
  forward!(supports_texture_sample_count, sel!(supportsTextureSampleCount:), (i: usize) -> bool);

  // Rust Helpers

  fn texture_sample_counts(&self) -> Vec<usize> where Self: Sized {
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

pub trait MTLRenderPassDescriptor : NSObject {
  
}

id!(MTLRenderPassDescriptorID, MTLRenderPassDescriptor, "MTLRenderPassDescriptor");

impl NSObject for MTLRenderPassDescriptorID {}

pub fn all_devices() -> NSArrayID {
  unsafe {
    return NSArrayID::from_ptr(MTLCopyAllDevices());
  }
}

pub fn system_default_device() -> MTLDeviceID {
  unsafe {
    return MTLDeviceID::from_ptr(MTLCreateSystemDefaultDevice());
  }
}
