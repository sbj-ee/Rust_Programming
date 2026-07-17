// Exercise 20: Channels
//
// Demonstrates: `std::sync::mpsc` (multi-producer, single-consumer)
// channels — Rust's direct equivalent of Go's channels, minus `select`
// (there's no built-in multi-channel select in std; see topics/07 for
// crossbeam's alternative, which this project doesn't depend on).

use std::sync::mpsc;
use std::thread;

fn main() {
    println!("=== Exercise 20: Channels ===");

    // Section 1: a simple send/receive
    println!("\n--- Section 1: basic channel ---");
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("hello from a thread").unwrap();
    });
    let received = rx.recv().unwrap(); // blocks until a message arrives
    println!("received: {received}");

    // Section 2: sending multiple values, receiving as an iterator
    println!("\n--- Section 2: streaming values ---");
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i).unwrap();
        }
        // tx is dropped here when the closure ends — that's what lets rx's
        // iterator terminate below instead of blocking forever
    });
    for value in rx {
        // iterating a Receiver yields values until the channel is closed
        print!("{value} ");
    }
    println!();

    // Section 3: multiple producers — "mpsc" means many senders, one receiver
    println!("\n--- Section 3: multiple producers ---");
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    for id in 0..3 {
        let tx_clone = tx.clone(); // cloning a Sender gives another producer handle
        handles.push(thread::spawn(move || {
            tx_clone
                .send(format!("message from producer {id}"))
                .unwrap();
        }));
    }
    drop(tx); // drop the original so rx's loop below ends once all clones are dropped too
    for h in handles {
        h.join().unwrap();
    }
    let mut messages: Vec<String> = rx.into_iter().collect();
    messages.sort(); // producers may finish in any order
    for m in &messages {
        println!("  {m}");
    }

    // Section 4: try_recv — non-blocking receive
    println!("\n--- Section 4: try_recv (non-blocking) ---");
    let (tx, rx) = mpsc::channel();
    match rx.try_recv() {
        Ok(v) => println!("got {v}"),
        Err(mpsc::TryRecvError::Empty) => println!("nothing available yet (would block on recv())"),
        Err(mpsc::TryRecvError::Disconnected) => println!("sender gone"),
    }
    tx.send(42).unwrap();
    println!("try_recv after send: {:?}", rx.try_recv());

    println!("\nNotes:");
    println!("  - mpsc = multi-producer, single-consumer; clone a Sender to get another producer.");
    println!("  - Iterating a Receiver blocks for each value and stops once every Sender has been dropped.");
    println!("  - Go's `chan` unifies unbounded/buffered/select; std::sync::mpsc has no built-in select —");
    println!("    reach for crossbeam-channel in a real project if you need one (not used here, zero deps).");
    println!("  - recv() blocks; try_recv() polls without blocking; recv_timeout() blocks with a deadline.");
}
