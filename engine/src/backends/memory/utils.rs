use std::cmp::{Ord, Ordering};
use std::time::Instant;

#[derive(Eq)]
pub struct DueTime(Instant);

impl Ord for DueTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

impl PartialOrd for DueTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DueTime {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
