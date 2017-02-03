extern crate metal;

use metal::*;
use metal::rust_metal::*;

#[repr(C)]
struct Uniform {
  model_view_projection: float4x4,
  model_view: float4x4,
  normal: float4x4 // TODO: This is a 3x3 matrix, but we send in a 4x4 since we know that this is in this case technically acceptable, and technically acceptable is the best kind of acceptable.
}

struct Example04Renderer {
  command_queue: MTLCommandQueueID,
  time: std::time::Instant,
  uniform_buffer: MTLBufferID,
  mesh_buffer: MTKMeshBufferID,
  submeshes: Vec<MTKSubmeshID>,
  depth_stencil_state: MTLDepthStencilStateID,
  pipeline_state: MTLRenderPipelineStateID
}

impl RSMRenderer for Example04Renderer {
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

    let asset = MDLAssetID::new_with_url_vertex_descriptor_buffer_allocator(&NSURLID::new_with_string(&NSStringID::from_str("../engine/hulls/cc1_t2/CC1_TShape1.obj")), &MDLVertexDescriptorID::nil(), &MTKMeshBufferAllocatorID::new_with_device(&device));

    if asset.count() != 1 { panic!("Not one single mesh in file") }

    let mesh = asset.object_at_index::<MDLMeshID>(0);
    let mesh = MTKMeshID::new_with_mesh_device_error(&mesh, &device).unwrap();
    
    self.mesh_buffer = mesh.vertex_buffers().object_at_index::<MTKMeshBufferID>(0);
    self.submeshes = mesh.submeshes().to_vec::<MTKSubmeshID>();

    self.uniform_buffer = device.new_buffer_with_length_options(std::mem::size_of::<Uniform>(), MTLResourceCPUCacheModeDefaultCache);

    let library = device.new_library_with_file_error(&NSStringID::from_str("src/examples/example-04.metallib")).unwrap();

    let pipeline_descriptor = MTLRenderPipelineDescriptorID::new();
    pipeline_descriptor.set_vertex_function(&library.new_function_with_name(&NSStringID::from_str("vertex_main")));
    pipeline_descriptor.set_fragment_function(&library.new_function_with_name(&NSStringID::from_str("fragment_main")));
    pipeline_descriptor.color_attachments().object_at_indexed_subscript(0).set_pixel_format(MTLPixelFormatBGRA8Unorm);

    self.pipeline_state = device.new_render_pipeline_state_with_descriptor_error(&pipeline_descriptor).unwrap();
  }

  fn draw(&mut self, view: RSMViewID) {
    let elapsed = self.time.elapsed();
    let seconds = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 / 1_000_000_000.0;

    let model_matrix = float4x4::from_scale(0.01).dot(float4x4::from_euler_angles(0.0, seconds * std::f32::consts::FRAC_PI_3, 0.0));
    let projection_matrix = float4x4::perspective(1.0f32, 0.4f32 * std::f32::consts::PI, 1.0f32, 100.0f32);
    let view_matrix = float3(0.0, 0.0, -5.0).look_at(float3(0.0, 0.0, 0.0), float3(0.0, 1.0, 0.0));
    let model_view_matrix = view_matrix.dot(model_matrix);

    unsafe {
      let uniform = Uniform {
        model_view_projection: projection_matrix.dot(model_view_matrix),
        model_view: model_view_matrix,
        normal: model_view_matrix
      };

      std::intrinsics::copy(&uniform, self.uniform_buffer.contents() as *mut Uniform, 1);
    }

    let command_buffer = self.command_queue.command_buffer();
    let command_encoder = command_buffer.render_command_encoder_with_descriptor(&view.current_render_pass_descriptor());
    command_encoder.set_render_pipeline_state(&self.pipeline_state);
    command_encoder.set_depth_stencil_state(&self.depth_stencil_state);
    command_encoder.set_front_facing_winding(MTLWindingCounterClockwise);
    command_encoder.set_cull_mode(MTLCullModeBack);
    command_encoder.set_vertex_buffer_offset_at_index(&self.mesh_buffer.buffer(), self.mesh_buffer.offset(), 0);
    command_encoder.set_vertex_buffer_offset_at_index(&self.uniform_buffer, 0, 1);

    for &ref submesh in &self.submeshes {
      let index_buffer = submesh.index_buffer();

      command_encoder.draw_indexed_primitives_index_count_index_type_index_buffer_index_buffer_offset(submesh.primitive_type(), submesh.index_count(), submesh.index_type(), &index_buffer.buffer(), index_buffer.offset());
    }

    command_encoder.end_encoding();
    command_buffer.present_drawable(&view.current_drawable());
    command_buffer.commit();
  }
}

fn main() {
  rust_metal::load_classes();

  let renderer = Box::new(Example04Renderer {
    command_queue: MTLCommandQueueID::nil(),
    time: std::time::Instant::now(),
    uniform_buffer: MTLBufferID::nil(),
    mesh_buffer: MTKMeshBufferID::nil(),
    submeshes: vec![],
    depth_stencil_state: MTLDepthStencilStateID::nil(),
    pipeline_state: MTLRenderPipelineStateID::nil()
  });

  let content_rect = CGRect { origin: CGPoint { x: 100.0, y: 300.0 }, size: CGSize { width: 400.0, height: 400.0 } };
  let window = NSWindowID::new_with_content_rect_style_mask_backing_defer(content_rect, 7, 2, false);
  window.set_title(&NSStringID::from_str("Metal Example 04"));
  window.set_content_view(&RSMViewID::from_renderer(renderer, content_rect, &metal::system_default_device()));
  window.set_delegate(&RSMWindowDelegateID::new().retain());
  window.make_key_and_order_front(&NSObjectID::nil());

  NSApplicationID::shared_application().run();
}
