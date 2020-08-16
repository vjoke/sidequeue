use std::io;
/// The engine defines the interface for all the underlying backend
pub trait Engine {
    /// Publish a new job
    fn publish(
        &self,
        namespace: String,
        queue: String,
        body: Vec<u8>,
        ttl: u32,
        delay: u32,
        tries: u16,
    ) -> Result<String, io::Error>;
}
