//! Errors which may arise from this crate.

use thiserror::Error;

/// An enum of errors this crate may produce.
#[derive(Debug, Error)]
pub enum Error {
    /// The given message is too large to be sent to Segment's API.
    #[error("message too large")]
    MessageTooLarge,
}
