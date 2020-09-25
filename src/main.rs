#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crossbeam;
use std::sync::RwLock;
use std::{thread, time};
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::{ClientConfig, ClientContext, Message};
use rdkafka::consumer::{ConsumerContext, StreamConsumer, Consumer, CommitMode};
use sphinx::env::{PAN_DIR, JURY};
use tokio::stream::StreamExt;
use rdkafka::message::Headers;
use sphinx_core::{Language, ProblemConfigOptions, ProblemConfig};


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
    let sum = RwLock::new(1usize);
    let topics = vec!["in"];
    let brokers = "localhost:9092";
    let group_id = "Q";
    let context = CustomContext;

    crossbeam::thread::scope(|s| {

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

        let mut rt = tokio::runtime::Runtime::new().unwrap();

        while let Some(message) = rt.block_on(message_stream.next()) {
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

                    let lang = Language::from(get_number(headers.get(1).unwrap().1));

                    let uid: u64 = get_number(headers.get(2).unwrap().1);
                    let options = ProblemConfigOptions {
                        spj_path: JURY.to_owned(),
                    };

                    let _conf = ProblemConfig::read(&format!("{}/problem-config.toml", path), &options);
                    if let Ok(conf) = _conf {
                        let ref_sum = &sum;
                        println!("{}", payload);
                        println!("{} {} ", path, uid);
                        s.spawn(move |_| {
                            *ref_sum.write().unwrap() += 1;
                            sphinx_core_docker::run(uid, lang, conf, payload, &path);
                            *ref_sum.write().unwrap() -= 1;
                        });
                    } else {
                        println!("File Not Found,{:?}", _conf);
                        sphinx_core_docker::client::oj_server::kafka::update::update_real_time_info(
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
