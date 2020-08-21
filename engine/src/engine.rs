use std::io;
/// The engine defines the interface for all the underlying backend
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
    ) -> Result<Vec<Job>, io::Error>;

    /// Delete a job from queue
    ///
    /// # Arguments
    /// * `namespace` - A string that holds the queue for the job
    /// * `queue` - A string that holds the queue name
    /// * `job_id` - A string that holds the job id to be deleted
    fn delete(&self, namespace: String, queue: String, job_id: String) -> Result<(), io::Error>;

    /// Peek a job from queue
    ///
    /// # Arguments
    /// * `namespace` - A string that holds the queue for the job
    /// * `queue` - A string that holds the queue name
    /// * `job_id` - A optional string that holds the job id to be peeked
    fn peek(
        &self,
        namespace: String,
        queue: String,
        job_id: Option<String>,
    ) -> Result<Job, io::Error>;

    /// Get size of the queue
    ///
    /// # Arguments
    /// * `namespace` - A string that holds the namespace for the job
    /// * `queue` - A string that holds the queue name
    fn size(&self, namespace: String, queue: String) -> Result<u64, io::Error>;

    /// Destroy the queue
    ///
    /// # Arguments
    /// * `namespace` - A string that holds the namespace for the job
    /// * `queue` - A string that holds the queue name
    fn destroy(&self, namespace: String, queue: String) -> Result<u64, io::Error>;

    /// Peek dead letter from the queue
    ///
    /// # Arguments
    /// * `namespace` - A string that holds the namespace for the job
    /// * `queue` - A string that holds the queue name
    fn peek_dead_letter(
        &self,
        namespace: String,
        queue: String,
    ) -> Result<(u64, String), io::Error>;

    /// Delete dead letter from the queue
    ///
    /// # Arguments
    /// * `namespace` - A string that holds the namespace for the job
    /// * `queue` - A string that holds the queue name
    fn delete_dead_letter(
        &self,
        namespace: String,
        queue: String,
        limit: u64,
    ) -> Result<u64, io::Error>;

    /// Respawn dead letter of the queue
    ///
    /// # Arguments
    /// * `namespace` - A string that holds the namespace for the job
    /// * `queue` - A string that holds the queue name
    fn respawn_dead_letter(
        &self,
        namespace: String,
        queue: String,
        limit: u64,
        ttl: u64,
    ) -> Result<u64, io::Error>;

    /// Get the size of dead letters of the queue
    ///
    /// # Arguments
    /// * `namespace` - A string that holds the namespace for the job
    /// * `queue` - A string that holds the queue name
    fn size_of_dead_letter(&self, namespace: String, queue: String) -> Result<u64, io::Error>;

    /// Dump the runtime info
    fn dump_info(&self) -> Result<(), io::Error>;

    /// Shutdown the engine
    fn shutdown(&self) -> Result<(), io::Error>;
}

/// Job holds details for the work to be done
pub struct Job {
    /// the id of the job
    pub id: String,
}
