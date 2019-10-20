use log::{debug, error};
use syslog::{Error, Facility};

fn main() -> Result<(), Error> {
    syslog::init(Facility::LOG_USER, log::LevelFilter::Debug, Some("cats"))?;
    debug!("this is a debug {}", "message");
    error!("this is an error!");
    Ok(())
}
