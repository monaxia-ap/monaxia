use crate::{retry::Retry, tracer::Tracer};

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

/// Processes specific type jobs.
#[async_trait]
pub trait Processor {
    /// Type for job payload.
    type Job: Serialize + DeserializeOwned + Send + Sync + 'static;

    /// Type for job tag. Used in statistics.
    type Tag: Serialize + DeserializeOwned + Send + Sync + 'static;

    /// Type for job status. Used in statistics.
    type Status: Serialize + DeserializeOwned + Send + Sync + 'static;

    /// Processes a job.
    fn process(&self, job: Self::Job, current_try: &Retry) -> Self::Status;

    /// Creates tag for a job.
    fn tag(&self, job: &Self::Job) -> Self::Tag;
}

#[derive(Debug)]
pub struct Consumer<P, Q, T> {
    processor: P,
    queue: Q,
    tracer: T,
}

/*
impl<P, Q, T> Consumer<P, Q, T>
where
    P: Processor,
    Q: Queue,
    T: Tracer,
{
}
*/
