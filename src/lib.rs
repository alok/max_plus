#![feature(associated_type_bounds)]

mod max_plus;
mod min_plus;

#[cfg(test)]
mod tests {
    use crate::{max_plus::MaxPlus, min_plus::MinPlus};
    #[test]
    fn test_max_plus_ops() {
        let (a, b) = (3, 5);
        let sum = MaxPlus::add(a, b);
        let product = MaxPlus::mul(a, b);
        assert_eq!(sum, 5);
        assert_eq!(product, 8);
    }
    fn a() {
        let (a, b) = (MinPlus(3), MinPlus(5));
        let (sum, product) = (a + b, a * b);
        assert_eq!(sum, MinPlus(3));
        assert_eq!(product, MinPlus(8));
    }
}
