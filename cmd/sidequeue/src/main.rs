use api_service::start_api_service;
use sidequeue::options::*;
use sq_logger::{info, Logger};
use sq_metrics::metric_server;
use std::env;
use structopt::StructOpt;
use tokio::runtime::Runtime;
use tokio::signal;
use sq_shutdown::Context;
use std::thread;

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
    let _api = setup_api(&options.api, &context);
    let _metrics_server = setup_metrics(&options.metrics);

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

fn setup_api(options: &APIServiceOptions, context: &Context) -> Runtime {
    start_api_service(options.api_address, context)
}

fn setup_metrics(options: &MetricsServiceOptions) {
    metric_server::start_server(options.metrics_address, false);
}