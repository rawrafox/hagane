use std;
use simd::Dot;
use simd::types::*;

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct int3(pub i32, pub i32, pub i32);
pub type vector_int3 = int3;

extern "platform-intrinsic" {
  fn simd_add<T>(x: T, y: T) -> T;
  fn simd_sub<T>(x: T, y: T) -> T;
  fn simd_mul<T>(x: T, y: T) -> T;
  fn simd_div<T>(x: T, y: T) -> T;
}

impl std::ops::Add for int3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i32> for int3 {
  type Output = Self;

  #[inline]
  fn add(self, other: i32) -> Self {
    return unsafe { simd_add(self, int3::broadcast(other)) };
  }
}

impl std::ops::Add<int3> for i32 {
  type Output = int3;

  #[inline]
  fn add(self, other: int3) -> int3 {
    return unsafe { simd_add(int3::broadcast(self), other) };
  }
}

impl std::ops::Sub for int3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i32> for int3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i32) -> Self {
    return unsafe { simd_sub(self, int3::broadcast(other)) };
  }
}

impl std::ops::Sub<int3> for i32 {
  type Output = int3;

  #[inline]
  fn sub(self, other: int3) -> int3 {
    return unsafe { simd_sub(int3::broadcast(self), other) };
  }
}

impl std::ops::Mul for int3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i32> for int3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i32) -> Self {
    return unsafe { simd_mul(self, int3::broadcast(other)) };
  }
}

impl std::ops::Mul<int3> for i32 {
  type Output = int3;

  #[inline]
  fn mul(self, other: int3) -> int3 {
    return unsafe { simd_mul(int3::broadcast(self), other) };
  }
}

impl std::ops::Div for int3 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i32> for int3 {
  type Output = Self;

  #[inline]
  fn div(self, other: i32) -> Self {
    return unsafe { simd_div(self, int3::broadcast(other)) };
  }
}

impl std::ops::Div<int3> for i32 {
  type Output = int3;

  #[inline]
  fn div(self, other: int3) -> int3 {
    return unsafe { simd_div(int3::broadcast(self), other) };
  }
}

impl Dot for int3 {
  type Output = i32;

  #[inline]
  fn dot(self, other: int3) -> i32 {
    return (self * other).reduce_add();
  }
}

impl int3 {
  #[inline]
  pub fn broadcast(x: i32) -> int3 {
    return int3(x, x, x);
  }

  #[inline]
  pub fn low(self) -> int2 {
    return int2(self.0, self.1);
  }

  #[inline]
  pub fn high(self) -> int2 {
    return int2(self.2, 0);
  }

  #[inline]
  pub fn reduce_add(self) -> i32 {
    return self.0 + self.1 + self.2;
  }
}
