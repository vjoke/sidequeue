use serde_derive::{Serialize, Deserialize};
use super::utils::{handle_rejection, unwrap_or_500, with_backend};
use warp::{self, filters::BoxedFilter, reply::Reply, Filter, http::StatusCode};
use std::convert::Infallible;

#[derive(Debug, Deserialize)]
pub struct PublishOptions {
    pub delay: u32,
    pub ttl: u32,
    pub tries: u16
}

/// PUT /api/:namespace/:queue
pub fn publish(backend: sq_engine::Backend) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path!("api")
        .and(warp::put())
        .and(with_backend(backend))
        .and(warp::query::<PublishOptions>())
        .and_then(publish_job)
}

async fn publish_job(backend: sq_engine::Backend, opts: PublishOptions) -> Result<impl warp::Reply, Infallible> {
    let engine = backend.engine.lock().await;
    let _r = engine.publish("myns".into(), "myqueue".into(), Vec::new(), 3, 3, 1);

    Ok(StatusCode::CREATED)
}

#[derive(Debug, Deserialize)]
pub struct ConsumeOptions {
    pub ttl: u32,
    pub timeout: u32,
    pub count: u32,
}

// GET /:namespace/:queue[,:queue]*
pub fn consume(backend: sq_engine::Backend) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path!("api")
        .and(warp::get())
        .and(with_backend(backend))
        .and(warp::query::<ConsumeOptions>())
        .and_then(consume_jobs)
}

async fn consume_jobs(backend: sq_engine::Backend, opts: ConsumeOptions) -> Result<impl warp::Reply, Infallible> {
    let engine = backend.engine.lock().await;
    let _r = engine.consume("myns".into(), vec!["queue1".into()], 3, 3, 1);
    Ok(StatusCode::OK) 
} 