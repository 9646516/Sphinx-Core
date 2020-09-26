extern crate futures;
extern crate rdkafka;

use std::time::Duration;

use rdkafka::{config::*, message::*, producer::*};

use sphinx_core::{JudgeReply, MainServerClient};

use self::rdkafka::util::Timeout;

pub struct MainServerClientImpl {}

impl MainServerClientImpl {
    pub fn new() -> MainServerClientImpl {
        MainServerClientImpl {}
    }
}

impl MainServerClient for MainServerClientImpl {
    fn update_real_time_info(&mut self, reply: &JudgeReply) {
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
}

