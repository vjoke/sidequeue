mod utils;
mod queue;

use sq_logger::prelude::*;
use warp::{filters::BoxedFilter, reply::Reply, Filter};

pub(crate) fn get_routes(backend: sq_engine::Backend) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    let be = backend.clone();
    // TODO:
    // let routes = warp::path!("hello" / String).map(|name| {
    //     info!("got hello from: {}", name);
    //     format!("Hello, {}!", name)
    // });

    let routes = queue::publish(be);
    routes
    // Serve all routes for GET only.
    // warp::any().and(routes).boxed()
}
