use std::collections::{HashMap, VecDeque};

use crate::message::Message;
pub struct Broker {
    queues: HashMap<String, VecDeque<Message>>,
}

impl Broker {
    pub fn new() -> Broker {
        Broker {
            queues: HashMap::new(),
        }
    }

    pub fn send(&mut self, message: Message) {
        match self.queues.get_mut(&message.topic) {
            Some(queue) => queue.push_back(message),
            None => {
                self.queues
                    .insert(message.topic.clone(), VecDeque::from([message]));
            }
        }
    }

    pub fn listen(&mut self, topic: &str) -> Option<Message> {
        match self.queues.get_mut(topic) {
            Some(queue) => queue.pop_front(),
            None => None,
        }
    }
}

#[cfg(test)]
mod broker_test {
    use super::*;
    use std::time::Duration;

    #[test]
    fn should_receive_message_and_send_immediately_when_delay_is_0() {
        let mut broker = Broker::new();
        let topic = "broker_tests";
        let message = Message::new(
            topic.to_string(),
            b"test message".into(),
            Duration::from_millis(0),
        );
        broker.send(message.clone());
        let new_message = broker.listen(topic);
        assert_eq!(new_message, Some(message));
    }
}
