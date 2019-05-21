extern crate crossbeam;

type CB = FnMut(&mut i32) + 'static + Send + Sync;

struct Processor {
    callback: Box<CB>,
}

impl Processor {
    fn new<CB>(c: CB) -> Self
    where
        CB: FnMut(&mut i32) + 'static + Send + Sync,
    {
        Processor {
            callback: Box::new(c),
        }
    }

    fn set_callback<CB>(&mut self, c: CB)
    where
        CB: FnMut(&mut i32) + 'static + Send + Sync,
    {
        self.callback = Box::new(c);
    }

    fn process_events(&mut self) {
        let _th = crossbeam::scope(|scope| {
            let _jh = scope.spawn(move |_| {
                let mut i = 100;
                (self.callback)(&mut i);
                println!("{}", i);
            });
        })
        .unwrap();
    }
}

fn simple_callback(i: &mut i32) {
    println!("hello {}", i);
}

fn main() {
    let mut p = Processor::new(simple_callback);
    p.process_events();
    let s = "world!".to_string();
    let callback2 = move |i: &mut i32| {
        *i += 1;
        println!("hello {} {}", s, i);
    };
    p.set_callback(callback2);
    p.process_events();
}
