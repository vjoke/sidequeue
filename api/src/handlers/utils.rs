use anyhow::Result;
use sq_logger::prelude::*;
use std::{convert::Infallible, future::Future};
use warp::{reply::Response, Rejection, Reply, Filter};

/// Inject engine
pub(super) fn with_backend(backend: sq_engine::Backend) -> impl Filter<Extract = (sq_engine::Backend, ), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || backend.clone())
}

/// Return 500 on any error raised by the request handler.
pub(super) fn unwrap_or_500(result: Result<Box<dyn Reply>>) -> Box<dyn Reply> {
    match result {
        Ok(resp) => resp,
        Err(e) => {
            warn!("Request handler exception: {:#}", e);
            Box::new(warp::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Return 400 on any rejections (parameter parsing errors).
pub(super) async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    warn!("bad request: {:?}", err);
    Ok(warp::http::StatusCode::BAD_REQUEST)
}