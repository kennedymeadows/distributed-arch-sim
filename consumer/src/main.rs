use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::topic_partition_list::TopicPartitionList;
use tokio::runtime::Runtime;
use log::{info, error};
use std::env;

fn main() {
    println!("Starting consumer");
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", "test-consumer-group")
            .set("bootstrap.servers", "kafka.default.svc.cluster.local:9092")            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("auto.offset.reset", "earliest")
            .set("security.protocol", "SASL_PLAINTEXT")
            .set("sasl.mechanism", "SCRAM-SHA-256")
            .set("sasl.username", "user1")
            .set("sasl.password", env::var("KAFKA_PASSWORD").expect("KAFKA_PASSWORD must be set"))
            .set("fetch.max.bytes", "1048576") // Increase fetch size to 1MB
            .set("fetch.min.bytes", "50000") // Wait for at least 50KB of data
            .set("max.partition.fetch.bytes", "256000") // Fetch more data per partition
            .set("max.poll.records", "500") // Increase number of records per poll
            .set("auto.commit.interval.ms", "5000") // Adjust auto commit interval
            .create()
            .expect("Consumer creation failed");

        let mut tpl = TopicPartitionList::new();
        tpl.add_partition("tasks", 0);

        consumer
            .subscribe(&["tasks"])
            .expect("Can't subscribe to specified topic");

        loop {
            match consumer.recv().await {
                Err(e) => error!("Error receiving message: {:?}", e),
                Ok(m) => {
                    let payload = match m.payload_view::<str>() {
                        None => "",
                        Some(Ok(s)) => s,
                        Some(Err(e)) => {
                            error!("Error while deserializing message payload: {:?}", e);
                            ""
                        }
                    };
                    info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                          m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                    consumer.commit_message(&m, rdkafka::consumer::CommitMode::Async).unwrap();
                    println!("Message received: {}", payload);
                }
            }
        }
    });
}