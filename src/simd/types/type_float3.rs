use std;
use simd::Dot;
use simd::types::*;

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct float3(pub f32, pub f32, pub f32);
pub type vector_float3 = float3;

extern "platform-intrinsic" {
  fn simd_add<T>(x: T, y: T) -> T;
  fn simd_sub<T>(x: T, y: T) -> T;
  fn simd_mul<T>(x: T, y: T) -> T;
  fn simd_div<T>(x: T, y: T) -> T;
}

impl std::ops::Add for float3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<f32> for float3 {
  type Output = Self;

  #[inline]
  fn add(self, other: f32) -> Self {
    return unsafe { simd_add(self, float3::broadcast(other)) };
  }
}

impl std::ops::Add<float3> for f32 {
  type Output = float3;

  #[inline]
  fn add(self, other: float3) -> float3 {
    return unsafe { simd_add(float3::broadcast(self), other) };
  }
}

impl std::ops::Sub for float3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<f32> for float3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: f32) -> Self {
    return unsafe { simd_sub(self, float3::broadcast(other)) };
  }
}

impl std::ops::Sub<float3> for f32 {
  type Output = float3;

  #[inline]
  fn sub(self, other: float3) -> float3 {
    return unsafe { simd_sub(float3::broadcast(self), other) };
  }
}

impl std::ops::Mul for float3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<f32> for float3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f32) -> Self {
    return unsafe { simd_mul(self, float3::broadcast(other)) };
  }
}

impl std::ops::Mul<float3> for f32 {
  type Output = float3;

  #[inline]
  fn mul(self, other: float3) -> float3 {
    return unsafe { simd_mul(float3::broadcast(self), other) };
  }
}

impl std::ops::Div for float3 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<f32> for float3 {
  type Output = Self;

  #[inline]
  fn div(self, other: f32) -> Self {
    return unsafe { simd_div(self, float3::broadcast(other)) };
  }
}

impl std::ops::Div<float3> for f32 {
  type Output = float3;

  #[inline]
  fn div(self, other: float3) -> float3 {
    return unsafe { simd_div(float3::broadcast(self), other) };
  }
}

impl Dot for float3 {
  type Output = f32;

  #[inline]
  fn dot(self, other: float3) -> f32 {
    return (self * other).reduce_add();
  }
}

impl float3 {
  #[inline]
  pub fn broadcast(x: f32) -> float3 {
    return float3(x, x, x);
  }

  #[inline]
  pub fn length_squared(self) -> f32 {
    return self.dot(self);
  }

  #[inline]
  pub fn normalize(self) -> float3 {
    return self / self.length_squared().sqrt();
  }

  #[inline]
  pub fn low(self) -> float2 {
    return float2(self.0, self.1);
  }

  #[inline]
  pub fn high(self) -> float2 {
    return float2(self.2, 0.0);
  }

  #[inline]
  pub fn reduce_add(self) -> f32 {
    return self.0 + self.1 + self.2;
  }
}
