mod error;
mod kafka;
fn main() {
    let con = kafka::create_kafka_consumer();
    tokio::spawn(async move {
        kafka::kafka_consumer_task(con).await;
    });
}
