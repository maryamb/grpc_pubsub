A pub-sub service based on GRPC. Not scalable yet. Good enough for few subscribers per topic.
Can handle many topics.

# Usage
## Server
Run the following command:
```
cargo run --bin pubsub-server -- --address="[::1]:60041"
```
Note `address` is optional. Default is localhost and port 60041 as specified above.
## Publish client
Run the following command:
```
cargo run --bin pubsub-publish-client -- -t "my_topic" -m "my_message" -a "http://[::1]:60041"
```
Note `-a` is optional.
## Subscribe client
Run the following command:
```
cargo run --bin pubsub-subscribe-client -- -t "my_topic" -a "http://[::1]:60041"
```
Again `-a` is optional. 
Obviously publish and subscribe clients are just an example of how the API can be used.

