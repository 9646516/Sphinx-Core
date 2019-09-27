extern crate futures;
extern crate rdkafka;

use std::fs::read_to_string;

use futures::*;
use rdkafka::{client::*, config::*, consumer::*, message::*, producer::*};

fn produce(brokers: &str, topic_name: &str, uid: i32) {
    let cpp = read_to_string("./test/a+b/Main.java").unwrap();
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("produce.offset.report", "true")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");
    let futures = producer
        .send(
            FutureRecord::to(topic_name)
                .payload(&cpp)
                .key(&format!("233"))
                .headers(
                    OwnedHeaders::new()
                        .add("problem", "a+b")
                        .add("time", "1000")
                        .add("mem", "256000000")
                        .add("lang", "JAVA")
                        .add("uid", &uid.to_string())
                        .add("spj", "")
                        .add("interactive", ""),
                ),
            0,
        )
        .map(move |delivery_status| {
            println!("Delivery status for message 1 received");
            delivery_status
        });

    println!("Future completed. Result: {:?}", futures.wait());
}

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {}

type LoggingConsumer = StreamConsumer<CustomContext>;

fn consume_and_print(brokers: &str, group_id: &str, topics: &[&str]) {
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
                };
                println!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                         m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                if let Some(headers) = m.headers() {
                    for i in 0..headers.count() {
                        let header = headers.get(i).unwrap();
                        println!("  Header {:#?}: {:?}", header.0, header.1);
                    }
                }
                consumer.commit_message(&m, CommitMode::Async).unwrap();
                if payload != "RUNNING" {
                    //  break;
                }
            }
        };
    }
}

#[test]
fn main() {
    let topic = "in";
    let brokers = "localhost:9092";
    for _i in 0..100 {
        produce(brokers, topic, _i as i32);
    }
    let topics = vec!["result"];
    let group_id = "Q2";
    let brokers = "localhost:9092";
    consume_and_print(brokers, group_id, &topics);
}
