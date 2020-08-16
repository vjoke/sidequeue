mod memory_engine;
mod redis_engine;

use crate::engine::Engine;
use memory_engine::MemoryEngine;
use redis_engine::RedisEngine;

/// EngineType defines the supported engine type
pub enum EngineType {
    MemoryEngine,
    RedisEngine(String),
}

/// FIXME:
/// new_engine creates a new engine acording to the type
pub fn new_engine(typ: EngineType) -> Box<dyn Engine> {
    match typ {
        EngineType::MemoryEngine => Box::new(MemoryEngine::new()),
        EngineType::RedisEngine(url) => Box::new(RedisEngine::new(url)),
    }
}
