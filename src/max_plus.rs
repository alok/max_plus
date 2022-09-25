use {
    core::ops::Neg,
    derive_more::{Display, From},
    num::{
        self,
        traits::{Bounded, One, Zero},
    },
    std::{
        cmp::{max, Ord},
        ops::{Add, Div, Mul, Sub},
    },
};

#[derive(Clone, Copy, Debug, Display, Eq, From, Ord, PartialEq, PartialOrd)]
pub(crate) struct MaxPlus<T>(pub(crate) T);

impl<T: Ord> Add for MaxPlus<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output { Self(max(self.0, rhs.0)) }
}

impl<T: Add + Add<Output = T>> Mul for MaxPlus<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output { Self(self.0 + rhs.0) }
}

impl<T: Zero + Bounded + Ord> Zero for MaxPlus<T> {
    fn zero() -> Self { MaxPlus(Bounded::min_value()) }

    fn is_zero(&self) -> bool { self.0 == Bounded::min_value() }
}

impl<T: Zero + Eq> One for MaxPlus<T> {
    fn one() -> Self { MaxPlus(Zero::zero()) }

    fn is_one(&self) -> bool { self.0 == Zero::zero() }
}

impl<T: Bounded> Bounded for MaxPlus<T> {
    fn min_value() -> Self { MaxPlus(Bounded::min_value()) }

    fn max_value() -> Self { MaxPlus(Bounded::max_value()) }
}

impl<T: Neg + Neg<Output = T>> Neg for MaxPlus<T> {
    type Output = Self;

    fn neg(self) -> Self::Output { Self(-self.0) }
}

impl<T: Sub + Sub<Output = T>> Div for MaxPlus<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output { Self(self.0 - rhs.0) }
}
