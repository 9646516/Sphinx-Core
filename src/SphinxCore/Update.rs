extern crate futures;
extern crate rdkafka;

use rdkafka::{config::*, message::*, producer::*};

pub fn UpdateRealTimeInfo(
    status: &str,
    mem: &u32,
    time: &u32,
    SubmissionID: &u32,
    last: &u32,
    info: &str,
) {
    let topic_name = "result";
    let brokers = "localhost:9092";
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("produce.offset.report", "true")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");
    producer.send(
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
    );
    println!(
        "{} {} {} {} {} {}",
        status, mem, time, SubmissionID, last, info
    );
}
