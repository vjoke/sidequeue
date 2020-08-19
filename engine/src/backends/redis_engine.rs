use crate::engine::Engine;
use std::io;

/// RedisEngine implements an engine with redis
#[derive(Clone)]
pub struct RedisEngine {}

impl RedisEngine {
    pub fn new(url: String) -> Self {
        RedisEngine {}
    }
}

impl Engine for RedisEngine {
    fn publish(
        &self,
        namespace: String,
        queue: String,
        body: Vec<u8>,
        ttl: u32,
        delay: u32,
        tries: u16,
    ) -> Result<String, io::Error> {
        Ok("redis engine".into())
    }
}
