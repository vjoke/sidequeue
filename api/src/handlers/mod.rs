mod utils;
use sq_engine::Engine;
use sq_logger::prelude::*;
use warp::{filters::BoxedFilter, reply::Reply, Filter};

pub(crate) fn get_routes(engine: &dyn Engine) -> BoxedFilter<(impl Reply,)> {
    let _eg = engine.clone();
    // TODO:
    let routes = warp::path!("hello" / String).map(|name| {
        info!("got hello from: {}", name);
        format!("Hello, {}!", name)
    });
    // Serve all routes for GET only.
    warp::get().and(routes).boxed()
}
