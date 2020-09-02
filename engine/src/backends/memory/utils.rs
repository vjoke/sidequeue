use std::cmp::{Ord, Ordering};
use std::time::Instant;

pub type BusinessID = String;
pub type QueueID = String;
pub type JobID = String;

#[derive(Eq)]
pub struct JobMeta(Instant, JobID);

impl JobMeta {
    pub fn is_due(&self) -> bool {
        self.0 < Instant::now()
    }
}

impl Ord for JobMeta {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

impl PartialOrd for JobMeta {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for JobMeta {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
