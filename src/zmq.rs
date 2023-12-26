use zmq::{Context, Socket, PollItem, POLLIN, PUB, SUB};
use std::time::{Duration, Instant};

struct PublisherNode {
    socket: Socket,
}

impl PublisherNode {
    fn new() -> Self {
        let context = Context::new();
        let socket = context.socket(PUB).unwrap();
        socket.bind("tcp://127.0.0.1:5555").unwrap();

        PublisherNode { socket }
    }

    fn start(self) {
        std::thread::spawn(move || {
            let mut last_publish_time = Instant::now();

            loop {
                let elapsed_time = last_publish_time.elapsed();
                last_publish_time = Instant::now();

                let elapsed_micros = elapsed_time.as_micros();

                println!("Publisher: Elapsed time since last publish: {}us", elapsed_micros);

                // Publish a message (replace with your specific message)
                self.socket.send("Hello, world!", 0).unwrap();

                std::thread::yield_now();
            }
        });
    }
}

struct SubscriberNode {
    socket: Socket,
}

impl SubscriberNode {
    fn new() -> Self {
        let context = Context::new();
        let socket = context.socket(SUB).unwrap();
        socket.connect("tcp://127.0.0.1:5555").unwrap();
        socket.set_subscribe(b"").unwrap();

        SubscriberNode { socket }
    }

    fn start(self) {
        std::thread::spawn(move || {
            let mut poll_items = [PollItem::from(self.socket.as_poll_item(POLLIN))];

            loop {
                zmq::poll(&mut poll_items, -1).unwrap();

                if poll_items[0].is_readable() {
                    let message = self.socket.recv_string(0).unwrap().unwrap();
                    println!("Subscriber: Received: {}", message);
                }

                std::thread::yield_now();
            }
        });
    }
}

fn main() {
    // Instantiate and start the publisher thread
    let publisher_node = PublisherNode::new();
    publisher_node.start();

    // Wait for a short time to allow the publisher to bind
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Instantiate and start the subscriber thread
    let subscriber_node = SubscriberNode::new();
    subscriber_node.start();

    // Sleep to keep the main thread alive (replace with your own logic)
    std::thread::sleep(std::time::Duration::from_secs(10));
}
