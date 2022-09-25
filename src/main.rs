use num;
struct MaxPlus<T>(T);
struct MinPlus<T>(T);
// add

// times

impl num::traits::Zero for MaxPlus<f64> {
    fn zero() -> Self {
        MaxPlus(f64::NEG_INFINITY)
    }
    fn is_zero(&self) -> bool {
        self.0 == f64::NEG_INFINITY
    }
}

impl num::traits::Zero for MinPlus<f64> {
    fn zero() -> Self {
        MinPlus(f64::INFINITY)
    }
    fn is_zero(&self) -> bool {
        self.0 == f64::INFINITY }
}

impl num::traits::One for MaxPlus<f64> {
    fn one() -> Self {
        MaxPlus(0.0)
    }
    fn is_one(&self) -> bool { self.0 == 0.0 } }  
impl num::traits::One for MinPlus<f64> {
    fn one() -> Self {
        MinPlus(0.0)
    }
    fn is_one(&self) -> bool {
        self.0 == 0.0
    }
}



fn main() {
    println!("Hello, world!");
}

