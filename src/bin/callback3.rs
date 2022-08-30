extern crate crossbeam;

type CB1 = dyn FnMut(i32, i32) -> i32 + Send + Sync + 'static;
type CB2 = dyn FnMut(&mut i32) -> i32 + Send + Sync + 'static;

struct Processor {
    callback1: Box<CB1>,
    callback2: Box<CB2>,
}

impl Processor {
    fn new<CB1, CB2>(cb1: CB1, cb2: CB2) -> Self
    where
        CB1: FnMut(i32, i32) -> i32 + Send + Sync + 'static,
        CB2: FnMut(&mut i32) -> i32 + Send + Sync + 'static,
    {
        Processor {
            callback1: Box::new(cb1),
            callback2: Box::new(cb2),
        }
    }

    fn process_event(&mut self) {
        let mut i = 5;
        (self.callback1)(i, i * 2);
        (self.callback2)(&mut i);
        println!("{}", i);
    }

    fn process_event_thread(&mut self) {
        let th = crossbeam::scope(|scope| {
            let mut i = 5;
            let j = 6;
            let jh = scope.spawn(move |_| {
                (self.callback1)(i, j * 2);
                (self.callback2)(&mut i);
                i
            });
            jh.join().unwrap()
        })
        .unwrap();
        println!("{}", th * 100);
    }

    fn set_callback<CB>(&mut self, c: CB)
    where
        CB: FnMut(&mut i32) -> i32 + Send + Sync + 'static,
    {
        self.callback2 = Box::new(c);
    }
}

fn simple_callback(i: &mut i32) -> i32 {
    let s = "my country!";
    *i += 2;
    println!("good afternoon {} {}", s, i);
    *i
}

fn main() {
    let k = 20;
    let s = "world!".to_string();

    // do callbacks specified closures
    let mut p = Processor::new(
        move |i, j| {
            println!("{}, {}, {}", i, j, k);
            i + j + k
        },
        move |i: &mut i32| {
            *i += 1;
            println!("hello {} {}", s, i);
            *i
        },
    );
    p.process_event();
    p.process_event_thread();

    // do callbacks from function.
    p.set_callback(simple_callback);
    p.process_event();
    p.process_event_thread();

    // do callbacks from closure
    let t = "my house!".to_string();
    let callback2 = move |m: &mut i32| {
        *m += 3;
        println!("good evening {} {}", t, m);
        *m
    };
    p.set_callback(callback2);
    p.process_event();
    p.process_event_thread();
}
