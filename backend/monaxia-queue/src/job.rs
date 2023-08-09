use crate::queue::{ReceiveQueue, SendQueue};

use std::{fmt::Debug, sync::Arc};

use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Clone)]
pub struct Producer<T> {
    pub(crate) sender: Arc<dyn SendQueue<T>>,
}

impl<T> Producer<T> where T: Debug + Serialize + DeserializeOwned + Send + Sync + 'static {}

#[derive(Debug)]
pub struct Consumer<T> {
    pub(crate) receiver: Box<dyn ReceiveQueue<T>>,
    pub(crate) shared_sender: Arc<dyn SendQueue<T>>,
}

impl<T> Consumer<T> where T: Debug + Serialize + DeserializeOwned + Send + Sync + 'static {}
