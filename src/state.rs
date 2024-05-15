use crate::kafka;
use rdkafka::producer::FutureProducer;

#[derive(Clone)]
pub struct AppState {
    kafka_producer: FutureProducer,
}

impl AppState {
    pub fn new() -> Self {
        let kafka_producer = kafka::create_kafka_producer();
        Self { kafka_producer }
    }
}

impl<'a> AppState {
    pub fn producer(&'a self) -> &'a FutureProducer {
        &self.kafka_producer
    }
}
