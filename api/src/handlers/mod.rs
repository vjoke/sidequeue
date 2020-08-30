mod errors;
mod queue;
mod utils;

use crate::handlers::utils::handle_rejection;
use sq_logger::prelude::*;
use warp::{filters::BoxedFilter, reply::Reply, Filter};
use utils::{LATENCY_HISTOGRAM};

pub(crate) fn get_routes(backend: sq_engine::Backend) -> BoxedFilter<(impl Reply,)> {
    let publish = queue::publish(backend.clone());
    let consume = queue::consume(backend.clone());
    let delete = queue::delete(backend.clone());
    let peek = queue::peek(backend.clone());
    let peek_job = queue::peek_job(backend.clone());

    warp::any()
        .and(publish)
        .or(consume)
        .or(delete)
        .or(peek)
        .or(peek_job)
        .recover(handle_rejection)
        .with(warp::log::custom(|info| {
            let endpoint = info.path().split('/').nth(1).unwrap_or("-");
            info!("got info {} {}", info.method(), info.path());
            LATENCY_HISTOGRAM
                .with_label_values(&[endpoint, info.status().as_str()])
                .observe(info.elapsed().as_secs_f64())
        }))
        .boxed()
}
