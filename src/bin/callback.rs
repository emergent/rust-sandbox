type CB = FnMut(i32, i32);

struct Processor {
    callback: Box<CB>,
}

impl Processor {
    fn process_event(&mut self) {
        let i = 5;
        (self.callback)(i, i * 2);
    }
}

fn main() {
    let mut p = Processor {
        callback: Box::new(|i, j| {
            println!("{}, {}", i, j);
        }),
    };

    p.process_event();
}
