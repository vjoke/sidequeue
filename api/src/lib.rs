mod handlers;

use crate::handlers::get_routes;
use sq_engine::{Backend, EngineType};
use sq_logger::prelude::*;
use std::net::SocketAddr;
use tokio::runtime::{Builder, Runtime};
use warp::Filter;
use std::future::Future;

pub fn start_api_service(address: SocketAddr, shutdown: impl Future) -> Runtime {
    let runtime = Builder::new()
        .thread_name("api-")
        .threaded_scheduler()
        .enable_all()
        .build()
        .expect("[api] failed to create runtime");

    let backend = Backend::new(EngineType::MemoryEngine, shutdown);
    let routes = get_routes(backend);
    let server = runtime.enter(move || warp::serve(routes).bind(address));
    runtime.handle().spawn(server);
    runtime
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
