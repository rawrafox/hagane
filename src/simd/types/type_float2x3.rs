use std;
use simd::types::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct float2x3(pub float3, pub float3);
pub type matrix_float2x3 = float2x3;

impl std::ops::Add for float2x3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return float2x3(self.0.add(other.0), self.1.add(other.1));
  }
}

impl std::ops::Add<f32> for float2x3 {
  type Output = Self;

  #[inline]
  fn add(self, other: f32) -> Self {
    let other = float3::broadcast(other);

    return float2x3(self.0.add(other), self.1.add(other));
  }
}

impl std::ops::Add<float2x3> for f32 {
  type Output = float2x3;

  #[inline]
  fn add(self, other: float2x3) -> float2x3 {
    let scalar = float3::broadcast(self);

    return float2x3(scalar.add(other.0), scalar.add(other.1));
  }
}

impl std::ops::Sub for float2x3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return float2x3(self.0.sub(other.0), self.1.sub(other.1));
  }
}

impl std::ops::Sub<f32> for float2x3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: f32) -> Self {
    let other = float3::broadcast(other);

    return float2x3(self.0.sub(other), self.1.sub(other));
  }
}

impl std::ops::Sub<float2x3> for f32 {
  type Output = float2x3;

  #[inline]
  fn sub(self, other: float2x3) -> float2x3 {
    let scalar = float3::broadcast(self);

    return float2x3(scalar.sub(other.0), scalar.sub(other.1));
  }
}

impl std::ops::Mul for float2x3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return float2x3(self.0.mul(other.0), self.1.mul(other.1));
  }
}

impl std::ops::Mul<f32> for float2x3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f32) -> Self {
    let other = float3::broadcast(other);

    return float2x3(self.0.mul(other), self.1.mul(other));
  }
}

impl std::ops::Mul<float2x3> for f32 {
  type Output = float2x3;

  #[inline]
  fn mul(self, other: float2x3) -> float2x3 {
    let scalar = float3::broadcast(self);

    return float2x3(scalar.mul(other.0), scalar.mul(other.1));
  }
}

impl std::ops::Div for float2x3 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return float2x3(self.0.div(other.0), self.1.div(other.1));
  }
}

impl std::ops::Div<f32> for float2x3 {
  type Output = Self;

  #[inline]
  fn div(self, other: f32) -> Self {
    let other = float3::broadcast(other);

    return float2x3(self.0.div(other), self.1.div(other));
  }
}

impl std::ops::Div<float2x3> for f32 {
  type Output = float2x3;

  #[inline]
  fn div(self, other: float2x3) -> float2x3 {
    let scalar = float3::broadcast(self);

    return float2x3(scalar.div(other.0), scalar.div(other.1));
  }
}
