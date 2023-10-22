use clap::Parser;
use pub_sub_service::{pub_sub_service_client::PubSubServiceClient, PublishRequest};
use tonic::Request;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The pubsub topic
    #[arg(short, long)]
    topic: String,

    /// Pubsub message
    #[arg(short, long)]
    message: String,

    /// Ip and port as a string
    #[arg(short, long, default_value = "http://[::1]:60041")]
    address: String,
}

pub mod pub_sub_service {
    tonic::include_proto!("pubsub");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut client = PubSubServiceClient::connect(args.address).await?;
    
    let resp = client
        .publish(Request::new(PublishRequest {
            topic: args.topic,
            message: args.message,
        }))
        .await?.into_inner();

    println!("THE PUBLISH RESPONSE IS = {:?}", resp);

     Ok(())
}