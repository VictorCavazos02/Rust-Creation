
use std::sync::{mpsc, Arc, Mutex};

use std::thread;

// Message to be sent to the workers
enum Message {
    NewJob(Job),
    Terminate,
}

// Job type is a boxed closure that can be sent across threads
type Job = Box<dyn FnOnce() + Send + 'static>;

// ThreadPool struct
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    // Create a new ThreadPool with the specified size
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // Create channel
        let (sender, receiver) = mpsc::channel();

        // Share receiver safely across threads
        let receiver = Arc::new(Mutex::new(receiver));

        // Create workers
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    // Execute a job in the thread pool
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// Clean up resources when ThreadPool is dropped
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Send terminate message to all workers
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        // Wait for all workers to finish
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// Worker struct represents a thread that can process jobs
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // Create a new worker with the specified ID
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn main() {
    let pool = ThreadPool::new(4);

    for i in 1..=10 {
        pool.execute(move || {
            println!("Processing task {}", i);
            thread::sleep(std::time::Duration::from_millis(500));
            println!("Completed task {}", i);
        });
    }

    println!("Main thread waiting for tasks to complete...");
}


use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    const ITEM_COUNT: usize = 20;

    // Create channel
    let (tx, rx) = mpsc::channel();

    let rx = Arc::new(Mutex::new(rx));

    // Create producer threads
    let mut producer_handles = vec![];
    for i in 0..2 {
        let tx_clone = tx.clone();
        producer_handles.push(thread::spawn(move || {
            producer(i, tx_clone, ITEM_COUNT);
        }));
    }

    // Create consumer threads
    let mut consumer_handles = vec![];
    for i in 0..3 {
        let rx_clone = Arc::clone(&rx);
        consumer_handles.push(thread::spawn(move || {
            consumer(i, rx_clone);
        }));
    }

    // Wait for producers to finish
    for handle in producer_handles {
        handle.join().unwrap();
    }

    println!("All producers finished. Sending termination signals...");

    // Send termination signals (one per consumer)
    for _ in 0..3 {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    // Wait for consumers to finish
    for handle in consumer_handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

// Producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();

    for _ in 0..item_count {
        let num = rng.gen_range(1..100);

        println!("Producer {} produced {}", id, num);
        tx.send(num).unwrap();

        thread::sleep(Duration::from_millis(100));
    }

    println!("Producer {} finished", id);
}

// Consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let value = {
            // lock scope
            let receiver = rx.lock().unwrap();
            receiver.recv().unwrap()
        };

        if value == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal", id);
            break;
        }

        println!("Consumer {} consumed {}", id, value);

        thread::sleep(Duration::from_millis(150));
    }

    println!("Consumer {} exiting", id);
}