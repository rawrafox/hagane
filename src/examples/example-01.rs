#[macro_use] extern crate objc;
#[macro_use] extern crate objc_id;
#[macro_use] extern crate objc_foundation;

extern crate metal;

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};

use objc_id::Id;

use objc_foundation::NSString;
use objc_foundation::{INSObject, INSString};

use metal::{Device};

#[link(name = "Cocoa", kind = "framework")]
extern { }

#[link(name = "MetalKit", kind = "framework")]
extern { }


type CGFloat = f64;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CGSize {
  pub width: CGFloat,
  pub height: CGFloat
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CGRect {
  pub origin: CGPoint,
  pub size: CGSize
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MTLClearColor {
  pub red: f64,
  pub green: f64,
  pub blue: f64,
  pub alpha: f64
}

object_struct!(CAMetalDrawable);

object_struct!(MTLCommandBuffer);
object_struct!(MTLRenderCommandEncoder);
object_struct!(MTLCommandQueue);
object_struct!(MTLRenderPassDescriptor);

object_struct!(NSApplication);
object_struct!(NSWindow);

object_struct!(MTKView);

fn metal_view() {
  let superclass = Class::get("MTKView").unwrap();
  let mut klass = ClassDecl::new("MetalView", superclass).unwrap();

  extern fn initialize_object(this: &mut Object, _cmd: Sel) {
    unsafe {
      msg_send![this, setColorPixelFormat: 80]; // MTLPixelFormatBGRA8Unorm
      msg_send![this, setPreferredFramesPerSecond: 60];
      msg_send![this, setClearColor: MTLClearColor { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }];
    }
  }

  extern fn draw_rect(this: &mut Object, _cmd: Sel, _rect: usize) {
    unsafe {
      let device: *mut Device = msg_send![this, device];

      let drawable: *mut CAMetalDrawable = msg_send![this, currentDrawable];
      let render_pass_descriptor: *mut MTLRenderPassDescriptor = msg_send![this, currentRenderPassDescriptor];

      let command_queue: *mut MTLCommandQueue = msg_send![device, newCommandQueue];
      let command_buffer: *mut MTLCommandBuffer = msg_send![command_queue, commandBuffer];
      let command_encoder: *mut MTLRenderCommandEncoder = msg_send![command_buffer, renderCommandEncoderWithDescriptor: render_pass_descriptor];

      msg_send![command_encoder, endEncoding];
      msg_send![command_buffer, presentDrawable: drawable];
      msg_send![command_buffer, commit];
    }
  }

  extern fn window_will_close(this: &mut Object, _cmd: Sel, _notification: usize) {
    unsafe {
      let application: *mut NSApplication = msg_send![NSApplication::class(), sharedApplication];

      msg_send![application, terminate: this];
    }
  }

  unsafe {
    klass.add_method(sel!(initializeObject), initialize_object as extern fn(&mut Object, Sel));
    klass.add_method(sel!(drawRect:), draw_rect as extern fn(&mut Object, Sel, usize));
    klass.add_method(sel!(windowWillClose:), window_will_close as extern fn(&mut Object, Sel, usize));
  }

  klass.register();
}

fn setup() -> Id<NSWindow> {
  let content_rect = CGRect { origin: CGPoint { x: 100.0, y: 300.0 }, size: CGSize { width: 400.0, height: 400.0 } };

  unsafe {
    let window: *mut NSWindow = msg_send![NSWindow::class(), alloc];
    let window: *mut NSWindow = msg_send![window, initWithContentRect: content_rect styleMask: 7 backing: 2 defer: false];

    let view: *mut MTKView = msg_send![Class::get("MetalView").unwrap(), alloc];
    let view: *mut MTKView = msg_send![view, initWithFrame: content_rect device: Device::system_default()];

    msg_send![view, initializeObject];

    msg_send![window, setTitle: NSString::from_str("Metal Example 01")];
    msg_send![window, setContentView: view];
    msg_send![window, setDelegate: view];

    msg_send![window, makeKeyAndOrderFront: 0];

    return Id::from_retained_ptr(window);
  }
}

fn main() {
  metal_view();

  let application: Id<NSApplication> = unsafe { Id::from_ptr(msg_send![NSApplication::class(), sharedApplication]) };

  let _ = setup();

  unsafe {
    msg_send![application, run];
  }
}