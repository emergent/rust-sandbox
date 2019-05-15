fn main() {
    let a = (0..10_000_000).collect::<Vec<i64>>();
    // 本当はaはcollectせずにそのままmap/filter/foldしたい
    let res = a
        .iter()
        .map(|a| a * 2)
        .filter(|&a| a % 3 == 0)
        .fold(0, |acc, x| acc + x);
    println!("{}", res);
}
