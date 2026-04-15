/*
use std::thread;
use std::time::Duration;

fn main() {
    println!("Main thread starting");
    
    // TODO: Create a vector to store thread handles
    let mut handles = vec![];
    
    // TODO: Spawn 3 threads
    for i in 1..=3 {
        // TODO: Spawn a thread and store its handle
        let handle = thread::spawn(move || {
            // Simulate some work
            println!("Thread {} starting", i);
            thread::sleep(Duration::from_millis(500));
            println!("Thread {} finished", i);
        });
        
        // TODO: Store the handle
        handles.push(handle);
    }
    
    // TODO: Wait for all threads to complete
    for handle in handles{
    handle.join().unwrap();
    }
    println!("All threads completed.");
}
*/

use std::sync::{Arc, Mutex};


fn main() {
    // TODO: Create a shared counter using Arc and Mutex
    let cnt = Arc::new(Mutex::new(0));
    
    // TODO: Create a vector to store thread handles
    let mut handles = vec![];
    
    // TODO: Spawn 5 threads
    for i in 1..=5 {
        // TODO: Clone the Arc for the thread
        let thread = {
            let cnt1 = cnt.clone();
            std::thread::spawn(move || {
                for j in 0..10{
                    *cnt1.lock().unwrap() += 1;
                    println!("Thread {} at step {}", i, j);
                }
            })
        };
        
        // TODO: Spawn a thread that increments the counter 10 times
        handles.push(thread);
    }
    
    // TODO: Wait for all threads to complete
    for handle in handles{
        handle.join().unwrap();
    }
    
    // TODO: Print the final value of the counter
    println!("The final value of coutner is: {}", *cnt.lock().unwrap());
    
}