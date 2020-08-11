use std::net::SocketAddr;
use tokio::runtime::{Builder, Runtime};
use warp::Filter;
use sq_logger::prelude::*;

pub fn start_api_service(address: SocketAddr) -> Runtime {
    let runtime = Builder::new()
        .thread_name("api-")
        .threaded_scheduler()
        .enable_all()
        .build()
        .expect("[api] failed to create runtime");

    // TODO: get true routers
    let routes = warp::path!("hello" / String).map(|name| {
        info!("got hello from: {}", name);
        format!("Hello, {}!", name)
    });
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
