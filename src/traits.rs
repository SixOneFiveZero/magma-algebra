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


pub fn is_associative<T: Semigroup + Clone + PartialEq>(a: T, b: T, c: T) -> bool {
    let left = a.clone().op(b.clone().op(c.clone()));
    let right: T = (a.clone().op(b.clone())).op(c.clone());
    left == right
} 

pub fn has_identity<T: Monoid + Clone + PartialEq>(a: T) -> bool {
    let e = T::identity();
    a.clone() == a.clone().op(e.clone()) && a.clone() == e.clone().op(a.clone())
}

pub fn has_inverse<T: Group + Clone +PartialEq>(a: T) -> bool {
    let inverse: T = -a.clone();
    T::identity() == a.clone().op(inverse)
}