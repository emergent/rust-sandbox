use std::ops::{Add, Mul, Neg};

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul for Point {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 1, z: 1 } + Point { x: 2, y: 3, z: 4 },
        Point { x: 3, y: 4, z: 5 }
    );

    assert_eq!(
        Point { x: 1, y: 1, z: 1 } * Point { x: 2, y: 3, z: 4 },
        Point { x: 2, y: 3, z: 4 }
    );

    assert_eq!(
        -Point { x: 1, y: 1, z: 1 },
        Point {
            x: -1,
            y: -1,
            z: -1
        }
    );
}
