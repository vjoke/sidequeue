use crate::engine::{Engine, Job};
use std::io;

/// RedisEngine implements an engine with redis
// #[derive(Clone)]
pub struct RedisEngine {}

impl RedisEngine {
    pub fn new(url: String) -> Self {
        RedisEngine {}
    }
}

impl Engine for RedisEngine {
    /// Publish a job to the queue
    ///
    /// # Arguments
    ///
    /// * `namespace` - A string that holds the queue for the job
    /// * `body` - A vector of bytes that holds the job body
    /// * `ttl` - A u32 value that holds the time-to-live value
    /// * `delay` - A u32 value that holds the delay value in second
    /// * `tries` - A u16 value that holds th e maximize retry count
    fn publish(
        &self,
        namespace: String,
        queue: String,
        body: Vec<u8>,
        ttl: u32,
        delay: u32,
        tries: u16,
    ) -> Result<String, io::Error> {
        Ok("redis engine".into())
    }

    /// Consume jobs from queues
    ///
    /// # Arguments
    ///
    /// * `namespace` - A string that holds the queue for the job
    /// * `body` - A vector of bytes that holds the job body
    /// * `ttl` - A u32 value that holds the time-to-live value
    /// * `delay` - A u32 value that holds the delay value in second
    /// * `tries` - A u16 value that holds th e maximize retry count
    fn consume(
        &self,
        namespace: String,
        queues: Vec<String>,
        ttl: u32,
        timeout: u32,
        count: u32,
    ) -> Result<Vec<Job>, io::Error> {
        let jobs = Vec::new();
        Ok(jobs)
    }
}
