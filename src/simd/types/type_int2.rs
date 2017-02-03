use std;
use simd::Dot;
use simd::types::*;

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct int2(pub i32, pub i32);
pub type vector_int2 = int2;

extern "platform-intrinsic" {
  fn simd_add<T>(x: T, y: T) -> T;
  fn simd_sub<T>(x: T, y: T) -> T;
  fn simd_mul<T>(x: T, y: T) -> T;
  fn simd_div<T>(x: T, y: T) -> T;
}

impl std::ops::Add for int2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i32> for int2 {
  type Output = Self;

  #[inline]
  fn add(self, other: i32) -> Self {
    return unsafe { simd_add(self, int2::broadcast(other)) };
  }
}

impl std::ops::Add<int2> for i32 {
  type Output = int2;

  #[inline]
  fn add(self, other: int2) -> int2 {
    return unsafe { simd_add(int2::broadcast(self), other) };
  }
}

impl std::ops::Sub for int2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i32> for int2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i32) -> Self {
    return unsafe { simd_sub(self, int2::broadcast(other)) };
  }
}

impl std::ops::Sub<int2> for i32 {
  type Output = int2;

  #[inline]
  fn sub(self, other: int2) -> int2 {
    return unsafe { simd_sub(int2::broadcast(self), other) };
  }
}

impl std::ops::Mul for int2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i32> for int2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i32) -> Self {
    return unsafe { simd_mul(self, int2::broadcast(other)) };
  }
}

impl std::ops::Mul<int2> for i32 {
  type Output = int2;

  #[inline]
  fn mul(self, other: int2) -> int2 {
    return unsafe { simd_mul(int2::broadcast(self), other) };
  }
}

impl std::ops::Div for int2 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i32> for int2 {
  type Output = Self;

  #[inline]
  fn div(self, other: i32) -> Self {
    return unsafe { simd_div(self, int2::broadcast(other)) };
  }
}

impl std::ops::Div<int2> for i32 {
  type Output = int2;

  #[inline]
  fn div(self, other: int2) -> int2 {
    return unsafe { simd_div(int2::broadcast(self), other) };
  }
}

impl Dot for int2 {
  type Output = i32;

  #[inline]
  fn dot(self, other: int2) -> i32 {
    return (self * other).reduce_add();
  }
}

impl int2 {
  #[inline]
  pub fn broadcast(x: i32) -> int2 {
    return int2(x, x);
  }

  #[inline]
  pub fn low(self) -> int1 {
    return self.0;
  }

  #[inline]
  pub fn high(self) -> int1 {
    return self.1;
  }

  #[inline]
  pub fn reduce_add(self) -> i32 {
    return self.0 + self.1;
  }
}
