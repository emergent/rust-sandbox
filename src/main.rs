use std::thread;
use std::time::Duration;

struct ThreadPool {
    th: std::thread::JoinHandle<()>,
}

impl ThreadPool {
    fn start() -> Self {
        let th = thread::spawn(|| {
            for i in 0 .. 10 {
                println!("hi! I'm {}", i);
            }
        });
        ThreadPool { th }
    }

    fn end(self) {
        self.th.join().unwrap()
    }
}

fn th3() {
    let tp = ThreadPool::start();
    println!("hogehoge!");
    tp.end();
}

fn th2() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}

fn th1() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn main() {
    th1();
    th2();
    th3();
}
