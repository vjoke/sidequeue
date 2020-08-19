use std::io;
// use dyn_clonable::*;
/// The engine defines the interface for all the underlying backend
// #[dyn_clonable::clonable]
// pub trait Engine: Clone + Sync + Send {
// pub trait Engine: Sync + Send {
pub trait Engine: Send {
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
    ) -> Result<String, io::Error>;

    /// Consume jobs from queues
    fn consume(
        &self,
        namespace: String,
        queues: Vec<String>,
        ttl: u32,
        timeout: u32,
        count: u32,
    ) -> Result<Vec<Job>, io::Error>;
}

/// Job holds details for the work to be done
pub struct Job {
    /// the id of the job
    id: String,
}
