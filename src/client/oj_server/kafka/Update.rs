extern crate futures;
extern crate rdkafka;

use rdkafka::{config::*, message::*, producer::*};
use self::rdkafka::util::Timeout;
use std::time::Duration;

pub fn update_real_time_info(
    status: &str,
    mem: u64,
    time: u64,
    submission_id: u64,
    last: u32,
    score: u64,
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
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(producer.send(
        FutureRecord::to(topic_name)
            .payload(status)
            .key("")
            .headers(
                OwnedHeaders::new()
                    .add("mem", &format!("{}", mem))
                    .add("time", &format!("{}", time))
                    .add("uid", &format!("{}", submission_id))
                    .add("last", &format!("{}", last))
                    .add("score", &format!("{}", score))
                    .add("info", info),
            ),
        Timeout::from(Duration::from_secs(10)),
    )).unwrap();
    println!(
        "status:{} mem:{} time:{} uid:{} last:{} info:{} score:{}",
        status, mem, time, submission_id, last, info, score
    );
}
