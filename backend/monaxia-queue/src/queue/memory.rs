use super::{BoxedTag, ProcessTag, ReceiveQueue, SendQueue};
use crate::{
    error::{Error, Result},
    job::{Consumer, Producer},
};

use std::{fmt::Debug, sync::Arc, time::Duration};

use async_trait::async_trait;
use futures::{
    channel::mpsc::{Receiver, Sender},
    lock::Mutex,
    SinkExt,
};

#[derive(Debug, Clone)]
pub struct SenderQueue<T>(Arc<Mutex<Sender<T>>>);

#[async_trait]
impl<T> SendQueue<T> for SenderQueue<T>
where
    T: Debug + Send + Sync + 'static,
{
    async fn enqueue(&self, data: T, _delay: Option<Duration>) -> Result<()> {
        let mut locked = self.0.lock().await;
        locked
            .send(data)
            .await
            .map_err(|e| Error::Queue(e.into()))?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct ReceiverQueue<T>(Mutex<Receiver<T>>);

#[async_trait]
impl<T> ReceiveQueue<T> for ReceiverQueue<T>
where
    T: Debug + Send + Sync + 'static,
{
    async fn dequeue(&self) -> Result<Option<(T, BoxedTag)>> {
        let mut locked = self.0.lock().await;
        let next = locked.try_next().map_err(|e| Error::Queue(e.into()))?;
        Ok(next.map(|d| (d, Box::new(EmptyTag) as Box<dyn ProcessTag>)))
    }
}

#[derive(Debug)]
pub struct EmptyTag;

#[async_trait]
impl ProcessTag for EmptyTag {
    async fn resolve(self) -> Result<()> {
        Ok(())
    }

    async fn reject(self) -> Result<()> {
        Ok(())
    }
}

pub fn create_memory_producer<T>(sender: Sender<T>) -> Producer<T>
where
    T: Debug + Send + Sync + 'static,
{
    Producer {
        sender: Arc::new(SenderQueue(Arc::new(Mutex::new(sender)))),
    }
}

pub fn create_memory_consumer<T>(sender: Sender<T>, receiver: Receiver<T>) -> Consumer<T>
where
    T: Debug + Send + Sync + 'static,
{
    Consumer {
        shared_sender: Arc::new(SenderQueue(Arc::new(Mutex::new(sender)))),
        receiver: Box::new(ReceiverQueue(Mutex::new(receiver))),
    }
}
