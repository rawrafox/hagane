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
      self.release();
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
      self.release();
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
      self.release();
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
      self.release();
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
      self.release();
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
      self.release();
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
      self.release();
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
      self.release();
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
      self.release();
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
