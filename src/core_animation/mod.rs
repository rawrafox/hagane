use std;
use objc;

use super::NSObject;

pub trait CAMetalDrawable : NSObject {}

id!(CAMetalDrawableID, CAMetalDrawable);

impl NSObject for CAMetalDrawableID {}
