use std::collections::HashMap;

use tonic::{transport::Server, Request, Response, Status};
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream};

// This makes a separate module for the proto. As if it was in its own files. 
// This is to be able to compile the proto.
pub mod pub_sub_service {
    tonic::include_proto!("pubsub");
}

use pub_sub_service::{pub_sub_service_server::{PubSubService, PubSubServiceServer}, SubscribeRequest, PublishRequest, SubscribeResponseStream, PublishResponse};


struct PubSub {}


#[tonic::async_trait]
impl PubSubService for PubSub {
    type SubscribeStream = ReceiverStream<Result<SubscribeResponseStream, Status>>;

    async fn subscribe(&self, request: Request<SubscribeRequest>) -> Result<Response<Self::SubscribeStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            tx.send(Ok(SubscribeResponseStream{
                message: format!("subscribe to {}", request.get_ref().topic),
            })).await.unwrap();
            tx.send(Ok(SubscribeResponseStream{
                message: "Disconnected.".to_string(),
            })).await.unwrap();
        });
        
        Ok(Response::new(Self::SubscribeStream::new(rx)))
    }

    async fn publish(&self, request: Request<PublishRequest>) -> Result<Response<PublishResponse>, Status> {
//         let request = request.into_inner();
//         let topic = request.topic;
//         let message = request.message;

        let response = PublishResponse {};

//         if let Some(subscribers) = self.subscribers.read().await.get(&topic) {
//             for subscriber in subscribers {
//                 let _ = subscriber.send(response.clone());
//             }
//         }

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:60041".parse()?;
    println!("Server listening on {}", addr);

    let pubsub_service = PubSub {};

    tonic::transport::Server::builder()
        .add_service(PubSubServiceServer::new(pubsub_service))
        .serve(addr)
        .await?;

    Ok(())
}
