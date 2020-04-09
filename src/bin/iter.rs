fn main() {
    let s = "aaa aaa aaaa";
    let mut siter = s.splitn(2, ' ');
    println!("{:?}", siter.next());
    println!("{:?}", siter.next());
    println!("{:?}", siter.next());
    println!("{:?}", siter.next());
    println!("{:?}", siter.next());
}
