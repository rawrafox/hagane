use std;
use objc;

use super::{ObjectiveC, CGRect};

pub type NSInteger = isize;
pub type NSUInteger = usize;

pub trait NSObject : ObjectiveC {
  forward!(is_equal_to, sel!(isEqualTo:), (object: T) -> bool, <T: NSObject>);
}

id!(NSObjectID, NSObject, "NSObject");

pub trait NSApplication : NSObject {
  forward!(run, sel!(run), () -> ());
  forward!(terminate, sel!(terminate:), (sender: T) -> (), <T: ObjectiveC>);
}

id!(NSApplicationID, NSApplication, "NSApplication");

impl NSApplicationID {
  pub fn shared_application() -> Self {
    unsafe {
      return msg_send![NSApplicationID::class(), sharedApplication];
    }
  }
}

impl NSObject for NSApplicationID {}

pub trait NSArray : NSObject {
  forward!(count, sel!(count), () -> usize);
  forward!(object_at_index, sel!(objectAtIndex:), (i: usize) -> T, <T: ObjectiveC>, retain);

  // Rust Helpers

  forward!(len, sel!(count), () -> usize);
  
  fn to_vec<T: 'static + ObjectiveC>(&self) -> Vec<T> where Self: 'static + Sized {
    let n = self.count();
    let mut result = Vec::with_capacity(n);

    for i in 0 .. n {
      result.push(self.object_at_index::<T>(i));
    }

    return result;
  }
}

id!(NSArrayID, NSArray, "NSArray");

impl NSObject for NSArrayID {}

pub trait NSCoder : NSObject {
  
}

id!(NSCoderID, NSCoder);

impl NSObject for NSCoderID {}

pub trait NSError : NSObject {
  
}

id!(NSErrorID, NSError, "NSError");

impl NSObject for NSErrorID {}

pub enum NSStringEncoding {
  NSASCIIStringEncoding = 1,
  NSNEXTSTEPStringEncoding = 2,
  NSJapaneseEUCStringEncoding = 3,
  NSUTF8StringEncoding = 4,
  NSISOLatin1StringEncoding = 5,
  NSSymbolStringEncoding = 6,
  NSNonLossyASCIIStringEncoding = 7,
  NSShiftJISStringEncoding = 8,
  NSISOLatin2StringEncoding = 9,
  NSUnicodeStringEncoding = 10,
  NSWindowsCP1251StringEncoding = 11,
  NSWindowsCP1252StringEncoding = 12,
  NSWindowsCP1253StringEncoding = 13,
  NSWindowsCP1254StringEncoding = 14,
  NSWindowsCP1250StringEncoding = 15,
  NSISO2022JPStringEncoding = 21,
  NSMacOSRomanStringEncoding = 30,

  NSUTF16BigEndianStringEncoding = 0x90000100,
  NSUTF16LittleEndianStringEncoding = 0x94000100,

  NSUTF32StringEncoding = 0x8c000100,
  NSUTF32BigEndianStringEncoding = 0x98000100,
  NSUTF32LittleEndianStringEncoding = 0x9c000100
}

pub trait NSString : ObjectiveC {
  initializer!(init_with_bytes_length_encoding, sel!(initWithBytes:length:encoding:), (bytes: *const std::os::raw::c_void, length: usize, encoding: NSStringEncoding));

  forward!(length_of_bytes_using_encoding, sel!(lengthOfBytesUsingEncoding:), (encoding: NSStringEncoding) -> usize);
  forward!(utf8_string, sel!(UTF8String), () -> *const u8);

  // Rust Helpers

  fn as_str(&self) -> &str where Self: 'static + Sized {
    let bytes = self.utf8_string();
    let len = self.len();

    unsafe {
      let bytes = std::slice::from_raw_parts(bytes, len);

      std::str::from_utf8(bytes).unwrap()
    }
  }

  fn len(&self) -> usize where Self: 'static + Sized {
    return self.length_of_bytes_using_encoding(NSStringEncoding::NSUTF8StringEncoding);
  }
}

id!(NSStringID, NSString, "NSString");

impl NSStringID {
  pub fn from_str(string: &str) -> NSStringID {
    let bytes = string.as_ptr() as *const std::os::raw::c_void;

    return Self::alloc().init_with_bytes_length_encoding(bytes, string.len(), NSStringEncoding::NSUTF8StringEncoding);
  }
}

impl NSObject for NSStringID {}

pub trait NSView : ObjectiveC {
}

id!(NSViewID, NSView, "NSView");

impl NSObject for NSViewID {}

pub trait NSWindow : NSObject {
  initializer!(init_with_content_rect_style_mask_backing_defer, sel!(initWithContentRect:styleMask:backing:defer:), (rect: CGRect, style_mask: usize, backing: usize, defer: bool));

  forward!(make_key_and_order_front, sel!(makeKeyAndOrderFront:), (sender: T) -> (), <T: ObjectiveC>);
  forward!(set_content_view, sel!(setContentView:), (view: T) -> (), <T: NSView>);
  forward!(set_delegate, sel!(setDelegate:), (delegate: T) -> (), <T: ObjectiveC>);
  forward!(set_title, sel!(setTitle:), (title: T) -> (), <T: NSString>);
}

id!(NSWindowID, NSWindow, "NSWindow");

impl NSObject for NSWindowID {}
