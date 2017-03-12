extern crate hagane_objc;
extern crate hagane_simd;
extern crate metal;

use hagane_objc::*;
use hagane_simd::*;

use metal::*;
use metal::rust_metal::*;

#[repr(C)]
struct Uniform {
  model_view_projection: float4x4
}

#[repr(C)]
struct Vertex {
  position: [f32; 4],
  color: [f32; 4]
}

static VERTICES: &'static [Vertex] = &[
  Vertex { position: [-1.0,  1.0,  1.0, 1.0], color: [0.0, 1.0, 1.0, 1.0] },
  Vertex { position: [-1.0, -1.0,  1.0, 1.0], color: [0.0, 0.0, 1.0, 1.0] },
  Vertex { position: [ 1.0, -1.0,  1.0, 1.0], color: [1.0, 0.0, 1.0, 1.0] },
  Vertex { position: [ 1.0,  1.0,  1.0, 1.0], color: [1.0, 1.0, 1.0, 1.0] },
  Vertex { position: [-1.0,  1.0, -1.0, 1.0], color: [0.0, 1.0, 0.0, 1.0] },
  Vertex { position: [-1.0, -1.0, -1.0, 1.0], color: [0.0, 0.0, 0.0, 1.0] },
  Vertex { position: [ 1.0, -1.0, -1.0, 1.0], color: [1.0, 0.0, 0.0, 1.0] },
  Vertex { position: [ 1.0,  1.0, -1.0, 1.0], color: [1.0, 1.0, 0.0, 1.0] }
];

static INDICES: &'static [u16] = &[
  3, 2, 6, 6, 7, 3,
  4, 5, 1, 1, 0, 4,
  4, 0, 3, 3, 7, 4,
  1, 5, 6, 6, 2, 1,
  0, 1, 2, 2, 3, 0,
  7, 6, 5, 5, 4, 7
];

#[derive(Debug, Default)]
struct Example03Renderer {
  command_queue: MTLCommandQueueID,
  time: Option<std::time::Instant>,
  index_buffer: MTLBufferID,
  uniform_buffer: MTLBufferID,
  vertex_buffer: MTLBufferID,
  depth_stencil_state: MTLDepthStencilStateID,
  pipeline_state: MTLRenderPipelineStateID
}

impl RSMRenderer for Example03Renderer {
  fn initialize(&mut self, view: RSMViewID) {
    view.set_color_pixel_format(MTLPixelFormatBGRA8Unorm);
    view.set_depth_stencil_pixel_format(MTLPixelFormatDepth32Float);

    view.set_clear_color(MTLClearColor { red: 1.0, green: 0.3, blue: 0.3, alpha: 1.0 });
    view.set_clear_depth(1.0);

    let device = view.device();
    self.command_queue = device.new_command_queue();

    let depth_stencil_descriptor = MTLDepthStencilDescriptorID::new();
    depth_stencil_descriptor.set_depth_compare_function(MTLCompareFunctionLess);
    depth_stencil_descriptor.set_depth_write_enabled(true);
    self.depth_stencil_state = device.new_depth_stencil_state_with_descriptor(&depth_stencil_descriptor);

    let buffer_size = INDICES.len() * std::mem::size_of::<u16>();
    self.index_buffer = device.new_buffer_with_bytes_length_options(INDICES.as_ptr() as *const std::os::raw::c_void, buffer_size, MTLResourceCPUCacheModeDefaultCache);

    let buffer_size = 1 * std::mem::size_of::<Uniform>();
    self.uniform_buffer = device.new_buffer_with_length_options(buffer_size, MTLResourceCPUCacheModeDefaultCache);

    let buffer_size = VERTICES.len() * std::mem::size_of::<Vertex>();
    self.vertex_buffer = device.new_buffer_with_bytes_length_options(VERTICES.as_ptr() as *const std::os::raw::c_void, buffer_size, MTLResourceCPUCacheModeDefaultCache);

    let library = device.new_library_with_file_error(&NSStringID::from_str("src/examples/example-03.metallib")).unwrap();
    let pipeline_descriptor = MTLRenderPipelineDescriptorID::new();
    pipeline_descriptor.set_vertex_function(&library.new_function_with_name(&NSStringID::from_str("vertex_main")));
    pipeline_descriptor.set_fragment_function(&library.new_function_with_name(&NSStringID::from_str("fragment_main")));
    pipeline_descriptor.color_attachments().object_at_indexed_subscript(0).set_pixel_format(MTLPixelFormatBGRA8Unorm);
    pipeline_descriptor.set_depth_attachment_pixel_format(MTLPixelFormatDepth32Float);

    self.pipeline_state = device.new_render_pipeline_state_with_descriptor_error(&pipeline_descriptor).unwrap();
  }

  fn draw(&mut self, view: RSMViewID) {
    let elapsed = self.time.unwrap().elapsed();

    let seconds = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 / 1_000_000_000.0;

    let rotation_x = seconds * std::f32::consts::FRAC_PI_2;
    let rotation_y = seconds * std::f32::consts::FRAC_PI_3;
    let model_matrix = float4x4::from_scale(0.5 + (5.0 * seconds).sin() * 0.25) * float4x4::from_euler_angles(rotation_x, rotation_y, 0.0);
    let view_matrix = float4x4::look_at(float3(0.0, 0.0, -3.0), float3(0.0, 0.0, 0.0), float3(0.0, 1.0, 0.0));
    let projection_matrix = float4x4::perspective_fov(0.4 * std::f32::consts::PI, 1.0, 0.1, 100.0);

    let uniform = Uniform { model_view_projection: projection_matrix * view_matrix * model_matrix };

    unsafe { std::intrinsics::copy(&uniform, self.uniform_buffer.contents() as *mut Uniform, 1) };

    let command_buffer = self.command_queue.command_buffer();
    let command_encoder = command_buffer.render_command_encoder_with_descriptor(&view.current_render_pass_descriptor());
    command_encoder.set_render_pipeline_state(&self.pipeline_state);
    command_encoder.set_depth_stencil_state(&self.depth_stencil_state);
    command_encoder.set_cull_mode(MTLCullModeBack);
    command_encoder.set_vertex_buffer_offset_at_index(&self.vertex_buffer, 0, 0);
    command_encoder.set_vertex_buffer_offset_at_index(&self.uniform_buffer, 0, 1);
    command_encoder.draw_indexed_primitives_index_count_index_type_index_buffer_index_buffer_offset(MTLPrimitiveTypeTriangle, INDICES.len(), MTLIndexTypeUInt16, &self.index_buffer, 0);
    command_encoder.end_encoding();
    command_buffer.present_drawable(&view.current_drawable());
    command_buffer.commit();
  }
}

fn main() {
  let renderer = Box::new(Example03Renderer { time: Some(std::time::Instant::now()), ..Default::default() });

  let content_rect = CGRect { origin: CGPoint { x: 100.0, y: 300.0 }, size: CGSize { width: 600.0, height: 600.0 } };
  let window = NSWindowID::new_with_content_rect_style_mask_backing_defer(content_rect, 7, 2, false);
  window.set_title(&NSStringID::from_str("Metal Example 03"));
  window.set_content_view(&RSMViewID::from_renderer(renderer, content_rect, &metal::system_default_device()));
  window.set_delegate(&RSMWindowDelegateID::new().retain());
  window.make_key_and_order_front(&NSObjectID::nil());

  NSApplicationID::shared_application().run();
}
