mod memory_engine;
mod redis_engine;

use crate::engine::Engine;
use memory_engine::MemoryEngine;
use redis_engine::RedisEngine;
use std::sync::Arc;
use tokio::sync::Mutex;

/// EngineType defines the supported engine type
pub enum EngineType {
    MemoryEngine,
    RedisEngine(String),
}

/// Backend is for concrete operations
#[derive(Clone)]
pub struct Backend {
    pub engine: Arc<Mutex<Box<dyn Engine>>>,
}

impl Backend {
    pub fn new(typ: EngineType) -> Self {
        Backend {
            engine: Arc::new(Mutex::new(new_engine(typ))),
        }
    }
}

/// new_engine creates a new engine acording to the type
fn new_engine(typ: EngineType) -> Box<dyn Engine> {
    match typ {
        EngineType::MemoryEngine => Box::new(MemoryEngine::new()),
        EngineType::RedisEngine(url) => Box::new(RedisEngine::new(url)),
    }
}
