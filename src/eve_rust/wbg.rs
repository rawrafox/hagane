use std;
use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt};

use foundation::*;
use model_io::*;

pub fn import(input: &[u8]) -> MDLAssetID {
  let mut cursor = Cursor::new(&input[..]);

  match cursor.read_u8().unwrap() {
    0 | 60 => {},
    file_version => panic!("Unknown file version {}", file_version)
  }

  let asset = MDLAssetID::new();
  let count = cursor.read_u8().unwrap();

  for _ in 0 .. count {
    let mesh = read_mesh(&mut cursor);
    let _ = read_bone_bindings(&mut cursor);
    let _ = read_blend_shapes(&mut cursor);

    asset.add_object(&mesh);
  }

  return asset;
}

pub fn read_bone_bindings(cursor: &mut Cursor<&[u8]>) -> NSMutableArrayID {
  let bones = NSMutableArrayID::alloc().init();

  for i in 0 .. cursor.read_u8().unwrap() {
    let name = read_string(cursor);

    bones.insert_object_at_index(&name, i as NSUInteger);
  }

  return bones;
}

pub fn read_blend_shapes(cursor: &mut Cursor<&[u8]>) -> () {
  for _ in 0 .. cursor.read_u16::<LittleEndian>().unwrap() {
    read_string(cursor);
    read_vertex_buffer(cursor);
    read_index_buffer(cursor);
  }
}

pub fn read_buffer(cursor: &mut Cursor<&[u8]>, element_size: NSUInteger) -> NSDataID {
  let count = cursor.read_u32::<LittleEndian>().unwrap() as usize;

  if count == 0 {
    panic!("Nothing in buffer in .wbg")
  }

  let mut data = Vec::new();
  cursor.take((count * element_size) as u64).read_to_end(&mut data).unwrap();

  return NSDataID::new_with_bytes_length(data.as_ptr() as *const std::os::raw::c_void, data.len());
}

pub fn read_index_buffer(cursor: &mut Cursor<&[u8]>) -> (MDLIndexBitDepth, NSUInteger, NSDataID) {
  let (index_type, index_size) = match cursor.read_u8().unwrap() {
    0 => (MDLIndexBitDepthUInt16, 2),
    1 => (MDLIndexBitDepthUInt32, 4),
    n => panic!("Unknown index bit depth in .wbg ({:?})", n)
  };

  return (index_type, index_size, read_buffer(cursor, index_size));
}

pub fn read_mesh(cursor: &mut Cursor<&[u8]>) -> MDLMeshID {
  let name = read_string(cursor);

  let (vertex_descriptor, vertex_buffer, vertex_count) = read_vertex_buffer(cursor);

  let submeshes = read_submeshes(cursor);

  let mesh = MDLMeshID::new_with_vertex_buffer_vertex_count_descriptor_submeshes(&vertex_buffer, vertex_count, &vertex_descriptor, &submeshes);
  mesh.set_name(&name);
  return mesh;
}

fn read_string(cursor: &mut Read) -> NSStringID {
  let length = cursor.read_u8().unwrap() as u64;

  let mut string = String::new();
  cursor.take(length).read_to_string(&mut string).unwrap();

  return NSStringID::from_str(string.as_str());
}

fn read_submeshes(cursor: &mut Cursor<&[u8]>) -> NSMutableArrayID {
  let (index_type, index_size, index_buffer) = read_index_buffer(cursor);

  let submeshes = NSMutableArrayID::alloc().init();
  let submesh_count = cursor.read_u8().unwrap();

  for i in 0 .. submesh_count {
    let name = read_string(cursor);
    let start = cursor.read_u32::<LittleEndian>().unwrap() as NSUInteger * index_size;
    let index_count = cursor.read_u32::<LittleEndian>().unwrap() as NSUInteger * 3;
    let buffer = MDLMeshBufferDataID::new_with_type_data(MDLMeshBufferTypeIndex, &index_buffer.subdata_with_range(NSRange { location: start, length: index_size * index_count }));
    let _ = [cursor.read_f32::<LittleEndian>().unwrap(), cursor.read_f32::<LittleEndian>().unwrap(), cursor.read_f32::<LittleEndian>().unwrap()];
    let _ = [cursor.read_f32::<LittleEndian>().unwrap(), cursor.read_f32::<LittleEndian>().unwrap(), cursor.read_f32::<LittleEndian>().unwrap()];

    let submesh = MDLSubmeshID::new_with_index_buffer_index_count_index_type_geometry_type_material(&buffer, index_count, index_type, MDLGeometryTypeTriangles, &MDLMaterialID::nil());
    submesh.set_name(&name);
    submeshes.insert_object_at_index(&submesh, i as NSUInteger);
  }

  return submeshes;
}

fn read_vertex_buffer(cursor: &mut Cursor<&[u8]>) -> (MDLVertexDescriptorID, MDLMeshBufferDataID, NSUInteger) {
  let (stride, vertex_descriptor) = read_vertex_descriptor(cursor);

  let buffer = MDLMeshBufferDataID::new_with_type_data(MDLMeshBufferTypeVertex, &read_buffer(cursor, stride));
  let vertex_count = buffer.length() / stride;

  return (vertex_descriptor, buffer, vertex_count);
}

fn read_vertex_descriptor(cursor: &mut Cursor<&[u8]>) -> (NSUInteger, MDLVertexDescriptorID) {
  let mut stride = 0;

  let descriptor = MDLVertexDescriptorID::new();

  let count = cursor.read_u8().unwrap();

  for _ in 0 .. count {
    let name = unsafe {
      match cursor.read_u8().unwrap() {
        0 => MDLVertexAttributePosition.clone(),
        1 => MDLVertexAttributeColor.clone(),
        2 => MDLVertexAttributeNormal.clone(),
        3 => MDLVertexAttributeTangent.clone(),
        4 => MDLVertexAttributeBitangent.clone(),
        5 => MDLVertexAttributeTextureCoordinate.clone(),
        6 => MDLVertexAttributeJointWeights.clone(),
        7 => MDLVertexAttributeJointIndices.clone(),
        t => panic!("Unknown name index in .wbg ({:?})", t)
      }
    };

    let name = match cursor.read_u8().unwrap() {
      0 => name,
      i => NSStringID::from_str(&format!("{} {}", name.as_str(), i)[..])
    };

    let file_format = cursor.read_u8().unwrap();
    let file_width = ((file_format >> 5) + 1) as NSUInteger;

    let (format, size) = match (file_format & 0x0F, file_width, MDLVertexFormat::from_bits(file_width).unwrap(), file_format & 0x10 == 0x10) {
      (0, w, wf, true) => (MDLVertexFormatCharNormalizedBits | wf, 1 * w),
      (0, w, wf, false) => (MDLVertexFormatCharBits | wf, 1 * w),
      (1, w, wf, true) => (MDLVertexFormatShortNormalizedBits | wf, 2 * w),
      (1, w, wf, false) => (MDLVertexFormatShortBits | wf, 2 * w),
      (2, w, wf, false) => (MDLVertexFormatIntBits | wf, 4 * w),
      (3, w, wf, false) => (MDLVertexFormatHalfBits | wf, 2 * w),
      (4, w, wf, false) => (MDLVertexFormatFloatBits | wf, 4 * w),
      (8, w, wf, true) => (MDLVertexFormatUCharNormalizedBits | wf, 1 * w),
      (8, w, wf, false) => (MDLVertexFormatUCharBits | wf, 1 * w),
      (9, w, wf, true) => (MDLVertexFormatUShortNormalizedBits | wf, 2 * w),
      (9, w, wf, false) => (MDLVertexFormatUShortBits | wf, 2 * w),
      (10, w, wf, false) => (MDLVertexFormatUIntBits | wf, 4 * w),
      (t, w, _, n) => panic!("Unknown name index in .wbg ({:?}, {:?}, {:?})", t, w, n)
    };

    let offset = stride;

    stride += size;

    descriptor.add_or_replace_attribute(&MDLVertexAttributeID::new_with_name_format_offset_buffer_index(&name, format, offset, 0));
  }

  descriptor.layouts().insert_object_at_index(&MDLVertexBufferLayoutID::new_with_stride(stride), 0);

  return (stride, descriptor);
}