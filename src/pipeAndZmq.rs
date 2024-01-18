use std::sync::mpsc;
use std::thread;
use serde::{Serialize, Deserialize};
use serde_json::{json, to_string};
use zmq::{Context, Socket, SUB, PUB};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    sender: String,
    content: String,
}

struct PublisherNode {
    internal_sender: mpsc::Sender<Message>,
    network_socket: Socket,
}

impl PublisherNode {
    fn new(context: &Context, internal_sender: mpsc::Sender<Message>, address: &str) -> Self {
        let network_socket = context.socket(PUB).unwrap();
        network_socket.bind(address).unwrap();

        PublisherNode {
            internal_sender,
            network_socket,
        }
    }

    fn start(self) {
        thread::spawn(move || {
            let sender = "Internal Publisher".to_string();

            loop {
                println!("pub");
                let message = Message {
                    sender: sender.clone(),
                    content: "Hello, internal channel".to_string(),
                };

                self.internal_sender.send(message.clone()).unwrap();

                // Serialize the message to JSON
                let json_str = serde_json::to_string(&message).unwrap();

                // Create a new zmq::Message and set its content
                let mut zmq_message = zmq::Message::from(&json_str);

                // Send the zmq::Message
                self.network_socket.send(zmq_message, 0).unwrap();

                thread::sleep(std::time::Duration::from_secs(1));
            }
        });
    }
}

struct SubscriberNode {
    internal_receiver: mpsc::Receiver<Message>,
    network_socket: Socket,
}

impl SubscriberNode {
    fn new(context: &Context, internal_receiver: mpsc::Receiver<Message>, address: &str) -> Self {
        let network_socket = context.socket(SUB).unwrap();
        network_socket.connect(address).unwrap();
        network_socket.set_subscribe(b"").unwrap();

        SubscriberNode {
            internal_receiver,
            network_socket,
        }
    }

    fn start(self) {
        thread::spawn(move || {
            loop {
                println!("sub");
                if let Ok(message) = self.internal_receiver.recv() {
                    println!("Internal Subscriber: Received {:?}", message);
                }

                if let Ok(zmq_message) = self.network_socket.recv_msg(0) {
                    // Extract the string content from the zmq::Message
                    let message_str = zmq_message.as_str().unwrap();
                    // Deserialize the received message string
                    if let Ok(message) = serde_json::from_str::<Message>(message_str) {
                        println!("Network Subscriber: Received {:?}", message);
                    }
                }

                thread::sleep(std::time::Duration::from_secs(1));
            }
        });
    }
}

fn main() {
    let context = Context::new();
    let (internal_sender, internal_receiver) = mpsc::channel();

    let network_address = "tcp://127.0.0.1:5555";

    let publisher_node = PublisherNode::new(&context, internal_sender.clone(), network_address);
    let subscriber_node = SubscriberNode::new(&context, internal_receiver, network_address);

    publisher_node.start();
    subscriber_node.start();

    thread::sleep(std::time::Duration::from_secs(10));
}
