fn main() {
    // let x = 1u32 - 2; // compile error

    /* compile error
    const X: u32 = 1;
    const Y: u32 = 2;
    let z = X - Y;
    */

    let x: u32 = 1;
    let y: u32 = 2;
    let z = x - y; // runtime error
    println!("{}", z);
}
