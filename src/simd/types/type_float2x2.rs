use std;
use simd::Dot;
use simd::types::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct float2x2(pub float2, pub float2);
pub type matrix_float2x2 = float2x2;
pub static identity_float2x2: float2x2 = float2x2(float2(1.0, 0.0), float2(0.0, 1.0));
pub static matrix_identity_float2x2: float2x2 = float2x2(float2(1.0, 0.0), float2(0.0, 1.0));

impl std::ops::Add for float2x2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return float2x2(self.0.add(other.0), self.1.add(other.1));
  }
}

impl std::ops::Add<f32> for float2x2 {
  type Output = Self;

  #[inline]
  fn add(self, other: f32) -> Self {
    let other = float2::broadcast(other);

    return float2x2(self.0.add(other), self.1.add(other));
  }
}

impl std::ops::Add<float2x2> for f32 {
  type Output = float2x2;

  #[inline]
  fn add(self, other: float2x2) -> float2x2 {
    let scalar = float2::broadcast(self);

    return float2x2(scalar.add(other.0), scalar.add(other.1));
  }
}

impl std::ops::Sub for float2x2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return float2x2(self.0.sub(other.0), self.1.sub(other.1));
  }
}

impl std::ops::Sub<f32> for float2x2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: f32) -> Self {
    let other = float2::broadcast(other);

    return float2x2(self.0.sub(other), self.1.sub(other));
  }
}

impl std::ops::Sub<float2x2> for f32 {
  type Output = float2x2;

  #[inline]
  fn sub(self, other: float2x2) -> float2x2 {
    let scalar = float2::broadcast(self);

    return float2x2(scalar.sub(other.0), scalar.sub(other.1));
  }
}

impl std::ops::Mul for float2x2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return float2x2(self.0.mul(other.0), self.1.mul(other.1));
  }
}

impl std::ops::Mul<f32> for float2x2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f32) -> Self {
    let other = float2::broadcast(other);

    return float2x2(self.0.mul(other), self.1.mul(other));
  }
}

impl std::ops::Mul<float2x2> for f32 {
  type Output = float2x2;

  #[inline]
  fn mul(self, other: float2x2) -> float2x2 {
    let scalar = float2::broadcast(self);

    return float2x2(scalar.mul(other.0), scalar.mul(other.1));
  }
}

impl std::ops::Div for float2x2 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return float2x2(self.0.div(other.0), self.1.div(other.1));
  }
}

impl std::ops::Div<f32> for float2x2 {
  type Output = Self;

  #[inline]
  fn div(self, other: f32) -> Self {
    let other = float2::broadcast(other);

    return float2x2(self.0.div(other), self.1.div(other));
  }
}

impl std::ops::Div<float2x2> for f32 {
  type Output = float2x2;

  #[inline]
  fn div(self, other: float2x2) -> float2x2 {
    let scalar = float2::broadcast(self);

    return float2x2(scalar.div(other.0), scalar.div(other.1));
  }
}

impl Dot for float2x2 {
  type Output = float2x2;

  #[inline]
  fn dot(self, other: float2x2) -> float2x2 {
    return float2x2(self.dot(other.0), self.dot(other.1));
  }
}

impl Dot<float2> for float2x2 {
  type Output = float2;

  #[inline]
  fn dot(self, other: float2) -> float2 {
    return self.0 * other.0 + self.1 * other.1;
  }
}
