use core:: ops::Neg;
use num::traits::{Bounded, Zero};
use std::ops::{Add, Sub};
use std::cmp::Ord;

pub trait MaxPlus {
    type Output;

    fn add(self, rhs: Self) -> Self::Output;

    fn mul(self, rhs: Self) -> Self::Output;

    fn zero() -> Self;

    fn is_zero(&self) -> bool;

    fn min_value() -> Self;

    fn max_value() -> Self;

    fn neg(self) -> Self::Output;

    fn div(self, rhs: Self) -> Self::Output;
}

impl<T> MaxPlus for T
where
T:Ord
    + Add
    + Add<Output = T>
    + Zero
    + Bounded
    + Eq
    + Neg
    + Neg<Output = T>
    + Sub
    + Sub<Output = T>
{
    type Output = T;

    fn add(self, rhs: T) -> T { self.max(rhs) }

    fn mul(self, rhs: T) -> T { self + rhs }

    fn zero() -> T { Bounded::min_value() }

    fn is_zero(&self) -> bool {
        let zero: T = Zero::zero();
        self == &zero
    }

    fn min_value() -> T { Bounded::min_value() }

    fn max_value() -> T { Bounded::max_value() }

    fn neg(self) -> T { -self }

    fn div(self, rhs: T) -> T { self - rhs }
}
