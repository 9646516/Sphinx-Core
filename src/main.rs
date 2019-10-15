#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
extern crate futures;
extern crate rdkafka;
extern crate rdkafka_sys;
extern crate tar;


use futures::lazy;
use futures::stream::*;
use rdkafka::{client::*, config::*, consumer::*, message::*};
use tokio;
use tokio::runtime::current_thread;

use SphinxCore::Config;
use SphinxCore::Env::*;

use crate::SphinxCore::Language::language;

#[cfg(test)]
mod test;

pub mod SphinxCore;

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {}

type LoggingConsumer = StreamConsumer<CustomContext>;

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
    let context = CustomContext;

    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer
        .subscribe(&topics.to_vec())
        .expect("Can't subscribe to specified topics");

    let mut thread_pool = tokio::runtime::Builder::new()
        .name_prefix("pool-")
        .core_threads(24)
        .build()
        .unwrap();
    let mut io_thread = current_thread::Runtime::new().unwrap();

    let stream_processor = consumer.start()
        .filter_map(|result| {  // Filter out errors
            match result {
                Ok(msg) => Some(msg),
                Err(kafka_error) => {
                    panic!("Error while receiving from Kafka: {:?}", kafka_error);
                }
            }
        }).for_each(move |borrowed_message| {
        let m = borrowed_message.detach();
        let message_future = lazy(move || {
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
            let path: String = format!(
                "{}/{}",
                PAN_DIR,
                String::from_utf8_lossy(headers.get(0).unwrap().1)
            );

            let lang = language::from(get_number(headers.get(1).unwrap().1));

            let uid: u64 = get_number(headers.get(2).unwrap().1);
            let _conf = Config::Config::read(&format!("{}/problem-config.toml", path));
            if let Ok(conf) = _conf {
                println!("{}", payload);
                println!("{} {} ", path, uid);
                SphinxCore::Run::Run(uid, lang, conf, payload, &path);
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
            Ok(())
        });
        thread_pool.spawn(message_future);
        Ok(())
    });

    println!("Starting event loop");
    let _ = io_thread.block_on(stream_processor);
    println!("Stream processing terminated");
}
