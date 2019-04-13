use std::sync::mpsc::{channel, SendError};
use std::thread;
use std::time::Duration;

fn main() {
    let (sender, receiver) = channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        match sender.send(12345) {
            Ok(n) => println!("{:?}", n),
            Err(SendError(e)) => println!("{}", e),
        }
    });

    println!("waiting...");
    println!("{:?}", receiver.recv().unwrap());
}
