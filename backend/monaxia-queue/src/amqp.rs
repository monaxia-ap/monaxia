mod receive;
mod send;

pub use self::{receive::ReceiverQueue, send::SenderQueue};

use std::result::Result as StdResult;

use bincode::Error as BincodeError;
use lapin::Error as LapinError;
use thiserror::Error as ThisError;

const AMQP_PERSISTENT_DELIVERY_MODE: u8 = 2;
const AMQP_X_DELAY: &str = "x-delay";
const AMQP_X_DELAYED_TYPE: &str = "x-delayed-type";
const AMQP_X_DELAYED_MESSAGE: &str = "x-delayed-message";
const DEFAULT_EXCHANGE_NAME: &str = "";
const DELAYED_EXCHANGE_NAME: &str = "batan-delayed-exchange";

pub type Result<T> = StdResult<T, Error>;

/// Wrapped error type for AMQP queue.
#[derive(Debug, ThisError)]
pub enum Error {
    /// Error from AMQP.
    #[error("AMQP error: {0}")]
    Amqp(#[from] LapinError),

    /// Error from serialization.
    #[error("serialization error: {0}")]
    Serialization(#[from] BincodeError),
}
