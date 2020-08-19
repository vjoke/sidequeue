mod utils;
mod queue;
mod errors;

use sq_logger::prelude::*;
use crate::handlers::utils::handle_rejection;
use warp::{filters::BoxedFilter, reply::Reply, Filter};

pub(crate) fn get_routes(backend: sq_engine::Backend) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    let be = backend.clone();
    // TODO:
    // let routes = warp::path!("hello" / String).map(|name| {
    //     info!("got hello from: {}", name);
    //     format!("Hello, {}!", name)
    // });

    let publish = queue::publish(backend.clone());
    let consume = queue::consume(backend.clone());

    publish.or(consume)
}
