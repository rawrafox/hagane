#![allow(non_upper_case_globals)]

pub type CGFloat = f64;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CGPoint {
  pub x: CGFloat,
  pub y: CGFloat,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CGSize {
  pub width: CGFloat,
  pub height: CGFloat,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CGRect {
  pub origin: CGPoint,
  pub size: CGSize,
}
