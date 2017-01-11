#![allow(non_upper_case_globals)]
use std;
use objc;
use super::ObjectiveC;
use foundation::*;

#[link(name = "ModelIO", kind = "framework")]
extern {}

bitflags! {
  pub flags MDLGeometryType: NSInteger {
    const MDLGeometryTypePoints = 0,
    const MDLGeometryTypeLines = 1,
    const MDLGeometryTypeTriangles = 2,
    const MDLGeometryTypeTriangleStrips = 3,
    const MDLGeometryTypeQuads = 4,
    const MDLGeometryTypeVariableTopology = 5,
  }
}

bitflags! {
  pub flags MDLIndexBitDepth: NSUInteger {
    const MDLIndexBitDepthInvalid = 0,
    const MDLIndexBitDepthUInt8 = 8,
    const MDLIndexBitDepthUInt16 = 16,
    const MDLIndexBitDepthUInt32 = 32,
  }
}

bitflags! {
  pub flags MDLVertexFormat: NSUInteger {
    const MDLVertexFormatInvalid = 0,
    const MDLVertexFormatPackedBit = 0x1000,
    const MDLVertexFormatUCharBits = 0x10000,
    const MDLVertexFormatCharBits = 0x20000,
    const MDLVertexFormatUCharNormalizedBits = 0x30000,
    const MDLVertexFormatCharNormalizedBits = 0x40000,
    const MDLVertexFormatUShortBits = 0x50000,
    const MDLVertexFormatShortBits = 0x60000,
    const MDLVertexFormatUShortNormalizedBits = 0x70000,
    const MDLVertexFormatShortNormalizedBits = 0x80000,
    const MDLVertexFormatUIntBits = 0x90000,
    const MDLVertexFormatIntBits = 0xA0000,
    const MDLVertexFormatHalfBits = 0xB0000,
    const MDLVertexFormatFloatBits = 0xC0000,
    const MDLVertexFormatUChar = MDLVertexFormatUCharBits.bits | 1,
    const MDLVertexFormatUChar2 = MDLVertexFormatUCharBits.bits | 2,
    const MDLVertexFormatUChar3 = MDLVertexFormatUCharBits.bits | 3,
    const MDLVertexFormatUChar4 = MDLVertexFormatUCharBits.bits | 4,
    const MDLVertexFormatChar = MDLVertexFormatCharBits.bits | 1,
    const MDLVertexFormatChar2 = MDLVertexFormatCharBits.bits | 2,
    const MDLVertexFormatChar3 = MDLVertexFormatCharBits.bits | 3,
    const MDLVertexFormatChar4 = MDLVertexFormatCharBits.bits | 4,
    const MDLVertexFormatUCharNormalized = MDLVertexFormatUCharNormalizedBits.bits | 1,
    const MDLVertexFormatUChar2Normalized = MDLVertexFormatUCharNormalizedBits.bits | 2,
    const MDLVertexFormatUChar3Normalized = MDLVertexFormatUCharNormalizedBits.bits | 3,
    const MDLVertexFormatUChar4Normalized = MDLVertexFormatUCharNormalizedBits.bits | 4,
    const MDLVertexFormatCharNormalized = MDLVertexFormatCharNormalizedBits.bits | 1,
    const MDLVertexFormatChar2Normalized = MDLVertexFormatCharNormalizedBits.bits | 2,
    const MDLVertexFormatChar3Normalized = MDLVertexFormatCharNormalizedBits.bits | 3,
    const MDLVertexFormatChar4Normalized = MDLVertexFormatCharNormalizedBits.bits | 4,
    const MDLVertexFormatUShort = MDLVertexFormatUShortBits.bits | 1,
    const MDLVertexFormatUShort2 = MDLVertexFormatUShortBits.bits | 2,
    const MDLVertexFormatUShort3 = MDLVertexFormatUShortBits.bits | 3,
    const MDLVertexFormatUShort4 = MDLVertexFormatUShortBits.bits | 4,
    const MDLVertexFormatShort = MDLVertexFormatShortBits.bits | 1,
    const MDLVertexFormatShort2 = MDLVertexFormatShortBits.bits | 2,
    const MDLVertexFormatShort3 = MDLVertexFormatShortBits.bits | 3,
    const MDLVertexFormatShort4 = MDLVertexFormatShortBits.bits | 4,
    const MDLVertexFormatUShortNormalized = MDLVertexFormatUShortNormalizedBits.bits | 1,
    const MDLVertexFormatUShort2Normalized = MDLVertexFormatUShortNormalizedBits.bits | 2,
    const MDLVertexFormatUShort3Normalized = MDLVertexFormatUShortNormalizedBits.bits | 3,
    const MDLVertexFormatUShort4Normalized = MDLVertexFormatUShortNormalizedBits.bits | 4,
    const MDLVertexFormatShortNormalized = MDLVertexFormatShortNormalizedBits.bits | 1,
    const MDLVertexFormatShort2Normalized = MDLVertexFormatShortNormalizedBits.bits | 2,
    const MDLVertexFormatShort3Normalized = MDLVertexFormatShortNormalizedBits.bits | 3,
    const MDLVertexFormatShort4Normalized = MDLVertexFormatShortNormalizedBits.bits | 4,
    const MDLVertexFormatUInt = MDLVertexFormatUIntBits.bits | 1,
    const MDLVertexFormatUInt2 = MDLVertexFormatUIntBits.bits | 2,
    const MDLVertexFormatUInt3 = MDLVertexFormatUIntBits.bits | 3,
    const MDLVertexFormatUInt4 = MDLVertexFormatUIntBits.bits | 4,
    const MDLVertexFormatInt = MDLVertexFormatIntBits.bits | 1,
    const MDLVertexFormatInt2 = MDLVertexFormatIntBits.bits | 2,
    const MDLVertexFormatInt3 = MDLVertexFormatIntBits.bits | 3,
    const MDLVertexFormatInt4 = MDLVertexFormatIntBits.bits | 4,
    const MDLVertexFormatHalf = MDLVertexFormatHalfBits.bits | 1,
    const MDLVertexFormatHalf2 = MDLVertexFormatHalfBits.bits | 2,
    const MDLVertexFormatHalf3 = MDLVertexFormatHalfBits.bits | 3,
    const MDLVertexFormatHalf4 = MDLVertexFormatHalfBits.bits | 4,
    const MDLVertexFormatFloat = MDLVertexFormatFloatBits.bits | 1,
    const MDLVertexFormatFloat2 = MDLVertexFormatFloatBits.bits | 2,
    const MDLVertexFormatFloat3 = MDLVertexFormatFloatBits.bits | 3,
    const MDLVertexFormatFloat4 = MDLVertexFormatFloatBits.bits | 4,
    const MDLVertexFormatInt1010102Normalized = MDLVertexFormatIntBits.bits | MDLVertexFormatPackedBit.bits | 4,
    const MDLVertexFormatUInt1010102Normalized = MDLVertexFormatUIntBits.bits | MDLVertexFormatPackedBit.bits | 4,
  }
}

pub trait MDLAsset : NSObject {
  fn init_with_url<T5: 'static + NSURL>(self, url: T5) -> Self where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(initWithURL:), (url.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(result) => {
          std::mem::forget(self);

          return result;
        }
      }
    }
  }

  fn init_with_url_vertex_descriptor_buffer_allocator<T5: 'static + NSURL, T4: 'static + MDLVertexDescriptor, T3: 'static + MDLMeshBufferAllocator>(self, url: T5, vertex_descriptor: T4, buffer_allocator: T3) -> Self where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(initWithURL:vertexDescriptor:bufferAllocator:), (url.as_ptr(), vertex_descriptor.as_ptr(), buffer_allocator.as_ptr())) {
        Err(s) => panic!("{}", s),
        Ok(result) => {
          std::mem::forget(self);

          return result;
        }
      }
    }
  }

  fn object_at_index<T5: 'static + MDLObject>(&self, index: NSUInteger) -> T5 where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(objectAtIndex:), (index,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: T5 = r;

          return result.retain();
        }
      }
    }
  }

  fn object_at_indexed_subscript<T5: 'static + MDLObject>(&self, index: NSUInteger) -> T5 where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(objectAtIndexedSubscript:), (index,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: T5 = r;

          return result.retain();
        }
      }
    }
  }

  fn count(&self) -> NSUInteger where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(count), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }
}

pub struct MDLAssetID(*mut std::os::raw::c_void);

impl MDLAssetID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLAssetID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLAssetID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLAssetID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLAsset").unwrap();
  }
}

impl NSObject for MDLAssetID {}
impl MDLAsset for MDLAssetID {}

impl Clone for MDLAssetID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLAssetID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLAssetID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLAssetID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLAssetID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLAssetID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLMesh : MDLObject + NSObject {
  fn submeshes(&self) -> NSMutableArrayID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(submeshes), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: NSMutableArrayID = r;

          return r.retain();
        }
      }
    }
  }

  fn set_submeshes<T: 'static + ObjectiveC + NSMutableArray>(&self, submeshes: T) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setSubmeshes:), (submeshes.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn vertex_buffers(&self) -> NSArrayID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(vertexBuffers), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: NSArrayID = r;

          return r.retain();
        }
      }
    }
  }

  fn set_vertex_buffers<T: 'static + ObjectiveC + NSArray>(&self, vertex_buffers: T) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setVertexBuffers:), (vertex_buffers.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn vertex_count(&self) -> NSUInteger where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(vertexCount), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_vertex_count(&self, vertex_count: NSUInteger) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setVertexCount:), (vertex_count,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn vertex_descriptor(&self) -> MDLVertexDescriptorID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(vertexDescriptor), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: MDLVertexDescriptorID = r;

          return r.retain();
        }
      }
    }
  }

  fn set_vertex_descriptor<T: 'static + ObjectiveC + MDLVertexDescriptor>(&self, vertex_descriptor: T) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setVertexDescriptor:), (vertex_descriptor.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }
}

pub struct MDLMeshID(*mut std::os::raw::c_void);

impl MDLMeshID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLMeshID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLMeshID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLMeshID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLMesh").unwrap();
  }
}

impl MDLObject for MDLMeshID {}
impl NSObject for MDLMeshID {}
impl MDLMesh for MDLMeshID {}

impl Clone for MDLMeshID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLMeshID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLMeshID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLMeshID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLMeshID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLMeshID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLMeshBufferData : NSObject {
}

pub struct MDLMeshBufferDataID(*mut std::os::raw::c_void);

impl MDLMeshBufferDataID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLMeshBufferDataID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLMeshBufferDataID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLMeshBufferDataID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLMeshBufferData").unwrap();
  }
}

impl NSObject for MDLMeshBufferDataID {}
impl MDLMeshBufferData for MDLMeshBufferDataID {}

impl Clone for MDLMeshBufferDataID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLMeshBufferDataID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLMeshBufferDataID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLMeshBufferDataID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLMeshBufferDataID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLMeshBufferDataID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLMeshBufferMap : NSObject {
  fn bytes(&self) -> *const std::os::raw::c_void where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(bytes), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }
}

pub struct MDLMeshBufferMapID(*mut std::os::raw::c_void);

impl MDLMeshBufferMapID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLMeshBufferMapID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLMeshBufferMapID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLMeshBufferMapID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLMeshBufferMap").unwrap();
  }
}

impl NSObject for MDLMeshBufferMapID {}
impl MDLMeshBufferMap for MDLMeshBufferMapID {}

impl Clone for MDLMeshBufferMapID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLMeshBufferMapID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLMeshBufferMapID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLMeshBufferMapID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLMeshBufferMapID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLMeshBufferMapID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLObject : NSObject {
}

pub struct MDLObjectID(*mut std::os::raw::c_void);

impl MDLObjectID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLObjectID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLObjectID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLObjectID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLObject").unwrap();
  }
}

impl NSObject for MDLObjectID {}
impl MDLObject for MDLObjectID {}

impl Clone for MDLObjectID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLObjectID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLObjectID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLObjectID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLObjectID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLObjectID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLSubmesh : NSObject {
  fn index_buffer(&self) -> MDLMeshBufferID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(indexBuffer), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: MDLMeshBufferID = r;

          return r.retain();
        }
      }
    }
  }

  fn index_count(&self) -> NSUInteger where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(indexCount), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn index_type(&self) -> MDLIndexBitDepth where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(indexType), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn geometry_type(&self) -> MDLGeometryType where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(geometryType), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }
}

pub struct MDLSubmeshID(*mut std::os::raw::c_void);

impl MDLSubmeshID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLSubmeshID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLSubmeshID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLSubmeshID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLSubmesh").unwrap();
  }
}

impl NSObject for MDLSubmeshID {}
impl MDLSubmesh for MDLSubmeshID {}

impl Clone for MDLSubmeshID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLSubmeshID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLSubmeshID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLSubmeshID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLSubmeshID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLSubmeshID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLVertexAttribute : NSObject {
  fn name(&self) -> NSStringID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(name), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: NSStringID = r;

          return r.retain();
        }
      }
    }
  }

  fn set_name<T: 'static + ObjectiveC + NSString>(&self, name: T) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setName:), (name.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn format(&self) -> MDLVertexFormat where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(format), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_format(&self, format: MDLVertexFormat) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setFormat:), (format,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn offset(&self) -> NSUInteger where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(offset), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_offset(&self, offset: NSUInteger) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setOffset:), (offset,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn buffer_index(&self) -> NSUInteger where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(bufferIndex), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_buffer_index(&self, buffer_index: NSUInteger) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setBufferIndex:), (buffer_index,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn initialization_value(&self) -> [f32; 4] where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(initializationValue), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_initialization_value(&self, initialization_value: [f32; 4]) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setInitializationValue:), (initialization_value,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }
}

pub struct MDLVertexAttributeID(*mut std::os::raw::c_void);

impl MDLVertexAttributeID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLVertexAttributeID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLVertexAttributeID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLVertexAttributeID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLVertexAttribute").unwrap();
  }
}

impl NSObject for MDLVertexAttributeID {}
impl MDLVertexAttribute for MDLVertexAttributeID {}

impl Clone for MDLVertexAttributeID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLVertexAttributeID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLVertexAttributeID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLVertexAttributeID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLVertexAttributeID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLVertexAttributeID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLVertexDescriptor : NSObject {
  fn attributes(&self) -> NSMutableArrayID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(attributes), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: NSMutableArrayID = r;

          return r.retain();
        }
      }
    }
  }

  fn set_attributes<T: 'static + ObjectiveC + NSMutableArray>(&self, attributes: T) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setAttributes:), (attributes.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }
}

pub struct MDLVertexDescriptorID(*mut std::os::raw::c_void);

impl MDLVertexDescriptorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLVertexDescriptorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLVertexDescriptorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLVertexDescriptorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLVertexDescriptor").unwrap();
  }
}

impl NSObject for MDLVertexDescriptorID {}
impl MDLVertexDescriptor for MDLVertexDescriptorID {}

impl Clone for MDLVertexDescriptorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLVertexDescriptorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLVertexDescriptorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLVertexDescriptorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLVertexDescriptorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLVertexDescriptorID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLMeshBuffer : NSObject {
  fn map(&self) -> MDLMeshBufferMapID where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(map), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: MDLMeshBufferMapID = r;

          return result.retain();
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

  fn set_length(&self, length: NSUInteger) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setLength:), (length,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }
}

pub struct MDLMeshBufferID(*mut std::os::raw::c_void);

impl MDLMeshBufferID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLMeshBufferID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLMeshBufferID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLMeshBufferID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLMeshBuffer").unwrap();
  }
}

impl NSObject for MDLMeshBufferID {}
impl MDLMeshBuffer for MDLMeshBufferID {}

impl Clone for MDLMeshBufferID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLMeshBufferID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLMeshBufferID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLMeshBufferID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLMeshBufferID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLMeshBufferID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLMeshBufferAllocator : NSObject {
}

pub struct MDLMeshBufferAllocatorID(*mut std::os::raw::c_void);

impl MDLMeshBufferAllocatorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLMeshBufferAllocatorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLMeshBufferAllocatorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLMeshBufferAllocatorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for MDLMeshBufferAllocatorID {}
impl MDLMeshBufferAllocator for MDLMeshBufferAllocatorID {}

impl Clone for MDLMeshBufferAllocatorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLMeshBufferAllocatorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLMeshBufferAllocatorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLMeshBufferAllocatorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLMeshBufferAllocatorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLMeshBufferAllocatorID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}
