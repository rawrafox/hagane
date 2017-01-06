#[macro_use] extern crate metal;

use metal::*;

struct Example01Renderer { }

impl RSMRenderer for Example01Renderer {
  fn initialize(&mut self, view: RSMViewID) {
    view.set_color_pixel_format(80); // MTLPixelFormatBGRA8Unorm
    view.set_preferred_frames_per_second(60);
    view.set_clear_color(MTLClearColor { red: 1.0, green: 0.3, blue: 0.3, alpha: 1.0 });
  }
  
  fn draw(&mut self, view: RSMViewID) {
    let command_queue = view.device().new_command_queue();
    let command_buffer = command_queue.command_buffer();
    let command_encoder = command_buffer.render_command_encoder_with_descriptor(view.current_render_pass_descriptor());

    command_encoder.end_encoding();
    command_buffer.present_drawable(view.current_drawable());
    command_buffer.commit();
  }
}

fn main() {
  RSMViewID::load_class();

  let content_rect = CGRect { origin: CGPoint { x: 100.0, y: 300.0 }, size: CGSize { width: 400.0, height: 400.0 } };

  let view = RSMViewID::from_renderer(Box::new(Example01Renderer { }), content_rect, metal::system_default_device());

  let window = NSWindowID::alloc();
  window.init_with_content_rect_style_mask_backing_defer(content_rect, 7, 2, false);
  window.set_title(NSStringID::from_str("Metal Example 01"));
  window.set_content_view(view);
  window.set_delegate(view);
  window.make_key_and_order_front(NSObjectID::nil());

  NSApplicationID::shared_application().run();
}
