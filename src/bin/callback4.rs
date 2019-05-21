struct Processor {
    callback: Box<FnMut(i32)>,
}

impl Processor {
    fn new<CB: 'static + FnMut(i32)>(c: CB) -> Self {
        Processor {
            callback: Box::new(c),
        }
    }

    fn set_callback<CB: 'static + FnMut(i32)>(&mut self, c: CB) {
        self.callback = Box::new(c);
    }

    fn process_events(&mut self) {
        (self.callback)(101);
    }
}

fn main() {
    let k = 3;
    let mut p = Processor::new(move |i| println!("good morning {} {} ", i, k));
    p.process_events();
    let s = "world!".to_string();
    let callback2 = move |i| println!("hello {} {}", s, i);
    p.set_callback(callback2);
    p.process_events();
}
