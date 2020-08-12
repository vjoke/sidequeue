use api_service::start_api_service;
use sidequeue::options::*;
use sq_logger::{info, Logger};
use std::env;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::runtime::Runtime;
use structopt::StructOpt;

pub struct SideQueueHandle {
    _api: Runtime,
}

fn main() {

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    Logger::new().is_async(true).init();

    info!("hello, sidequeue with info");

    let options = SideQueueOptions::from_args();
    info!("options: {:#?}", options);
    let _handle = setup(&options.api);

    let term = Arc::new(AtomicBool::new(false));
    while !term.load(Ordering::Acquire) {
        info!("park, sidequeue with info");
        std::thread::park();
    }
    info!("bye, sidequeue with info");
}

fn setup(api: &APIServiceOptions) -> SideQueueHandle {
    let api_service = start_api_service(api.address);
    SideQueueHandle { _api: api_service }
}
