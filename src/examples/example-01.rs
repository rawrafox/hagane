extern crate hagane_objc;
extern crate metal;

use hagane_objc::*;
use metal::*;
use metal::rust_metal::*;

#[derive(Debug, Default)]
struct Example01Renderer {
  command_queue: MTLCommandQueueID
}

impl RSMRenderer for Example01Renderer {
  fn initialize(&mut self, view: RSMViewID) {
    view.set_color_pixel_format(MTLPixelFormatBGRA8Unorm);
    view.set_clear_color(MTLClearColor { red: 1.0, green: 0.3, blue: 0.3, alpha: 1.0 });

    self.command_queue = view.device().new_command_queue();
  }

  fn draw(&mut self, view: RSMViewID) {
    let command_buffer = self.command_queue.command_buffer();
    let command_encoder = command_buffer.render_command_encoder_with_descriptor(&view.current_render_pass_descriptor());

    command_encoder.end_encoding();
    command_buffer.present_drawable(&view.current_drawable());
    command_buffer.commit();
  }
}

fn main() {
  let renderer = Box::new(Example01Renderer { ..Default::default() });

  let content_rect = CGRect { origin: CGPoint { x: 100.0, y: 300.0 }, size: CGSize { width: 400.0, height: 400.0 } };
  let window = NSWindowID::new_with_content_rect_style_mask_backing_defer(content_rect, 7, 2, false);
  window.set_title(&NSStringID::from_str("Metal Example 01"));
  window.set_content_view(&RSMViewID::from_renderer(renderer, content_rect, &metal::system_default_device()));
  window.set_delegate(&RSMWindowDelegateID::new().retain());
  window.make_key_and_order_front(&NSObjectID::nil());

  NSApplicationID::shared_application().run();
}
