#![allow(non_upper_case_globals)]
use std;
use objc;
use super::ObjectiveC;

use foundation::*;

#[link(name = "ModelIO", kind = "framework")]
extern {}

bitflags! {
  pub flags MDLCameraProjection: NSUInteger {
    const MDLCameraProjectionPerspective = 0,
    const MDLCameraProjectionOrthographic = 1,
  }
}

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

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MDLAxisAlignedBoundingBox {
  pub max_bounds: [f32; 3],
  pub min_bounds: [f32; 3],
}

pub trait MDLAreaLight : MDLPhysicallyPlausibleLight + MDLLight + MDLObject + NSObject {
}

pub struct MDLAreaLightID(*mut std::os::raw::c_void);

impl MDLAreaLightID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLAreaLightID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLAreaLightID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLAreaLightID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLAreaLight").unwrap();
  }
}

impl MDLPhysicallyPlausibleLight for MDLAreaLightID {}
impl MDLLight for MDLAreaLightID {}
impl MDLObject for MDLAreaLightID {}
impl NSObject for MDLAreaLightID {}
impl MDLAreaLight for MDLAreaLightID {}

impl Clone for MDLAreaLightID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLAreaLightID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLAreaLightID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLAreaLightID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLAreaLightID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLAsset : NSObject {
  fn init_with_url<T0: 'static + NSURL>(self, url: &T0) -> Self where Self: 'static + Sized {
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

  fn init_with_url_vertex_descriptor_buffer_allocator<T0: 'static + NSURL, T1: 'static + MDLVertexDescriptor, T2: 'static + MDLMeshBufferAllocator>(self, url: &T0, vertex_descriptor: &T1, buffer_allocator: &T2) -> Self where Self: 'static + Sized {
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

  fn object_at_index<T0: 'static + MDLObject>(&self, index: NSUInteger) -> T0 where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(objectAtIndex:), (index,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: T0 = r;

          return result.retain();
        }
      }
    }
  }

  fn object_at_indexed_subscript<T0: 'static + MDLObject>(&self, index: NSUInteger) -> T0 where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(objectAtIndexedSubscript:), (index,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: T0 = r;

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

pub trait MDLCamera : MDLObject + NSObject {
  fn frame_bounding_box_set_near_and_far(&self, bounding_box: MDLAxisAlignedBoundingBox, set_near_and_far: bool) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(frameBoundingBox:setNearAndFar:), (bounding_box, set_near_and_far)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn look_at(&self, focus_position: [f32; 3]) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(lookAt:), (focus_position,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn look_at_from(&self, focus_position: [f32; 3], camera_position: [f32; 3]) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(lookAt:from:), (focus_position, camera_position)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn projection_matrix(&self) -> [f32; 16] where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(projectionMatrix), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn near_visibility_distance(&self) -> f32 where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(nearVisibilityDistance), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_near_visibility_distance(&self, near_visibility_distance: f32) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setNearVisibilityDistance:), (near_visibility_distance,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn far_visibility_distance(&self) -> f32 where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(farVisibilityDistance), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_far_visibility_distance(&self, far_visibility_distance: f32) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setFarVisibilityDistance:), (far_visibility_distance,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn field_of_view(&self) -> f32 where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(fieldOfView), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_field_of_view(&self, field_of_view: f32) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setFieldOfView:), (field_of_view,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn ray_to_for_view_port(&self, pixel: [i32; 2], size: [i32; 2]) -> [f32; 3] where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(rayTo:forViewPort:), (pixel, size)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: [f32; 3] = r;

          return result;
        }
      }
    }
  }

  fn world_to_meters_conversion_scale(&self) -> f32 where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(worldToMetersConversionScale), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_world_to_meters_conversion_scale(&self, world_to_meters_conversion_scale: f32) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setWorldToMetersConversionScale:), (world_to_meters_conversion_scale,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn projection(&self) -> MDLCameraProjection where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(projection), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_projection(&self, projection: MDLCameraProjection) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setProjection:), (projection,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }
}

pub struct MDLCameraID(*mut std::os::raw::c_void);

impl MDLCameraID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLCameraID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLCameraID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLCameraID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLCamera").unwrap();
  }
}

impl MDLObject for MDLCameraID {}
impl NSObject for MDLCameraID {}
impl MDLCamera for MDLCameraID {}

impl Clone for MDLCameraID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLCameraID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLCameraID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLCameraID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLCameraID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLCheckerboardTexture : MDLTexture + NSObject {
}

pub struct MDLCheckerboardTextureID(*mut std::os::raw::c_void);

impl MDLCheckerboardTextureID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLCheckerboardTextureID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLCheckerboardTextureID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLCheckerboardTextureID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLCheckerboardTexture").unwrap();
  }
}

impl MDLTexture for MDLCheckerboardTextureID {}
impl NSObject for MDLCheckerboardTextureID {}
impl MDLCheckerboardTexture for MDLCheckerboardTextureID {}

impl Clone for MDLCheckerboardTextureID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLCheckerboardTextureID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLCheckerboardTextureID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLCheckerboardTextureID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLCheckerboardTextureID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLColorSwatchTexture : MDLTexture + NSObject {
}

pub struct MDLColorSwatchTextureID(*mut std::os::raw::c_void);

impl MDLColorSwatchTextureID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLColorSwatchTextureID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLColorSwatchTextureID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLColorSwatchTextureID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLColorSwatchTexture").unwrap();
  }
}

impl MDLTexture for MDLColorSwatchTextureID {}
impl NSObject for MDLColorSwatchTextureID {}
impl MDLColorSwatchTexture for MDLColorSwatchTextureID {}

impl Clone for MDLColorSwatchTextureID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLColorSwatchTextureID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLColorSwatchTextureID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLColorSwatchTextureID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLColorSwatchTextureID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLLight : MDLObject + NSObject {
}

pub struct MDLLightID(*mut std::os::raw::c_void);

impl MDLLightID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLLightID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLLightID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLLightID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLLight").unwrap();
  }
}

impl MDLObject for MDLLightID {}
impl NSObject for MDLLightID {}
impl MDLLight for MDLLightID {}

impl Clone for MDLLightID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLLightID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLLightID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLLightID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLLightID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLLightProbe : MDLLight + MDLObject + NSObject {
}

pub struct MDLLightProbeID(*mut std::os::raw::c_void);

impl MDLLightProbeID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLLightProbeID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLLightProbeID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLLightProbeID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLLightProbe").unwrap();
  }
}

impl MDLLight for MDLLightProbeID {}
impl MDLObject for MDLLightProbeID {}
impl NSObject for MDLLightProbeID {}
impl MDLLightProbe for MDLLightProbeID {}

impl Clone for MDLLightProbeID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLLightProbeID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLLightProbeID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLLightProbeID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLLightProbeID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLMaterial : NSObject {
}

pub struct MDLMaterialID(*mut std::os::raw::c_void);

impl MDLMaterialID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLMaterialID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLMaterialID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLMaterialID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLMaterial").unwrap();
  }
}

impl NSObject for MDLMaterialID {}
impl MDLMaterial for MDLMaterialID {}

impl Clone for MDLMaterialID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLMaterialID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLMaterialID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLMaterialID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLMaterialID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLMaterialProperty : NSObject {
}

pub struct MDLMaterialPropertyID(*mut std::os::raw::c_void);

impl MDLMaterialPropertyID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLMaterialPropertyID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLMaterialPropertyID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLMaterialPropertyID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLMaterialProperty").unwrap();
  }
}

impl NSObject for MDLMaterialPropertyID {}
impl MDLMaterialProperty for MDLMaterialPropertyID {}

impl Clone for MDLMaterialPropertyID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLMaterialPropertyID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLMaterialPropertyID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLMaterialPropertyID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLMaterialPropertyID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLMaterialPropertyConnection : NSObject {
}

pub struct MDLMaterialPropertyConnectionID(*mut std::os::raw::c_void);

impl MDLMaterialPropertyConnectionID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLMaterialPropertyConnectionID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLMaterialPropertyConnectionID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLMaterialPropertyConnectionID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLMaterialPropertyConnection").unwrap();
  }
}

impl NSObject for MDLMaterialPropertyConnectionID {}
impl MDLMaterialPropertyConnection for MDLMaterialPropertyConnectionID {}

impl Clone for MDLMaterialPropertyConnectionID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLMaterialPropertyConnectionID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLMaterialPropertyConnectionID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLMaterialPropertyConnectionID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLMaterialPropertyConnectionID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLMaterialPropertyGraph : MDLMaterialPropertyNode + NSObject {
}

pub struct MDLMaterialPropertyGraphID(*mut std::os::raw::c_void);

impl MDLMaterialPropertyGraphID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLMaterialPropertyGraphID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLMaterialPropertyGraphID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLMaterialPropertyGraphID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLMaterialPropertyGraph").unwrap();
  }
}

impl MDLMaterialPropertyNode for MDLMaterialPropertyGraphID {}
impl NSObject for MDLMaterialPropertyGraphID {}
impl MDLMaterialPropertyGraph for MDLMaterialPropertyGraphID {}

impl Clone for MDLMaterialPropertyGraphID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLMaterialPropertyGraphID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLMaterialPropertyGraphID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLMaterialPropertyGraphID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLMaterialPropertyGraphID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLMaterialPropertyNode : NSObject {
}

pub struct MDLMaterialPropertyNodeID(*mut std::os::raw::c_void);

impl MDLMaterialPropertyNodeID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLMaterialPropertyNodeID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLMaterialPropertyNodeID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLMaterialPropertyNodeID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLMaterialPropertyNode").unwrap();
  }
}

impl NSObject for MDLMaterialPropertyNodeID {}
impl MDLMaterialPropertyNode for MDLMaterialPropertyNodeID {}

impl Clone for MDLMaterialPropertyNodeID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLMaterialPropertyNodeID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLMaterialPropertyNodeID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLMaterialPropertyNodeID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLMaterialPropertyNodeID {
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

  fn set_submeshes<T: 'static + ObjectiveC + NSMutableArray>(&self, submeshes: &T) where Self: 'static + Sized {
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

  fn set_vertex_buffers<T: 'static + ObjectiveC + NSArray>(&self, vertex_buffers: &T) where Self: 'static + Sized {
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

  fn set_vertex_descriptor<T: 'static + ObjectiveC + MDLVertexDescriptor>(&self, vertex_descriptor: &T) where Self: 'static + Sized {
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

pub trait MDLMeshBufferDataAllocator : MDLMeshBufferAllocator + NSObject {
}

pub struct MDLMeshBufferDataAllocatorID(*mut std::os::raw::c_void);

impl MDLMeshBufferDataAllocatorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLMeshBufferDataAllocatorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLMeshBufferDataAllocatorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLMeshBufferDataAllocatorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLMeshBufferDataAllocator").unwrap();
  }
}

impl MDLMeshBufferAllocator for MDLMeshBufferDataAllocatorID {}
impl NSObject for MDLMeshBufferDataAllocatorID {}
impl MDLMeshBufferDataAllocator for MDLMeshBufferDataAllocatorID {}

impl Clone for MDLMeshBufferDataAllocatorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLMeshBufferDataAllocatorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLMeshBufferDataAllocatorID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLMeshBufferDataAllocatorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLMeshBufferDataAllocatorID {
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

pub trait MDLMeshBufferZoneDefault : MDLMeshBufferZone + NSObject {
}

pub struct MDLMeshBufferZoneDefaultID(*mut std::os::raw::c_void);

impl MDLMeshBufferZoneDefaultID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLMeshBufferZoneDefaultID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLMeshBufferZoneDefaultID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLMeshBufferZoneDefaultID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLMeshBufferZoneDefault").unwrap();
  }
}

impl MDLMeshBufferZone for MDLMeshBufferZoneDefaultID {}
impl NSObject for MDLMeshBufferZoneDefaultID {}
impl MDLMeshBufferZoneDefault for MDLMeshBufferZoneDefaultID {}

impl Clone for MDLMeshBufferZoneDefaultID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLMeshBufferZoneDefaultID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLMeshBufferZoneDefaultID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLMeshBufferZoneDefaultID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLMeshBufferZoneDefaultID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLNoiseTexture : MDLTexture + NSObject {
}

pub struct MDLNoiseTextureID(*mut std::os::raw::c_void);

impl MDLNoiseTextureID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLNoiseTextureID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLNoiseTextureID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLNoiseTextureID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLNoiseTexture").unwrap();
  }
}

impl MDLTexture for MDLNoiseTextureID {}
impl NSObject for MDLNoiseTextureID {}
impl MDLNoiseTexture for MDLNoiseTextureID {}

impl Clone for MDLNoiseTextureID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLNoiseTextureID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLNoiseTextureID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLNoiseTextureID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLNoiseTextureID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLNormalMapTexture : MDLTexture + NSObject {
}

pub struct MDLNormalMapTextureID(*mut std::os::raw::c_void);

impl MDLNormalMapTextureID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLNormalMapTextureID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLNormalMapTextureID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLNormalMapTextureID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLNormalMapTexture").unwrap();
  }
}

impl MDLTexture for MDLNormalMapTextureID {}
impl NSObject for MDLNormalMapTextureID {}
impl MDLNormalMapTexture for MDLNormalMapTextureID {}

impl Clone for MDLNormalMapTextureID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLNormalMapTextureID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLNormalMapTextureID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLNormalMapTextureID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLNormalMapTextureID {
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

pub trait MDLObjectContainer : NSObject {
}

pub struct MDLObjectContainerID(*mut std::os::raw::c_void);

impl MDLObjectContainerID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLObjectContainerID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLObjectContainerID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLObjectContainerID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLObjectContainer").unwrap();
  }
}

impl NSObject for MDLObjectContainerID {}
impl MDLObjectContainer for MDLObjectContainerID {}

impl Clone for MDLObjectContainerID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLObjectContainerID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLObjectContainerID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLObjectContainerID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLObjectContainerID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLPhotometricLight : MDLPhysicallyPlausibleLight + MDLLight + MDLObject + NSObject {
}

pub struct MDLPhotometricLightID(*mut std::os::raw::c_void);

impl MDLPhotometricLightID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLPhotometricLightID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLPhotometricLightID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLPhotometricLightID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLPhotometricLight").unwrap();
  }
}

impl MDLPhysicallyPlausibleLight for MDLPhotometricLightID {}
impl MDLLight for MDLPhotometricLightID {}
impl MDLObject for MDLPhotometricLightID {}
impl NSObject for MDLPhotometricLightID {}
impl MDLPhotometricLight for MDLPhotometricLightID {}

impl Clone for MDLPhotometricLightID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLPhotometricLightID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLPhotometricLightID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLPhotometricLightID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLPhotometricLightID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLPhysicallyPlausibleLight : MDLLight + MDLObject + NSObject {
}

pub struct MDLPhysicallyPlausibleLightID(*mut std::os::raw::c_void);

impl MDLPhysicallyPlausibleLightID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLPhysicallyPlausibleLightID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLPhysicallyPlausibleLightID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLPhysicallyPlausibleLightID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLPhysicallyPlausibleLight").unwrap();
  }
}

impl MDLLight for MDLPhysicallyPlausibleLightID {}
impl MDLObject for MDLPhysicallyPlausibleLightID {}
impl NSObject for MDLPhysicallyPlausibleLightID {}
impl MDLPhysicallyPlausibleLight for MDLPhysicallyPlausibleLightID {}

impl Clone for MDLPhysicallyPlausibleLightID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLPhysicallyPlausibleLightID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLPhysicallyPlausibleLightID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLPhysicallyPlausibleLightID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLPhysicallyPlausibleLightID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLPhysicallyPlausibleScatteringFunction : MDLScatteringFunction + NSObject {
}

pub struct MDLPhysicallyPlausibleScatteringFunctionID(*mut std::os::raw::c_void);

impl MDLPhysicallyPlausibleScatteringFunctionID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLPhysicallyPlausibleScatteringFunctionID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLPhysicallyPlausibleScatteringFunctionID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLPhysicallyPlausibleScatteringFunctionID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLPhysicallyPlausibleScatteringFunction").unwrap();
  }
}

impl MDLScatteringFunction for MDLPhysicallyPlausibleScatteringFunctionID {}
impl NSObject for MDLPhysicallyPlausibleScatteringFunctionID {}
impl MDLPhysicallyPlausibleScatteringFunction for MDLPhysicallyPlausibleScatteringFunctionID {}

impl Clone for MDLPhysicallyPlausibleScatteringFunctionID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLPhysicallyPlausibleScatteringFunctionID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLPhysicallyPlausibleScatteringFunctionID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLPhysicallyPlausibleScatteringFunctionID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLPhysicallyPlausibleScatteringFunctionID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLScatteringFunction : NSObject {
}

pub struct MDLScatteringFunctionID(*mut std::os::raw::c_void);

impl MDLScatteringFunctionID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLScatteringFunctionID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLScatteringFunctionID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLScatteringFunctionID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLScatteringFunction").unwrap();
  }
}

impl NSObject for MDLScatteringFunctionID {}
impl MDLScatteringFunction for MDLScatteringFunctionID {}

impl Clone for MDLScatteringFunctionID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLScatteringFunctionID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLScatteringFunctionID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLScatteringFunctionID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLScatteringFunctionID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLSkyCubeTexture : MDLTexture + NSObject {
}

pub struct MDLSkyCubeTextureID(*mut std::os::raw::c_void);

impl MDLSkyCubeTextureID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLSkyCubeTextureID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLSkyCubeTextureID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLSkyCubeTextureID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLSkyCubeTexture").unwrap();
  }
}

impl MDLTexture for MDLSkyCubeTextureID {}
impl NSObject for MDLSkyCubeTextureID {}
impl MDLSkyCubeTexture for MDLSkyCubeTextureID {}

impl Clone for MDLSkyCubeTextureID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLSkyCubeTextureID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLSkyCubeTextureID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLSkyCubeTextureID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLSkyCubeTextureID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLStereoscopicCamera : MDLCamera + MDLObject + NSObject {
}

pub struct MDLStereoscopicCameraID(*mut std::os::raw::c_void);

impl MDLStereoscopicCameraID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLStereoscopicCameraID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLStereoscopicCameraID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLStereoscopicCameraID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLStereoscopicCamera").unwrap();
  }
}

impl MDLCamera for MDLStereoscopicCameraID {}
impl MDLObject for MDLStereoscopicCameraID {}
impl NSObject for MDLStereoscopicCameraID {}
impl MDLStereoscopicCamera for MDLStereoscopicCameraID {}

impl Clone for MDLStereoscopicCameraID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLStereoscopicCameraID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLStereoscopicCameraID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLStereoscopicCameraID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLStereoscopicCameraID {
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

pub trait MDLSubmeshTopology : NSObject {
}

pub struct MDLSubmeshTopologyID(*mut std::os::raw::c_void);

impl MDLSubmeshTopologyID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLSubmeshTopologyID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLSubmeshTopologyID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLSubmeshTopologyID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLSubmeshTopology").unwrap();
  }
}

impl NSObject for MDLSubmeshTopologyID {}
impl MDLSubmeshTopology for MDLSubmeshTopologyID {}

impl Clone for MDLSubmeshTopologyID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLSubmeshTopologyID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLSubmeshTopologyID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLSubmeshTopologyID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLSubmeshTopologyID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLTexture : NSObject {
}

pub struct MDLTextureID(*mut std::os::raw::c_void);

impl MDLTextureID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLTextureID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLTextureID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLTextureID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLTexture").unwrap();
  }
}

impl NSObject for MDLTextureID {}
impl MDLTexture for MDLTextureID {}

impl Clone for MDLTextureID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLTextureID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLTextureID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLTextureID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLTextureID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLTextureFilter : NSObject {
}

pub struct MDLTextureFilterID(*mut std::os::raw::c_void);

impl MDLTextureFilterID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLTextureFilterID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLTextureFilterID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLTextureFilterID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLTextureFilter").unwrap();
  }
}

impl NSObject for MDLTextureFilterID {}
impl MDLTextureFilter for MDLTextureFilterID {}

impl Clone for MDLTextureFilterID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLTextureFilterID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLTextureFilterID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLTextureFilterID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLTextureFilterID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLTextureSampler : NSObject {
}

pub struct MDLTextureSamplerID(*mut std::os::raw::c_void);

impl MDLTextureSamplerID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLTextureSamplerID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLTextureSamplerID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLTextureSamplerID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLTextureSampler").unwrap();
  }
}

impl NSObject for MDLTextureSamplerID {}
impl MDLTextureSampler for MDLTextureSamplerID {}

impl Clone for MDLTextureSamplerID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLTextureSamplerID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLTextureSamplerID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLTextureSamplerID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLTextureSamplerID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLTransform : NSObject {
}

pub struct MDLTransformID(*mut std::os::raw::c_void);

impl MDLTransformID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLTransformID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLTransformID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLTransformID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLTransform").unwrap();
  }
}

impl NSObject for MDLTransformID {}
impl MDLTransform for MDLTransformID {}

impl Clone for MDLTransformID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLTransformID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLTransformID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLTransformID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLTransformID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLURLTexture : MDLTexture + NSObject {
  fn init_with_url_name<T0: 'static + NSURL, T1: 'static + NSString>(self, url: &T0, name: &T1) -> Self where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(initWithURL:name:), (url.as_ptr(), name.as_ptr())) {
        Err(s) => panic!("{}", s),
        Ok(result) => {
          std::mem::forget(self);

          return result;
        }
      }
    }
  }

  fn url(&self) -> NSURLID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(URL), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: NSURLID = r;

          return r.retain();
        }
      }
    }
  }

  fn set_url<T: 'static + ObjectiveC + NSURL>(&self, url: &T) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setURL:), (url.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }
}

pub struct MDLURLTextureID(*mut std::os::raw::c_void);

impl MDLURLTextureID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLURLTextureID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLURLTextureID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLURLTextureID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLURLTexture").unwrap();
  }
}

impl MDLTexture for MDLURLTextureID {}
impl NSObject for MDLURLTextureID {}
impl MDLURLTexture for MDLURLTextureID {}

impl Clone for MDLURLTextureID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLURLTextureID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLURLTextureID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLURLTextureID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLURLTextureID {
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

  fn set_name<T: 'static + ObjectiveC + NSString>(&self, name: &T) where Self: 'static + Sized {
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

pub trait MDLVertexAttributeData : NSObject {
}

pub struct MDLVertexAttributeDataID(*mut std::os::raw::c_void);

impl MDLVertexAttributeDataID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLVertexAttributeDataID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLVertexAttributeDataID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLVertexAttributeDataID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLVertexAttributeData").unwrap();
  }
}

impl NSObject for MDLVertexAttributeDataID {}
impl MDLVertexAttributeData for MDLVertexAttributeDataID {}

impl Clone for MDLVertexAttributeDataID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLVertexAttributeDataID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLVertexAttributeDataID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLVertexAttributeDataID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLVertexAttributeDataID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLVertexBufferLayout : NSObject {
}

pub struct MDLVertexBufferLayoutID(*mut std::os::raw::c_void);

impl MDLVertexBufferLayoutID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLVertexBufferLayoutID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLVertexBufferLayoutID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLVertexBufferLayoutID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLVertexBufferLayout").unwrap();
  }
}

impl NSObject for MDLVertexBufferLayoutID {}
impl MDLVertexBufferLayout for MDLVertexBufferLayoutID {}

impl Clone for MDLVertexBufferLayoutID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLVertexBufferLayoutID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLVertexBufferLayoutID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLVertexBufferLayoutID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLVertexBufferLayoutID {
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

  fn set_attributes<T: 'static + ObjectiveC + NSMutableArray>(&self, attributes: &T) where Self: 'static + Sized {
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

pub trait MDLVoxelArray : NSObject {
}

pub struct MDLVoxelArrayID(*mut std::os::raw::c_void);

impl MDLVoxelArrayID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLVoxelArrayID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLVoxelArrayID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLVoxelArrayID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MDLVoxelArray").unwrap();
  }
}

impl NSObject for MDLVoxelArrayID {}
impl MDLVoxelArray for MDLVoxelArrayID {}

impl Clone for MDLVoxelArrayID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLVoxelArrayID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLVoxelArrayID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLVoxelArrayID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLVoxelArrayID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLComponent : NSObject {
}

pub struct MDLComponentID(*mut std::os::raw::c_void);

impl MDLComponentID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLComponentID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLComponentID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLComponentID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for MDLComponentID {}
impl MDLComponent for MDLComponentID {}

impl Clone for MDLComponentID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLComponentID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLComponentID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLComponentID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLComponentID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLLightProbeIrradianceDataSource : NSObject {
}

pub struct MDLLightProbeIrradianceDataSourceID(*mut std::os::raw::c_void);

impl MDLLightProbeIrradianceDataSourceID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLLightProbeIrradianceDataSourceID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLLightProbeIrradianceDataSourceID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLLightProbeIrradianceDataSourceID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for MDLLightProbeIrradianceDataSourceID {}
impl MDLLightProbeIrradianceDataSource for MDLLightProbeIrradianceDataSourceID {}

impl Clone for MDLLightProbeIrradianceDataSourceID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLLightProbeIrradianceDataSourceID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLLightProbeIrradianceDataSourceID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLLightProbeIrradianceDataSourceID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLLightProbeIrradianceDataSourceID {
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

pub trait MDLMeshBufferZone : NSObject {
}

pub struct MDLMeshBufferZoneID(*mut std::os::raw::c_void);

impl MDLMeshBufferZoneID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLMeshBufferZoneID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLMeshBufferZoneID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLMeshBufferZoneID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for MDLMeshBufferZoneID {}
impl MDLMeshBufferZone for MDLMeshBufferZoneID {}

impl Clone for MDLMeshBufferZoneID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLMeshBufferZoneID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLMeshBufferZoneID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLMeshBufferZoneID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLMeshBufferZoneID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLNamed : NSObject {
}

pub struct MDLNamedID(*mut std::os::raw::c_void);

impl MDLNamedID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLNamedID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLNamedID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLNamedID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for MDLNamedID {}
impl MDLNamed for MDLNamedID {}

impl Clone for MDLNamedID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLNamedID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLNamedID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLNamedID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLNamedID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLObjectContainerComponent : NSObject {
}

pub struct MDLObjectContainerComponentID(*mut std::os::raw::c_void);

impl MDLObjectContainerComponentID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLObjectContainerComponentID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLObjectContainerComponentID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLObjectContainerComponentID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for MDLObjectContainerComponentID {}
impl MDLObjectContainerComponent for MDLObjectContainerComponentID {}

impl Clone for MDLObjectContainerComponentID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLObjectContainerComponentID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLObjectContainerComponentID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLObjectContainerComponentID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLObjectContainerComponentID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MDLTransformComponent : NSObject {
}

pub struct MDLTransformComponentID(*mut std::os::raw::c_void);

impl MDLTransformComponentID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MDLTransformComponentID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MDLTransformComponentID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MDLTransformComponentID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for MDLTransformComponentID {}
impl MDLTransformComponent for MDLTransformComponentID {}

impl Clone for MDLTransformComponentID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MDLTransformComponentID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MDLTransformComponentID {
  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MDLTransformComponentID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MDLTransformComponentID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}
