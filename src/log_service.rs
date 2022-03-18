use crate::kafka_sender::LogProducer;
use flume::Receiver;
use rdkafka::producer::BaseRecord;

const TOPIC: &str = "log-servico-canal";

pub async fn start_log_service(log_producer: LogProducer, receiver: Receiver<String>) {
    loop {
        while let Some(log) = receiver.iter().next() {
            let kafka_record: BaseRecord<String, String> = BaseRecord::to(TOPIC).payload(&log);
            log_producer
                .producer
                .send(kafka_record)
                .expect("Failed to send message.");
        }
    }
}