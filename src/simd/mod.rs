#![allow(non_camel_case_types, non_upper_case_globals)]

pub trait Dot<RHS = Self> {
    type Output;

    fn dot(self, rhs: RHS) -> Self::Output;
}

mod additions;
mod types;

pub use self::additions::*;
pub use self::types::*;
