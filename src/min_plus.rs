use {
    core::ops::Neg,
    derive_more::{Display, From},
    num::{
        self,
        traits::{Bounded, One, Zero},
    },
    std::{
        cmp::{min, Ord},
        ops::{Add, Div, Mul, Sub},
    },
};

#[derive(Clone, Copy, Debug, Display, Eq, From, Ord, PartialEq, PartialOrd)]
pub(crate) struct MinPlus<T>(pub(crate) T);

impl<T: Ord> Add for MinPlus<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output { Self(min(self.0, rhs.0)) }
}

impl<T: Add + Add<Output = T>> Mul for MinPlus<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output { Self(self.0 + rhs.0) }
}

impl<T: Zero + Bounded + Ord> Zero for MinPlus<T> {
    fn zero() -> Self { MinPlus(Bounded::max_value()) }

    fn is_zero(&self) -> bool { self.0 == Bounded::max_value() }
}

impl<T: Zero + Eq> One for MinPlus<T> {
    fn one() -> Self { MinPlus(Zero::zero()) }

    fn is_one(&self) -> bool { self.0 == Zero::zero() }
}

impl<T: Bounded> Bounded for MinPlus<T> {
    fn min_value() -> Self { MinPlus(Bounded::min_value()) }

    fn max_value() -> Self { MinPlus(Bounded::max_value()) }
}

impl<T: Neg + Neg<Output = T>> Neg for MinPlus<T> {
    type Output = Self;

    fn neg(self) -> Self::Output { Self(-self.0) }
}

impl<T: Sub + Sub<Output = T>> Div for MinPlus<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output { Self(self.0 - rhs.0) }
}
