use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
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
