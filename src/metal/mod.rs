use std;
use objc;

use std::os::raw::c_void;

use super::{ObjectiveC, CAMetalDrawable, NSArrayID, NSObject, NSErrorID, NSString, NSStringID};

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

pub enum MTLCompareFunction {
  MTLCompareFunctionNever = 0,
  MTLCompareFunctionLess = 1,
  MTLCompareFunctionEqual = 2,
  MTLCompareFunctionLessEqual = 3,
  MTLCompareFunctionGreater = 4,
  MTLCompareFunctionNotEqual = 5,
  MTLCompareFunctionGreaterEqual = 6,
  MTLCompareFunctionAlways = 7
}

pub enum MTLCPUCacheMode {
  MTLCPUCacheModeDefaultCache = 0,
  MTLCPUCacheModeWriteCombined = 1
}

pub enum MTLCullMode {
  MTLCullModeNone = 0,
  MTLCullModeFront = 1,
  MTLCullModeBack = 2,
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

pub enum MTLIndexType {
  MTLIndexTypeUInt16 = 0,
  MTLIndexTypeUInt32 = 1
}

#[allow(non_camel_case_types)]
pub enum MTLPixelFormat {
    MTLPixelFormatInvalid = 0,

    MTLPixelFormatA8Unorm = 1,
    
    MTLPixelFormatR8Unorm = 10,
    MTLPixelFormatR8Unorm_sRGB = 11,

    MTLPixelFormatR8Snorm = 12,
    MTLPixelFormatR8Uint = 13,
    MTLPixelFormatR8Sint = 14,

    MTLPixelFormatR16Unorm = 20,
    MTLPixelFormatR16Snorm = 22,
    MTLPixelFormatR16Uint = 23,
    MTLPixelFormatR16Sint = 24,
    MTLPixelFormatR16Float = 25,

    MTLPixelFormatRG8Unorm = 30,
    MTLPixelFormatRG8Unorm_sRGB = 31,
    MTLPixelFormatRG8Snorm = 32,
    MTLPixelFormatRG8Uint = 33,
    MTLPixelFormatRG8Sint = 34,

    MTLPixelFormatB5G6R5Unorm = 40,
    MTLPixelFormatA1BGR5Unorm = 41,
    MTLPixelFormatABGR4Unorm = 42,
    MTLPixelFormatBGR5A1Unorm = 43,

    MTLPixelFormatR32Uint = 53,
    MTLPixelFormatR32Sint = 54,
    MTLPixelFormatR32Float = 55,

    MTLPixelFormatRG16Unorm = 60,
    MTLPixelFormatRG16Snorm = 62,
    MTLPixelFormatRG16Uint = 63,
    MTLPixelFormatRG16Sint = 64,
    MTLPixelFormatRG16Float = 65,

    MTLPixelFormatRGBA8Unorm = 70,
    MTLPixelFormatRGBA8Unorm_sRGB = 71,
    MTLPixelFormatRGBA8Snorm = 72,
    MTLPixelFormatRGBA8Uint = 73,
    MTLPixelFormatRGBA8Sint = 74,

    MTLPixelFormatBGRA8Unorm = 80,
    MTLPixelFormatBGRA8Unorm_sRGB = 81,

    MTLPixelFormatRGB10A2Unorm = 90,
    MTLPixelFormatRGB10A2Uint = 91,

    MTLPixelFormatRG11B10Float = 92,
    MTLPixelFormatRGB9E5Float = 93,

    MTLPixelFormatBGR10_XR = 554,
    MTLPixelFormatBGR10_XR_sRGB = 555,

    MTLPixelFormatRG32Uint = 103,
    MTLPixelFormatRG32Sint = 104,
    MTLPixelFormatRG32Float = 105,

    MTLPixelFormatRGBA16Unorm = 110,
    MTLPixelFormatRGBA16Snorm = 112,
    MTLPixelFormatRGBA16Uint = 113,
    MTLPixelFormatRGBA16Sint = 114,
    MTLPixelFormatRGBA16Float = 115,

    MTLPixelFormatBGRA10_XR = 552,
    MTLPixelFormatBGRA10_XR_sRGB = 553,

    MTLPixelFormatRGBA32Uint  = 123,
    MTLPixelFormatRGBA32Sint  = 124,
    MTLPixelFormatRGBA32Float = 125,

    MTLPixelFormatBC1_RGBA = 130,
    MTLPixelFormatBC1_RGBA_sRGB = 131,
    MTLPixelFormatBC2_RGBA = 132,
    MTLPixelFormatBC2_RGBA_sRGB = 133,
    MTLPixelFormatBC3_RGBA = 134,
    MTLPixelFormatBC3_RGBA_sRGB = 135,

    MTLPixelFormatBC4_RUnorm = 140,
    MTLPixelFormatBC4_RSnorm = 141,
    MTLPixelFormatBC5_RGUnorm = 142,
    MTLPixelFormatBC5_RGSnorm = 143,

    MTLPixelFormatBC6H_RGBFloat = 150,
    MTLPixelFormatBC6H_RGBUfloat = 151,
    MTLPixelFormatBC7_RGBAUnorm = 152,
    MTLPixelFormatBC7_RGBAUnorm_sRGB = 153,

    MTLPixelFormatPVRTC_RGB_2BPP = 160,
    MTLPixelFormatPVRTC_RGB_2BPP_sRGB = 161,
    MTLPixelFormatPVRTC_RGB_4BPP = 162,
    MTLPixelFormatPVRTC_RGB_4BPP_sRGB = 163,
    MTLPixelFormatPVRTC_RGBA_2BPP = 164,
    MTLPixelFormatPVRTC_RGBA_2BPP_sRGB = 165,
    MTLPixelFormatPVRTC_RGBA_4BPP = 166,
    MTLPixelFormatPVRTC_RGBA_4BPP_sRGB = 167,

    MTLPixelFormatEAC_R11Unorm = 170,
    MTLPixelFormatEAC_R11Snorm = 172,
    MTLPixelFormatEAC_RG11Unorm = 174,
    MTLPixelFormatEAC_RG11Snorm = 176,
    MTLPixelFormatEAC_RGBA8 = 178,
    MTLPixelFormatEAC_RGBA8_sRGB = 179,

    MTLPixelFormatETC2_RGB8 = 180,
    MTLPixelFormatETC2_RGB8_sRGB = 181,
    MTLPixelFormatETC2_RGB8A1 = 182,
    MTLPixelFormatETC2_RGB8A1_sRGB = 183,

    MTLPixelFormatASTC_4x4_sRGB = 186,
    MTLPixelFormatASTC_5x4_sRGB = 187,
    MTLPixelFormatASTC_5x5_sRGB = 188,
    MTLPixelFormatASTC_6x5_sRGB = 189,
    MTLPixelFormatASTC_6x6_sRGB = 190,
    MTLPixelFormatASTC_8x5_sRGB = 192,
    MTLPixelFormatASTC_8x6_sRGB = 193,
    MTLPixelFormatASTC_8x8_sRGB = 194,
    MTLPixelFormatASTC_10x5_sRGB = 195,
    MTLPixelFormatASTC_10x6_sRGB = 196,
    MTLPixelFormatASTC_10x8_sRGB = 197,
    MTLPixelFormatASTC_10x10_sRGB = 198,
    MTLPixelFormatASTC_12x10_sRGB = 199,
    MTLPixelFormatASTC_12x12_sRGB = 200,

    MTLPixelFormatASTC_4x4_LDR = 204,
    MTLPixelFormatASTC_5x4_LDR = 205,
    MTLPixelFormatASTC_5x5_LDR = 206,
    MTLPixelFormatASTC_6x5_LDR = 207,
    MTLPixelFormatASTC_6x6_LDR = 208,
    MTLPixelFormatASTC_8x5_LDR = 210,
    MTLPixelFormatASTC_8x6_LDR = 211,
    MTLPixelFormatASTC_8x8_LDR = 212,
    MTLPixelFormatASTC_10x5_LDR = 213,
    MTLPixelFormatASTC_10x6_LDR = 214,
    MTLPixelFormatASTC_10x8_LDR = 215,
    MTLPixelFormatASTC_10x10_LDR = 216,
    MTLPixelFormatASTC_12x10_LDR = 217,
    MTLPixelFormatASTC_12x12_LDR = 218,

    MTLPixelFormatGBGR422 = 240,

    MTLPixelFormatBGRG422 = 241,

    MTLPixelFormatDepth16Unorm = 250,
    MTLPixelFormatDepth32Float = 252,

    MTLPixelFormatStencil8 = 253,

    MTLPixelFormatDepth24Unorm_Stencil8 = 255,
    MTLPixelFormatDepth32Float_Stencil8 = 260,

    MTLPixelFormatX32_Stencil8 = 261,
    MTLPixelFormatX24_Stencil8 = 262,
}

pub enum MTLPrimitiveType {
  MTLPrimitiveTypePoint = 0,
  MTLPrimitiveTypeLine = 1,
  MTLPrimitiveTypeLineStrip = 2,
  MTLPrimitiveTypeTriangle = 3,
  MTLPrimitiveTypeTriangleStrip = 4
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MTLSize {
  pub width: usize,
  pub height: usize,
  pub depth: usize
}

pub enum MTLStorageMode {
  MTLStorageModeShared  = 0,
  MTLStorageModeManaged = 1,
  MTLStorageModePrivate = 2
}

pub enum MTLWinding {
  MTLWindingClockwise = 0,
  MTLWindingCounterClockwise = 1,
}

pub trait MTLBuffer : NSObject {
  forward!(contents, sel!(contents), () -> *mut c_void);
}

id!(MTLBufferID, MTLBuffer);

impl NSObject for MTLBufferID {}

pub trait MTLCommandBuffer : NSObject {
  forward!(commit, sel!(commit), () -> ());
  forward!(present_drawable, sel!(presentDrawable:), (drawable: T) -> (), <T: CAMetalDrawable>);
  forward!(render_command_encoder_with_descriptor, sel!(renderCommandEncoderWithDescriptor:), (render_pass_descriptor: T) -> MTLCommandEncoderID, <T: MTLRenderPassDescriptor>, retain);
}

id!(MTLCommandBufferID, MTLCommandBuffer);

impl NSObject for MTLCommandBufferID {}

pub trait MTLCommandEncoder : NSObject {
  forward!(draw_primitives_vertex_start_vertex_count, sel!(drawPrimitives:vertexStart:vertexCount:), (primitive_type: MTLPrimitiveType, start: usize, count: usize) -> ());
  forward!(draw_indexed_primitives_index_count_index_type_index_buffer_index_buffer_offset, sel!(drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:), (primitive_type: MTLPrimitiveType, count: usize, index_type: MTLIndexType, index_buffer: T, offset: usize) -> (), <T: MTLBuffer>);
  forward!(end_encoding, sel!(endEncoding), () -> ());
  forward!(set_cull_mode, sel!(setCullMode:), (mode: MTLCullMode) -> ());
  forward!(set_depth_stencil_state, sel!(setDepthStencilState:), (state: T) -> (), <T: MTLDepthStencilState>);
  forward!(set_front_facing_winding, sel!(setFrontFacingWinding:), (winding: MTLWinding) -> ());
  forward!(set_render_pipeline_state, sel!(setRenderPipelineState:), (pipeline_state: T) -> (), <T: MTLRenderPipelineState>);
  forward!(set_vertex_buffer_offset_at_index, sel!(setVertexBuffer:offset:atIndex:), (buffer: T, offset: usize, index: usize) -> (), <T: MTLBuffer>);
}

id!(MTLCommandEncoderID, MTLCommandEncoder);

impl NSObject for MTLCommandEncoderID {}

pub trait MTLCommandQueue : NSObject {
  forward!(command_buffer, sel!(commandBuffer), () -> MTLCommandBufferID, retain);
}

id!(MTLCommandQueueID, MTLCommandQueue);

impl NSObject for MTLCommandQueueID {}

pub trait MTLDepthStencilDescriptor : NSObject {
  initializer!(init, sel!(init), ());

  forward!(set_depth_compare_function, sel!(setDepthCompareFunction:), (compare_function: MTLCompareFunction) -> ());
  forward!(set_depth_write_enabled, sel!(setDepthWriteEnabled:), (enabled: bool) -> ());
}

id!(MTLDepthStencilDescriptorID, MTLDepthStencilDescriptor, "MTLDepthStencilDescriptor");

impl NSObject for MTLDepthStencilDescriptorID {}

pub trait MTLDepthStencilState : NSObject {
  
}

id!(MTLDepthStencilStateID, MTLDepthStencilState);

impl NSObject for MTLDepthStencilStateID {}

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

pub trait MTLFunction : NSObject {

}

id!(MTLFunctionID, MTLFunction);

impl NSObject for MTLFunctionID {}

pub trait MTLLibrary : NSObject {
  forward!(new_function_with_name, sel!(newFunctionWithName:), (name: T) -> MTLFunctionID, <T: NSString>);
}

id!(MTLLibraryID, MTLLibrary);

impl NSObject for MTLLibraryID {}

pub trait MTLRenderPassDescriptor : NSObject {

}

id!(MTLRenderPassDescriptorID, MTLRenderPassDescriptor, "MTLRenderPassDescriptor");

impl NSObject for MTLRenderPassDescriptorID {}

pub trait MTLRenderPipelineColorAttachmentDescriptor : NSObject {
  forward!(set_pixel_format, sel!(setPixelFormat:), (format: usize) -> ());
}

id!(MTLRenderPipelineColorAttachmentDescriptorID, MTLRenderPipelineColorAttachmentDescriptor, "MTLRenderPipelineColorAttachmentDescriptor");

impl NSObject for MTLRenderPipelineColorAttachmentDescriptorID {}

pub trait MTLRenderPipelineColorAttachmentDescriptorArray : NSObject {
  forward!(object_at_indexed_subscript, sel!(objectAtIndexedSubscript:), (i: usize) -> MTLRenderPipelineColorAttachmentDescriptorID, retain);
}

id!(MTLRenderPipelineColorAttachmentDescriptorArrayID, MTLRenderPipelineColorAttachmentDescriptorArray, "MTLRenderPipelineColorAttachmentDescriptorArray");

impl NSObject for MTLRenderPipelineColorAttachmentDescriptorArrayID {}

pub trait MTLRenderPipelineDescriptor : NSObject {
  initializer!(init, sel!(init), ());

  forward!(color_attachments, sel!(colorAttachments), () -> MTLRenderPipelineColorAttachmentDescriptorArrayID, retain);
  forward!(set_fragment_function, sel!(setFragmentFunction:), (function: T) -> (), <T: MTLFunction>);
  forward!(set_vertex_function, sel!(setVertexFunction:), (function: T) -> (), <T: MTLFunction>);
}

id!(MTLRenderPipelineDescriptorID, MTLRenderPipelineDescriptor, "MTLRenderPipelineDescriptor");

impl NSObject for MTLRenderPipelineDescriptorID {}

pub trait MTLRenderPipelineState {

}

id!(MTLRenderPipelineStateID, MTLRenderPipelineState);

impl NSObject for MTLRenderPipelineStateID {}

pub trait MTLTexture {

}

id!(MTLTextureID, MTLTexture);

impl NSObject for MTLTextureID {}

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
