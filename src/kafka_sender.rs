use rdkafka::producer::{ProducerContext, ThreadedProducer};
use rdkafka::{ClientConfig, ClientContext};
use std::env;
use std::sync::Arc;
use rdkafka::message::DeliveryResult;

const KAFKA_CONFIG: &str = "KAFKA";

pub struct ProducerCallbackLogger;

impl ClientContext for ProducerCallbackLogger {}

impl ProducerContext for ProducerCallbackLogger {
    type DeliveryOpaque = ();

    fn delivery(&self, delivery_result: &DeliveryResult<'_>, _delivery_opaque: Self::DeliveryOpaque) {
        let dr = delivery_result.as_ref();
        match dr {
            Ok(_) => {},
            Err(_) => println!("Failed to delivery message"),
        }
    }
}

#[derive(Clone)]
pub struct LogProducer {
    pub producer: Arc<ThreadedProducer<ProducerCallbackLogger>>,
}

impl LogProducer {
    pub async fn new() -> LogProducer {
        let config = build_config();
        let producer: ThreadedProducer<ProducerCallbackLogger> =
            config.create_with_context(ProducerCallbackLogger {}).unwrap();

        let future_producer: Arc<ThreadedProducer<ProducerCallbackLogger>> = Arc::new(producer);
        LogProducer {
            producer: future_producer,
        }
    }
}

fn build_config() -> ClientConfig {
    let mut kafka_config = ClientConfig::new();

    for (config_key, config_value) in env::vars_os() {
        if let (Ok(key), Ok(value)) = (config_key.into_string(), config_value.into_string()) {
            if let true = key.contains(KAFKA_CONFIG) {
                kafka_config.set(
                    &key.replace("_", ".").replace("KAFKA.", "").to_lowercase(),
                    &value,
                );
            }
        }
    }
    kafka_config
}
