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

    /// Consume jobs
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

    /// Delete a job from queue
    fn delete(
        &self,
        namespace: String,
        queues: String,
        job_id: String,
    ) -> Result<(), io::Error> {
        Ok(())
    }

    /// Peek a job from queue
    fn peek(
        &self,
        namespace: String,
        queues: String,
        job_id: Option<String>,
    ) -> Result<Job, io::Error> {
        Ok(Job{
            id: "TODO".into()
        })
    }

    /// Get size of the queue
    fn size(
        &self,
        namespace: String,
        queues: String,
    ) -> Result<u64, io::Error> {
        Ok(0)
    }

    /// Destroy the queue
    fn destroy(
        &self,
        namespace: String,
        queues: String,
    ) -> Result<u64, io::Error> {
        Ok(0)
    }

    /// Peek dead letter from the queue
    fn peek_dead_letter(
        &self,
        namespace: String,
        queues: String,
    ) -> Result<(u64, String), io::Error> {
        Ok((0, "none".into()))
    }

    /// Delete dead letter of the queue
    fn delete_dead_letter(
        &self,
        namespace: String,
        queues: String,
        limit: u64,
    ) -> Result<u64, io::Error> {
        Ok(0)
    }

    /// respawn dead letter of the queue
    fn respawn_dead_letter(
        &self,
        namespace: String,
        queues: String,
        limit: u64,
        ttl: u64,
    ) -> Result<u64, io::Error> {
        Ok(0)
    }

    /// Get size of dead letters of the queue
    fn size_of_dead_letter(
        &self,
        namespace: String,
        queues: String,
    ) -> Result<u64, io::Error> {
        Ok(0)
    }

    /// Dump the runtime info
    fn dump_info(&self) -> Result<(), io::Error> {
        Ok(())
    }

    /// Shutdown the engine
    fn shutdown(&self) -> Result<(), io::Error> {
        Ok(())
    } 
}
