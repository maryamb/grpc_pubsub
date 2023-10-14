use std::collections::HashMap;
use std::sync::Mutex;

use tonic::{transport::Server, Request, Response, Status};
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream};

use std::sync::RwLock;
use futures::future::{self, try_join_all};



// This makes a separate module for the proto. As if it was in its own files. 
// This is to be able to compile the proto.
pub mod pub_sub_service {
    tonic::include_proto!("pubsub");
}

use pub_sub_service::{pub_sub_service_server::{PubSubService, PubSubServiceServer}, SubscribeRequest, PublishRequest, SubscribeResponseStream, PublishResponse};
struct Subscribers {
    subscriber: Vec<tokio::sync::mpsc::Sender<Result<SubscribeResponseStream, Status>>>,
}

struct PubSub {
    all_subscribers: RwLock<HashMap<String, Subscribers>>,
}

impl PubSub {
    fn new() -> Self {
       Self {
         all_subscribers: RwLock::new(HashMap::new()),
       }
    }
}



#[tonic::async_trait]
impl PubSubService for PubSub {
    type SubscribeStream = ReceiverStream<Result<SubscribeResponseStream, Status>>;

    async fn subscribe(&self, request: Request<SubscribeRequest>) -> Result<Response<Self::SubscribeStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        {        
            let mut subs = self.all_subscribers.write().unwrap();
            let mut topic_subs = subs.entry(request.get_ref().topic.to_owned()).or_insert(Subscribers{subscriber: vec![]});
            topic_subs.subscriber.push(tx);
        }
        Ok(Response::new(Self::SubscribeStream::new(rx)))
    }

    async fn publish(&self, request: Request<PublishRequest>) -> Result<Response<PublishResponse>, Status> {
        let message = &request.get_ref().message;
        let futures = loop {
            let subs = self.all_subscribers.read().unwrap();
            println!("1 topic = {}", &request.get_ref().topic);
            let topic_subs = match subs.get(&request.get_ref().topic) {
                None => break vec![],
                Some(x) => x,
            };
            let futures = topic_subs.subscriber.iter().map(|tx|
                tx.send(Ok(SubscribeResponseStream{message: message.clone()}))).collect();
            break futures;
        };
        try_join_all(futures).await;
        let response = PublishResponse {};
        /*
        tokio::spawn(async move {
            tx.send(Ok(SubscribeResponseStream{
                message: format!("subscribe to {}", request.get_ref().topic),
            })).await.unwrap();
            tx.send(Ok(SubscribeResponseStream{
                message: "Disconnected.".to_string(),
            })).await.unwrap();
        });
        */
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:60041".parse()?;
    println!("Server listening on {}", addr);

    let pubsub_service = PubSub::new();

    tonic::transport::Server::builder()
        .add_service(PubSubServiceServer::new(pubsub_service))
        .serve(addr)
        .await?;

    Ok(())
}
