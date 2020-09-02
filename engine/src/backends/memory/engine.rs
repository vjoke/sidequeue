use super::utils::{BusinessID, QueueID, JobID, JobMeta};
use crate::engine::{Engine, Job};
use async_trait::async_trait;
use crossbeam_queue::{ArrayQueue, PushError};
use hashbrown::hash_map::DefaultHashBuilder;
use priority_queue::PriorityQueue;
use std::collections::{BinaryHeap, HashMap};
use std::io;
use std::time::Instant;

/// MemoryEngine implements an engine with an in-memory db
pub struct MemoryEngine {
    /// the jobs to be scheduled
    pub pq: BinaryHeap<JobMeta>,
    /// the job fifo queues separated with namespace
    pub ready_jobs: HashMap<BusinessID, HashMap<QueueID, ArrayQueue<JobID>>>,
    /// the job map that holds the actual job data
    pub all_jobs: HashMap<String, Job>,
}

impl MemoryEngine {
    pub fn new() -> Self {
        MemoryEngine {
            pq: BinaryHeap::new(),
            ready_jobs: HashMap::new(),
            all_jobs: HashMap::new(),
        }
    }
}

#[async_trait]
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
    fn delete(&self, namespace: String, queues: String, job_id: String) -> Result<(), io::Error> {
        Ok(())
    }

    /// Peek a job from queue
    fn peek(
        &self,
        namespace: String,
        queues: String,
        job_id: Option<String>,
    ) -> Result<Job, io::Error> {
        Ok(Job { id: "TODO".into() })
    }

    /// Get size of the queue
    fn size(&self, namespace: String, queues: String) -> Result<u64, io::Error> {
        Ok(0)
    }

    /// Destroy the queue
    fn destroy(&self, namespace: String, queues: String) -> Result<u64, io::Error> {
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
    fn size_of_dead_letter(&self, namespace: String, queues: String) -> Result<u64, io::Error> {
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

    /// Run kicks off the engine and starts to process jobs
    async fn run(& mut self) -> Result<(), io::Error> {
        // Move job to ready queue if reaches its due time
        // FIXME: run a long time task in async function
        loop {
            if let Some(ref jm) = self.pq.peek() {
                if jm.is_due() {
                    let jm = self.pq.pop().unwrap();
                    // TODO: add to ready queue
                } 
            } else {
                break;
            }
        }

        Ok(())
    }
}
