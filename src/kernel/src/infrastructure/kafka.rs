use std::{sync::Arc, time::Duration};
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::de::DeserializeOwned;

use crate::identify::Identifiable;
///
/// Создание продюсера Kafka
pub fn create_kafka_producer(brokers: &str) -> FutureProducer {
    rdkafka::ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .set("queue.buffering.max.ms", "20")
        .create()
        .expect("Producer creation error")
}
///
/// Отправка data в топик Kafka 
pub async fn send_to_kafka<T>(data: Vec<u8>, producer: Arc<FutureProducer>, topic: &str) 
where 
    T: DeserializeOwned + Identifiable,
{
    if let Ok(payload) = serde_json::from_slice::<T>(&data) {
        let record = FutureRecord::to(topic)
            .key(payload.get_id())
            .payload(&data);
        let _ = producer.send(record, Duration::from_secs(5)).await;
    } else {
        tracing::info!("Invalid or corrupted data payload recieved");
    }
}