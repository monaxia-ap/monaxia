use anyhow::Result;
use monaxia_job::job::MxJob;
use monaxia_queue::job::Consumer;

pub async fn worker(consumer: Consumer<MxJob>) -> Result<()> {
    // TODO: just loop
    while let Some((job, tag)) = consumer.fetch().await? {
        consumer.mark_success(tag).await?;
    }

    Ok(())
}
