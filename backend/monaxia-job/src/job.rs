use std::time::Duration;

use monaxia_data::ap::RequestValidation;
use monaxia_queue::retry::{Backoff, Retry};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MxJob {
    payload: Job,
    tag: String,
    retry: Retry,
}

impl MxJob {
    pub fn new_single(payload: Job) -> MxJob {
        MxJob {
            payload,
            tag: Default::default(),
            retry: Retry::new(0, Backoff::Constant(Duration::from_secs(1))),
        }
    }

    pub fn job(&self) -> &Job {
        &self.payload
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn next(self) -> Option<(MxJob, Duration)> {
        let Some((delay, retry)) = self.retry.retry() else {
            return None;
        };

        let next_job = MxJob {
            payload: self.payload,
            tag: self.tag,
            retry,
        };
        Some((next_job, delay))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Job {
    /// Server has started.
    Hello,

    /// Preprocesses activity json object.
    /// Validates signature header.
    ActivityPreprocess(String, RequestValidation),
}
