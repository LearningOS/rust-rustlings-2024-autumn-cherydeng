// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Arc<Queue>, tx: mpsc::Sender<u32>) {
    let qc1 = Arc::clone(&q);
    let tx1 = tx.clone(); // Clone the Sender for the first thread
    let thread1 = thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            if let Err(_) = tx1.send(*val) {
                println!("Receiver dropped");
                return; // Exit if the receiver is dropped
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    let qc2 = Arc::clone(&q);
    let tx2 = tx; // Use the original Sender for the second thread
    let thread2 = thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            if let Err(_) = tx2.send(*val) {
                println!("Receiver dropped");
                return; // Exit if the receiver is dropped
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Queue::new()); // Use Arc to share Queue
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}

