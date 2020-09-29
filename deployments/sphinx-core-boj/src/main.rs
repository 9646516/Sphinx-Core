use std::{thread, time};
use std::sync::RwLock;

use dockworker::Docker;
use rdkafka::{ClientConfig, ClientContext, Message};
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, StreamConsumer};
use rdkafka::message::Headers;
use tokio::stream::StreamExt;

use env::JURY;
use sphinx_core::{JudgeReply, Language, MainServerClient, ProblemConfig, ProblemConfigOptions};
use sphinx_core_kafka::MainServerClientImpl;

mod env;

#[macro_use]
extern crate log;


#[cfg(test)]
mod test;

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {}

type LoggingConsumer = StreamConsumer<CustomContext>;

fn get_number(v: &[u8]) -> u64 {
    let mut ret: u64 = 0;
    for i in v.iter() {
        ret = ret * 256u64 + u64::from(*i);
    }
    ret
}


#[tokio::main]
async fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("Hello, world!");

    let sum = RwLock::new(1usize);
    let topics = vec!["in"];
    let brokers = "localhost:9092";
    let group_id = "Q";
    let context = CustomContext;
    let docker = Docker::connect_with_defaults().unwrap();

    let mut main_client = MainServerClientImpl::new();

    println!("connecting {}:group_id={}", brokers, group_id);

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

    let mut message_stream = consumer.start();

    println!("beginning to listening");

    while let Some(message) = message_stream.next().await {
        println!("receiving message");

        while *sum.read().unwrap() > 20 {
            thread::sleep(time::Duration::from_millis(100));
        }
        match message {
            Err(e) => println!("Error while reading from stream. {}", e),
            Ok(m) => {
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

                let path = format!("{}", String::from_utf8_lossy(headers.get(0).unwrap().1));

                let lang = Language::from(get_number(headers.get(1).unwrap().1));

                let uid: u64 = get_number(headers.get(2).unwrap().1);
                let options = ProblemConfigOptions {
                    spj_path: JURY.to_owned(),
                };

                let _conf = ProblemConfig::read(&path, &options);
                if let Ok(conf) = _conf {
                    let ref_sum = &sum;
                    println!("{}", payload);
                    println!("{} {} ", path, uid);
                    *ref_sum.write().unwrap() += 1;
                    sphinx_core_docker::run(
                        &docker, uid, lang, conf, payload, env::PAN_DIR, &mut main_client).await;
                    *ref_sum.write().unwrap() -= 1;
                } else {
                    println!("File {} Not Found,{:?}", path, _conf);
                    main_client.update_real_time_info(&JudgeReply {
                        status: "SYSTEM ERROR",
                        mem: 0,
                        time: 0,
                        submission_id: uid,
                        last: 0,
                        score: 0,
                        info: "File Not Found",
                    }).await;
                }
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}
