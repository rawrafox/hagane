use std;
use objc;

use super::{CAMetalDrawableID, CGRect, MTLClearColor, MTLDevice, MTLDeviceID, MTLRenderPassDescriptorID, NSObject, NSView};

#[link(name = "MetalKit", kind = "framework")]
extern { }

pub trait MTKView : NSView {
  initializer!(init_with_frame_device, sel!(initWithFrame:device:), (frame: CGRect, device: T), <T: MTLDevice>);

  forward!(current_drawable, sel!(currentDrawable), () -> CAMetalDrawableID, retain);
  forward!(current_render_pass_descriptor, sel!(currentRenderPassDescriptor), () -> MTLRenderPassDescriptorID, retain);
  forward!(device, sel!(device), () -> MTLDeviceID, retain);
  forward!(set_clear_color, sel!(setClearColor:), (color: MTLClearColor) -> ());
  forward!(set_clear_depth, sel!(setClearDepth:), (depth: f64) -> ());
  forward!(set_color_pixel_format, sel!(setColorPixelFormat:), (format: usize) -> ());
  forward!(set_depth_stencil_pixel_format, sel!(setDepthStencilPixelFormat:), (format: usize) -> ());
  forward!(set_preferred_frames_per_second, sel!(setPreferredFramesPerSecond:), (frames: usize) -> ());
}

id!(MTKViewID, MTKView, "MTKView");

impl NSObject for MTKViewID {}
impl NSView for MTKViewID {}
