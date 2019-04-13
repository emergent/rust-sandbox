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
                thread::sleep(Duration::from_millis(1));
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
    println!("waiting the end of threads!");
    tp.end();
}

fn main() {
    th3();
}
