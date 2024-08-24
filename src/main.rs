mod broker;
mod message;
use message::Message;
use std::time::Duration;

fn main() {
    let sleep_duration = Duration::from_millis(16);
    let new_message = Message::new("tasqu".to_string(), b"hello tasqu".into(), sleep_duration);
    println!("Hello, world!");
}
