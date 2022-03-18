mod kafka_sender;
mod log_service;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use crate::kafka_sender::LogProducer;
use crate::log_service::start_log_service;
use anyhow::Result;

use futures::StreamExt;
use std::net::SocketAddr;
use flume::Sender;
use tonic::{transport::Server, Request, Response, Status, Streaming};

pub mod logcanal {
    tonic::include_proto!("logcanal");
}
use logcanal::log_canal_server::{LogCanal, LogCanalServer};
use logcanal::{MessageRequest, ResponseStatus};



#[derive(Debug)]
pub struct LogCanalService{
    addr: SocketAddr,
    sender: Sender<String>
}

#[tonic::async_trait]
impl LogCanal for LogCanalService{
    async fn sink(&self,
                  request: Request<tonic::Streaming<MessageRequest>>)->Result<Response<ResponseStatus>, Status>
    {

        let mut stream: Streaming<MessageRequest> = request.into_inner();
        let resp = ResponseStatus{ fall_back_enabled: 0 };

        while let Some(req) = stream.next().await {
            let event = req.unwrap().message.unwrap();
            self.sender.send(serde_json::to_string(&event).unwrap()).unwrap();
        }
        Ok(Response::new(resp))

    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    dotenv().ok();
    let (tx, rx) = flume::unbounded();
    let log_producer = LogProducer::new().await;

    tokio::spawn(async move {
        start_log_service(log_producer.clone(), rx.clone()).await;
    });

    let addr = "127.0.0.1:50002".parse().unwrap();

    let log_canal = LogCanalService {
        addr,
        sender: tx.clone(),
    };

    let svc = LogCanalServer::new(log_canal);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}

