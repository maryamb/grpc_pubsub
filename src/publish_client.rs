use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The pubsub topic
    #[arg(short, long)]
    topic: String,

    /// Pubsub message
    #[arg(short, long)]
    message: String,
}

pub mod pub_sub_service {
    tonic::include_proto!("pubsub");
}

use pub_sub_service::{pub_sub_service_client::PubSubServiceClient, PublishRequest, PublishResponse};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut client = PubSubServiceClient::connect("http://[::1]:60041").await?;
    
    let resp = client
        .publish(Request::new(PublishRequest {
            topic: args.topic,
            message: args.message,
        }))
        .await?.into_inner();

    println!("THE PUBLISH RESPONSE IS = {:?}", resp);

     Ok(())
}