#![allow(non_camel_case_types)]
pub type vector_char1 = i8;

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_char2(i8, i8);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_char3(i8, i8, i8);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_char4(i8, i8, i8, i8);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_char8(i8, i8, i8, i8, i8, i8, i8, i8);
pub type vector_uchar1 = u8;

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_uchar2(u8, u8);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_uchar3(u8, u8, u8);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_uchar4(u8, u8, u8, u8);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_uchar8(u8, u8, u8, u8, u8, u8, u8, u8);
pub type vector_short1 = i16;

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_short2(i16, i16);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_short3(i16, i16, i16);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_short4(i16, i16, i16, i16);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_short8(i16, i16, i16, i16, i16, i16, i16, i16);
pub type vector_ushort1 = u16;

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_ushort2(u16, u16);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_ushort3(u16, u16, u16);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_ushort4(u16, u16, u16, u16);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_ushort8(u16, u16, u16, u16, u16, u16, u16, u16);
pub type vector_int1 = i32;

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_int2(i32, i32);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_int3(i32, i32, i32);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_int4(i32, i32, i32, i32);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_int8(i32, i32, i32, i32, i32, i32, i32, i32);
pub type vector_uint1 = u32;

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_uint2(u32, u32);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_uint3(u32, u32, u32);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_uint4(u32, u32, u32, u32);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_uint8(u32, u32, u32, u32, u32, u32, u32, u32);
pub type vector_float1 = f32;

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_float2(f32, f32);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_float3(f32, f32, f32);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_float4(f32, f32, f32, f32);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_float8(f32, f32, f32, f32, f32, f32, f32, f32);
pub type vector_long1 = i64;

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_long2(i64, i64);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_long3(i64, i64, i64);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_long4(i64, i64, i64, i64);
pub type vector_ulong1 = u64;

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_ulong2(u64, u64);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_ulong3(u64, u64, u64);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_ulong4(u64, u64, u64, u64);
pub type vector_double1 = f64;

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_double2(f64, f64);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_double3(f64, f64, f64);

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct vector_double4(f64, f64, f64, f64);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct matrix_float2x2(vector_float2, vector_float2);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct matrix_float3x2(vector_float2, vector_float2, vector_float2);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct matrix_float4x2(vector_float2, vector_float2, vector_float2, vector_float2);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct matrix_float2x3(vector_float3, vector_float3);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct matrix_float3x3(vector_float3, vector_float3, vector_float3);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct matrix_float4x3(vector_float3, vector_float3, vector_float3, vector_float3);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct matrix_float2x4(vector_float4, vector_float4);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct matrix_float3x4(vector_float4, vector_float4, vector_float4);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct matrix_float4x4(vector_float4, vector_float4, vector_float4, vector_float4);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct matrix_double2x2(vector_double2, vector_double2);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct matrix_double3x2(vector_double2, vector_double2, vector_double2);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct matrix_double4x2(vector_double2, vector_double2, vector_double2, vector_double2);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct matrix_double2x3(vector_double3, vector_double3);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct matrix_double3x3(vector_double3, vector_double3, vector_double3);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct matrix_double4x3(vector_double3, vector_double3, vector_double3, vector_double3);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct matrix_double2x4(vector_double4, vector_double4);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct matrix_double3x4(vector_double4, vector_double4, vector_double4);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct matrix_double4x4(vector_double4, vector_double4, vector_double4, vector_double4);
