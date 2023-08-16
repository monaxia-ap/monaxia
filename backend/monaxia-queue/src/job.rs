use crate::{
    error::Result,
    queue::{BoxedTag, ReceiveQueue, SendQueue},
};

use std::{fmt::Debug, sync::Arc, time::Duration};

use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Clone)]
pub struct Producer<T> {
    pub(crate) sender: Arc<dyn SendQueue<T>>,
}

impl<T> Producer<T>
where
    T: Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    pub async fn enqueue(&self, data: T, delay: Option<Duration>) -> Result<()> {
        self.sender.enqueue(data, delay).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Consumer<T> {
    pub(crate) receiver: Box<dyn ReceiveQueue<T>>,
    pub(crate) shared_sender: Arc<dyn SendQueue<T>>,
}

impl<T> Consumer<T>
where
    T: Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    pub async fn get_sender(&self) -> Arc<dyn SendQueue<T>> {
        self.shared_sender.clone()
    }

    pub async fn enqueue(&self, data: T, delay: Option<Duration>) -> Result<()> {
        self.shared_sender.enqueue(data, delay).await?;
        Ok(())
    }

    pub async fn fetch(&self) -> Result<Option<(T, BoxedTag)>> {
        let data = self.receiver.dequeue().await?;
        Ok(data)
    }

    pub async fn mark_success(&self, tag: BoxedTag) -> Result<()> {
        tag.resolve().await?;
        Ok(())
    }

    pub async fn mark_failure(&self, tag: BoxedTag) -> Result<()> {
        tag.reject().await?;
        Ok(())
    }
}
