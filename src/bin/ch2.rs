use std::sync::mpsc::sync_channel;
use std::thread;
use std::time::Duration;

fn main() {
    let (sender, receiver) = sync_channel(1);

    sender.send(1).unwrap();
    println!("finished sending 1");

    thread::spawn(move || {
        // this will block until the previous message has been received
        sender.send(2).unwrap();
        println!("finished sending 2");
    });

    thread::sleep(Duration::from_millis(100));
    println!("{}", receiver.recv().unwrap());
    thread::sleep(Duration::from_millis(100));
    println!("{}", receiver.recv().unwrap());
}
