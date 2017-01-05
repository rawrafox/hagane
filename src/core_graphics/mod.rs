pub type CGFloat = f64;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CGSize {
  pub width: CGFloat,
  pub height: CGFloat
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CGRect {
  pub origin: CGPoint,
  pub size: CGSize
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MTLClearColor {
  pub red: f64,
  pub green: f64,
  pub blue: f64,
  pub alpha: f64
}
