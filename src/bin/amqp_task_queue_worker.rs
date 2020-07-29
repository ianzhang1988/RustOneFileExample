use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result};
use std::thread;
use std::time::Duration;
use serde_json::{self, Value };

const TASK_QUEUE: &'static str = "task_queue";

fn main() -> Result<()> {
    env_logger::init();

    // guest:guest@10.19.17.188:5672
    let mut connection = Connection::insecure_open("amqp://guest:guest@10.19.17.188:5672")?;

    let channel = connection.open_channel(None)?;

    let queue = channel.queue_declare(
        TASK_QUEUE,
        QueueDeclareOptions {
            durable: true,
            ..QueueDeclareOptions::default()
        },
    )?;

    channel.qos(0, 1, false)?;

    let consumer = queue.consume(ConsumerOptions::default())?;

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                println!("({:>3}) Received [{}]", i, body);

                // Sleep for n seconds, where n is the number of '.' chars in the body,
                // before we ack the message.
                // let dits = delivery.body.iter().filter(|&&b| b == b'.').count();

                // how can we unify this error?
                let v: Value = serde_json::from_str(&body).unwrap();

                if let Some(sec) = v["work"].as_u64() {
                    thread::sleep(Duration::from_secs(sec as u64));
                    println!("({:>3}) ... done sleeping", i);
                }

                consumer.ack(delivery)?;
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()?;


    Ok(())
}