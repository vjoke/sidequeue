use crate::engine::{Engine, Job, DEFAULT_QUEUE_SIZE};
use crate::utils::{JobID, JobMeta, NamespaceID, QueueID};
use async_trait::async_trait;
use crossbeam_queue::{ArrayQueue, PushError};
use hashbrown::hash_map::DefaultHashBuilder;
use priority_queue::PriorityQueue;
use sq_logger::prelude::*;
use std::cell::RefCell;
use std::collections::{BinaryHeap, HashMap};
use std::io;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

type ReadyJobs = HashMap<NamespaceID, HashMap<QueueID, ArrayQueue<JobMeta>>>;

/// MemoryEngine implements an engine with an in-memory db
pub struct MemoryEngine {
    /// the jobs to be scheduled
    pub pq: BinaryHeap<JobMeta>,
    /// the job fifo queues separated with namespace
    pub ready_jobs: RefCell<ReadyJobs>,
    /// the job map that holds the actual job data
    pub all_jobs: HashMap<JobID, Job>,
}

impl MemoryEngine {
    pub fn new() -> Self {
        MemoryEngine {
            pq: BinaryHeap::new(),
            // ready_jobs: HashMap::new(),
            ready_jobs: RefCell::new(HashMap::new()),
            // ready_jobs: Arc::new(RwLock::new(RefCell::new(HashMap::new()))),
            all_jobs: HashMap::new(),
        }
    }
}

/// Helper functions go here
impl MemoryEngine {
    fn check_namespace(&self, namespace: NamespaceID) -> bool {
        self.ready_jobs.borrow().get(&namespace).is_some()
    }

    fn push_job_to_ready_queue(&self, job: &Job) -> Result<(), PushError<JobMeta>> {
        // FIXME: is this right for extracting ref values?
        let Job {
            ref namespace,
            ref queue,
            ..
        } = job;

        let mut queue_map = self.ready_jobs.borrow_mut();

        let queue_map = queue_map
            .entry(namespace.clone())
            .or_insert_with(|| HashMap::new());

        let queue = queue_map
            .entry(queue.clone())
            .or_insert_with(|| ArrayQueue::new(DEFAULT_QUEUE_SIZE));

        queue.push(job.get_meta())
    }
}

// #[async_trait]
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
        Ok(Job::default())
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
    fn run(&mut self) -> Result<(), io::Error> {
        // Move job to ready queue if reaches its due time
        loop {
            if let Some(jm) = self.pq.peek() {
                if jm.is_due() {
                    // Move the dued job to ready queue
                    let job = self.all_jobs.get(&jm.id).unwrap();

                    match self.push_job_to_ready_queue(job) {
                        Ok(_) => {
                            // NOTICE: this jm will be seen in two queues before the following line
                            let jm = self.pq.pop().unwrap();
                            info!("push job {} from priority queue to ready queue ok", jm);
                        }
                        Err(err) => {
                            warn!("failed to push job {} from priority queue to ready queue with err {}", jm, err);
                            break;
                        }
                    }
                } else {
                    info!("no dued job available, relax");
                }
            } else {
                info!("no job available, relax");
                break;
            }
        }

        Ok(())
    }
}
