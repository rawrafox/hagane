use std;
use objc;
use super::ObjectiveC;

#[repr(u64)]
pub enum MTLCompareFunction {
  MTLCompareFunctionNever = 0,
  MTLCompareFunctionLess = 1,
  MTLCompareFunctionEqual = 2,
  MTLCompareFunctionLessEqual = 3,
  MTLCompareFunctionGreater = 4,
  MTLCompareFunctionNotEqual = 5,
  MTLCompareFunctionGreaterEqual = 6,
  MTLCompareFunctionAlways = 7,
}

#[repr(u64)]
pub enum MTLCPUCacheMode {
  MTLCPUCacheModeDefaultCache = 0,
  MTLCPUCacheModeWriteCombined = 1,
}

#[repr(u64)]
pub enum MTLCullMode {
  MTLCullModeNone = 0,
  MTLCullModeFront = 1,
  MTLCullModeBack = 2,
}

#[repr(u64)]
pub enum MTLIndexType {
  MTLIndexTypeUInt16 = 0,
  MTLIndexTypeUInt32 = 1,
}

#[repr(u64)]
pub enum MTLPrimitiveType {
  MTLPrimitiveTypePoint = 0,
  MTLPrimitiveTypeLine = 1,
  MTLPrimitiveTypeLineStrip = 2,
  MTLPrimitiveTypeTriangle = 3,
  MTLPrimitiveTypeTriangleStrip = 4,
}

#[repr(u64)]
pub enum MTLStorageMode {
  MTLStorageModeShared = 0,
  MTLStorageModeManaged = 1,
  MTLStorageModePrivate = 2,
}

#[repr(u64)]
pub enum MTLWinding {
  MTLWindingClockwise = 0,
  MTLWindingCounterClockwise = 1,
}
