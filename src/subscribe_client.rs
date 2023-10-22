use pub_sub_service::{pub_sub_service_client::PubSubServiceClient, SubscribeRequest};
use tonic::Request;
use clap::Parser;

// This is to be able to compile the proto.
pub mod pub_sub_service {
    tonic::include_proto!("pubsub");
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The pubsub topic
    #[arg(short, long)]
    topic: String,

    /// Ip and port as a string
    #[arg(short, long, default_value = "http://[::1]:60041")]
    address: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut client = PubSubServiceClient::connect(args.address).await?;
    
let mut stream = client
    .subscribe(Request::new(SubscribeRequest {
        topic: args.topic,
    }))
    .await?.into_inner();

    while let Some(resp) = stream.message().await? {
        println!("THE RESPONSE IS = {}", resp.message);
    }

     Ok(())
}