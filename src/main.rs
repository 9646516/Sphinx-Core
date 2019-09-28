#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
extern crate futures;
extern crate rdkafka;
extern crate rdkafka_sys;
extern crate tar;
use crossbeam;
use std::sync::RwLock;
use std::{thread, time};

use futures::stream::*;
use rdkafka::{client::*, config::*, consumer::*, message::*};

use crate::SphinxCore::Judge::JudgeOption;
use crate::SphinxCore::Language::language;

#[cfg(test)]
mod test;

pub mod SphinxCore;

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {}

type LoggingConsumer = StreamConsumer<CustomContext>;

fn main() {
    let topics = vec!["in"];
    let brokers = "localhost:9092";
    let group_id = "Q";
    let sum = RwLock::new(1usize);
    let context = CustomContext;

    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer
        .subscribe(&topics.to_vec())
        .expect("Can't subscribe to specified topics");

    let message_stream = consumer.start();
    crossbeam::thread::scope(|s| {
        for message in message_stream.wait() {
            match message {
                Err(_) => println!("Error while reading from stream."),
                Ok(Err(e)) => println!("Kafka error: {}", e),
                Ok(Ok(m)) => {
                    let payload = match m.payload_view::<str>() {
                        None => "",
                        Some(Ok(s)) => s,
                        Some(Err(e)) => {
                            println!("Error while deserializing message payload: {:?}", e);
                            ""
                        }
                    }
                    .to_string();
                    let headers = m.headers().unwrap();
                    assert_eq!(headers.count(), 7);

                    let problem = String::from_utf8_lossy(headers.get(0).unwrap().1).to_string();

                    let time: u32 = String::from_utf8_lossy(headers.get(1).unwrap().1)
                        .to_string()
                        .parse()
                        .unwrap();

                    let mem: u32 = String::from_utf8_lossy(headers.get(2).unwrap().1)
                        .to_string()
                        .parse()
                        .unwrap();

                    let lang = language::from(
                        &String::from_utf8_lossy(headers.get(3).unwrap().1).to_string(),
                    );

                    let uid: u32 = String::from_utf8_lossy(headers.get(4).unwrap().1)
                        .to_string()
                        .parse()
                        .unwrap();
                    println!("OK");
                    let JudgeType: u8 = String::from_utf8_lossy(headers.get(5).unwrap().1)
                        .to_string()
                        .parse()
                        .unwrap();
                    println!("OK");

                    let judge: String =
                        String::from_utf8_lossy(headers.get(6).unwrap().1).to_string();

                    let opt = JudgeOption::new(time, mem);
                    println!("{}", payload);
                    let ref_sum = &sum;

                    s.spawn(move |_| {
                        while *ref_sum.read().unwrap() > 40 {
                            thread::sleep(time::Duration::from_millis(100));
                        }
                        *ref_sum.write().unwrap() += 1;
                        SphinxCore::Run::Run(uid, problem, lang, judge, opt, payload, JudgeType);
                        *ref_sum.write().unwrap() -= 1;
                    });
                    consumer.commit_message(&m, CommitMode::Async).unwrap();
                }
            };
        }
    })
    .expect("crossbeam Failed");
}
