mod receive;
mod send;

pub use self::{receive::ReceiverQueue, send::SenderQueue};

const AMQP_PERSISTENT_DELIVERY_MODE: u8 = 2;
const AMQP_X_DELAY: &str = "x-delay";
const AMQP_X_DELAYED_TYPE: &str = "x-delayed-type";
const AMQP_X_DELAYED_MESSAGE: &str = "x-delayed-message";
const DEFAULT_EXCHANGE_NAME: &str = "";
const DELAYED_EXCHANGE_NAME: &str = "monaxia-delayed-exchange";
