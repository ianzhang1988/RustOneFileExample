use anyhow::Result;
use serde_json::{self, json, Value };
use crossbeam::{thread,sync::WaitGroup};
use reqwest::blocking::Response;
use std::time::Instant;

static WORK_DATA: &str =  "rest are just some random data: This guide provides an overview of the AMQP 0-9-1 protocol, one of the protocols supported by RabbitMQ.  High-level Overview of AMQP 0-9-1 and the AMQP Model  What is AMQP 0-9-1?  AMQP 0-9-1 (Advanced Message Queuing Protocol) is a messaging protocol that enables conforming client applications to communicate with conforming messaging middleware brokers.  Brokers and Their Role  Messaging brokers receive messages from publishers (applications that publish them, also known as producers) and route them to consumers (applications that process them).    Since it is a network protocol, the publishers, consumers and the broker can all reside on different machines.    AMQP 0-9-1 Model in Brief  The AMQP 0-9-1 Model has the following view of the world: messages are published to exchanges, which are often compared to post offices or mailboxes. Exchanges then distribute message copies to queues using rules called bindings. Then the broker either deliver messages to consumers subscribed to queues, or consumers fetch/pull messages from queues on demand.    Publish path from publisher to consumer via exchange and queue    When publishing a message, publishers may specify various message attributes (message meta-data). Some of this meta-data may be used by the broker, however, the rest of it is completely opaque to the broker and is only used by applications that receive the message.    Networks are unreliable and applications may fail to process messages therefore the AMQP 0-9-1 model has a notion of message acknowledgements: when a message is delivered to a consumer the consumer notifies the broker, either automatically or as soon as the application developer chooses to do so. When message acknowledgements are in use, a broker will only completely remove a message from a queue when it receives a notification for that message (or group of messages).    In certain situations, for example, when a message cannot be routed, messages may be returned to publishers, dropped, or, if the broker implements an extension, placed into a so-called  dead letter queue . Publishers choose how to handle situations like this by publishing messages using certain parameters.    Queues, exchanges and bindings are collectively referred to as AMQP entities.    AMQP 0-9-1 is a Programmable Protocol  AMQP 0-9-1 is a programmable protocol in the sense that AMQP 0-9-1 entities and routing schemes are primarily defined by applications themselves, not a broker administrator. Accordingly, provision is made for protocol operations that declare queues and exchanges, define bindings between them, subscribe to queues and so on.    This gives application developers a lot of freedom but also requires them to be aware of potential definition conflicts. In practice, definition conflicts are rare and often indicate a misconfiguration.    Applications declare the AMQP 0-9-1 entities that they need, define necessary routing schemes and may choose to delete AMQP 0-9-1 entities when they are no longer used.";
static USER_DATA: &str =  "rest are just some random data: Default Exchange The default exchange is a direct exchange with no name (empty string) pre-declared by the broker. It has one special property that makes it very useful for simple applications: every queue that is created is automatically bound to it with a routing key which is the same as the queue name.   For example, when you declare a queue with the name of  search-indexing-online , the AMQP 0-9-1 broker will bind it to the default exchange using  search-indexing-online  as the routing key (in this context sometimes referred to as the binding key). Therefore, a message published to the default exchange with the routing key  search-indexing-online  will be routed to the queue  search-indexing-online . In other words, the default exchange makes it seem like it is possible to deliver messages directly to queues, even though that is not technically what is happening.   Direct Exchange A direct exchange delivers messages to queues based on the message routing key. A direct exchange is ideal for the unicast routing of messages (although they can be used for multicast routing as well). Here is how it works:   A queue binds to the exchange with a routing key K When a new message with routing key R arrives at the direct exchange, the exchange routes it to the queue if K = R Direct exchanges are often used to distribute tasks between multiple workers (instances of the same application) in a round robin manner. When doing so, it is important to understand that, in AMQP 0-9-1, messages are load balanced between consumers and not between queues.   A direct exchange can be represented graphically as follows:   exchange delivering messages to  queues based on routing key   Fanout Exchange A fanout exchange routes messages to all of the queues that are bound to it and the routing key is ignored. If N queues are bound to a fanout exchange, when a new message is published to that exchange a copy of the message is delivered to all N queues. Fanout exchanges are ideal for the broadcast routing of messages.   Because a fanout exchange delivers a copy of a message to every queue bound to it, its use cases are quite similar:   Massively multi-player online (MMO) games can use it for leaderboard updates or other global events Sport news sites can use fanout exchanges for distributing score updates to mobile clients in near real-time Distributed systems can broadcast various state and configuration updates Group chats can distribute messages between participants using a fanout exchange (although AMQP does not have a built-in concept of presence, so XMPP may be a better choice) A fanout exchange can be represented graphically as follows:";

fn make_job_data(id: String) -> String{
    let work_data = format!("this is work data for id:{}. {}", '1', WORK_DATA);
    let user_data = format!("this is user data for id:{}. {}", '1', USER_DATA, );
    let new_job = json!({
      "id": id,
      "work_data": work_data,
      "user_data": user_data
    });

    new_job.to_string()
}

fn add_job(start_id:u32, number: u32, thread: u32) {
    1.to_string();


    // println!("{}",new_job);
}

fn print_respond(mut res: Response) -> Result<()> {
    println!("Status: {}", res.status());
    println!("Headers:\n{:?}", res.headers());
    // copy the response body directly to stdout
    res.copy_to(&mut std::io::stdout())?;
    Ok(())
}

fn get_one() -> Result<()>{


    let client = reqwest::blocking::Client::builder()
    .no_proxy()
    .build()?;


    let now = Instant::now();
    let res = client.get("http://127.0.0.1:12345/job/1001").send()?;
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);


    print_respond(res)?;
    Ok(())
}

fn get_job() {

    get_one();

}

fn main(){
    get_job();
    add_job(1,1,1);
}
