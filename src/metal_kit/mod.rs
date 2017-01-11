#![allow(non_upper_case_globals)]

use std;
use objc;
use super::ObjectiveC;
use cocoa::*;
use core_animation::*;
use core_graphics::*;
use foundation::*;
use metal::*;
use legacy_metal::*;

#[link(name = "MetalKit", kind = "framework")]
extern {}

pub trait MTKView : NSView + NSObject {
  fn init_with_coder<T5: 'static + NSCoder>(self, coder: T5) -> Self where Self: 'static + Sized {
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

  fn init_with_frame_device<T4: 'static + MTLDevice>(self, frame_rect: CGRect, device: T4) -> Self where Self: 'static + Sized {
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
  fn mtk_view_drawable_size_will_change<T5: 'static + MTKView>(&self, view: T5, size: CGSize) where Self: 'static + Sized {
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

  fn draw_in_mtk_view<T5: 'static + MTKView>(&self, view: T5) where Self: 'static + Sized {
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
