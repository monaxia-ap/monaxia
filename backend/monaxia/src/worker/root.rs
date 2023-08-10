use anyhow::Result;
use monaxia_job::job::{Job, MxJob};
use monaxia_queue::job::Consumer;
use tracing::{error, info};

pub async fn worker(consumer: Consumer<MxJob>) -> Result<()> {
    // TODO: just loop
    while let Some((job, tag)) = consumer.fetch().await? {
        match do_job(job.job().clone(), job.tag().to_string()).await {
            Ok(()) => {
                consumer.mark_success(tag).await?;
            }
            Err(e) => {
                error!("job error: {e}");
                consumer.mark_failure(tag).await?;
                if let Some((data, delay)) = job.next() {
                    consumer.enqueue(data, Some(delay)).await?;
                }
            }
        }
    }

    Ok(())
}

async fn do_job(job: Job, _tag: String) -> Result<()> {
    match job {
        Job::Hello => {
            info!("hello monaxia!");
        }
    }

    Ok(())
}
