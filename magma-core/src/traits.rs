// Core algebraic traits
use std::ops::{Add, Neg, Sub, Mul, Div};


// Magma: Set with Binary Operation
pub trait AddMagma: Add<Output = Self> + Sized {
    fn op(self, rhs: Self) -> Self {
        self + rhs
    }
}
pub trait MultMagma: Mul<Output = Self> + Sized {
    fn op(self, rhs: Self) -> Self {
        self * rhs
    }
}

// Semigroup: Magma with Associativity (TODO)
pub trait AddSemigroup: AddMagma {}
pub trait MultSemigroup: MultMagma {}

// Monoid: Semigroup with Identity Element (From Default)
pub trait AddMonoid: AddSemigroup + Default {
    fn identity() -> Self {
        Self::default()
    }
}
pub trait MultMonoid: MultSemigroup + Default {
    fn identity() -> Self {
        Self::default()
    }
}

// Group: Monoid with Inverse to the Binary Operation
pub trait AddGroup: AddMonoid + Neg<Output = Self> + Sub<Output = Self>{
    fn inv_op(self, rhs: Self) -> Self {
        self - rhs
    }
}
pub trait MultGroup: MultMonoid + Div<Output = Self> {
    fn inv_op(self, rhs: Self) -> Self {
        self / rhs
    }
}


// Algebraic Law Verification
pub fn check_associativity_add<T: AddSemigroup + Clone + PartialEq + std::fmt::Debug>(a: T, b: T, c: T) {
    let left = a.clone().op(b.clone().op(c.clone()));
    let right: T = (a.clone().op(b.clone())).op(c.clone());
    assert_eq!(left, right, "Associativity Law Violated")
} 

pub fn check_identity_add<T: AddMonoid + Clone + PartialEq + std::fmt::Debug>(a: T) {
    let e = T::identity();
    assert_eq!(a.clone(), a.clone().op(e.clone()), "Right Identity Check Failed");
    assert_eq!(a.clone(), e.clone().op(a.clone()), "Left Identity Failed");
}

pub fn check_inverse_add<T: AddGroup + Clone +PartialEq + std::fmt::Debug>(a: T) {
    let inverse: T = -a.clone();
    assert_eq!(T::identity(), a.clone().op(inverse.clone()), "Right Inverse Failed");
    assert_eq!(a.clone().op(inverse.clone()), T::identity(), "Left Inverse Failed");
}