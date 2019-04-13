use std::thread;

fn th2() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
        v
    });
    let v2 = handle.join().unwrap();
    println!("joined vector: {:?}", v2);
}

fn main() {
    th2();
}
