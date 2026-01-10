// Core algebraic traits

pub trait Magma {}

pub trait Semigroup: Magma {}

pub trait Monoid: Semigroup {}

pub trait Group: Monoid {}
