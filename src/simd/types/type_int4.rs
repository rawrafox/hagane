use std;
use simd::Dot;
use simd::types::*;

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct int4(pub i32, pub i32, pub i32, pub i32);
pub type vector_int4 = int4;

extern "platform-intrinsic" {
  fn simd_add<T>(x: T, y: T) -> T;
  fn simd_sub<T>(x: T, y: T) -> T;
  fn simd_mul<T>(x: T, y: T) -> T;
  fn simd_div<T>(x: T, y: T) -> T;
}

impl std::ops::Add for int4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i32> for int4 {
  type Output = Self;

  #[inline]
  fn add(self, other: i32) -> Self {
    return unsafe { simd_add(self, int4::broadcast(other)) };
  }
}

impl std::ops::Add<int4> for i32 {
  type Output = int4;

  #[inline]
  fn add(self, other: int4) -> int4 {
    return unsafe { simd_add(int4::broadcast(self), other) };
  }
}

impl std::ops::Sub for int4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i32> for int4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i32) -> Self {
    return unsafe { simd_sub(self, int4::broadcast(other)) };
  }
}

impl std::ops::Sub<int4> for i32 {
  type Output = int4;

  #[inline]
  fn sub(self, other: int4) -> int4 {
    return unsafe { simd_sub(int4::broadcast(self), other) };
  }
}

impl std::ops::Mul for int4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i32> for int4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i32) -> Self {
    return unsafe { simd_mul(self, int4::broadcast(other)) };
  }
}

impl std::ops::Mul<int4> for i32 {
  type Output = int4;

  #[inline]
  fn mul(self, other: int4) -> int4 {
    return unsafe { simd_mul(int4::broadcast(self), other) };
  }
}

impl std::ops::Div for int4 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i32> for int4 {
  type Output = Self;

  #[inline]
  fn div(self, other: i32) -> Self {
    return unsafe { simd_div(self, int4::broadcast(other)) };
  }
}

impl std::ops::Div<int4> for i32 {
  type Output = int4;

  #[inline]
  fn div(self, other: int4) -> int4 {
    return unsafe { simd_div(int4::broadcast(self), other) };
  }
}

impl Dot for int4 {
  type Output = i32;

  #[inline]
  fn dot(self, other: int4) -> i32 {
    return (self * other).reduce_add();
  }
}

impl int4 {
  #[inline]
  pub fn broadcast(x: i32) -> int4 {
    return int4(x, x, x, x);
  }

  #[inline]
  pub fn low(self) -> int2 {
    return int2(self.0, self.1);
  }

  #[inline]
  pub fn high(self) -> int2 {
    return int2(self.1, self.0);
  }

  #[inline]
  pub fn reduce_add(self) -> i32 {
    return (self.low() + self.high()).reduce_add();
  }
}
