#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
extern crate futures;
extern crate rdkafka;
extern crate rdkafka_sys;
extern crate tar;
use crossbeam;
use std::sync::RwLock;
use std::{thread, time};

use crate::SphinxCore::Language::language;
use futures::stream::*;
use rdkafka::{client::*, config::*, consumer::*, message::*};
use SphinxCore::Config;
use SphinxCore::Env::*;
#[cfg(test)]
mod test;

pub mod SphinxCore;

fn get_number(V: &[u8]) -> u64 {
    let mut ret: u64 = 0;
    for i in V.iter() {
        ret = ret * 256u64 + u64::from(*i);
    }
    ret
}
fn main() {
    let topics = vec!["in"];
    let brokers = "localhost:9092";
    let group_id = "Q";
    let sum = RwLock::new(1usize);

    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&topics.to_vec())
        .expect("Can't subscribe to specified topics");

    let message_stream = consumer.start();
    crossbeam::thread::scope(|s| {
        for message in message_stream.wait() {
            while *sum.read().unwrap() > 20 {
                thread::sleep(time::Duration::from_millis(100));
            }
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
                    assert_eq!(headers.count(), 3);
                    // for i in 0..3 {
                    //     println!(
                    //         "{:?} {:?}",
                    //         headers.get(i).unwrap().0,
                    //         headers.get(i).unwrap().1
                    //     );
                    // }
                    let path: String = format!(
                        "{}/{}",
                        PAN_DIR,
                        String::from_utf8_lossy(headers.get(0).unwrap().1)
                    );

                    let lang = language::from(get_number(headers.get(1).unwrap().1));

                    let uid: u64 = get_number(headers.get(2).unwrap().1);
                    let _conf = Config::Config::read(&format!("{}/problem-config.toml", path));
                    if let Ok(conf) = _conf {
                        let ref_sum = &sum;
                        println!("{}", payload);
                        println!("{} {} ", path, uid);
                        s.spawn(move |_| {
                            *ref_sum.write().unwrap() += 1;
                            SphinxCore::Run::Run(uid, lang, conf, payload, &path);
                            *ref_sum.write().unwrap() -= 1;
                        });
                    } else {
                        println!("File Not Found,{:?}", _conf);
                        SphinxCore::Update::UpdateRealTimeInfo(
                            "SYSTEM ERROR",
                            0,
                            0,
                            uid,
                            0,
                            0,
                            "File Not Found",
                        )
                    }
                    consumer.commit_message(&m, CommitMode::Async).unwrap();
                }
            };
        }
    })
    .expect("crossbeam Failed");
}
