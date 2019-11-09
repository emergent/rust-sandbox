// [dependencies]
// tokio = "0.2.0-alpha.6"
// tokio-timer = "0.3.0-alpha.5"

use std::time::Duration;

#[tokio::main]
async fn main() {
    for i in 0..10 {
        println!("{}", i);
        tokio_timer::delay_for(Duration::from_secs(1)).await;
    }
}
