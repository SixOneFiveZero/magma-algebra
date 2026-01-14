// Core algebraic traits
use std::ops::{Add, Neg, Sub};


// Magma: Set with Binary Operation
pub trait Magma: Add<Output = Self> + Sized {
    fn op(self, rhs: Self) -> Self {
        self + rhs
    }
}

// Semigroup: Magma with Associativity (TODO)
pub trait Semigroup: Magma {}

// Monoid: Semigroup with Identity Element (From Default)
pub trait Monoid: Semigroup + Default {
    fn identity() -> Self {
        Self::default()
    }
}

// Group: Monoid with Inverse to the Binary Operation
pub trait Group: Monoid + Neg<Output = Self> + Sub<Output = Self>{
    fn inv_op(self, rhs: Self) -> Self {
        self - rhs
    }
}
