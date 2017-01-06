#[macro_use] extern crate objc;
#[macro_use] extern crate metal;

use objc::declare::ClassDecl;
use objc::runtime::{Object, Sel};

use metal::*;

pub trait RSMView : MTKView {
  forward!(initialize_object, sel!(initializeObject), () -> ());
}

id!(RSMViewID, RSMView, "RSMView");
impl NSObject for RSMViewID {}
impl NSView for RSMViewID {}
impl MTKView for RSMViewID {}

fn create_metal_view_class() {
  let mut klass = ClassDecl::new("RSMView", MTKViewID::class()).unwrap();

  extern fn initialize_object(this: &mut Object, _cmd: Sel) {
    let view = RSMViewID::from_object(this);

    view.set_color_pixel_format(80); // MTLPixelFormatBGRA8Unorm
    view.set_preferred_frames_per_second(60);
    view.set_clear_color(MTLClearColor { red: 1.0, green: 0.3, blue: 0.3, alpha: 1.0 });
  }

  extern fn draw_rect(this: &mut Object, _cmd: Sel, _rect: usize) {
    let view = RSMViewID::from_object(this);

    let command_queue = view.device().new_command_queue();
    let command_buffer = command_queue.command_buffer();
    let command_encoder = command_buffer.render_command_encoder_with_descriptor(view.current_render_pass_descriptor());

    command_encoder.end_encoding();
    command_buffer.present_drawable(view.current_drawable());
    command_buffer.commit();
  }

  extern fn window_will_close(this: &mut Object, _cmd: Sel, _notification: usize) {
    NSApplicationID::shared_application().terminate(NSObjectID::from_object(this));
  }

  unsafe {
    klass.add_method(sel!(initializeObject), initialize_object as extern fn(&mut Object, Sel));
    klass.add_method(sel!(drawRect:), draw_rect as extern fn(&mut Object, Sel, usize));
    klass.add_method(sel!(windowWillClose:), window_will_close as extern fn(&mut Object, Sel, usize));
  }

  klass.register();
}

fn setup() {
  let content_rect = CGRect { origin: CGPoint { x: 100.0, y: 300.0 }, size: CGSize { width: 400.0, height: 400.0 } };

  let view = RSMViewID::alloc();
  view.init_with_frame_device(content_rect, metal::system_default_device());
  view.initialize_object();

  let window = NSWindowID::alloc();
  window.init_with_content_rect_style_mask_backing_defer(content_rect, 7, 2, false);
  window.set_title(NSStringID::from_str("Metal Example 01"));
  window.set_content_view(view);
  window.set_delegate(view);
  window.make_key_and_order_front(NSObjectID::nil());
}

fn main() {
  create_metal_view_class();
  setup();

  NSApplicationID::shared_application().run();
}
