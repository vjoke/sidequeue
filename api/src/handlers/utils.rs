use crate::handlers::errors;
use anyhow::Result;
use sq_logger::prelude::*;
use std::{convert::Infallible, future::Future};
use warp::{reply::Response, Rejection, Reply, Filter, http::StatusCode};

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
    warn!("request: {:?}", err);
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(errors::InvalidParameter) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "INVALID_PARAMETER";
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        // We should have expected this... Just log and say its a 500
        error!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&errors::ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}