use {
    num::traits::{Bounded, Zero},
    std::{
        cmp::Ord,
        ops::{Add, Sub},
    },
};

pub trait MaxPlus {
    type Output;

    fn add(self, rhs: Self) -> Self::Output;

    fn mul(self, rhs: Self) -> Self::Output;

    fn zero() -> Self;

    fn is_zero(&self) -> bool;

    fn min_value() -> Self;

    fn max_value() -> Self;

    fn div(self, rhs: Self) -> Self::Output;
}

impl<T> MaxPlus for T
where T: Ord
        + Add<Output = T>
        + Zero
        + Bounded
        + Eq
        + Sub<Output = T>
{
    type Output = T;

    fn add(self, rhs: T) -> Self::Output { self.max(rhs) }

    fn mul(self, rhs: T) -> Self::Output { self + rhs }

    fn zero() -> Self::Output { Bounded::min_value() }

    fn is_zero(&self) -> bool {
        self == &Zero::zero()
    }

    fn min_value() -> Self { Bounded::min_value() }

    fn max_value() -> Self { Bounded::max_value() }

    fn div(self, rhs: T) -> Self::Output { self - rhs }
}

// // TODO 
// #[test]
// fn test_f64() {
//     let a = 1.0;
//     let b = MaxPlus::add(f64::INFINITY, a);
//     println!("{b}");
// }
