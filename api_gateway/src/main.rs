use actix_web::{web, App, HttpResponse, HttpServer};
use rdkafka::config::ClientConfig;
use rdkafka::admin::{AdminClient, AdminOptions};
use rdkafka::client::DefaultClientContext;
use std::time::Duration;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

async fn check_kafka_connectivity() -> Result<(), Box<dyn std::error::Error>> {
    let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
        .set("bootstrap.servers", "kafka.default.svc.cluster.local:9092")
        .set("message.timeout.ms", "5000")
        .set("security.protocol", "SASL_PLAINTEXT")
        .set("sasl.mechanism", "SCRAM-SHA-256")
        .set("sasl.username", "user1")
        .set("sasl.password", env::var("KAFKA_PASSWORD").expect("KAFKA_PASSWORD must be set"))
        .set("debug", "all")
        .create()
        .expect("Producer creation error");

    let metadata = admin_client
        .inner()
        .fetch_metadata(Some("non_existent_topic_for_test"), Duration::from_secs(5))
        .expect("Failed to fetch metadata");

    if metadata.brokers().is_empty() {
        Err("No brokers found. Unable to connect to Kafka cluster.".into())
    } else {
        println!("Successfully connected to Kafka cluster. Broker count: {}", metadata.brokers().len());
        Ok(())
    }
}

async fn produce_message_to_kafka(data: String) -> Result<HttpResponse, actix_web::Error> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "kafka-controller-0.kafka-controller-headless.default.svc.cluster.local:9092,kafka-controller-1.kafka-controller-headless.default.svc.cluster.local:9092,kafka-controller-2.kafka-controller-headless.default.svc.cluster.local:9092")
        .set("message.timeout.ms", "5000")
        .set("security.protocol", "SASL_PLAINTEXT")
        .set("sasl.mechanism", "SCRAM-SHA-256")
        .set("sasl.username", "user1")
        .set("sasl.password", env::var("KAFKA_PASSWORD").expect("KAFKA_PASSWORD must be set"))
        .set("debug", "all")
        .set("linger.ms", "5")
        .set("batch.size", "16384")
        .set("queue.buffering.max.messages", "100000") 
        .create()
        .expect("Producer creation error");

    let topic = "tasks";

    println!("Sending message to Kafka: {}", data);

    producer.send(
        FutureRecord::<(), String>::to(topic)
            .payload(&data)
            .timestamp(now()),
        std::time::Duration::from_secs(5),
    ).await.map_err(|e| {
        eprintln!("Failed to send message: {:?}", e);
        actix_web::error::ErrorInternalServerError("Failed to send message")
    })?;
    
    println!("Message sent: {}", data);

    Ok(HttpResponse::Ok().body(format!(
        "Message sent: {}, this is the Http Response\n", data)))

}

async fn index() -> impl actix_web::Responder {
    HttpResponse::Ok().body("Hello from API Gateway!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match check_kafka_connectivity().await {
        Ok(_) => println!("Kafka connectivity check passed"),
        Err(e) => {
            eprintln!("Kafka connectivity check failed: {:?}", e);
            std::process::exit(1);
        }
    }
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            // Adjust the route to accept POST requests with a String body
            .route("/produce", web::post().to(produce_message_to_kafka))
    })
    .bind("0.0.0.0:8080")?
    .workers(100)
    .run()
    .await
}

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .try_into()
        .unwrap()
}