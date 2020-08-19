use serde_derive::{Deserialize, Serialize};
use std::convert::Infallible;
use std::error::Error;
use warp::http::StatusCode;
use warp::{reject, Filter, Rejection, Reply};

#[derive(Debug)]
pub struct InvalidParameter;

impl reject::Reject for InvalidParameter {}

/// An API error serializable to JSON.
#[derive(Serialize)]
pub struct ErrorMessage {
    pub code: u16,
    pub message: String,
}
