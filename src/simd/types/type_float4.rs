use std;
use simd::Dot;
use simd::types::*;

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct float4(pub f32, pub f32, pub f32, pub f32);
pub type vector_float4 = float4;

extern "platform-intrinsic" {
  fn simd_add<T>(x: T, y: T) -> T;
  fn simd_sub<T>(x: T, y: T) -> T;
  fn simd_mul<T>(x: T, y: T) -> T;
  fn simd_div<T>(x: T, y: T) -> T;
}

impl std::ops::Add for float4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<f32> for float4 {
  type Output = Self;

  #[inline]
  fn add(self, other: f32) -> Self {
    return unsafe { simd_add(self, float4::broadcast(other)) };
  }
}

impl std::ops::Add<float4> for f32 {
  type Output = float4;

  #[inline]
  fn add(self, other: float4) -> float4 {
    return unsafe { simd_add(float4::broadcast(self), other) };
  }
}

impl std::ops::Sub for float4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<f32> for float4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: f32) -> Self {
    return unsafe { simd_sub(self, float4::broadcast(other)) };
  }
}

impl std::ops::Sub<float4> for f32 {
  type Output = float4;

  #[inline]
  fn sub(self, other: float4) -> float4 {
    return unsafe { simd_sub(float4::broadcast(self), other) };
  }
}

impl std::ops::Mul for float4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<f32> for float4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f32) -> Self {
    return unsafe { simd_mul(self, float4::broadcast(other)) };
  }
}

impl std::ops::Mul<float4> for f32 {
  type Output = float4;

  #[inline]
  fn mul(self, other: float4) -> float4 {
    return unsafe { simd_mul(float4::broadcast(self), other) };
  }
}

impl std::ops::Div for float4 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<f32> for float4 {
  type Output = Self;

  #[inline]
  fn div(self, other: f32) -> Self {
    return unsafe { simd_div(self, float4::broadcast(other)) };
  }
}

impl std::ops::Div<float4> for f32 {
  type Output = float4;

  #[inline]
  fn div(self, other: float4) -> float4 {
    return unsafe { simd_div(float4::broadcast(self), other) };
  }
}

impl Dot for float4 {
  type Output = f32;

  #[inline]
  fn dot(self, other: float4) -> f32 {
    return (self * other).reduce_add();
  }
}

impl float4 {
  #[inline]
  pub fn broadcast(x: f32) -> float4 {
    return float4(x, x, x, x);
  }

  #[inline]
  pub fn length_squared(self) -> f32 {
    return self.dot(self);
  }

  #[inline]
  pub fn normalize(self) -> float4 {
    return self / self.length_squared().sqrt();
  }

  #[inline]
  pub fn low(self) -> float2 {
    return float2(self.0, self.1);
  }

  #[inline]
  pub fn high(self) -> float2 {
    return float2(self.1, self.0);
  }

  #[inline]
  pub fn reduce_add(self) -> f32 {
    return (self.low() + self.high()).reduce_add();
  }
}
