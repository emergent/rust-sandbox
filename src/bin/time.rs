use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // using `time`
    let now_unix = time::now().to_timespec().sec;
    let now_unix_ns = time::now().to_timespec().nsec;

    println!("time::now_sec: {}", now_unix);
    println!("time::now_nsec: {}", now_unix_ns);

    let now_unix_str = now_unix as f64 + now_unix_ns as f64 / 1000_000_000.0;
    println!("time::now: {}", now_unix_str);

    // using `chrono`
    let now = SystemTime::now();
    let chrono_unixtime = now.duration_since(UNIX_EPOCH).expect("bttf");
    println!(
        "chrono_unixtime: {}.{:09}",
        chrono_unixtime.as_secs(),
        chrono_unixtime.subsec_nanos()
    );

    println!(
        "chrono_unixtime: {}",
        chrono_unixtime.as_secs() as f64
            + ((chrono_unixtime.subsec_nanos() / 1000_000) as f64) / 1000.0
    );
}
