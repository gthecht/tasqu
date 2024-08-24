use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// The message is what is sent to the delayed queue, we'll change this behave like a message sent on AMQP.
/// for now a message is recieved with delay-time, and will send it after this delay

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub topic: String,
    data: Vec<u8>,
    pub insert_time: SystemTime,
    pub delay: Duration,
}

impl Message {
    pub fn new(topic: String, data: Vec<u8>, delay: Duration) -> Message {
        Message {
            topic,
            data,
            insert_time: SystemTime::now(),
            delay,
        }
    }
}
