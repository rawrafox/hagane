#![feature(crate_in_paths)]
#![feature(crate_visibility_modifier)]
#![feature(repr_transparent)]

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

extern crate hagane_core;

mod allocator;
// mod array;
// mod attributed_string;
// mod bag;
// mod binary_heap;
// mod bit_vector;
// mod bundle;
// mod byte_order;
// mod calendar;
// mod character_set;
// mod data;
// mod date;
// mod date_formatter;
// mod dictionary;
// mod error;
// mod file_descriptor;
// mod file_security;
// mod locale;
// mod mach_port;
// mod message_port;
// mod notification_center;
mod null;
// mod number;
// mod number_formatter;
mod object;
// mod plug_in;
// mod preferences;
// mod property_list;
// mod run_loop;
// mod set;
// mod socket;
// mod stream;
mod string;
// mod string_tokenizer;
// mod time_zone;
// mod tree;
// mod url;
// mod url_access;
// mod url_enumerator;
// mod user_notification;
// Utilities?
// mod uuid;
// mod xml_node;
// mod xml_parser;

pub use allocator::*;
pub use null::*;
pub use object::*;
pub use string::*;

use std::mem;
use std::os::raw::c_void;

use hagane_core::Subtype;

#[derive(Debug, Eq, PartialEq)]
#[repr(u8)] pub enum Boolean {
  TRUE = 1,
  FALSE = 0
}

pub type Byte = u8;
pub type SignedByte = i8;

pub type UInt8 = u8;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub type SInt8 = i8;
pub type SInt16 = i16;
pub type SInt32 = i32;
pub type SInt64 = i64;

pub type Float32 = f32;
pub type Float64 = f64;

pub type UniChar = u16;
pub type UniCharCount = u64;
pub type StringPtr = *mut u8;
pub type ConstStringPtr = *const u8;
pub type Str255 = [u8; 256];
pub type ConstStr255Param = *const u8;

pub type UTF8Char = u8;
pub type UTF16Char = u16;
pub type UTF32Char = u32;

pub type CFIndex = i64;

#[repr(transparent)] pub struct OSStatus(i32);
#[repr(transparent)] pub struct OSErr(i16);
#[repr(transparent)] pub struct RegionCode(i16);
#[repr(transparent)] pub struct LangCode(i16);
#[repr(transparent)] pub struct ScriptCode(i16);
#[repr(transparent)] pub struct FourCharCode(u32);
#[repr(transparent)] pub struct OSType(u32);

#[repr(transparent)] #[derive(PartialEq, Eq, Debug)] pub struct CFTypeID(u64);
#[repr(transparent)] pub struct CFOptionFlags(u64);
#[repr(transparent)] pub struct CFHashCode(u64);

#[repr(transparent)] pub struct CFPropertyListRef(usize);

#[repr(i64)]
pub enum CFComparisonResult {
  kCFCompareLessThan = -1,
  kCFCompareEqualTo = 0,
  kCFCompareGreaterThan = 1
}

pub type CFComparatorFunction = unsafe extern fn(val1: *const c_void, val2: *const c_void, context: *mut c_void) -> CFComparisonResult;

pub const kCFNotFound: CFIndex = -1;

pub struct CFRange {
  location: CFIndex,
  length: CFIndex
}

pub fn CFRangeMake(loc: CFIndex, len: CFIndex) -> CFRange {
  return CFRange { location: loc, length: len };
}

mod ext {
  use crate::*;
  
  #[link(name = "CoreFoundation", kind = "framework")]
  extern {
    pub fn CFCopyTypeIDDescription(type_id: CFTypeID) -> CFStringRef;
  }
}

pub fn CFCopyTypeIDDescription(type_id: CFTypeID) -> CFStringRef {
  return unsafe { ext::CFCopyTypeIDDescription(type_id) };
}
