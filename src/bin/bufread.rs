use std::io;
use std::io::{BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    //let reader = BufReader::new(stdin);

    let mut count = 0;
    for _ in reader.lines() {
        count += 1;
    }
    println!("{}", count);
}
