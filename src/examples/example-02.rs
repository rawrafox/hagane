#[macro_use] extern crate metal;

use metal::*;
use metal::rust_metal::*;

#[allow(dead_code)]
struct Vertex {
  position: [f32; 4],
  color: [f32; 4]
}

static VERTICES: &'static [Vertex] = &[
  Vertex { position: [ 0.0,  0.5, 0.0, 1.0], color: [1.0, 0.0, 0.0, 1.0] },
  Vertex { position: [-0.5, -0.5, 0.0, 1.0], color: [0.0, 1.0, 0.0, 1.0] },
  Vertex { position: [ 0.5, -0.5, 0.0, 1.0], color: [0.0, 0.0, 1.0, 1.0] }
];

struct Example02Renderer {
  buffer: MTLBufferID,
  pipeline_state: MTLRenderPipelineStateID
}

impl RSMRenderer for Example02Renderer {
  fn initialize(&mut self, view: RSMViewID) {
    view.set_color_pixel_format(MTLPixelFormat::MTLPixelFormatBGRA8Unorm);
    view.set_preferred_frames_per_second(60);
    view.set_clear_color(MTLClearColor { red: 1.0, green: 0.3, blue: 0.3, alpha: 1.0 });

    let device = view.device();

    let vertex_size = std::mem::size_of::<Vertex>();
    let buffer_size = VERTICES.len() * vertex_size;
    self.buffer = device.new_buffer_with_bytes_length_options(VERTICES.as_ptr() as *const std::os::raw::c_void, buffer_size, 0);

    let library = match device.new_library_with_file(NSStringID::from_str("src/examples/example-02.metallib")) {
      Ok(l) => l,
      Err(_) => panic!("Error loading Metal library")
    };

    let vertex_function = library.new_function_with_name(NSStringID::from_str("vertex_main"));
    let fragment_function = library.new_function_with_name(NSStringID::from_str("fragment_main"));

    let pipeline_descriptor = MTLRenderPipelineDescriptorID::alloc().init();
    pipeline_descriptor.set_vertex_function(vertex_function);
    pipeline_descriptor.set_fragment_function(fragment_function);

    let color_attachments = pipeline_descriptor.color_attachments();
    let color_attachment = color_attachments.object_at_indexed_subscript(0);
    color_attachment.set_pixel_format(80);

    self.pipeline_state = match device.new_render_pipeline_state_with_descriptor(pipeline_descriptor) {
      Ok(p) => p,
      Err(_) => panic!("Error creating Metal pipeline")
    };
  }
  
  fn draw(&mut self, view: RSMViewID) {
    let command_queue = view.device().new_command_queue();
    let command_buffer = command_queue.command_buffer();
    let command_encoder = command_buffer.render_command_encoder_with_descriptor(view.current_render_pass_descriptor());
    command_encoder.set_render_pipeline_state(self.pipeline_state.clone());
    command_encoder.set_vertex_buffer_offset_at_index(self.buffer.clone(), 0, 0);
    command_encoder.draw_primitives_vertex_start_vertex_count(MTLPrimitiveTypeTriangle, 0, VERTICES.len());
    command_encoder.end_encoding();
    command_buffer.present_drawable(view.current_drawable());
    command_buffer.commit();
  }
}

fn main() {
  rust_metal::load_classes();

  let renderer = Box::new(Example02Renderer {
    buffer: MTLBufferID::nil(),
    pipeline_state: MTLRenderPipelineStateID::nil()
  });

  let content_rect = CGRect { origin: CGPoint { x: 100.0, y: 300.0 }, size: CGSize { width: 400.0, height: 400.0 } };
  let window = NSWindowID::alloc().init_with_content_rect_style_mask_backing_defer(content_rect, 7, 2, false);
  window.set_title(NSStringID::from_str("Metal Example 02"));
  window.set_content_view(RSMViewID::from_renderer(renderer, content_rect, metal::system_default_device()));
  window.set_delegate(RSMWindowDelegateID::new().retain());
  window.make_key_and_order_front(NSObjectID::nil());

  NSApplicationID::shared_application().run();
}
