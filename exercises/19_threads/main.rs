// Exercise 19: Threads
//
// Demonstrates: `std::thread::spawn`, `JoinHandle::join`, `move` closures
// for handing data to a thread, and why the borrow checker — not just
// convention — prevents data races at compile time. This is Rust's
// "fearless concurrency": the same rules from exercises 05/06 extended
// across threads via the `Send`/`Sync` marker traits (see topics/07).

use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Exercise 19: Threads ===");

    // Section 1: spawning a thread and joining it
    println!("\n--- Section 1: spawn and join ---");
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("  spawned thread: {i}");
            thread::sleep(Duration::from_millis(1));
        }
        "done" // the closure's return value becomes the JoinHandle's result
    });
    for i in 1..=2 {
        println!("main thread: {i}");
    }
    let result = handle.join().unwrap(); // blocks until the thread finishes
    println!("spawned thread returned: {result}");

    // Section 2: `move` — required to hand OWNED data into a thread, because
    // the closure might outlive the current stack frame
    println!("\n--- Section 2: move closures ---");
    #[allow(clippy::useless_vec)]
    // Vec (heap-owned) is deliberate here, not an array, to show a move of owned heap data
    let data = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        // without `move`, this would try to borrow `data` — and the compiler
        // cannot prove `data` outlives the spawned thread, so it refuses to compile
        let sum: i32 = data.iter().sum();
        println!("  sum computed on spawned thread: {sum}");
        sum
    });
    let sum = handle.join().unwrap();
    println!("sum={sum}");

    // Section 3: spawning several threads and collecting their results
    println!("\n--- Section 3: multiple threads ---");
    let mut handles = Vec::new();
    for id in 0..4 {
        handles.push(thread::spawn(move || id * id));
    }
    let squares: Vec<i32> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("squares from 4 threads: {squares:?}");

    // Section 4: what the borrow checker prevents — a compile-time data race
    println!("\n--- Section 4: what does NOT compile ---");
    println!("  let mut counter = 0;");
    println!(
        "  thread::spawn(|| counter += 1); // COMPILE ERROR: may outlive borrowed value `counter`"
    );
    println!("  (fix: move ownership in, or share via Arc<Mutex<_>> — see exercise 21)");

    println!("\nNotes:");
    println!("  - thread::spawn returns a JoinHandle; join() blocks and yields the closure's return value.");
    println!("  - `move` transfers ownership of captured variables into the thread — required whenever the");
    println!("    thread might outlive the stack frame that spawned it, which the compiler assumes it might.");
    println!(
        "  - Data races are a COMPILE error here, not a runtime bug you find with a race detector."
    );
    println!("  - Threads are OS threads (1:1), unlike Go's M:N goroutines — spawn cost is higher, correctness bar is stricter.");
}
