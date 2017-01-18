use std;
use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt};

use foundation::*;
use metal::*;

#[derive(Debug)]
struct Header {
  flags: u32,
  height: u32,
  width: u32,
  pitch_or_linear_size: u32,
  depth: u32,
  mip_map_count: u32,
  reserved1: bool,
  pixel_format: PixelFormat,
  caps: [u32; 4],
  reserved2: bool
}

#[derive(Debug)]
struct PixelFormat {
  size: u32,
  flags: u32,
  fourcc: [u8; 4],
  rgb_bit_count: u32,
  r_bit_mask: u32,
  g_bit_mask: u32,
  b_bit_mask: u32,
  a_bit_mask: u32
}

pub fn import<T: 'static + MTLDevice>(input: &[u8], device: &T) -> Option<MTLTextureID> {
  let mut cursor = Cursor::new(&input[..]);

  if &read_fourcc(&mut cursor) != b"DDS " {
    return None;
  }

  if read_u32(&mut cursor) != 124 {
    return None; // Header is not standard length
  }

  let header = read_header(&mut cursor);

  if !header.reserved1 || !header.reserved2 {
    return None;
  }

  if header.pixel_format.size != 32 {
    return None;
  }

  let valid_bits = header.flags;

  let valid_caps = valid_bits & 0x1 != 0;
  let valid_height = valid_bits & 0x2 != 0;
  let valid_width = valid_bits & 0x4 != 0;
  // let valid_pitch = valid_bits & 0x8 != 0;
  let valid_pixel_format = valid_bits & 0x1000 != 0;
  let valid_mip_map_count = valid_bits & 0x20000 != 0;
  // let valid_linear_size = valid_bits & 0x80000 != 0;
  let valid_depth = valid_bits & 0x800000 != 0;

  if !valid_caps || !valid_height || !valid_width || !valid_pixel_format {
    return None; // Required bits of a DDS file
  }

  let descriptor = MTLTextureDescriptorID::new();

  descriptor.set_height(header.height as NSUInteger);
  descriptor.set_width(header.width as NSUInteger);

  if valid_depth {
    descriptor.set_depth(header.depth as NSUInteger);
  }

  if valid_mip_map_count {
    descriptor.set_mipmap_level_count(header.mip_map_count as NSUInteger);
  }

  let mut width = descriptor.width();
  let mut height = descriptor.height();
  let mut depth = descriptor.depth();
  let size_per_pixel;

  match &header.pixel_format.fourcc {
    b"DXT1" => {
      descriptor.set_pixel_format(MTLPixelFormatBC1_RGBA);
      size_per_pixel = 4;
    }
    b"DXT2" | b"DXT3" => {
      descriptor.set_pixel_format(MTLPixelFormatBC2_RGBA);
      size_per_pixel = 8;
    }
    b"DXT4" | b"DXT5" => {
      descriptor.set_pixel_format(MTLPixelFormatBC3_RGBA);
      size_per_pixel = 8;
    }
    _ => return None
  };

  let texture = device.new_texture_with_descriptor(&descriptor);

  for level in 0 .. descriptor.mipmap_level_count() {
    let region = MTLRegion {
      origin: MTLOrigin { x: 0, y: 0, z: 0 },
      size: MTLSize { width: width, height: height, depth: depth }
    };

    let data = read_vec(&mut cursor, width * height * depth * size_per_pixel / 8);

    texture.replace_region_mipmap_level_with_bytes_bytes_per_row(region, level, data.as_ptr() as *const std::os::raw::c_void, 0);

    width = std::cmp::max(width / 2, 1);
    height = std::cmp::max(height / 2, 1);
    depth = std::cmp::max(depth / 2, 1);
  }
  
  return Some(texture);
}

fn read_fourcc(cursor: &mut Read) -> [u8; 4] {
  let mut string = [0; 4];

  cursor.read_exact(&mut string).unwrap();

  return string;
}

fn read_header(cursor: &mut Read) -> Header {
  return Header {
    flags: read_u32(cursor),
    height: read_u32(cursor),
    width: read_u32(cursor),
    pitch_or_linear_size: read_u32(cursor),
    depth: read_u32(cursor),
    mip_map_count: read_u32(cursor),
    reserved1: read_reserved(cursor, 11),
    pixel_format: read_pixel_format(cursor),
    caps: [read_u32(cursor), read_u32(cursor), read_u32(cursor), read_u32(cursor)],
    reserved2: read_reserved(cursor, 1)
  };
}

fn read_pixel_format(cursor: &mut Read) -> PixelFormat {
  return PixelFormat {
    size: read_u32(cursor),
    flags: read_u32(cursor),
    fourcc: read_fourcc(cursor),
    rgb_bit_count: read_u32(cursor),
    r_bit_mask: read_u32(cursor),
    g_bit_mask: read_u32(cursor),
    b_bit_mask: read_u32(cursor),
    a_bit_mask: read_u32(cursor)
  };
}

fn read_reserved(cursor: &mut Read, n: usize) -> bool {
  for _ in 0 .. n {
    if read_u32(cursor) != 0 {
      return false;
    }
  }

  return true;
}

fn read_u32(cursor: &mut Read) -> u32 {
  return cursor.read_u32::<LittleEndian>().unwrap();
}

fn read_vec(cursor: &mut Read, bytes: usize) -> Vec<u8> {
  let mut data = Vec::new();

  cursor.take(bytes as u64).read_to_end(&mut data).unwrap();

  return data;
}