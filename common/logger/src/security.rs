//!
//! The security module gathers security-related logs:
//! logs to detect malicious behavior from other modules.
//!
//! ```
//! use libra_logger::prelude::*;
//!
//! send_struct_log!(
//!   security_log(security_events::INVALID_JOB)
//!     .data("some_data", "the data")
//! );
//! ```
//!

use crate::StructuredLogEntry;

/// helper function to create a security log
pub fn security_log(name: &'static str) -> StructuredLogEntry {
    StructuredLogEntry::new_named("security", &name)
        // set the level to critical
        .critical()
    // set the error description
}

/// Security events that are possible
pub mod security_events {
    // JobPool
    // -------

    /// JobPool received a job with invalid job ID
    pub const INVALID_JOB_ID: &str = "InvalidJobID";
}
