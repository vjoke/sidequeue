use sq_logger::{info, Logger};
use std::env;

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    Logger::new().is_async(true).init();
    info!("hello, sidequeue with info");
}
