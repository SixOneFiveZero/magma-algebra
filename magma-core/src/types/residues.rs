// Modular Arithmatic under Addition

use crate::traits::Group;
use magma_macros::{group};

use std::ops::{Add, Sub, Neg};

#[group]
#[derive(Copy)]
pub struct Z<const N: u64>(u64);

impl<const N: u64> Add for Z<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 + rhs.0) % N)
    }
}

impl<const N: u64> Neg for Z<N> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        if self.0 == 0 {
            Self(0)
        } else {
            Self(N - self.0)
        }
    }
}

impl<const N: u64> Sub for Z<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}


impl<const N: u64> Z<N> {
    pub fn new(val: u64) -> Self {
        Self(val % N)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

impl<const N: u64> Group for Z<N> {}