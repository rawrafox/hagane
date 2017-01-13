#![allow(non_upper_case_globals)]
use std;
use objc;
use super::ObjectiveC;

use cocoa::*;
use core_animation::*;
use core_graphics::*;
use foundation::*;
use metal::*;
use model_io::*;

#[link(name = "MetalKit", kind = "framework")]
extern {}

pub trait MTKMesh : NSObject {
  fn init_with_mesh_device_error<T0: 'static + MDLMesh, T1: 'static + MTLDevice>(self, mesh: T0, device: T1) -> Result<Self, NSErrorID> where Self: 'static + Sized {
    let mut error = NSErrorID::nil();

    unsafe {
      match objc::__send_message(self.as_object(), sel!(initWithMesh:device:error:), (mesh.as_ptr(), device.as_ptr(), &mut error)) {
        Err(s) => panic!("{}", s),
        Ok(result) => {
          std::mem::forget(self);

          if !error.is_nil() {
            return Err(error)
          }

          return Ok(result);
        }
      }
    }
  }

  fn submeshes(&self) -> NSArrayID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(submeshes), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: NSArrayID = r;

          return r.retain();
        }
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

  fn vertex_count(&self) -> NSUInteger where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(vertexCount), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
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
}

pub struct MTKMeshID(*mut std::os::raw::c_void);

impl MTKMeshID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTKMeshID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTKMeshID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTKMeshID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTKMesh").unwrap();
  }
}

impl NSObject for MTKMeshID {}
impl MTKMesh for MTKMeshID {}

impl Clone for MTKMeshID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTKMeshID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTKMeshID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTKMeshID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }

  fn as_mut_ptr(&mut self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTKMeshID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTKMeshID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTKMeshBuffer : NSObject {
  fn allocator(&self) -> MTKMeshBufferAllocatorID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(allocator), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: MTKMeshBufferAllocatorID = r;

          return r.retain();
        }
      }
    }
  }

  fn buffer(&self) -> MTLBufferID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(buffer), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: MTLBufferID = r;

          return r.retain();
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

  fn offset(&self) -> NSUInteger where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(offset), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }
}

pub struct MTKMeshBufferID(*mut std::os::raw::c_void);

impl MTKMeshBufferID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTKMeshBufferID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTKMeshBufferID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTKMeshBufferID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTKMeshBuffer").unwrap();
  }
}

impl NSObject for MTKMeshBufferID {}
impl MTKMeshBuffer for MTKMeshBufferID {}

impl Clone for MTKMeshBufferID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTKMeshBufferID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTKMeshBufferID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTKMeshBufferID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }

  fn as_mut_ptr(&mut self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTKMeshBufferID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTKMeshBufferID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTKMeshBufferAllocator : MDLMeshBufferAllocator + NSObject {
  fn init_with_device<T0: 'static + MTLDevice>(self, device: T0) -> Self where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(initWithDevice:), (device.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(result) => {
          std::mem::forget(self);

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

  fn set_device<T: 'static + ObjectiveC + MTLDevice>(&self, device: T) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setDevice:), (device.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }
}

pub struct MTKMeshBufferAllocatorID(*mut std::os::raw::c_void);

impl MTKMeshBufferAllocatorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTKMeshBufferAllocatorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTKMeshBufferAllocatorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTKMeshBufferAllocatorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTKMeshBufferAllocator").unwrap();
  }
}

impl MDLMeshBufferAllocator for MTKMeshBufferAllocatorID {}
impl NSObject for MTKMeshBufferAllocatorID {}
impl MTKMeshBufferAllocator for MTKMeshBufferAllocatorID {}

impl Clone for MTKMeshBufferAllocatorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTKMeshBufferAllocatorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTKMeshBufferAllocatorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTKMeshBufferAllocatorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }

  fn as_mut_ptr(&mut self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTKMeshBufferAllocatorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTKMeshBufferAllocatorID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTKSubmesh : NSObject {
  fn mesh(&self) -> MTKMeshID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(mesh), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: MTKMeshID = r;

          return r.retain();
        }
      }
    }
  }

  fn index_buffer(&self) -> MTKMeshBufferID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(indexBuffer), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: MTKMeshBufferID = r;

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

  fn index_type(&self) -> MTLIndexType where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(indexType), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn primitive_type(&self) -> MTLPrimitiveType where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(primitiveType), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

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
}

pub struct MTKSubmeshID(*mut std::os::raw::c_void);

impl MTKSubmeshID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTKSubmeshID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTKSubmeshID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTKSubmeshID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTKSubmesh").unwrap();
  }
}

impl NSObject for MTKSubmeshID {}
impl MTKSubmesh for MTKSubmeshID {}

impl Clone for MTKSubmeshID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTKSubmeshID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTKSubmeshID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTKSubmeshID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }

  fn as_mut_ptr(&mut self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTKSubmeshID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTKSubmeshID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTKTextureLoader : NSObject {
}

pub struct MTKTextureLoaderID(*mut std::os::raw::c_void);

impl MTKTextureLoaderID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTKTextureLoaderID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTKTextureLoaderID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTKTextureLoaderID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTKTextureLoader").unwrap();
  }
}

impl NSObject for MTKTextureLoaderID {}
impl MTKTextureLoader for MTKTextureLoaderID {}

impl Clone for MTKTextureLoaderID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTKTextureLoaderID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTKTextureLoaderID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTKTextureLoaderID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }

  fn as_mut_ptr(&mut self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTKTextureLoaderID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTKTextureLoaderID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTKView : NSView + NSObject {
  fn init_with_coder<T0: 'static + NSCoder>(self, coder: T0) -> Self where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(initWithCoder:), (coder.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(result) => {
          std::mem::forget(self);

          return result;
        }
      }
    }
  }

  fn init_with_frame_device<T1: 'static + MTLDevice>(self, frame_rect: CGRect, device: T1) -> Self where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(initWithFrame:device:), (frame_rect, device.as_ptr())) {
        Err(s) => panic!("{}", s),
        Ok(result) => {
          std::mem::forget(self);

          return result;
        }
      }
    }
  }

  fn delegate(&self) -> MTKViewDelegateID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(delegate), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: MTKViewDelegateID = r;

          return r.retain();
        }
      }
    }
  }

  fn set_delegate<T: 'static + ObjectiveC + MTKViewDelegate>(&self, delegate: T) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setDelegate:), (delegate.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
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

  fn set_device<T: 'static + ObjectiveC + MTLDevice>(&self, device: T) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setDevice:), (device.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn clear_color(&self) -> MTLClearColor where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(clearColor), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_clear_color(&self, clear_color: MTLClearColor) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setClearColor:), (clear_color,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn clear_depth(&self) -> f64 where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(clearDepth), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_clear_depth(&self, clear_depth: f64) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setClearDepth:), (clear_depth,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn clear_stencil(&self) -> u32 where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(clearStencil), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_clear_stencil(&self, clear_stencil: u32) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setClearStencil:), (clear_stencil,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn color_pixel_format(&self) -> MTLPixelFormat where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(colorPixelFormat), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_color_pixel_format(&self, color_pixel_format: MTLPixelFormat) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setColorPixelFormat:), (color_pixel_format,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn depth_stencil_pixel_format(&self) -> MTLPixelFormat where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(depthStencilPixelFormat), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_depth_stencil_pixel_format(&self, depth_stencil_pixel_format: MTLPixelFormat) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setDepthStencilPixelFormat:), (depth_stencil_pixel_format,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn sample_count(&self) -> NSUInteger where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(sampleCount), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_sample_count(&self, sample_count: NSUInteger) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setSampleCount:), (sample_count,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn current_render_pass_descriptor(&self) -> MTLRenderPassDescriptorID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(currentRenderPassDescriptor), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: MTLRenderPassDescriptorID = r;

          return r.retain();
        }
      }
    }
  }

  fn depth_stencil_texture(&self) -> MTLTextureID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(depthStencilTexture), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: MTLTextureID = r;

          return r.retain();
        }
      }
    }
  }

  fn multisample_color_texture(&self) -> MTLTextureID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(multisampleColorTexture), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: MTLTextureID = r;

          return r.retain();
        }
      }
    }
  }

  fn preferred_frames_per_second(&self) -> NSInteger where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(preferredFramesPerSecond), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_preferred_frames_per_second(&self, preferred_frames_per_second: NSInteger) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setPreferredFramesPerSecond:), (preferred_frames_per_second,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn is_paused(&self) -> bool where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(isPaused), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_paused(&self, paused: bool) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setPaused:), (paused,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn enable_set_needs_display(&self) -> bool where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(enableSetNeedsDisplay), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_enable_set_needs_display(&self, enable_set_needs_display: bool) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setEnableSetNeedsDisplay:), (enable_set_needs_display,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn draw(&self) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(draw), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn auto_resize_drawable(&self) -> bool where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(autoResizeDrawable), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_auto_resize_drawable(&self, auto_resize_drawable: bool) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setAutoResizeDrawable:), (auto_resize_drawable,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn current_drawable(&self) -> CAMetalDrawableID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(currentDrawable), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: CAMetalDrawableID = r;

          return r.retain();
        }
      }
    }
  }

  fn drawable_size(&self) -> CGSize where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(drawableSize), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_drawable_size(&self, drawable_size: CGSize) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setDrawableSize:), (drawable_size,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn framebuffer_only(&self) -> bool where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(framebufferOnly), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_framebuffer_only(&self, framebuffer_only: bool) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setFramebufferOnly:), (framebuffer_only,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn presents_with_transaction(&self) -> bool where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(presentsWithTransaction), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn set_presents_with_transaction(&self, presents_with_transaction: bool) where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(setPresentsWithTransaction:), (presents_with_transaction,)) {
        Err(s) => panic!("{}", s),
        Ok(()) => ()
      }
    }
  }

  fn release_drawables(&self) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(releaseDrawables), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }
}

pub struct MTKViewID(*mut std::os::raw::c_void);

impl MTKViewID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTKViewID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTKViewID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTKViewID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("MTKView").unwrap();
  }
}

impl NSView for MTKViewID {}
impl NSObject for MTKViewID {}
impl MTKView for MTKViewID {}

impl Clone for MTKViewID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTKViewID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTKViewID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTKViewID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }

  fn as_mut_ptr(&mut self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTKViewID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTKViewID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}

pub trait MTKViewDelegate : NSObject {
  fn mtk_view_drawable_size_will_change<T0: 'static + MTKView>(&self, view: T0, size: CGSize) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(mtkView:drawableSizeWillChange:), (view.as_ptr(), size)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }

  fn draw_in_mtk_view<T0: 'static + MTKView>(&self, view: T0) where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(drawInMTKView:), (view.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: () = r;

          return result;
        }
      }
    }
  }
}

pub struct MTKViewDelegateID(*mut std::os::raw::c_void);

impl MTKViewDelegateID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTKViewDelegateID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return MTKViewDelegateID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return MTKViewDelegateID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for MTKViewDelegateID {}
impl MTKViewDelegate for MTKViewDelegateID {}

impl Clone for MTKViewDelegateID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for MTKViewDelegateID {
  fn drop(&mut self) {
    if !self.is_nil() {
      unsafe { self.release() };
    }
  }
}

impl ObjectiveC for MTKViewDelegateID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return MTKViewDelegateID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }

  fn as_mut_ptr(&mut self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for MTKViewDelegateID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

impl std::fmt::Debug for MTKViewDelegateID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}", self.debug_description().as_str());
  }
}
