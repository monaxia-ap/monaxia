use crate::retry::Retry;

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

/// Job interface.
pub trait Job: Serialize + Deserialize<'static> + Sized + Debug + Send + Sync + 'static {
    fn make_retry(&self) -> Retry;
    fn tag(&self) -> String;
}

/// Defines the status for a job.
#[derive(Debug)]
pub enum JobStatus<S> {
    /// Job succeeded.
    Succeeded(S),

    /// Job failed.
    Failed(S),

    /// Job was aborted. No more retry will occur.
    Aborted(S),
}
