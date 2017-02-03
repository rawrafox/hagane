use std;
use simd::Dot;
use simd::types::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct float4x4(pub float4, pub float4, pub float4, pub float4);
pub type matrix_float4x4 = float4x4;
pub static identity_float4x4: float4x4 = float4x4(float4(1.0, 0.0, 0.0, 0.0), float4(0.0, 1.0, 0.0, 0.0), float4(0.0, 0.0, 1.0, 0.0), float4(0.0, 0.0, 0.0, 1.0));
pub static matrix_identity_float4x4: float4x4 = float4x4(float4(1.0, 0.0, 0.0, 0.0), float4(0.0, 1.0, 0.0, 0.0), float4(0.0, 0.0, 1.0, 0.0), float4(0.0, 0.0, 0.0, 1.0));

impl std::ops::Add for float4x4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return float4x4(self.0.add(other.0), self.1.add(other.1), self.2.add(other.2), self.3.add(other.3));
  }
}

impl std::ops::Add<f32> for float4x4 {
  type Output = Self;

  #[inline]
  fn add(self, other: f32) -> Self {
    let other = float4::broadcast(other);

    return float4x4(self.0.add(other), self.1.add(other), self.2.add(other), self.3.add(other));
  }
}

impl std::ops::Add<float4x4> for f32 {
  type Output = float4x4;

  #[inline]
  fn add(self, other: float4x4) -> float4x4 {
    let scalar = float4::broadcast(self);

    return float4x4(scalar.add(other.0), scalar.add(other.1), scalar.add(other.2), scalar.add(other.3));
  }
}

impl std::ops::Sub for float4x4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return float4x4(self.0.sub(other.0), self.1.sub(other.1), self.2.sub(other.2), self.3.sub(other.3));
  }
}

impl std::ops::Sub<f32> for float4x4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: f32) -> Self {
    let other = float4::broadcast(other);

    return float4x4(self.0.sub(other), self.1.sub(other), self.2.sub(other), self.3.sub(other));
  }
}

impl std::ops::Sub<float4x4> for f32 {
  type Output = float4x4;

  #[inline]
  fn sub(self, other: float4x4) -> float4x4 {
    let scalar = float4::broadcast(self);

    return float4x4(scalar.sub(other.0), scalar.sub(other.1), scalar.sub(other.2), scalar.sub(other.3));
  }
}

impl std::ops::Mul for float4x4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return float4x4(self.0.mul(other.0), self.1.mul(other.1), self.2.mul(other.2), self.3.mul(other.3));
  }
}

impl std::ops::Mul<f32> for float4x4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f32) -> Self {
    let other = float4::broadcast(other);

    return float4x4(self.0.mul(other), self.1.mul(other), self.2.mul(other), self.3.mul(other));
  }
}

impl std::ops::Mul<float4x4> for f32 {
  type Output = float4x4;

  #[inline]
  fn mul(self, other: float4x4) -> float4x4 {
    let scalar = float4::broadcast(self);

    return float4x4(scalar.mul(other.0), scalar.mul(other.1), scalar.mul(other.2), scalar.mul(other.3));
  }
}

impl std::ops::Div for float4x4 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return float4x4(self.0.div(other.0), self.1.div(other.1), self.2.div(other.2), self.3.div(other.3));
  }
}

impl std::ops::Div<f32> for float4x4 {
  type Output = Self;

  #[inline]
  fn div(self, other: f32) -> Self {
    let other = float4::broadcast(other);

    return float4x4(self.0.div(other), self.1.div(other), self.2.div(other), self.3.div(other));
  }
}

impl std::ops::Div<float4x4> for f32 {
  type Output = float4x4;

  #[inline]
  fn div(self, other: float4x4) -> float4x4 {
    let scalar = float4::broadcast(self);

    return float4x4(scalar.div(other.0), scalar.div(other.1), scalar.div(other.2), scalar.div(other.3));
  }
}

impl Dot for float4x4 {
  type Output = float4x4;

  #[inline]
  fn dot(self, other: float4x4) -> float4x4 {
    return float4x4(self.dot(other.0), self.dot(other.1), self.dot(other.2), self.dot(other.3));
  }
}

impl Dot<float4> for float4x4 {
  type Output = float4;

  #[inline]
  fn dot(self, other: float4) -> float4 {
    return self.0 * other.0 + self.1 * other.1 + self.2 * other.2 + self.3 * other.3;
  }
}
