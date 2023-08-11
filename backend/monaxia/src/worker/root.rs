use super::{JobState, WorkerState};

use anyhow::Result;
use monaxia_job::job::Job;
use serde_variant::to_variant_name;
use tracing::{debug, error, info, instrument};

#[instrument(skip(state), fields(worker = state.name))]
pub async fn worker(state: WorkerState) -> Result<()> {
    // TODO: just loop
    while let Some((job, delivery)) = state.consumer.fetch().await? {
        let job_state = state.job_state.clone();
        let job_payload = job.job().clone();
        let job_tag = job.tag().to_string();

        match do_job(job_state, job_payload, job_tag).await {
            Ok(()) => {
                state.consumer.mark_success(delivery).await?;
            }
            Err(e) => {
                error!("job error: {e}");
                state.consumer.mark_failure(delivery).await?;
                if let Some((data, delay)) = job.next() {
                    state.consumer.enqueue(data, Some(delay)).await?;
                }
            }
        }
    }

    Ok(())
}

#[instrument(
    skip(state, _tag),
    fields(job = to_variant_name(&payload).expect("invalid job"))
)]
async fn do_job(state: JobState, payload: Job, _tag: String) -> Result<()> {
    match payload {
        Job::Hello => {
            info!("hello monaxia!");
        }
        Job::ActivityPreprocess(json_text, validation) => {
            debug!(
                "validating signature with headers: {:?}",
                validation.signature_header.headers
            );
        }
    }

    Ok(())
}
