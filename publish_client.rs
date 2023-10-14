// This is to be able to compile the proto.
pub mod pub_sub_service {
    tonic::include_proto!("pubsub");
}

use pub_sub_service::{pub_sub_service_client::PubSubServiceClient, PublishRequest, PublishResponse};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PubSubServiceClient::connect("http://[::1]:60041").await?;
    
    client.publish(Request::new(PublishRequest {
        topic: "first_topic".to_string(),
        message: "first message".to_string(),
    })).await?;
     Ok(())
}