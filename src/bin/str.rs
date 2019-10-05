fn main() {
    let ss = vec!["aaa#bbb", "aaa", "", "aaa!bbb"];

    for s in ss {
        let mut sp = s.split('#');
        print!("1: ");
        match sp.next() {
            Some(t) => println!("{}", t),
            _ => println!("None"),
        }
        print!("2: ");
        match sp.next() {
            Some(t) => println!("{}", t),
            _ => println!("None"),
        }
    }
}
