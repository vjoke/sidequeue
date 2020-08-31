mod memory;
mod redis;

use crate::engine::Engine;
use memory::engine::MemoryEngine;
use redis::engine::RedisEngine;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, interval, Interval};
use tokio::runtime::Runtime;
use std::thread;

use sq_logger::info;
use sq_shutdown::Stub;

/// EngineType defines the supported engine type
pub enum EngineType {
    MemoryEngine,
    RedisEngine(String),
}

/// Backend is for concrete operations
#[derive(Clone)]
pub struct Backend {
    pub engine: Arc<Mutex<Box<dyn Engine>>>,
    stub: Arc<Mutex<Stub>>,
}

impl Backend {
    pub fn new(typ: EngineType, stub: Stub) -> Self {
        Backend {
            engine: Arc::new(Mutex::new(new_engine(typ))),
            stub: Arc::new(Mutex::new(stub)),
        }
    }

    pub async fn run(&self) {
        let mut stub = self.stub.lock().await;
        let mut ticker = interval(Duration::new(1, 0));
        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    info!("start processing jobs ...");
                    let engine = self.engine.lock().await;
                    let _ = engine.run().await;
                    info!("stop processing jobs ...");
                }
                _ = stub.shutdown.recv() => {
                    info!("exit run");
                    return;
                }
            }
        }
    }
}

impl Drop for Backend {
    fn drop(&mut self) {
        info!("backend dropped");
    }
}

/// new_engine creates a new engine acording to the type
fn new_engine(typ: EngineType) -> Box<dyn Engine> {
    match typ {
        EngineType::MemoryEngine => Box::new(MemoryEngine::new()),
        EngineType::RedisEngine(url) => Box::new(RedisEngine::new(url)),
    }
}
