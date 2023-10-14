pub mod pub_sub_service {
    tonic::include_proto!("pubsub");
}

use pub_sub_service::{pub_sub_service_client::PubSubServiceClient, PublishRequest, PublishResponse};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PubSubServiceClient::connect("http://[::1]:60041").await?;
    
let resp = client
    .publish(Request::new(PublishRequest {
        topic: "Stein".to_string(),
        message: "the message of Stein topic".to_string(),
    }))
    .await?.into_inner();

    println!("THE PUBLISH RESPONSE IS = {:?}", resp);

     Ok(())
}