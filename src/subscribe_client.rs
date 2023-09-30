// This is to be able to compile the proto.
pub mod pub_sub_service {
    tonic::include_proto!("pubsub");
}

use pub_sub_service::{pub_sub_service_client::PubSubServiceClient, SubscribeRequest, PublishRequest, SubscribeResponseStream, PublishResponse};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PubSubServiceClient::connect("http://[::1]:60041").await?;
    
let mut stream = client
    .subscribe(Request::new(SubscribeRequest {
        topic: "Stein".to_string(),
    }))
    .await?.into_inner();

    while let Some(resp) = stream.message().await? {
        println!("THE RESPONSE IS = {}", resp.message);
    }

     Ok(())
}