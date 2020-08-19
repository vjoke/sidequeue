use crate::engine::{Engine, Job};
use std::io;
// use std::sync::Arc;
// use tokio::sync::Mutex;

/// MemoryEngine implements an engine with an in-memory db
// #[derive(Clone)]
pub struct MemoryEngine {
    /// the jobs to be scheduled
    pub jobs: Vec<Job>,
}

impl MemoryEngine {
    pub fn new() -> Self {
        MemoryEngine {
            // db: Arc::new(Mutex::new(Vec::new())),
            jobs: Vec::new(),
        }
    }
}

impl Engine for MemoryEngine {
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
        Ok("some".into())
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
