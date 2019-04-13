fn even(x: i32) -> Result<i32, String> {
    match x % 2 {
        0 => Ok(x),
        _ => Err(format!("{} is not even number.", x)),
    }
}

fn main() {
    println!("{}", even(2).unwrap());
    println!("{}", even(1).unwrap_err());

    let res = (1..10)
        .map(|i| {
            match even(i)
                .map(|j| j * 2)
                .map_err(|s| s.len() as i32) {
                Ok(k) => k,
                Err(e) => e,
            }
        })
        .collect::<Vec<_>>();
    println!("{:?}", res);

    let res2 = (1..10)
        .map(|i| {
            match even(i).and_then(|j| even(j + 1)) {
                Ok(k) => k.to_string(),
                Err(e) => e,
            }
        })
        .collect::<Vec<String>>();
    println!("{:?}", res2);
}
