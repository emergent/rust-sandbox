#[derive(Debug)]
struct A;
#[derive(Debug)]
struct B;

impl From<B> for A {
    fn from(_: B) -> Self {
        Self
    }
}

fn main() {
    let b = B;
    let a: A = A::from(b);
    println!("{:?}", a);

    let b2 = B;
    let a2: A = b2.into();
    println!("{:?}", a2)
}
