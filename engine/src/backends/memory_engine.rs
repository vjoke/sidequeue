use crate::engine::Engine;
use std::io;
use std::sync::Arc;
use tokio::sync::Mutex;

/// MemoryEngine implements an engine with an in-memory db
pub struct MemoryEngine {
    pub db: Arc<Mutex<Vec<Job>>>,
}

impl MemoryEngine {
    pub fn new() -> Self {
        MemoryEngine {
            db: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Engine for MemoryEngine {
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
}

/// Job holds details for the work to be done
pub struct Job {
    /// the id of the job
    id: String,
}
