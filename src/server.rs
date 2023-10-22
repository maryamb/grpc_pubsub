use std::collections::HashMap;

use tonic::{Request, Response, Status};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream};

use std::sync::RwLock;
use futures::future::{try_join_all};

use clap::Parser;

// This makes a separate module for the proto. As if it was in its own files. 
// This is to be able to compile the proto.
pub mod pub_sub_service {
    tonic::include_proto!("pubsub");
}

use pub_sub_service::{pub_sub_service_server::{PubSubService, PubSubServiceServer}, SubscribeRequest, PublishRequest, SubscribeResponseStream, PublishResponse};
struct Subscribers {
    subscribers: Vec<tokio::sync::mpsc::Sender<Result<SubscribeResponseStream, Status>>>,
}

struct PubSub {
    topic_subscribers: RwLock<HashMap<String, Subscribers>>,
}

impl PubSub {
    fn new() -> Self {
       Self {
         topic_subscribers: RwLock::new(HashMap::new()),
       }
    }
}


#[tonic::async_trait]
impl PubSubService for PubSub {
    type SubscribeStream = ReceiverStream<Result<SubscribeResponseStream, Status>>;

    async fn subscribe(&self, request: Request<SubscribeRequest>) -> Result<Response<Self::SubscribeStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        {    
            // .write in this line does mutexing.
            let mut subs = self.topic_subscribers.write().unwrap();
            let topic_subs = subs.entry(request.get_ref().topic.to_owned()).or_insert(Subscribers{subscribers: vec![]});
            topic_subs.subscribers.push(tx);
        }
        Ok(Response::new(Self::SubscribeStream::new(rx)))
    }

    async fn publish(&self, request: Request<PublishRequest>) -> Result<Response<PublishResponse>, Status> {
        let message = &request.get_ref().message;
        let topic_subs = loop {
            // .read also does the mutexing.
            let subs = self.topic_subscribers.read().unwrap();
            println!("1 topic = {}", &request.get_ref().topic);
            let topic_subs = match subs.get(&request.get_ref().topic) {
                None => break vec![],
                Some(x) => x,
            };
            break topic_subs.subscribers.clone();
        };
        let result_holder: Vec<_> = topic_subs.iter().map(|tx|
            tx.send(Ok(SubscribeResponseStream{message: message.clone()}))).collect();
        
        let _ = try_join_all(result_holder).await;
        let response = PublishResponse {};
        Ok(Response::new(response))
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Ip and port as a string
    #[arg(short, long, default_value = "[::1]:60041")]
    address: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let addr = args.address.parse()?;
    println!("Server listening on {}", addr);

    let pubsub_service = PubSub::new();

    tonic::transport::Server::builder()
        .add_service(PubSubServiceServer::new(pubsub_service))
        .serve(addr)
        .await?;

    Ok(())
}
