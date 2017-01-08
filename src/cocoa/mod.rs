use std;
use objc;

use super::{ObjectiveC, CGRect, NSObject, NSString};

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
