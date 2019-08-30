extern crate futures;
extern crate rdkafka;

use futures::*;
use rdkafka::config::*;
use rdkafka::message::*;
use rdkafka::producer::*;

pub fn UpdateRealTimeInfo(status: &str, mem: &u32, time: &u32, uid: &u32, last: &u32, info: &str) {
    let topic_name = "result";
    let brokers = "localhost:9092";
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("produce.offset.report", "true")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");
    let futures = producer
        .send(
            FutureRecord::to(topic_name)
                .payload(status)
                .key(&format!("2333"))
                .headers(
                    OwnedHeaders::new()
                        .add("mem", &format!("{}", mem))
                        .add("time", &format!("{}", time))
                        .add("uid", &format!("{}", uid))
                        .add("last", &format!("{}", last))
                        .add("info", info),
                ),
            0,
        )
        .map(move |delivery_status| {
            println!("Delivery status for message 1 received");
            delivery_status
        });

    println!("Future completed. Result: {:?}", futures.wait());
}
