mod utils;
mod queue;
mod errors;

use sq_logger::prelude::*;
use crate::handlers::utils::handle_rejection;
use warp::{filters::BoxedFilter, reply::Reply, Filter};

pub(crate) fn get_routes(backend: sq_engine::Backend) -> BoxedFilter<(impl Reply, )> {
    let be = backend.clone();
    // TODO:
    // let routes = warp::path!("hello" / String).map(|name| {
    //     info!("got hello from: {}", name);
    //     format!("Hello, {}!", name)
    // });

    let publish = queue::publish(backend.clone());
    let consume = queue::consume(backend.clone());

    warp::any()
    .and(publish)
    .or(consume)
    .recover(handle_rejection)
    .with(warp::log::custom(|info| {
        let endpoint = info.path().split('/').nth(1).unwrap_or("-");
        info!("got info {} {}", info.method(), info.path());
        // TODO: LATENCY_HISTOGRAM
    }))
    .boxed()
}
