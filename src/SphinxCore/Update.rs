extern crate futures;
extern crate rdkafka;

use std::time::Instant;

use futures::*;
use rdkafka::{config::*, message::*, producer::*};

pub fn UpdateRealTimeInfo(
    status: &str,
    mem: &u32,
    time: &u32,
    SubmissionID: &u32,
    last: &u32,
    info: &str,
) {
    let op = Instant::now();
    let topic_name = "result";
    let brokers = "localhost:9092";
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("produce.offset.report", "true")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");
    println!(
        "Connected,Elapsed {} secs",
        op.elapsed().as_micros() as f64 / 1000f64
    );
    let futures = producer
        .send(
            FutureRecord::to(topic_name)
                .payload(status)
                .key("")
                .headers(
                    OwnedHeaders::new()
                        .add("mem", &format!("{}", mem))
                        .add("time", &format!("{}", time))
                        .add("uid", &format!("{}", SubmissionID))
                        .add("last", &format!("{}", last))
                        .add("info", info),
                ),
            0,
        )
        .map(move |delivery_status| delivery_status);
    println!(
        "{} {} {} {} {} {}",
        status, mem, time, SubmissionID, last, info
    );
    println!("Future completed. Result: {:?}", futures.wait());
    println!("Elapsed {} secs", op.elapsed().as_micros() as f64 / 1000f64);
}
