use api_service::start_api_service;
use sidequeue::options::*;
use sq_logger::{info, Logger};
use std::env;
use structopt::StructOpt;
use tokio::runtime::Runtime;
use tokio::signal;
use sq_shutdown::Context;

pub struct SideQueueHandle {
    api: Runtime,
}

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    Logger::new().is_async(true).init();

    info!("hello, sidequeue with info");

    let options = SideQueueOptions::from_args();
    info!("options: {:#?}", options);

    let shutdown = signal::ctrl_c();
    let context = Context::new();
    let _api = setup(&options.api, &context);

    let mut rt = Runtime::new().unwrap();
    rt.block_on(async move {
        info!("waiting for shutdown");
        let _ = shutdown.await;
        info!("request to shutdown now ...");
        context.terminate().await;
        info!("shutdown completely");
    });
    
    info!("bye, sidequeue");
}

fn setup(api: &APIServiceOptions, context: &Context) -> Runtime {
    start_api_service(api.address, context)
}
