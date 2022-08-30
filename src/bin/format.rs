const _CONST_FORMAT: &'static str = "hoge:{}:hoge";

fn main() {
    //let a = format!(CONST_FORMAT, "fuga");
    let a = format!("hoho:{}:hoho", "fuga");
    println!("{}", a);
}
