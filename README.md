# rust-pubsub-nodes
Processing nodes with pub/sub system in Rust

This Rust framework is designed for real-time data stream processing with ZeroMQ. In this framework, a "node" acts as a processing unit that can receive input from a subscriber or publish its output through a publisher.

It also contains Rust code template for common use case reference such as `threadInStructWithDrop`.

## Build and Run

```
cargo run --bin zmq

cargo run --bin pipeAndZmq

cargo run --bin threadInStructWithDrop
```