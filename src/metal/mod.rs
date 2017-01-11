#![allow(non_upper_case_globals)]

use std;
use objc;
use super::ObjectiveC;
use core_foundation::*;
use foundation::*;
use legacy_metal::*;
bitflags! {
  pub flags MTLCompareFunction: NSUInteger {
    const MTLCompareFunctionNever = 0,
    const MTLCompareFunctionLess = 1,
    const MTLCompareFunctionEqual = 2,
    const MTLCompareFunctionLessEqual = 3,
    const MTLCompareFunctionGreater = 4,
    const MTLCompareFunctionNotEqual = 5,
    const MTLCompareFunctionGreaterEqual = 6,
    const MTLCompareFunctionAlways = 7,
  }
}
bitflags! {
  pub flags MTLCPUCacheMode: NSUInteger {
    const MTLCPUCacheModeDefaultCache = 0,
    const MTLCPUCacheModeWriteCombined = 1,
  }
}
bitflags! {
  pub flags MTLCullMode: NSUInteger {
    const MTLCullModeNone = 0,
    const MTLCullModeFront = 1,
    const MTLCullModeBack = 2,
  }
}
bitflags! {
  pub flags MTLIndexType: NSUInteger {
    const MTLIndexTypeUInt16 = 0,
    const MTLIndexTypeUInt32 = 1,
  }
}
bitflags! {
  pub flags MTLPrimitiveType: NSUInteger {
    const MTLPrimitiveTypePoint = 0,
    const MTLPrimitiveTypeLine = 1,
    const MTLPrimitiveTypeLineStrip = 2,
    const MTLPrimitiveTypeTriangle = 3,
    const MTLPrimitiveTypeTriangleStrip = 4,
  }
}
bitflags! {
  pub flags MTLStorageMode: NSUInteger {
    const MTLStorageModeShared = 0,
    const MTLStorageModeManaged = 1,
    const MTLStorageModePrivate = 2,
  }
}
bitflags! {
  pub flags MTLWinding: NSUInteger {
    const MTLWindingClockwise = 0,
    const MTLWindingCounterClockwise = 1,
  }
}

pub trait MTLRenderPassDescriptor : NSObject {
}

pub struct MTLRenderPassDescriptorID(*mut std::os::raw::c_void);

impl MTLRenderPassDescriptorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderPassDescriptorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLRenderPassDescriptorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLRenderPassDescriptorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLRenderPassDescriptor").unwrap();
  }
}

impl NSObject for MTLRenderPassDescriptorID {}
impl MTLRenderPassDescriptor for MTLRenderPassDescriptorID {}

impl Clone for MTLRenderPassDescriptorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLRenderPassDescriptorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLRenderPassDescriptorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderPassDescriptorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLRenderPassDescriptorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLRenderPassDescriptorID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLBlitCommandEncoder : MTLCommandEncoder + NSObject {
}

pub struct MTLBlitCommandEncoderID(*mut std::os::raw::c_void);

impl MTLBlitCommandEncoderID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLBlitCommandEncoderID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLBlitCommandEncoderID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLBlitCommandEncoderID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl MTLCommandEncoder for MTLBlitCommandEncoderID {}
impl NSObject for MTLBlitCommandEncoderID {}
impl MTLBlitCommandEncoder for MTLBlitCommandEncoderID {}

impl Clone for MTLBlitCommandEncoderID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLBlitCommandEncoderID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLBlitCommandEncoderID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLBlitCommandEncoderID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLBlitCommandEncoderID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLBlitCommandEncoderID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLBuffer : MTLResource + NSObject {
  fn contents(&self) -> *mut std::os::raw::c_void where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(contents), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: *mut std::os::raw::c_void = r;

          return result;
        }
      }
    }
  }

  fn length(&self) -> NSUInteger where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(length), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn remove_all_debug_markers(&self) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(removeAllDebugMarkers), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }
}

pub struct MTLBufferID(*mut std::os::raw::c_void);

impl MTLBufferID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLBufferID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLBufferID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLBufferID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl MTLResource for MTLBufferID {}
impl NSObject for MTLBufferID {}
impl MTLBuffer for MTLBufferID {}

impl Clone for MTLBufferID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLBufferID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLBufferID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLBufferID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLBufferID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLBufferID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLCommandBuffer : NSObject {
  fn render_command_encoder_with_descriptor<T5: 'static + MTLRenderPassDescriptor>(&self, render_pass_descriptor: T5) -> MTLRenderCommandEncoderID where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(renderCommandEncoderWithDescriptor:), (render_pass_descriptor.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: MTLRenderCommandEncoderID = r;

          return result.retain();
        }
      }
    }
  }

  fn compute_command_encoder(&self) -> MTLComputeCommandEncoderID where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(computeCommandEncoder), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: MTLComputeCommandEncoderID = r;

          return result.retain();
        }
      }
    }
  }

  fn blit_command_encoder(&self) -> MTLBlitCommandEncoderID where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(blitCommandEncoder), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: MTLBlitCommandEncoderID = r;

          return result.retain();
        }
      }
    }
  }

  fn parallel_render_command_encoder_with_descriptor<T5: 'static + MTLRenderPassDescriptor>(&self, render_pass_descriptor: T5) -> MTLParallelRenderCommandEncoderID where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(parallelRenderCommandEncoderWithDescriptor:), (render_pass_descriptor.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: MTLParallelRenderCommandEncoderID = r;

          return result.retain();
        }
      }
    }
  }

  fn enqueue(&self) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(enqueue), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn commit(&self) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(commit), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn present_drawable<T5: 'static + MTLDrawable>(&self, drawable: T5) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(presentDrawable:), (drawable.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn present_drawable_at_time<T5: 'static + MTLDrawable>(&self, drawable: T5, time: CFTimeInterval) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(presentDrawable:atTime:), (drawable.as_ptr(), time)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn wait_until_scheduled(&self) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(waitUntilScheduled), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn wait_until_completed(&self) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(waitUntilCompleted), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn retained_references(&self) -> bool where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(retainedReferences), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn device(&self) -> MTLDeviceID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(device), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: MTLDeviceID = r;

          return r.retain();
        }
      }
    }
  }

  fn command_queue(&self) -> MTLCommandQueueID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(commandQueue), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: MTLCommandQueueID = r;

          return r.retain();
        }
      }
    }
  }

  fn label(&self) -> NSStringID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(label), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: NSStringID = r;

          return r.retain();
        }
      }
    }
  }

  fn set_label<T: 'static + ObjectiveC + NSString>(&self, label: T) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setLabel:), (label.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }
}

pub struct MTLCommandBufferID(*mut std::os::raw::c_void);

impl MTLCommandBufferID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLCommandBufferID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLCommandBufferID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLCommandBufferID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for MTLCommandBufferID {}
impl MTLCommandBuffer for MTLCommandBufferID {}

impl Clone for MTLCommandBufferID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLCommandBufferID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLCommandBufferID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLCommandBufferID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLCommandBufferID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLCommandBufferID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLCommandQueue : NSObject {
  fn command_buffer(&self) -> MTLCommandBufferID where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(commandBuffer), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: MTLCommandBufferID = r;

          return result.retain();
        }
      }
    }
  }

  fn command_buffer_with_unretained_references(&self) -> MTLCommandBufferID where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(commandBufferWithUnretainedReferences), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: MTLCommandBufferID = r;

          return result.retain();
        }
      }
    }
  }

  fn insert_debug_capture_boundary(&self) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(insertDebugCaptureBoundary), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn device(&self) -> MTLDeviceID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(device), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: MTLDeviceID = r;

          return r.retain();
        }
      }
    }
  }

  fn label(&self) -> NSStringID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(label), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: NSStringID = r;

          return r.retain();
        }
      }
    }
  }

  fn set_label<T: 'static + ObjectiveC + NSString>(&self, label: T) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setLabel:), (label.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }
}

pub struct MTLCommandQueueID(*mut std::os::raw::c_void);

impl MTLCommandQueueID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLCommandQueueID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLCommandQueueID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLCommandQueueID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for MTLCommandQueueID {}
impl MTLCommandQueue for MTLCommandQueueID {}

impl Clone for MTLCommandQueueID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLCommandQueueID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLCommandQueueID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLCommandQueueID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLCommandQueueID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLCommandQueueID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLComputeCommandEncoder : MTLCommandEncoder + NSObject {
}

pub struct MTLComputeCommandEncoderID(*mut std::os::raw::c_void);

impl MTLComputeCommandEncoderID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLComputeCommandEncoderID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLComputeCommandEncoderID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLComputeCommandEncoderID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl MTLCommandEncoder for MTLComputeCommandEncoderID {}
impl NSObject for MTLComputeCommandEncoderID {}
impl MTLComputeCommandEncoder for MTLComputeCommandEncoderID {}

impl Clone for MTLComputeCommandEncoderID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLComputeCommandEncoderID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLComputeCommandEncoderID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLComputeCommandEncoderID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLComputeCommandEncoderID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLComputeCommandEncoderID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLComputePipelineState : NSObject {
}

pub struct MTLComputePipelineStateID(*mut std::os::raw::c_void);

impl MTLComputePipelineStateID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLComputePipelineStateID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLComputePipelineStateID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLComputePipelineStateID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for MTLComputePipelineStateID {}
impl MTLComputePipelineState for MTLComputePipelineStateID {}

impl Clone for MTLComputePipelineStateID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLComputePipelineStateID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLComputePipelineStateID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLComputePipelineStateID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLComputePipelineStateID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLComputePipelineStateID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLDepthStencilState : NSObject {
}

pub struct MTLDepthStencilStateID(*mut std::os::raw::c_void);

impl MTLDepthStencilStateID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLDepthStencilStateID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLDepthStencilStateID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLDepthStencilStateID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for MTLDepthStencilStateID {}
impl MTLDepthStencilState for MTLDepthStencilStateID {}

impl Clone for MTLDepthStencilStateID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLDepthStencilStateID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLDepthStencilStateID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLDepthStencilStateID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLDepthStencilStateID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLDepthStencilStateID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLDrawable : NSObject {
  fn present(&self) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(present), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn present_at_time(&self, presentation_time: CFTimeInterval) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(presentAtTime:), (presentation_time,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }
}

pub struct MTLDrawableID(*mut std::os::raw::c_void);

impl MTLDrawableID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLDrawableID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLDrawableID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLDrawableID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for MTLDrawableID {}
impl MTLDrawable for MTLDrawableID {}

impl Clone for MTLDrawableID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLDrawableID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLDrawableID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLDrawableID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLDrawableID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLDrawableID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLFence : NSObject {
  fn device(&self) -> MTLDeviceID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(device), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: MTLDeviceID = r;

          return r.retain();
        }
      }
    }
  }

  fn label(&self) -> NSStringID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(label), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: NSStringID = r;

          return r.retain();
        }
      }
    }
  }

  fn set_label<T: 'static + ObjectiveC + NSString>(&self, label: T) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setLabel:), (label.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }
}

pub struct MTLFenceID(*mut std::os::raw::c_void);

impl MTLFenceID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLFenceID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLFenceID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLFenceID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for MTLFenceID {}
impl MTLFence for MTLFenceID {}

impl Clone for MTLFenceID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLFenceID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLFenceID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLFenceID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLFenceID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLFenceID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLFunction : NSObject {
}

pub struct MTLFunctionID(*mut std::os::raw::c_void);

impl MTLFunctionID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLFunctionID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLFunctionID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLFunctionID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for MTLFunctionID {}
impl MTLFunction for MTLFunctionID {}

impl Clone for MTLFunctionID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLFunctionID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLFunctionID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLFunctionID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLFunctionID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLFunctionID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLHeap : NSObject {
}

pub struct MTLHeapID(*mut std::os::raw::c_void);

impl MTLHeapID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLHeapID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLHeapID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLHeapID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for MTLHeapID {}
impl MTLHeap for MTLHeapID {}

impl Clone for MTLHeapID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLHeapID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLHeapID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLHeapID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLHeapID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLHeapID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLLibrary : NSObject {
  fn new_function_with_name<T5: 'static + NSString>(&self, function_name: T5) -> MTLFunctionID where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(newFunctionWithName:), (function_name.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: MTLFunctionID = r;

          return result;
        }
      }
    }
  }

  fn function_names(&self) -> NSArrayID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(functionNames), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: NSArrayID = r;

          return r.retain();
        }
      }
    }
  }

  fn device(&self) -> MTLDeviceID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(device), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: MTLDeviceID = r;

          return r.retain();
        }
      }
    }
  }

  fn label(&self) -> NSStringID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(label), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: NSStringID = r;

          return r.retain();
        }
      }
    }
  }

  fn set_label<T: 'static + ObjectiveC + NSString>(&self, label: T) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setLabel:), (label.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }
}

pub struct MTLLibraryID(*mut std::os::raw::c_void);

impl MTLLibraryID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLLibraryID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLLibraryID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLLibraryID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for MTLLibraryID {}
impl MTLLibrary for MTLLibraryID {}

impl Clone for MTLLibraryID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLLibraryID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLLibraryID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLLibraryID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLLibraryID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLLibraryID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLParallelRenderCommandEncoder : MTLCommandEncoder + NSObject {
}

pub struct MTLParallelRenderCommandEncoderID(*mut std::os::raw::c_void);

impl MTLParallelRenderCommandEncoderID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLParallelRenderCommandEncoderID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLParallelRenderCommandEncoderID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLParallelRenderCommandEncoderID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl MTLCommandEncoder for MTLParallelRenderCommandEncoderID {}
impl NSObject for MTLParallelRenderCommandEncoderID {}
impl MTLParallelRenderCommandEncoder for MTLParallelRenderCommandEncoderID {}

impl Clone for MTLParallelRenderCommandEncoderID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLParallelRenderCommandEncoderID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLParallelRenderCommandEncoderID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLParallelRenderCommandEncoderID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLParallelRenderCommandEncoderID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLParallelRenderCommandEncoderID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLRenderCommandEncoder : MTLCommandEncoder + NSObject {
}

pub struct MTLRenderCommandEncoderID(*mut std::os::raw::c_void);

impl MTLRenderCommandEncoderID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderCommandEncoderID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLRenderCommandEncoderID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLRenderCommandEncoderID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl MTLCommandEncoder for MTLRenderCommandEncoderID {}
impl NSObject for MTLRenderCommandEncoderID {}
impl MTLRenderCommandEncoder for MTLRenderCommandEncoderID {}

impl Clone for MTLRenderCommandEncoderID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLRenderCommandEncoderID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLRenderCommandEncoderID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderCommandEncoderID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLRenderCommandEncoderID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLRenderCommandEncoderID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLRenderPipelineState : NSObject {
}

pub struct MTLRenderPipelineStateID(*mut std::os::raw::c_void);

impl MTLRenderPipelineStateID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderPipelineStateID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLRenderPipelineStateID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLRenderPipelineStateID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for MTLRenderPipelineStateID {}
impl MTLRenderPipelineState for MTLRenderPipelineStateID {}

impl Clone for MTLRenderPipelineStateID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLRenderPipelineStateID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLRenderPipelineStateID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderPipelineStateID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLRenderPipelineStateID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLRenderPipelineStateID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLResource : NSObject {
}

pub struct MTLResourceID(*mut std::os::raw::c_void);

impl MTLResourceID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLResourceID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLResourceID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLResourceID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for MTLResourceID {}
impl MTLResource for MTLResourceID {}

impl Clone for MTLResourceID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLResourceID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLResourceID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLResourceID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLResourceID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLResourceID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLSamplerState : NSObject {
}

pub struct MTLSamplerStateID(*mut std::os::raw::c_void);

impl MTLSamplerStateID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLSamplerStateID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLSamplerStateID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLSamplerStateID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for MTLSamplerStateID {}
impl MTLSamplerState for MTLSamplerStateID {}

impl Clone for MTLSamplerStateID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLSamplerStateID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLSamplerStateID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLSamplerStateID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLSamplerStateID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLSamplerStateID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLTexture : NSObject {
}

pub struct MTLTextureID(*mut std::os::raw::c_void);

impl MTLTextureID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLTextureID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLTextureID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLTextureID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for MTLTextureID {}
impl MTLTexture for MTLTextureID {}

impl Clone for MTLTextureID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLTextureID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLTextureID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLTextureID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLTextureID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLTextureID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}
