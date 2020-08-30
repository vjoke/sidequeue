mod handlers;
mod error;

use crate::handlers::get_routes;
use sq_engine::{Backend, EngineType};
use sq_logger::prelude::*;
use sq_shutdown::{Stub, Context};
use std::net::SocketAddr;
use std::thread;
use tokio::runtime::{Builder, Runtime};

pub use self::error::APIError;

pub fn start_api_service(address: SocketAddr, context: &Context) -> Runtime {
    let runtime = Builder::new()
        .thread_name("api-")
        .threaded_scheduler()
        .enable_all()
        .build()
        .expect("[api] failed to create runtime");

    let Stub {
        mut shutdown,
        _shutdown_complete,
        ..
    } = Stub::new(context);

    let backend = Backend::new(EngineType::MemoryEngine, Stub::new(context));
    let routes = get_routes(backend.clone());

    let (_addr, server) = runtime.enter(move || {
        warp::serve(routes)
        .bind_with_graceful_shutdown(address, async move {
            shutdown.recv().await;
            // let _ = shutdown_complete.clone();
            info!("api service shutdown gracefully");
        })
    });
    runtime.handle().spawn(server);
    runtime.handle().spawn(async move {backend.run().await});
    runtime
}

