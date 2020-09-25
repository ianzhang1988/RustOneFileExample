// use anyhow::Result;
use serde_json::{self, json, Value };
use crossbeam::{thread};
//use crossbeam::thread::ScopedJoinHandle;
use crossbeam::crossbeam_channel::bounded;
use crossbeam::atomic::AtomicCell;
// use reqwest::blocking::Response;
use std::time::{Instant, Duration};
use std::sync::Arc;

static WORK_DATA: &str =  "rest are just some random data: This guide provides an overview of the AMQP 0-9-1 protocol, one of the protocols supported by RabbitMQ.  High-level Overview of AMQP 0-9-1 and the AMQP Model  What is AMQP 0-9-1?  AMQP 0-9-1 (Advanced Message Queuing Protocol) is a messaging protocol that enables conforming client applications to communicate with conforming messaging middleware brokers.  Brokers and Their Role  Messaging brokers receive messages from publishers (applications that publish them, also known as producers) and route them to consumers (applications that process them).    Since it is a network protocol, the publishers, consumers and the broker can all reside on different machines.    AMQP 0-9-1 Model in Brief  The AMQP 0-9-1 Model has the following view of the world: messages are published to exchanges, which are often compared to post offices or mailboxes. Exchanges then distribute message copies to queues using rules called bindings. Then the broker either deliver messages to consumers subscribed to queues, or consumers fetch/pull messages from queues on demand.    Publish path from publisher to consumer via exchange and queue    When publishing a message, publishers may specify various message attributes (message meta-data). Some of this meta-data may be used by the broker, however, the rest of it is completely opaque to the broker and is only used by applications that receive the message.    Networks are unreliable and applications may fail to process messages therefore the AMQP 0-9-1 model has a notion of message acknowledgements: when a message is delivered to a consumer the consumer notifies the broker, either automatically or as soon as the application developer chooses to do so. When message acknowledgements are in use, a broker will only completely remove a message from a queue when it receives a notification for that message (or group of messages).    In certain situations, for example, when a message cannot be routed, messages may be returned to publishers, dropped, or, if the broker implements an extension, placed into a so-called  dead letter queue . Publishers choose how to handle situations like this by publishing messages using certain parameters.    Queues, exchanges and bindings are collectively referred to as AMQP entities.    AMQP 0-9-1 is a Programmable Protocol  AMQP 0-9-1 is a programmable protocol in the sense that AMQP 0-9-1 entities and routing schemes are primarily defined by applications themselves, not a broker administrator. Accordingly, provision is made for protocol operations that declare queues and exchanges, define bindings between them, subscribe to queues and so on.    This gives application developers a lot of freedom but also requires them to be aware of potential definition conflicts. In practice, definition conflicts are rare and often indicate a misconfiguration.    Applications declare the AMQP 0-9-1 entities that they need, define necessary routing schemes and may choose to delete AMQP 0-9-1 entities when they are no longer used.";
static USER_DATA: &str =  "rest are just some random data: Default Exchange The default exchange is a direct exchange with no name (empty string) pre-declared by the broker. It has one special property that makes it very useful for simple applications: every queue that is created is automatically bound to it with a routing key which is the same as the queue name.   For example, when you declare a queue with the name of  search-indexing-online , the AMQP 0-9-1 broker will bind it to the default exchange using  search-indexing-online  as the routing key (in this context sometimes referred to as the binding key). Therefore, a message published to the default exchange with the routing key  search-indexing-online  will be routed to the queue  search-indexing-online . In other words, the default exchange makes it seem like it is possible to deliver messages directly to queues, even though that is not technically what is happening.   Direct Exchange A direct exchange delivers messages to queues based on the message routing key. A direct exchange is ideal for the unicast routing of messages (although they can be used for multicast routing as well). Here is how it works:   A queue binds to the exchange with a routing key K When a new message with routing key R arrives at the direct exchange, the exchange routes it to the queue if K = R Direct exchanges are often used to distribute tasks between multiple workers (instances of the same application) in a round robin manner. When doing so, it is important to understand that, in AMQP 0-9-1, messages are load balanced between consumers and not between queues.   A direct exchange can be represented graphically as follows:   exchange delivering messages to  queues based on routing key   Fanout Exchange A fanout exchange routes messages to all of the queues that are bound to it and the routing key is ignored. If N queues are bound to a fanout exchange, when a new message is published to that exchange a copy of the message is delivered to all N queues. Fanout exchanges are ideal for the broadcast routing of messages.   Because a fanout exchange delivers a copy of a message to every queue bound to it, its use cases are quite similar:   Massively multi-player online (MMO) games can use it for leaderboard updates or other global events Sport news sites can use fanout exchanges for distributing score updates to mobile clients in near real-time Distributed systems can broadcast various state and configuration updates Group chats can distribute messages between participants using a fanout exchange (although AMQP does not have a built-in concept of presence, so XMPP may be a better choice) A fanout exchange can be represented graphically as follows:";

// static POST_URL: &str = "http://127.0.0.1:12345/job";
// static GET_URL: &str = "http://127.0.0.1:12345/job";
static POST_URL: &str = "http://10.19.17.188:12345/job";
static GET_URL: &str = "http://10.19.17.188:12345/job";

fn make_job_data(id: String) -> String{
    let work_data = format!("this is work data for id:{}. {}", '1', WORK_DATA);
    let user_data = format!("this is user data for id:{}. {}", '1', USER_DATA);
    let new_job = json!({
      "id": id,
      "work_data": work_data,
      "user_data": user_data
    });

    new_job.to_string()
}

fn add_job(start_id:u32, number: u32, thread: u32) {

    thread::scope(|s| {
        let failed = Arc::new(AtomicCell::<u32>::new(0));
        let min = Arc::new(AtomicCell::new(Duration::new(3600*24,0)));
        let max = Arc::new(AtomicCell::new(Duration::default()));
        let mut handles = Vec::new();

        let (sender, r) = bounded(500);

        let all_time = Instant::now();

        for _i in 0..thread {

            let receiver = r.clone();
            let failed_ref = failed.clone();
            let min_ref = min.clone();
            let max_ref = max.clone();

            let handle = s.spawn( move |_| {
                let client = reqwest::blocking::Client::builder()
                        .no_proxy()
                        .build().unwrap();

                loop {

                    let data:String = receiver.recv().unwrap();
                    if data.is_empty() {
                        break;
                    }

                    let now = Instant::now();

                    let ret = client.post(POST_URL)
                    .body(data)
                    .send();

                    let elapsed = now.elapsed();

                    if elapsed < min_ref.load() {
                        min_ref.store(elapsed);
                    }

                    if elapsed > max_ref.load() {
                        max_ref.store(elapsed);
                    }

                    match ret {
                        Ok(mut resp) => {
                            resp.copy_to(&mut std::io::sink()).unwrap();
                        },
                        Err(e) =>{
                            println!("{}", e);
                            // failed.fetch_add(1);
                            failed_ref.fetch_add(1);
                        }
                    }
                }
            });

            handles.push(handle);
        }

        for i in start_id..start_id+number {
            let data = make_job_data(i.to_string());
            sender.send(data).unwrap();
        }

        for _i in 0..thread {
            sender.send("".to_string()).unwrap();
        }

        for h in handles{
            h.join().unwrap();
        }

        let elapsed_all = all_time.elapsed();

        println!("all time {:0.3?}, avg {:0.5?}", elapsed_all, elapsed_all/number);
        println!("min time {:0.5?}, max time {:0.5?}", min.load(), max.load());
        println!("error: {}", failed.load());


    }).unwrap();


    // println!("{}",new_job);
}

// fn print_respond(mut res: Response) -> Result<()> {
//     println!("Status: {}", res.status());
//     println!("Headers:\n{:?}", res.headers());
//     // copy the response body directly to stdout
//     res.copy_to(&mut std::io::stdout())?;
//     Ok(())
// }
//
// fn get_one() -> Result<()>{
//
//
//     let client = reqwest::blocking::Client::builder()
//     .no_proxy()
//     .build()?;
//
//
//     let now = Instant::now();
//     let res = client.get("http://127.0.0.1:12345/job/1001").send()?;
//     let elapsed = now.elapsed();
//     println!("Elapsed: {:.2?}", elapsed);
//
//
//     print_respond(res)?;
//     Ok(())
// }

fn get_job(start_id:u32, number: u32, thread: u32) {

    thread::scope(|s| {
        let failed = Arc::new(AtomicCell::<u32>::new(0));
        let min = Arc::new(AtomicCell::new(Duration::new(3600*24,0)));
        let max = Arc::new(AtomicCell::new(Duration::default()));
        let mut handles = Vec::new();

        let (sender, r) = bounded(500);

        let all_time = Instant::now();

        for _i in 0..thread {

            let receiver = r.clone();
            let failed_ref = failed.clone();
            let min_ref = min.clone();
            let max_ref = max.clone();

            let handle = s.spawn( move |_| {
                let client = reqwest::blocking::Client::builder()
                        .no_proxy()
                        .build().unwrap();

                loop {

                    let id:String = receiver.recv().unwrap();
                    if id.is_empty() {
                        break;
                    }

                    let now = Instant::now();

                    let ret = client.get(format!("{}/{}", GET_URL, id).as_str()).send();

                    let elapsed = now.elapsed();

                    if elapsed < min_ref.load() {
                        min_ref.store(elapsed);
                    }

                    if elapsed > max_ref.load() {
                        max_ref.store(elapsed);
                    }

                    match ret {
                        Ok(resp) => {
                            // resp.copy_to(&mut std::io::sink()).unwrap();


                            // let data = resp.text().unwrap();
                            // println!("{}", &data);

                            // let json_data: Value = serde_json::from_str(
                            //     data.trim_start_matches('\u{feff}')).unwrap();


                            let json_data: Value = resp.json().unwrap();
                            if json_data["id"] != id {
                                println!("id not match");
                                failed_ref.fetch_add(1);
                            }

                            // println!("{:?}", json_data);
                        },
                        Err(e) =>{
                            println!("{}", e);
                            // failed.fetch_add(1);
                            failed_ref.fetch_add(1);
                        }
                    }
                }
            });

            handles.push(handle);
        }

        for i in start_id..start_id+number {
            sender.send(i.to_string()).unwrap();
        }

        for _i in 0..thread {
            sender.send("".to_string()).unwrap();
        }

        for h in handles{
            h.join().unwrap();
        }

        let elapsed_all = all_time.elapsed();

        println!("all time {:0.3?}, avg {:0.5?}", elapsed_all, elapsed_all/number);
        println!("min time {:0.5?}, max time {:0.5?}", min.load(), max.load());
        println!("error: {}", failed.load());


    }).unwrap();

}

fn main(){
    println!("start");

    println!("----------add 1000 thread 1");
    add_job(2000,1000,1);

    println!("----------add 10000 thread 10");
    add_job(3000,10000,10);

    println!("----------add 50000 thread 100");
    add_job(13000,50000,100);

    println!("----------get 1000 thread 1");
    get_job(2000,1000,1);

    println!("----------get 10000 thread 10");
    get_job(3000,10000,10);

    println!("----------get 50000 thread 100");
    get_job(13000,50000,100);
}
