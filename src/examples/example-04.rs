#[macro_use] extern crate metal;
extern crate nalgebra;

use std::ops::Mul;

use metal::*;
use metal::rust_metal::*;

use nalgebra::ToHomogeneous;

#[allow(dead_code)]
struct Uniform {
  model_view_projection: [f32; 16],
  model_view: [f32; 16],
  normal: [f32; 9]
}

struct Example04Renderer {
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

    view.set_preferred_frames_per_second(60);
    view.set_clear_color(MTLClearColor { red: 1.0, green: 0.3, blue: 0.3, alpha: 1.0 });
    view.set_clear_depth(1.0);

    let device = view.device();

    let depth_stencil_descriptor = MTLDepthStencilDescriptorID::alloc().init();
    depth_stencil_descriptor.set_depth_compare_function(MTLCompareFunctionLess);
    depth_stencil_descriptor.set_depth_write_enabled(true);
    self.depth_stencil_state = device.new_depth_stencil_state_with_descriptor(depth_stencil_descriptor);

    let asset = MDLAssetID::alloc().init_with_url_vertex_descriptor_buffer_allocator(NSURLID::alloc().init_with_string(NSStringID::from_str("../engine/hulls/ab3_t1/AB3_TShape1.obj")), MDLVertexDescriptorID::nil(), MTKMeshBufferAllocatorID::alloc().init_with_device(device.clone()));

    if asset.count() != 1 { panic!("Not one single mesh in file") }

    let mesh = asset.object_at_index::<MDLMeshID>(0);
    let mesh = MTKMeshID::alloc().init_with_mesh_device_error(mesh, device.clone()).unwrap();
    
    self.mesh_buffer = mesh.vertex_buffers().object_at_index::<MTKMeshBufferID>(0);
    self.submeshes = mesh.submeshes().to_vec::<MTKSubmeshID>();

    self.uniform_buffer = device.new_buffer_with_length_options(std::mem::size_of::<Uniform>(), MTLResourceCPUCacheModeDefaultCache);

    let library = device.new_library_with_file_error(NSStringID::from_str("src/examples/example-04.metallib")).unwrap();

    let pipeline_descriptor = MTLRenderPipelineDescriptorID::alloc().init();
    pipeline_descriptor.set_vertex_function(library.new_function_with_name(NSStringID::from_str("vertex_main")));
    pipeline_descriptor.set_fragment_function(library.new_function_with_name(NSStringID::from_str("fragment_main")));
    pipeline_descriptor.color_attachments().object_at_indexed_subscript(0).set_pixel_format(MTLPixelFormatBGRA8Unorm);

    self.pipeline_state = device.new_render_pipeline_state_with_descriptor_error(pipeline_descriptor).unwrap();
  }

  fn draw(&mut self, view: RSMViewID) {
    let elapsed = self.time.elapsed();

    let seconds = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 / 1_000_000_000.0;

    let rotation_y = seconds * std::f32::consts::FRAC_PI_3;
    let scale_factor = 0.01f32;

    let rotation = nalgebra::Rotation3::from_euler_angles(0.0, rotation_y, 0.0);

    let model_matrix = nalgebra::Similarity3::from_rotation_matrix(nalgebra::Vector3::new(0.0f32, 0.0f32, 0.0f32), rotation, scale_factor);

    let origin = nalgebra::Point3::new(0.0f32, 0.0f32,  0.0f32);
    let camera = nalgebra::Point3::new(0.0f32, 0.0f32, -7.0f32);

    let camera_matrix = nalgebra::Isometry3::new_observer_frame(&camera, &origin, &nalgebra::Vector3::new(0.0f32, 1.0f32, 0.0f32));
    let projection_matrix = nalgebra::PerspectiveMatrix3::new(1.0f32, 0.4f32 * std::f32::consts::PI, 1.0f32, 100.0f32);

    let model_view = camera_matrix.mul(model_matrix);
    let model_view_matrix = model_view.to_homogeneous();
    let matrix = (*projection_matrix.as_matrix()).mul(model_view_matrix);

    unsafe {
      let mv: [f32; 16] = std::mem::transmute(model_view_matrix);

      let uniform = Uniform {
        model_view_projection: std::mem::transmute(matrix),
        model_view: mv,
        normal: [mv[0], mv[1], mv[2], mv[4], mv[5], mv[6], mv[8], mv[9], mv[10]]
      };

      std::intrinsics::copy(&uniform, self.uniform_buffer.contents() as *mut Uniform, 1);
    }

    let drawable = view.current_drawable();

    let command_queue = view.device().new_command_queue();
    let command_buffer = command_queue.command_buffer();
    let command_encoder = command_buffer.render_command_encoder_with_descriptor(view.current_render_pass_descriptor());
    command_encoder.set_render_pipeline_state(self.pipeline_state.clone());
    command_encoder.set_depth_stencil_state(self.depth_stencil_state.clone());
    command_encoder.set_front_facing_winding(MTLWindingCounterClockwise);
    command_encoder.set_cull_mode(MTLCullModeBack);
    command_encoder.set_vertex_buffer_offset_at_index(self.mesh_buffer.buffer(), self.mesh_buffer.offset(), 0);
    command_encoder.set_vertex_buffer_offset_at_index(self.uniform_buffer.clone(), 0, 1);

    for &ref submesh in &self.submeshes {
      let index_buffer = submesh.index_buffer();

      command_encoder.draw_indexed_primitives_index_count_index_type_index_buffer_index_buffer_offset(submesh.primitive_type(), submesh.index_count(), submesh.index_type(), index_buffer.buffer(), index_buffer.offset());
    }

    command_encoder.end_encoding();
    command_buffer.present_drawable(drawable);
    command_buffer.commit();
  }
}

fn main() {
  rust_metal::load_classes();

  let renderer = Box::new(Example04Renderer {
    time: std::time::Instant::now(),
    uniform_buffer: MTLBufferID::nil(),
    mesh_buffer: MTKMeshBufferID::nil(),
    submeshes: vec![],
    depth_stencil_state: MTLDepthStencilStateID::nil(),
    pipeline_state: MTLRenderPipelineStateID::nil()
  });

  let content_rect = CGRect { origin: CGPoint { x: 100.0, y: 300.0 }, size: CGSize { width: 400.0, height: 400.0 } };
  let window = NSWindowID::alloc().init_with_content_rect_style_mask_backing_defer(content_rect, 7, 2, false);
  window.set_title(NSStringID::from_str("Metal Example 04"));
  window.set_content_view(RSMViewID::from_renderer(renderer, content_rect, metal::system_default_device()));
  window.set_delegate(RSMWindowDelegateID::new().retain());
  window.make_key_and_order_front(NSObjectID::nil());

  NSApplicationID::shared_application().run();
}
