fn main() {
    // let x = 1u32 - 2; // compile error

    /* compile error
    const X: u32 = 1;
    const Y: u32 = 2;
    let z = X - Y;
    */

    /* runtime error
    let x: u32 = 1;
    let y: u32 = 2;
    let z = x - y;
    */

    let x: i32 = 1;
    let y: i32 = 2;
    let z = x - y;
    println!("{}", z);
}
