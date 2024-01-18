use std::{sync, thread, time};
use std::sync::atomic::{AtomicBool, Ordering};

struct Data {
    counter: usize,
}

pub struct Timer {
    handle: Option<thread::JoinHandle<()>>,
    alive: sync::Arc<AtomicBool>
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            handle: None,
            alive: sync::Arc::new(AtomicBool::new(false))
        }
    }

    pub fn start<F>(&mut self, mut fun: F)
    where
        F: 'static + Send + FnMut() -> (),
    {
        self.alive.store(true, Ordering::SeqCst);

        let alive = self.alive.clone();

        self.handle = Some(thread::spawn(move || {
            while alive.load(Ordering::SeqCst) {
                fun();
                thread::sleep(time::Duration::from_millis(10));
            }
        }));
    }

    pub fn stop(&mut self) {
        self.alive.store(false, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            handle.join().expect("Could not join spawned thread");
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.stop();
        println!("ToDrop is being dropped");
    }
}

fn main() {
    let mut data = Data { counter: 0 };

    {
        let mut timer = Timer::new();
        timer.start(move || {
            // Modify a value in the Data struct
            data.counter += 1;
            println!("Counter: {}", data.counter);
        });

        println!("Feeling sleepy...");
        thread::sleep(time::Duration::from_millis(100));
    } // Timer goes out of scope here and is automatically destructed

    println!("Time for dinner!");
    thread::sleep(time::Duration::from_millis(300));
    // Timer has been destructed, and its resources have been cleaned up
}
