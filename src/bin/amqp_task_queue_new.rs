use amiquip::{AmqpProperties, Connection, Exchange, Publish, QueueDeclareOptions, Result};
use std::env;
use serde_json::{self, json };
use rand::{thread_rng, Rng};
use uuid::Uuid;

/// job
/// { "id":i, "work":1 }

const TASK_QUEUE: &'static str = "task_queue";

fn main() -> Result<()> {
    env_logger::init();

    // guest:guest@10.19.17.188:5672
    let mut connection = Connection::insecure_open("amqp://guest:guest@10.19.17.188:5672")?;

    let channel = connection.open_channel(None)?;

    let _ = channel.queue_declare(
        TASK_QUEUE,
        QueueDeclareOptions {
            durable: true,
            ..QueueDeclareOptions::default()
        },
    )?;

    // Get a handle to the direct exchange on our channel.
    let exchange = Exchange::direct(&channel);

    for _ in 0..10 {

        let mut rng = thread_rng();
        let sec: u32 = rng.gen_range(1, 6);

        // Publish a message to the "hello" queue.
        let mut message = env::args().skip(1).collect::<Vec<_>>().join(" ");
        if message.is_empty() {
            // message = "Hello world.".to_string();
            message = json!({
                "id":Uuid::new_v4(),
                "work":sec,
            }).to_string();
        }

        exchange.publish(Publish::with_properties(
            message.as_bytes(),
            TASK_QUEUE,
            // delivery_mode 2 makes message persistent
            AmqpProperties::default().with_delivery_mode(2),
        ))?;
        println!("Sent message [{}]", message);
    }


    Ok(())
}