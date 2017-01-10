#![allow(non_upper_case_globals)]

use std;
use objc;
use super::ObjectiveC;
use foundation::*;
bitflags! {
  pub flags MTLCompareFunction: NSUInteger {
    const MTLCompareFunctionNever = 0,
    const MTLCompareFunctionLess = 1,
    const MTLCompareFunctionEqual = 2,
    const MTLCompareFunctionLessEqual = 3,
    const MTLCompareFunctionGreater = 4,
    const MTLCompareFunctionNotEqual = 5,
    const MTLCompareFunctionGreaterEqual = 6,
    const MTLCompareFunctionAlways = 7,
  }
}
bitflags! {
  pub flags MTLCPUCacheMode: NSUInteger {
    const MTLCPUCacheModeDefaultCache = 0,
    const MTLCPUCacheModeWriteCombined = 1,
  }
}
bitflags! {
  pub flags MTLCullMode: NSUInteger {
    const MTLCullModeNone = 0,
    const MTLCullModeFront = 1,
    const MTLCullModeBack = 2,
  }
}
bitflags! {
  pub flags MTLIndexType: NSUInteger {
    const MTLIndexTypeUInt16 = 0,
    const MTLIndexTypeUInt32 = 1,
  }
}
bitflags! {
  pub flags MTLPrimitiveType: NSUInteger {
    const MTLPrimitiveTypePoint = 0,
    const MTLPrimitiveTypeLine = 1,
    const MTLPrimitiveTypeLineStrip = 2,
    const MTLPrimitiveTypeTriangle = 3,
    const MTLPrimitiveTypeTriangleStrip = 4,
  }
}
bitflags! {
  pub flags MTLStorageMode: NSUInteger {
    const MTLStorageModeShared = 0,
    const MTLStorageModeManaged = 1,
    const MTLStorageModePrivate = 2,
  }
}
bitflags! {
  pub flags MTLWinding: NSUInteger {
    const MTLWindingClockwise = 0,
    const MTLWindingCounterClockwise = 1,
  }
}
