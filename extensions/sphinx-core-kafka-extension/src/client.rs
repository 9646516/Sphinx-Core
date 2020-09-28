extern crate futures;
extern crate rdkafka;
use crate::client::futures::TryFutureExt;
use std::time::Duration;
use futures::{future, Future};
use rdkafka::{config::*, message::*, producer::*};

use sphinx_core::{JudgeReply, MainServerClient};

use self::rdkafka::util::Timeout;
use self::futures::{AsyncBufRead, TryStreamExt};

pub struct MainServerClientImpl<'a> {
    rt: &'a mut tokio::runtime::Runtime,
}

impl<'a> MainServerClientImpl<'a> {
    pub fn new(rt: &mut tokio::runtime::Runtime) -> MainServerClientImpl {
        MainServerClientImpl {
            rt
        }
    }
}

impl<'a> MainServerClient for MainServerClientImpl<'a> {
    fn update_real_time_info(&mut self, reply: &JudgeReply) {
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
        ).and_then(|_|{
            println!(
                "status:{} mem:{} time:{} uid:{} last:{} info:{} score:{}",
                reply.status, reply.mem, reply.time, reply.submission_id, reply.last, reply.info, reply.score
            )
        });
    }
}

