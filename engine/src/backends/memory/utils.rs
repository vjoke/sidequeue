use std::cmp::{Ord, Ordering};
use std::fmt;
use std::time::Instant;

pub type NamespaceID = String;
pub type QueueID = String;
pub type JobID = String;

#[derive(Eq, Debug, Clone)]
pub struct JobMeta {
    pub instant: Instant,
    pub job_id: String,
}

impl JobMeta {
    pub fn is_due(&self) -> bool {
        self.instant < Instant::now()
    }
}

impl Ord for JobMeta {
    fn cmp(&self, other: &Self) -> Ordering {
        self.instant.cmp(&other.instant).reverse()
    }
}

impl PartialOrd for JobMeta {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for JobMeta {
    fn eq(&self, other: &Self) -> bool {
        self.instant == other.instant
    }
}

impl fmt::Display for JobMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "due time: {:?}, id: {:?} ", self.instant, self.job_id)
    }
}
