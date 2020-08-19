use std::io;
// use dyn_clonable::*;
/// The engine defines the interface for all the underlying backend
// #[dyn_clonable::clonable]
// pub trait Engine: Clone + Sync + Send {
// pub trait Engine: Sync + Send {
pub trait Engine: Send {
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
