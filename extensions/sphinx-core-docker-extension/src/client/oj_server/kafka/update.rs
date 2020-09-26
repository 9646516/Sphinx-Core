extern crate futures;
extern crate rdkafka;

use rdkafka::{config::*, message::*, producer::*};
use self::rdkafka::util::Timeout;
use std::time::Duration;
use sphinx_core::JudgeReply;

pub fn update_real_time_info(
    reply: &JudgeReply,
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
            .payload(reply.status)
            .key("")
            .headers(
                OwnedHeaders::new()
                    .add("mem", &format!("{}", reply.mem))
                    .add("time", &format!("{}", reply.time))
                    .add("uid", &format!("{}", reply.submission_id))
                    .add("last", &format!("{}", reply.last))
                    .add("score", &format!("{}", reply.score))
                    .add("info", reply.info),
            ),
        Timeout::from(Duration::from_secs(10)),
    )).unwrap();
    println!(
        "status:{} mem:{} time:{} uid:{} last:{} info:{} score:{}",
        reply.status, reply.mem, reply.time, reply.submission_id, reply.last, reply.info, reply.score
    );
}
