use num::complex::Complex;
use std::f64::consts::PI;

fn main() {
    let c1 = Complex::new(10, 20);
    let c2 = Complex::new(4, 5);
    println!("{}", c1 + c2);
    println!("{}", c1 * c2);

    let cf1 = Complex::new(4.0, 3.0);
    let cf2 = Complex::new(8.0, 7.0);

    println!("{}", cf1.sin());
    println!("{}", cf1.cos());

    println!("{}", cf1 + cf2);

    let j = Complex::new(0.0, 1.0);
    let euler = PI.cos() + j * PI.sin();
    println!("{}", euler);
}
