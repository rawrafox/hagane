use std;
use objc;

use super::{CAMetalDrawableID, CGRect, MTLClearColor, MTLDevice, MTLDeviceID, MTLRenderPassDescriptorID, NSObject, NSView};

#[link(name = "MetalKit", kind = "framework")]
extern { }

pub trait MTKView : NSView {
  forward!(current_drawable, sel!(currentDrawable), () -> CAMetalDrawableID);
  forward!(current_render_pass_descriptor, sel!(currentRenderPassDescriptor), () -> MTLRenderPassDescriptorID);
  forward!(device, sel!(device), () -> MTLDeviceID);
  forward!(init_with_frame_device, sel!(initWithFrame:device:), (frame: CGRect, device: T) -> MTKViewID, <T: MTLDevice>);
  forward!(set_clear_color, sel!(setClearColor:), (color: MTLClearColor) -> ());
  forward!(set_color_pixel_format, sel!(setColorPixelFormat:), (format: usize) -> ());
  forward!(set_preferred_frames_per_second, sel!(setPreferredFramesPerSecond:), (frames: usize) -> ());
}

id!(MTKViewID, MTKView, "MTKView");

impl NSObject for MTKViewID {}
impl NSView for MTKViewID {}
