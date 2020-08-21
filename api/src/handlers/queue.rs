use serde_derive::{Serialize, Deserialize};
use super::utils::{handle_rejection, unwrap_or_500, with_backend};
use sq_logger::prelude::*;
use warp::{self, filters::BoxedFilter, reply::Reply, Filter, http::StatusCode};
use std::convert::Infallible;

#[derive(Debug, Deserialize)]
pub struct PublishOptions {
    pub delay: u32,
    pub ttl: u32,
    pub tries: u16
}

#[derive(Debug, Deserialize)]
pub struct ConsumeOptions {
    pub ttl: u32,
    pub timeout: u32,
    pub count: u32,
}

/// PUT /:namespace/:queue
pub fn publish(backend: sq_engine::Backend) -> impl Filter<Extract = (impl Reply,),  Error = warp::Rejection> + Clone {
    warp::path!(String / String)
        .and(warp::put())
        .and(with_backend(backend))
        .and(warp::query::<PublishOptions>())
        .and_then(do_publish_job)
}

/// PUT /:namespace/:queue/job/:job_id
pub fn publish_delete(backend: sq_engine::Backend) -> impl Filter<Extract = (impl Reply,),  Error = warp::Rejection> + Clone {
    warp::path!(String / String/ "job" / String)
        .and(warp::put())
        .and(with_backend(backend))
        .and_then(do_delete_job)
}

/// GET /:namespace/:queue[,:queue]*
pub fn consume(backend: sq_engine::Backend) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / String / String)
        .and(warp::get())
        .and(with_backend(backend))
        .and(warp::query::<ConsumeOptions>())
        .and_then(do_consume_jobs)
}

/// DELETE /api/:namespace/:queue/job/:job_id
pub fn delete(backend: sq_engine::Backend) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / String / String / "job" / String)
        .and(warp::delete())
        .and(with_backend(backend))
        .and_then(do_delete_job)
}

/// GET /api/:namespace/:queue/peek
pub fn peek(backend: sq_engine::Backend) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / String / String / "peek")
        .and(warp::get())
        .and(with_backend(backend))
        .and_then(|namespace, queue, backend| do_peek_job(namespace, queue, None, backend))
}

/// GET /api/:namespace/:queue/job/:job_id
pub fn peek_job(backend: sq_engine::Backend) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / String / String / "job" / String)
        .and(warp::get())
        .and(with_backend(backend))
        .and_then(|job_id, namespace, queue, backend| do_peek_job(namespace, queue, Some(job_id), backend))
}

/// Helper functions go here
/// publish a job
async fn do_publish_job(namespace: String, queue: String, backend: sq_engine::Backend, opts: PublishOptions) -> Result<impl warp::Reply, Infallible> {
    info!("publish_job called with {:#}, {:#}, {:#?}", namespace, queue, opts);
    let engine = backend.engine.lock().await;
    let _r = engine.publish(namespace, queue, Vec::new(), 3, 3, 1);

    Ok(StatusCode::CREATED)
}

/// Delete a job
async fn do_delete_job(namespace: String, queue: String, job_id: String, backend: sq_engine::Backend) -> Result<impl warp::Reply, Infallible> {
    info!("publish_delete_job called with {:#}, {:#} ", namespace, queue );
    let engine = backend.engine.lock().await;
    let _r = engine.delete(namespace, queue, job_id);

    Ok(StatusCode::OK) 
}

async fn do_consume_jobs(namespace: String, queue: String, backend: sq_engine::Backend, opts: ConsumeOptions) -> Result<impl warp::Reply, Infallible> {
    let engine = backend.engine.lock().await;
    info!("consume_jobs called with {:#}, {:#}, {:#?}", namespace, queue, opts);
    let _r = engine.consume(namespace, vec![queue], 3, 3, 1);

    Ok(StatusCode::OK) 
}

async fn do_peek_job(namespace: String, queue: String, job_id: Option<String>, backend: sq_engine::Backend) -> Result<impl warp::Reply, Infallible> {
    info!("peek_job called with {:#}, {:#}, {:#?}", namespace, queue, job_id);
    let engine = backend.engine.lock().await;
    let _r = engine.peek(namespace, queue, job_id);

    Ok(StatusCode::OK)
}