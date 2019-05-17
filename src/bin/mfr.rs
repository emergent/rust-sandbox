use std::time::Instant;

fn main() {
    let start_v = Instant::now();
    let a = (0..10_000_000).collect::<Vec<i64>>();
    // 本当はaはcollectせずにそのままmap/filter/foldしたい
    let end_v = start_v.elapsed();

    let start_it = Instant::now();
    //let res = (0..10_000_000)
    let res = a
        .iter()
        .map(|a| a * 2)
        .filter(|&a| a % 3 == 0)
        .fold(0, |acc, x| acc + x);
    let end_it = start_it.elapsed();
    println!("{}", res);
    println!("{}ms, {}ms", end_v.subsec_millis(), end_it.subsec_millis());
}
