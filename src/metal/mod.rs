#![allow(non_upper_case_globals)]

use std;
use objc;
use super::ObjectiveC;
use core_foundation::*;
use foundation::*;
use legacy_metal::*;
bitflags! {
  pub flags MTLBlendFactor: NSUInteger {
    const MTLBlendFactorZero = 0,
    const MTLBlendFactorOne = 1,
    const MTLBlendFactorSourceColor = 2,
    const MTLBlendFactorOneMinusSourceColor = 3,
    const MTLBlendFactorSourceAlpha = 4,
    const MTLBlendFactorOneMinusSourceAlpha = 5,
    const MTLBlendFactorDestinationColor = 6,
    const MTLBlendFactorOneMinusDestinationColor = 7,
    const MTLBlendFactorDestinationAlpha = 8,
    const MTLBlendFactorOneMinusDestinationAlpha = 9,
    const MTLBlendFactorSourceAlphaSaturated = 10,
    const MTLBlendFactorBlendColor = 11,
    const MTLBlendFactorOneMinusBlendColor = 12,
    const MTLBlendFactorBlendAlpha = 13,
    const MTLBlendFactorOneMinusBlendAlpha = 14,
    const MTLBlendFactorSource1Color = 15,
    const MTLBlendFactorOneMinusSource1Color = 16,
    const MTLBlendFactorSource1Alpha = 17,
    const MTLBlendFactorOneMinusSource1Alpha = 18,
  }
}
bitflags! {
  pub flags MTLBlendOperation: NSUInteger {
    const MTLBlendOperationAdd = 0,
    const MTLBlendOperationSubtract = 1,
    const MTLBlendOperationReverseSubtract = 2,
    const MTLBlendOperationMin = 3,
    const MTLBlendOperationMax = 4,
  }
}
bitflags! {
  pub flags MTLColorWriteMask: NSUInteger {
    const MTLColorWriteMaskNone = 0,
    const MTLColorWriteMaskRed = 0x1 << 3,
    const MTLColorWriteMaskGreen = 0x1 << 2,
    const MTLColorWriteMaskBlue = 0x1 << 1,
    const MTLColorWriteMaskAlpha = 0x1 << 0,
    const MTLColorWriteMaskAll = 0xf,
  }
}
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
  pub flags MTLDepthClipMode: NSUInteger {
    const MTLDepthClipModeClip = 0,
    const MTLDepthClipModeClamp = 1,
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
  pub flags MTLTriangleFillMode: NSUInteger {
    const MTLTriangleFillModeFill = 0,
    const MTLTriangleFillModeLines = 1,
  }
}
bitflags! {
  pub flags MTLVisibilityResultMode: NSUInteger {
    const MTLVisibilityResultModeDisabled = 0,
    const MTLVisibilityResultModeBoolean = 1,
    const MTLVisibilityResultModeCounting = 2,
  }
}
bitflags! {
  pub flags MTLWinding: NSUInteger {
    const MTLWindingClockwise = 0,
    const MTLWindingCounterClockwise = 1,
  }
}

pub trait MTLArgument : NSObject {
}

pub struct MTLArgumentID(*mut std::os::raw::c_void);

impl MTLArgumentID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLArgumentID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLArgumentID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLArgumentID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLArgument").unwrap();
  }
}

impl NSObject for MTLArgumentID {}
impl MTLArgument for MTLArgumentID {}

impl Clone for MTLArgumentID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLArgumentID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLArgumentID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLArgumentID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLArgumentID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLArgumentID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLArrayType : NSObject {
}

pub struct MTLArrayTypeID(*mut std::os::raw::c_void);

impl MTLArrayTypeID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLArrayTypeID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLArrayTypeID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLArrayTypeID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLArrayType").unwrap();
  }
}

impl NSObject for MTLArrayTypeID {}
impl MTLArrayType for MTLArrayTypeID {}

impl Clone for MTLArrayTypeID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLArrayTypeID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLArrayTypeID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLArrayTypeID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLArrayTypeID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLArrayTypeID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLAttribute : NSObject {
}

pub struct MTLAttributeID(*mut std::os::raw::c_void);

impl MTLAttributeID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLAttributeID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLAttributeID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLAttributeID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLAttribute").unwrap();
  }
}

impl NSObject for MTLAttributeID {}
impl MTLAttribute for MTLAttributeID {}

impl Clone for MTLAttributeID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLAttributeID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLAttributeID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLAttributeID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLAttributeID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLAttributeID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLAttributeDescriptor : NSObject {
}

pub struct MTLAttributeDescriptorID(*mut std::os::raw::c_void);

impl MTLAttributeDescriptorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLAttributeDescriptorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLAttributeDescriptorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLAttributeDescriptorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLAttributeDescriptor").unwrap();
  }
}

impl NSObject for MTLAttributeDescriptorID {}
impl MTLAttributeDescriptor for MTLAttributeDescriptorID {}

impl Clone for MTLAttributeDescriptorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLAttributeDescriptorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLAttributeDescriptorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLAttributeDescriptorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLAttributeDescriptorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLAttributeDescriptorID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLAttributeDescriptorArray : NSObject {
  fn object_at_index(&self, index: NSUInteger) -> MTLAttributeDescriptorID where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(objectAtIndex:), (index,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: MTLAttributeDescriptorID = r;

          return result.retain();
        }
      }
    }
  }

  fn object_at_indexed_subscript(&self, index: NSUInteger) -> MTLAttributeDescriptorID where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(objectAtIndexedSubscript:), (index,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: MTLAttributeDescriptorID = r;

          return result.retain();
        }
      }
    }
  }
}

pub struct MTLAttributeDescriptorArrayID(*mut std::os::raw::c_void);

impl MTLAttributeDescriptorArrayID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLAttributeDescriptorArrayID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLAttributeDescriptorArrayID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLAttributeDescriptorArrayID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLAttributeDescriptorArray").unwrap();
  }
}

impl NSObject for MTLAttributeDescriptorArrayID {}
impl MTLAttributeDescriptorArray for MTLAttributeDescriptorArrayID {}

impl Clone for MTLAttributeDescriptorArrayID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLAttributeDescriptorArrayID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLAttributeDescriptorArrayID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLAttributeDescriptorArrayID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLAttributeDescriptorArrayID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLAttributeDescriptorArrayID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLBufferLayoutDescriptor : NSObject {
}

pub struct MTLBufferLayoutDescriptorID(*mut std::os::raw::c_void);

impl MTLBufferLayoutDescriptorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLBufferLayoutDescriptorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLBufferLayoutDescriptorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLBufferLayoutDescriptorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLBufferLayoutDescriptor").unwrap();
  }
}

impl NSObject for MTLBufferLayoutDescriptorID {}
impl MTLBufferLayoutDescriptor for MTLBufferLayoutDescriptorID {}

impl Clone for MTLBufferLayoutDescriptorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLBufferLayoutDescriptorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLBufferLayoutDescriptorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLBufferLayoutDescriptorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLBufferLayoutDescriptorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLBufferLayoutDescriptorID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLBufferLayoutDescriptorArray : NSObject {
  fn object_at_index(&self, index: NSUInteger) -> MTLBufferLayoutDescriptorID where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(objectAtIndex:), (index,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: MTLBufferLayoutDescriptorID = r;

          return result.retain();
        }
      }
    }
  }

  fn object_at_indexed_subscript(&self, index: NSUInteger) -> MTLBufferLayoutDescriptorID where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(objectAtIndexedSubscript:), (index,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: MTLBufferLayoutDescriptorID = r;

          return result.retain();
        }
      }
    }
  }
}

pub struct MTLBufferLayoutDescriptorArrayID(*mut std::os::raw::c_void);

impl MTLBufferLayoutDescriptorArrayID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLBufferLayoutDescriptorArrayID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLBufferLayoutDescriptorArrayID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLBufferLayoutDescriptorArrayID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLBufferLayoutDescriptorArray").unwrap();
  }
}

impl NSObject for MTLBufferLayoutDescriptorArrayID {}
impl MTLBufferLayoutDescriptorArray for MTLBufferLayoutDescriptorArrayID {}

impl Clone for MTLBufferLayoutDescriptorArrayID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLBufferLayoutDescriptorArrayID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLBufferLayoutDescriptorArrayID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLBufferLayoutDescriptorArrayID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLBufferLayoutDescriptorArrayID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLBufferLayoutDescriptorArrayID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLCompileOptions : NSObject {
}

pub struct MTLCompileOptionsID(*mut std::os::raw::c_void);

impl MTLCompileOptionsID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLCompileOptionsID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLCompileOptionsID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLCompileOptionsID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLCompileOptions").unwrap();
  }
}

impl NSObject for MTLCompileOptionsID {}
impl MTLCompileOptions for MTLCompileOptionsID {}

impl Clone for MTLCompileOptionsID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLCompileOptionsID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLCompileOptionsID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLCompileOptionsID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLCompileOptionsID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLCompileOptionsID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLComputePipelineDescriptor : NSObject {
}

pub struct MTLComputePipelineDescriptorID(*mut std::os::raw::c_void);

impl MTLComputePipelineDescriptorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLComputePipelineDescriptorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLComputePipelineDescriptorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLComputePipelineDescriptorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLComputePipelineDescriptor").unwrap();
  }
}

impl NSObject for MTLComputePipelineDescriptorID {}
impl MTLComputePipelineDescriptor for MTLComputePipelineDescriptorID {}

impl Clone for MTLComputePipelineDescriptorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLComputePipelineDescriptorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLComputePipelineDescriptorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLComputePipelineDescriptorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLComputePipelineDescriptorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLComputePipelineDescriptorID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLComputePipelineReflection : NSObject {
}

pub struct MTLComputePipelineReflectionID(*mut std::os::raw::c_void);

impl MTLComputePipelineReflectionID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLComputePipelineReflectionID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLComputePipelineReflectionID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLComputePipelineReflectionID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLComputePipelineReflection").unwrap();
  }
}

impl NSObject for MTLComputePipelineReflectionID {}
impl MTLComputePipelineReflection for MTLComputePipelineReflectionID {}

impl Clone for MTLComputePipelineReflectionID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLComputePipelineReflectionID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLComputePipelineReflectionID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLComputePipelineReflectionID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLComputePipelineReflectionID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLComputePipelineReflectionID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLDepthStencilDescriptor : NSObject {
  fn init(self) -> Self where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(init), ()) {
        Err(s) => panic!("{}", s),
        Ok(result) => {
          std::mem::forget(self);

          return result;
        }
      }
    }
  }

  fn depth_compare_function(&self) -> MTLCompareFunction where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(depthCompareFunction), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_depth_compare_function(&self, depth_compare_function: MTLCompareFunction) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setDepthCompareFunction:), (depth_compare_function,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn is_depth_write_enabled(&self) -> bool where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(isDepthWriteEnabled), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_depth_write_enabled(&self, depth_write_enabled: bool) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setDepthWriteEnabled:), (depth_write_enabled,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn back_face_stencil(&self) -> MTLStencilDescriptorID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(backFaceStencil), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: MTLStencilDescriptorID = r;

          return r.retain();
        }
      }
    }
  }

  fn set_back_face_stencil<T: 'static + ObjectiveC + MTLStencilDescriptor>(&self, back_face_stencil: T) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setBackFaceStencil:), (back_face_stencil.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn front_face_stencil(&self) -> MTLStencilDescriptorID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(frontFaceStencil), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: MTLStencilDescriptorID = r;

          return r.retain();
        }
      }
    }
  }

  fn set_front_face_stencil<T: 'static + ObjectiveC + MTLStencilDescriptor>(&self, front_face_stencil: T) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setFrontFaceStencil:), (front_face_stencil.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
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

pub struct MTLDepthStencilDescriptorID(*mut std::os::raw::c_void);

impl MTLDepthStencilDescriptorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLDepthStencilDescriptorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLDepthStencilDescriptorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLDepthStencilDescriptorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLDepthStencilDescriptor").unwrap();
  }
}

impl NSObject for MTLDepthStencilDescriptorID {}
impl MTLDepthStencilDescriptor for MTLDepthStencilDescriptorID {}

impl Clone for MTLDepthStencilDescriptorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLDepthStencilDescriptorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLDepthStencilDescriptorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLDepthStencilDescriptorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLDepthStencilDescriptorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLDepthStencilDescriptorID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLFunctionConstant : NSObject {
}

pub struct MTLFunctionConstantID(*mut std::os::raw::c_void);

impl MTLFunctionConstantID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLFunctionConstantID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLFunctionConstantID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLFunctionConstantID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLFunctionConstant").unwrap();
  }
}

impl NSObject for MTLFunctionConstantID {}
impl MTLFunctionConstant for MTLFunctionConstantID {}

impl Clone for MTLFunctionConstantID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLFunctionConstantID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLFunctionConstantID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLFunctionConstantID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLFunctionConstantID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLFunctionConstantID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLFunctionConstantValues : NSObject {
}

pub struct MTLFunctionConstantValuesID(*mut std::os::raw::c_void);

impl MTLFunctionConstantValuesID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLFunctionConstantValuesID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLFunctionConstantValuesID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLFunctionConstantValuesID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLFunctionConstantValues").unwrap();
  }
}

impl NSObject for MTLFunctionConstantValuesID {}
impl MTLFunctionConstantValues for MTLFunctionConstantValuesID {}

impl Clone for MTLFunctionConstantValuesID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLFunctionConstantValuesID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLFunctionConstantValuesID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLFunctionConstantValuesID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLFunctionConstantValuesID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLFunctionConstantValuesID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLHeapDescriptor : NSObject {
}

pub struct MTLHeapDescriptorID(*mut std::os::raw::c_void);

impl MTLHeapDescriptorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLHeapDescriptorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLHeapDescriptorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLHeapDescriptorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLHeapDescriptor").unwrap();
  }
}

impl NSObject for MTLHeapDescriptorID {}
impl MTLHeapDescriptor for MTLHeapDescriptorID {}

impl Clone for MTLHeapDescriptorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLHeapDescriptorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLHeapDescriptorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLHeapDescriptorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLHeapDescriptorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLHeapDescriptorID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLRenderPassAttachmentDescriptor : NSObject {
}

pub struct MTLRenderPassAttachmentDescriptorID(*mut std::os::raw::c_void);

impl MTLRenderPassAttachmentDescriptorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderPassAttachmentDescriptorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLRenderPassAttachmentDescriptorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLRenderPassAttachmentDescriptorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLRenderPassAttachmentDescriptor").unwrap();
  }
}

impl NSObject for MTLRenderPassAttachmentDescriptorID {}
impl MTLRenderPassAttachmentDescriptor for MTLRenderPassAttachmentDescriptorID {}

impl Clone for MTLRenderPassAttachmentDescriptorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLRenderPassAttachmentDescriptorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLRenderPassAttachmentDescriptorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderPassAttachmentDescriptorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLRenderPassAttachmentDescriptorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLRenderPassAttachmentDescriptorID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLRenderPassColorAttachmentDescriptor : MTLRenderPassAttachmentDescriptor + NSObject {
}

pub struct MTLRenderPassColorAttachmentDescriptorID(*mut std::os::raw::c_void);

impl MTLRenderPassColorAttachmentDescriptorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderPassColorAttachmentDescriptorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLRenderPassColorAttachmentDescriptorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLRenderPassColorAttachmentDescriptorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLRenderPassColorAttachmentDescriptor").unwrap();
  }
}

impl MTLRenderPassAttachmentDescriptor for MTLRenderPassColorAttachmentDescriptorID {}
impl NSObject for MTLRenderPassColorAttachmentDescriptorID {}
impl MTLRenderPassColorAttachmentDescriptor for MTLRenderPassColorAttachmentDescriptorID {}

impl Clone for MTLRenderPassColorAttachmentDescriptorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLRenderPassColorAttachmentDescriptorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLRenderPassColorAttachmentDescriptorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderPassColorAttachmentDescriptorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLRenderPassColorAttachmentDescriptorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLRenderPassColorAttachmentDescriptorID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLRenderPassColorAttachmentDescriptorArray : NSObject {
  fn object_at_index(&self, index: NSUInteger) -> MTLRenderPassColorAttachmentDescriptorID where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(objectAtIndex:), (index,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: MTLRenderPassColorAttachmentDescriptorID = r;

          return result.retain();
        }
      }
    }
  }

  fn object_at_indexed_subscript(&self, index: NSUInteger) -> MTLRenderPassColorAttachmentDescriptorID where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(objectAtIndexedSubscript:), (index,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: MTLRenderPassColorAttachmentDescriptorID = r;

          return result.retain();
        }
      }
    }
  }
}

pub struct MTLRenderPassColorAttachmentDescriptorArrayID(*mut std::os::raw::c_void);

impl MTLRenderPassColorAttachmentDescriptorArrayID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderPassColorAttachmentDescriptorArrayID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLRenderPassColorAttachmentDescriptorArrayID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLRenderPassColorAttachmentDescriptorArrayID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLRenderPassColorAttachmentDescriptorArray").unwrap();
  }
}

impl NSObject for MTLRenderPassColorAttachmentDescriptorArrayID {}
impl MTLRenderPassColorAttachmentDescriptorArray for MTLRenderPassColorAttachmentDescriptorArrayID {}

impl Clone for MTLRenderPassColorAttachmentDescriptorArrayID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLRenderPassColorAttachmentDescriptorArrayID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLRenderPassColorAttachmentDescriptorArrayID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderPassColorAttachmentDescriptorArrayID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLRenderPassColorAttachmentDescriptorArrayID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLRenderPassColorAttachmentDescriptorArrayID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLRenderPassDepthAttachmentDescriptor : MTLRenderPassAttachmentDescriptor + NSObject {
}

pub struct MTLRenderPassDepthAttachmentDescriptorID(*mut std::os::raw::c_void);

impl MTLRenderPassDepthAttachmentDescriptorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderPassDepthAttachmentDescriptorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLRenderPassDepthAttachmentDescriptorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLRenderPassDepthAttachmentDescriptorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLRenderPassDepthAttachmentDescriptor").unwrap();
  }
}

impl MTLRenderPassAttachmentDescriptor for MTLRenderPassDepthAttachmentDescriptorID {}
impl NSObject for MTLRenderPassDepthAttachmentDescriptorID {}
impl MTLRenderPassDepthAttachmentDescriptor for MTLRenderPassDepthAttachmentDescriptorID {}

impl Clone for MTLRenderPassDepthAttachmentDescriptorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLRenderPassDepthAttachmentDescriptorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLRenderPassDepthAttachmentDescriptorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderPassDepthAttachmentDescriptorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLRenderPassDepthAttachmentDescriptorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLRenderPassDepthAttachmentDescriptorID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
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

pub trait MTLRenderPassStencilAttachmentDescriptor : MTLRenderPassAttachmentDescriptor + NSObject {
}

pub struct MTLRenderPassStencilAttachmentDescriptorID(*mut std::os::raw::c_void);

impl MTLRenderPassStencilAttachmentDescriptorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderPassStencilAttachmentDescriptorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLRenderPassStencilAttachmentDescriptorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLRenderPassStencilAttachmentDescriptorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLRenderPassStencilAttachmentDescriptor").unwrap();
  }
}

impl MTLRenderPassAttachmentDescriptor for MTLRenderPassStencilAttachmentDescriptorID {}
impl NSObject for MTLRenderPassStencilAttachmentDescriptorID {}
impl MTLRenderPassStencilAttachmentDescriptor for MTLRenderPassStencilAttachmentDescriptorID {}

impl Clone for MTLRenderPassStencilAttachmentDescriptorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLRenderPassStencilAttachmentDescriptorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLRenderPassStencilAttachmentDescriptorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderPassStencilAttachmentDescriptorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLRenderPassStencilAttachmentDescriptorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLRenderPassStencilAttachmentDescriptorID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLRenderPipelineColorAttachmentDescriptor : NSObject {
  fn pixel_format(&self) -> MTLPixelFormat where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(pixelFormat), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_pixel_format(&self, pixel_format: MTLPixelFormat) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setPixelFormat:), (pixel_format,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn write_mask(&self) -> MTLColorWriteMask where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(writeMask), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_write_mask(&self, write_mask: MTLColorWriteMask) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setWriteMask:), (write_mask,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn is_blending_enabled(&self) -> bool where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(isBlendingEnabled), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_blending_enabled(&self, blending_enabled: bool) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setBlendingEnabled:), (blending_enabled,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn alpha_blend_operation(&self) -> MTLBlendOperation where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(alphaBlendOperation), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_alpha_blend_operation(&self, alpha_blend_operation: MTLBlendOperation) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setAlphaBlendOperation:), (alpha_blend_operation,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn rgb_blend_operation(&self) -> MTLBlendOperation where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(rgbBlendOperation), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_rgb_blend_operation(&self, rgb_blend_operation: MTLBlendOperation) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setRgbBlendOperation:), (rgb_blend_operation,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn destination_alpha_blend_factor(&self) -> MTLBlendFactor where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(destinationAlphaBlendFactor), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_destination_alpha_blend_factor(&self, destination_alpha_blend_factor: MTLBlendFactor) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setDestinationAlphaBlendFactor:), (destination_alpha_blend_factor,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn destination_rgb_blend_factor(&self) -> MTLBlendFactor where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(destinationRGBBlendFactor), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_destination_rgb_blend_factor(&self, destination_rgb_blend_factor: MTLBlendFactor) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setDestinationRGBBlendFactor:), (destination_rgb_blend_factor,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn source_alpha_blend_factor(&self) -> MTLBlendFactor where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(sourceAlphaBlendFactor), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_source_alpha_blend_factor(&self, source_alpha_blend_factor: MTLBlendFactor) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setSourceAlphaBlendFactor:), (source_alpha_blend_factor,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn source_rgb_blend_factor(&self) -> MTLBlendFactor where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(sourceRGBBlendFactor), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_source_rgb_blend_factor(&self, source_rgb_blend_factor: MTLBlendFactor) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setSourceRGBBlendFactor:), (source_rgb_blend_factor,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }
}

pub struct MTLRenderPipelineColorAttachmentDescriptorID(*mut std::os::raw::c_void);

impl MTLRenderPipelineColorAttachmentDescriptorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderPipelineColorAttachmentDescriptorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLRenderPipelineColorAttachmentDescriptorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLRenderPipelineColorAttachmentDescriptorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLRenderPipelineColorAttachmentDescriptor").unwrap();
  }
}

impl NSObject for MTLRenderPipelineColorAttachmentDescriptorID {}
impl MTLRenderPipelineColorAttachmentDescriptor for MTLRenderPipelineColorAttachmentDescriptorID {}

impl Clone for MTLRenderPipelineColorAttachmentDescriptorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLRenderPipelineColorAttachmentDescriptorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLRenderPipelineColorAttachmentDescriptorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderPipelineColorAttachmentDescriptorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLRenderPipelineColorAttachmentDescriptorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLRenderPipelineColorAttachmentDescriptorID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLRenderPipelineColorAttachmentDescriptorArray : NSObject {
  fn object_at_index(&self, index: NSUInteger) -> MTLRenderPipelineColorAttachmentDescriptorID where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(objectAtIndex:), (index,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: MTLRenderPipelineColorAttachmentDescriptorID = r;

          return result.retain();
        }
      }
    }
  }

  fn object_at_indexed_subscript(&self, index: NSUInteger) -> MTLRenderPipelineColorAttachmentDescriptorID where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(objectAtIndexedSubscript:), (index,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: MTLRenderPipelineColorAttachmentDescriptorID = r;

          return result.retain();
        }
      }
    }
  }
}

pub struct MTLRenderPipelineColorAttachmentDescriptorArrayID(*mut std::os::raw::c_void);

impl MTLRenderPipelineColorAttachmentDescriptorArrayID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderPipelineColorAttachmentDescriptorArrayID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLRenderPipelineColorAttachmentDescriptorArrayID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLRenderPipelineColorAttachmentDescriptorArrayID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLRenderPipelineColorAttachmentDescriptorArray").unwrap();
  }
}

impl NSObject for MTLRenderPipelineColorAttachmentDescriptorArrayID {}
impl MTLRenderPipelineColorAttachmentDescriptorArray for MTLRenderPipelineColorAttachmentDescriptorArrayID {}

impl Clone for MTLRenderPipelineColorAttachmentDescriptorArrayID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLRenderPipelineColorAttachmentDescriptorArrayID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLRenderPipelineColorAttachmentDescriptorArrayID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderPipelineColorAttachmentDescriptorArrayID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLRenderPipelineColorAttachmentDescriptorArrayID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLRenderPipelineColorAttachmentDescriptorArrayID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLRenderPipelineDescriptor : NSObject {
  fn init(self) -> Self where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(init), ()) {
        Err(s) => panic!("{}", s),
        Ok(result) => {
          std::mem::forget(self);

          return result;
        }
      }
    }
  }

  fn fragment_function(&self) -> MTLFunctionID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(fragmentFunction), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: MTLFunctionID = r;

          return r.retain();
        }
      }
    }
  }

  fn set_fragment_function<T: 'static + ObjectiveC + MTLFunction>(&self, fragment_function: T) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setFragmentFunction:), (fragment_function.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn vertex_function(&self) -> MTLFunctionID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(vertexFunction), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: MTLFunctionID = r;

          return r.retain();
        }
      }
    }
  }

  fn set_vertex_function<T: 'static + ObjectiveC + MTLFunction>(&self, vertex_function: T) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setVertexFunction:), (vertex_function.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn vertex_descriptor(&self) -> MTLVertexDescriptorID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(vertexDescriptor), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: MTLVertexDescriptorID = r;

          return r.retain();
        }
      }
    }
  }

  fn set_vertex_descriptor<T: 'static + ObjectiveC + MTLVertexDescriptor>(&self, vertex_descriptor: T) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setVertexDescriptor:), (vertex_descriptor.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn reset(&self) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(reset), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn color_attachments(&self) -> MTLRenderPipelineColorAttachmentDescriptorArrayID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(colorAttachments), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: MTLRenderPipelineColorAttachmentDescriptorArrayID = r;

          return r.retain();
        }
      }
    }
  }

  fn set_color_attachments<T: 'static + ObjectiveC + MTLRenderPipelineColorAttachmentDescriptorArray>(&self, color_attachments: T) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setColorAttachments:), (color_attachments.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn depth_attachment_pixel_format(&self) -> MTLPixelFormat where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(depthAttachmentPixelFormat), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_depth_attachment_pixel_format(&self, depth_attachment_pixel_format: MTLPixelFormat) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setDepthAttachmentPixelFormat:), (depth_attachment_pixel_format,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn stencil_attachment_pixel_format(&self) -> MTLPixelFormat where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(stencilAttachmentPixelFormat), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_stencil_attachment_pixel_format(&self, stencil_attachment_pixel_format: MTLPixelFormat) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setStencilAttachmentPixelFormat:), (stencil_attachment_pixel_format,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
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

pub struct MTLRenderPipelineDescriptorID(*mut std::os::raw::c_void);

impl MTLRenderPipelineDescriptorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderPipelineDescriptorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLRenderPipelineDescriptorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLRenderPipelineDescriptorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLRenderPipelineDescriptor").unwrap();
  }
}

impl NSObject for MTLRenderPipelineDescriptorID {}
impl MTLRenderPipelineDescriptor for MTLRenderPipelineDescriptorID {}

impl Clone for MTLRenderPipelineDescriptorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLRenderPipelineDescriptorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLRenderPipelineDescriptorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderPipelineDescriptorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLRenderPipelineDescriptorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLRenderPipelineDescriptorID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLRenderPipelineReflection : NSObject {
}

pub struct MTLRenderPipelineReflectionID(*mut std::os::raw::c_void);

impl MTLRenderPipelineReflectionID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderPipelineReflectionID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLRenderPipelineReflectionID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLRenderPipelineReflectionID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLRenderPipelineReflection").unwrap();
  }
}

impl NSObject for MTLRenderPipelineReflectionID {}
impl MTLRenderPipelineReflection for MTLRenderPipelineReflectionID {}

impl Clone for MTLRenderPipelineReflectionID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLRenderPipelineReflectionID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLRenderPipelineReflectionID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLRenderPipelineReflectionID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLRenderPipelineReflectionID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLRenderPipelineReflectionID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLSamplerDescriptor : NSObject {
}

pub struct MTLSamplerDescriptorID(*mut std::os::raw::c_void);

impl MTLSamplerDescriptorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLSamplerDescriptorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLSamplerDescriptorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLSamplerDescriptorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLSamplerDescriptor").unwrap();
  }
}

impl NSObject for MTLSamplerDescriptorID {}
impl MTLSamplerDescriptor for MTLSamplerDescriptorID {}

impl Clone for MTLSamplerDescriptorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLSamplerDescriptorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLSamplerDescriptorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLSamplerDescriptorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLSamplerDescriptorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLSamplerDescriptorID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLStageInputOutputDescriptor : NSObject {
}

pub struct MTLStageInputOutputDescriptorID(*mut std::os::raw::c_void);

impl MTLStageInputOutputDescriptorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLStageInputOutputDescriptorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLStageInputOutputDescriptorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLStageInputOutputDescriptorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLStageInputOutputDescriptor").unwrap();
  }
}

impl NSObject for MTLStageInputOutputDescriptorID {}
impl MTLStageInputOutputDescriptor for MTLStageInputOutputDescriptorID {}

impl Clone for MTLStageInputOutputDescriptorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLStageInputOutputDescriptorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLStageInputOutputDescriptorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLStageInputOutputDescriptorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLStageInputOutputDescriptorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLStageInputOutputDescriptorID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLStencilDescriptor : NSObject {
}

pub struct MTLStencilDescriptorID(*mut std::os::raw::c_void);

impl MTLStencilDescriptorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLStencilDescriptorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLStencilDescriptorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLStencilDescriptorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLStencilDescriptor").unwrap();
  }
}

impl NSObject for MTLStencilDescriptorID {}
impl MTLStencilDescriptor for MTLStencilDescriptorID {}

impl Clone for MTLStencilDescriptorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLStencilDescriptorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLStencilDescriptorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLStencilDescriptorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLStencilDescriptorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLStencilDescriptorID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLStructMember : NSObject {
}

pub struct MTLStructMemberID(*mut std::os::raw::c_void);

impl MTLStructMemberID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLStructMemberID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLStructMemberID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLStructMemberID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLStructMember").unwrap();
  }
}

impl NSObject for MTLStructMemberID {}
impl MTLStructMember for MTLStructMemberID {}

impl Clone for MTLStructMemberID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLStructMemberID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLStructMemberID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLStructMemberID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLStructMemberID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLStructMemberID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLStructType : NSObject {
}

pub struct MTLStructTypeID(*mut std::os::raw::c_void);

impl MTLStructTypeID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLStructTypeID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLStructTypeID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLStructTypeID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLStructType").unwrap();
  }
}

impl NSObject for MTLStructTypeID {}
impl MTLStructType for MTLStructTypeID {}

impl Clone for MTLStructTypeID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLStructTypeID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLStructTypeID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLStructTypeID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLStructTypeID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLStructTypeID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLTextureDescriptor : NSObject {
}

pub struct MTLTextureDescriptorID(*mut std::os::raw::c_void);

impl MTLTextureDescriptorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLTextureDescriptorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLTextureDescriptorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLTextureDescriptorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLTextureDescriptor").unwrap();
  }
}

impl NSObject for MTLTextureDescriptorID {}
impl MTLTextureDescriptor for MTLTextureDescriptorID {}

impl Clone for MTLTextureDescriptorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLTextureDescriptorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLTextureDescriptorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLTextureDescriptorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLTextureDescriptorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLTextureDescriptorID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLVertexAttribute : NSObject {
}

pub struct MTLVertexAttributeID(*mut std::os::raw::c_void);

impl MTLVertexAttributeID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLVertexAttributeID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLVertexAttributeID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLVertexAttributeID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLVertexAttribute").unwrap();
  }
}

impl NSObject for MTLVertexAttributeID {}
impl MTLVertexAttribute for MTLVertexAttributeID {}

impl Clone for MTLVertexAttributeID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLVertexAttributeID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLVertexAttributeID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLVertexAttributeID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLVertexAttributeID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLVertexAttributeID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLVertexAttributeDescriptor : NSObject {
}

pub struct MTLVertexAttributeDescriptorID(*mut std::os::raw::c_void);

impl MTLVertexAttributeDescriptorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLVertexAttributeDescriptorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLVertexAttributeDescriptorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLVertexAttributeDescriptorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLVertexAttributeDescriptor").unwrap();
  }
}

impl NSObject for MTLVertexAttributeDescriptorID {}
impl MTLVertexAttributeDescriptor for MTLVertexAttributeDescriptorID {}

impl Clone for MTLVertexAttributeDescriptorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLVertexAttributeDescriptorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLVertexAttributeDescriptorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLVertexAttributeDescriptorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLVertexAttributeDescriptorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLVertexAttributeDescriptorID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLVertexAttributeDescriptorArray : NSObject {
  fn object_at_index(&self, index: NSUInteger) -> MTLVertexAttributeDescriptorID where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(objectAtIndex:), (index,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: MTLVertexAttributeDescriptorID = r;

          return result.retain();
        }
      }
    }
  }

  fn object_at_indexed_subscript(&self, index: NSUInteger) -> MTLVertexAttributeDescriptorID where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(objectAtIndexedSubscript:), (index,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: MTLVertexAttributeDescriptorID = r;

          return result.retain();
        }
      }
    }
  }
}

pub struct MTLVertexAttributeDescriptorArrayID(*mut std::os::raw::c_void);

impl MTLVertexAttributeDescriptorArrayID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLVertexAttributeDescriptorArrayID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLVertexAttributeDescriptorArrayID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLVertexAttributeDescriptorArrayID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLVertexAttributeDescriptorArray").unwrap();
  }
}

impl NSObject for MTLVertexAttributeDescriptorArrayID {}
impl MTLVertexAttributeDescriptorArray for MTLVertexAttributeDescriptorArrayID {}

impl Clone for MTLVertexAttributeDescriptorArrayID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLVertexAttributeDescriptorArrayID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLVertexAttributeDescriptorArrayID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLVertexAttributeDescriptorArrayID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLVertexAttributeDescriptorArrayID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLVertexAttributeDescriptorArrayID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLVertexBufferLayoutDescriptor : NSObject {
}

pub struct MTLVertexBufferLayoutDescriptorID(*mut std::os::raw::c_void);

impl MTLVertexBufferLayoutDescriptorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLVertexBufferLayoutDescriptorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLVertexBufferLayoutDescriptorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLVertexBufferLayoutDescriptorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLVertexBufferLayoutDescriptor").unwrap();
  }
}

impl NSObject for MTLVertexBufferLayoutDescriptorID {}
impl MTLVertexBufferLayoutDescriptor for MTLVertexBufferLayoutDescriptorID {}

impl Clone for MTLVertexBufferLayoutDescriptorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLVertexBufferLayoutDescriptorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLVertexBufferLayoutDescriptorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLVertexBufferLayoutDescriptorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLVertexBufferLayoutDescriptorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLVertexBufferLayoutDescriptorID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLVertexBufferLayoutDescriptorArray : NSObject {
  fn object_at_index(&self, index: NSUInteger) -> MTLVertexBufferLayoutDescriptorID where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(objectAtIndex:), (index,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: MTLVertexBufferLayoutDescriptorID = r;

          return result.retain();
        }
      }
    }
  }

  fn object_at_indexed_subscript(&self, index: NSUInteger) -> MTLVertexBufferLayoutDescriptorID where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(objectAtIndexedSubscript:), (index,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: MTLVertexBufferLayoutDescriptorID = r;

          return result.retain();
        }
      }
    }
  }
}

pub struct MTLVertexBufferLayoutDescriptorArrayID(*mut std::os::raw::c_void);

impl MTLVertexBufferLayoutDescriptorArrayID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLVertexBufferLayoutDescriptorArrayID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLVertexBufferLayoutDescriptorArrayID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLVertexBufferLayoutDescriptorArrayID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLVertexBufferLayoutDescriptorArray").unwrap();
  }
}

impl NSObject for MTLVertexBufferLayoutDescriptorArrayID {}
impl MTLVertexBufferLayoutDescriptorArray for MTLVertexBufferLayoutDescriptorArrayID {}

impl Clone for MTLVertexBufferLayoutDescriptorArrayID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLVertexBufferLayoutDescriptorArrayID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLVertexBufferLayoutDescriptorArrayID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLVertexBufferLayoutDescriptorArrayID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLVertexBufferLayoutDescriptorArrayID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLVertexBufferLayoutDescriptorArrayID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTLVertexDescriptor : NSObject {
}

pub struct MTLVertexDescriptorID(*mut std::os::raw::c_void);

impl MTLVertexDescriptorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLVertexDescriptorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLVertexDescriptorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLVertexDescriptorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTLVertexDescriptor").unwrap();
  }
}

impl NSObject for MTLVertexDescriptorID {}
impl MTLVertexDescriptor for MTLVertexDescriptorID {}

impl Clone for MTLVertexDescriptorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLVertexDescriptorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLVertexDescriptorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLVertexDescriptorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLVertexDescriptorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLVertexDescriptorID {
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

pub trait MTLCommandEncoder : NSObject {
  fn end_encoding(&self) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(endEncoding), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn insert_debug_signpost<T5: 'static + NSString>(&self, string: T5) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(insertDebugSignpost), (string.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn push_debug_group<T5: 'static + NSString>(&self, string: T5) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(pushDebugGroup), (string.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn pop_debug_group(&self) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(popDebugGroup), ()) {
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

pub struct MTLCommandEncoderID(*mut std::os::raw::c_void);

impl MTLCommandEncoderID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLCommandEncoderID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTLCommandEncoderID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTLCommandEncoderID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for MTLCommandEncoderID {}
impl MTLCommandEncoder for MTLCommandEncoderID {}

impl Clone for MTLCommandEncoderID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTLCommandEncoderID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTLCommandEncoderID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTLCommandEncoderID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTLCommandEncoderID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTLCommandEncoderID {
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
  fn set_blend_color_red_green_blue_alpha(&self, red: f32, green: f32, blue: f32, alpha: f32) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(setBlendColorRed:green:blue:alpha:), (red, green, blue, alpha)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn set_cull_mode(&self, cull_mode: MTLCullMode) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(setCullMode:), (cull_mode,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn set_depth_bias_slope_scale_clamp(&self, depth_bias: f32, slope_scale: f32, clamp: f32) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(setDepthBias:slopeScale:clamp:), (depth_bias, slope_scale, clamp)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn set_depth_clip_mode(&self, depth_clip_mode: MTLDepthClipMode) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(setDepthClipMode:), (depth_clip_mode,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn set_depth_stencil_state<T5: 'static + MTLDepthStencilState>(&self, depth_stencil_state: T5) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(setDepthStencilState:), (depth_stencil_state.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn set_front_facing_winding(&self, front_facing_winding: MTLWinding) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(setFrontFacingWinding:), (front_facing_winding,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn set_render_pipeline_state<T5: 'static + MTLRenderPipelineState>(&self, pipeline_state: T5) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(setRenderPipelineState:), (pipeline_state.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn set_stencil_front_reference_value_back_reference_value(&self, front_reference_value: u32, back_reference_value: u32) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(setStencilFrontReferenceValue:backReferenceValue:), (front_reference_value, back_reference_value)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn set_stencil_reference_value(&self, reference_value: u32) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(setStencilReferenceValue:), (reference_value,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn set_triangle_fill_mode(&self, fill_mode: MTLTriangleFillMode) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(setTriangleFillMode:), (fill_mode,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn set_visibility_result_mode_offset(&self, mode: MTLVisibilityResultMode, offset: NSUInteger) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(setVisibilityResultMode:offset:), (mode, offset)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn set_vertex_buffer_offset_at_index<T5: 'static + MTLBuffer>(&self, buffer: T5, offset: NSUInteger, index: NSUInteger) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(setVertexBuffer:offset:atIndex:), (buffer.as_ptr(), offset, index)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn draw_primitives_vertex_start_vertex_count(&self, primitive_type: MTLPrimitiveType, vertex_start: NSUInteger, vertex_count: NSUInteger) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(drawPrimitives:vertexStart:vertexCount:), (primitive_type, vertex_start, vertex_count)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn draw_indexed_primitives_index_count_index_type_index_buffer_index_buffer_offset<T5: 'static + MTLBuffer>(&self, primitive_type: MTLPrimitiveType, index_count: NSUInteger, index_type: MTLIndexType, index_buffer: T5, index_buffer_offset: NSUInteger) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:), (primitive_type, index_count, index_type, index_buffer.as_ptr(), index_buffer_offset)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }
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
