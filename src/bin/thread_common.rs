use crossbeam::{thread,sync::WaitGroup};
use std::{time, thread::sleep};
use std::sync::{Arc, Mutex};
// use crossbeam::crossbeam_channel::unbounded;
use crossbeam::crossbeam_channel::bounded;
use rand::Rng;

fn increase_count(counter: Arc<Mutex<u32>>) {
    for _ in 0..100 {
        let mut data = counter.lock().unwrap();
        *data += 1;
    }
}

fn count() {
    let counter = Arc::new(Mutex::new(0));

    thread::scope(|s| {
        for _ in 0..10 {
            let counter_t = Arc::clone(&counter);
            s.spawn(move |_| {
                increase_count(counter_t);
            });
        }
    }).unwrap();

    let data = counter.lock().unwrap();
    println!("counter {}", *data)
}

fn producer_customer() {
    thread::scope(|s| {

        let wg = WaitGroup::new();

        // let (sender, r) = unbounded();
        let (sender, r) = bounded(10);

        // producer
        for pid in 1..4 {
            let wg_t = wg.clone();
            let sender_t = sender.clone();
            s.spawn( move |_| {
                let mut rng = rand::thread_rng();

                let mut work_data = Vec::<(i32,i32)>::new();
                for _ in 0..50 {
                    let time: i32 = rng.gen_range(1, 4);
                    work_data.push((pid, time));
                }

                println!("{} will send {:?}", pid, &work_data);

                for data in work_data {
                    sender_t.send(data).unwrap();
                }

                drop(wg_t);

            });
        }

        // customer
        let customer_num = 50;
        for cid in 0..customer_num {
            let r_t = r.clone();
            s.spawn( move |_| {
                loop {
                    let (pid, time) = r_t.recv().unwrap();
                    if pid == 0 {
                        println!("customer {}: finished", cid);
                        break;
                    }

                    println!("customer {}: from {} working {}", cid, pid, time);
                    let sleep_time = time::Duration::from_secs(time as u64);
                    sleep(sleep_time);
                }
            });
        }


        // pid 0 for exit
        wg.wait();
        for _ in 0..customer_num {
            sender.send((0, 0)).unwrap();
        }

    }).unwrap();

}

fn main() {
    count();
    producer_customer();
}
