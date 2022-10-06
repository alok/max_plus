mod max_plus;
mod min_plus;

#[cfg(test)]
mod tests {
    use crate::{max_plus::MaxPlus, min_plus::MinPlus};
    #[test]
    fn test_max_plus_ops() {
        let (a, b) = (3, 5);
        let (sum, product) = (MaxPlus::add(a, b), MaxPlus::mul(a, b));
        assert_eq!(sum, 5);
        assert_eq!(product, 8);
    }
    #[test]
    fn test_min_plus_ops() {
        let (a, b) = (3, 5);
        let (sum, product) = (MinPlus::add(a, b), MinPlus::mul(a, b));
        assert_eq!(sum, 3);
        assert_eq!(product, 8);
    }
}
