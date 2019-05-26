extern crate num;
use num::{Integer, NumCast};

trait Prime {
    fn is_prime(&self) -> bool;
}

impl<T: Integer + NumCast + Copy> Prime for T {
    fn is_prime(&self) -> bool {
        let two: T = NumCast::from(2).unwrap();
        let three: T = NumCast::from(3).unwrap();
        match self {
            &i if i < two => false,
            &i if i == two => true,
            &i if i % two == T::zero() => false,
            _ => {
                let mut j = three;
                loop {
                    if *self % j == T::zero() {
                        return false;
                    }
                    if j * j >= *self {
                        break;
                    }
                    j = j + two;
                }
                true
            }
        }
    }
}

fn main() {
    let a: i128 = 20190523;
    println!("{}", a.is_prime());
}
