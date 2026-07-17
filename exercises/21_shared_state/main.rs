// Exercise 21: Shared State
//
// Demonstrates: `Arc<T>` (thread-safe reference counting — Rc's counterpart
// for multiple threads) combined with `Mutex<T>`/`RwLock<T>` for shared
// mutable state. This is the "share memory by locking it" style Go
// proverbially discourages — but here the compiler REFUSES to compile code
// that shares mutable state without a lock, unlike C/C++/Go.

use std::sync::{Arc, Mutex, RwLock};
use std::thread;

fn main() {
    println!("=== Exercise 21: Shared State ===");

    // Section 1: Arc<Mutex<T>> — the standard "share + mutate across threads" combo
    println!("\n--- Section 1: Arc<Mutex<T>> ---");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..10 {
        let counter = Arc::clone(&counter); // clone the Arc, not the data — cheap refcount bump
        handles.push(thread::spawn(move || {
            let mut guard = counter.lock().unwrap(); // blocks until the lock is free
            *guard += 1;
            // guard drops here at end of scope, releasing the lock automatically —
            // no manual unlock() to forget, unlike a raw pthread_mutex_t
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("counter after 10 threads: {}", *counter.lock().unwrap());

    // Section 2: what the compiler prevents — sharing a plain Mutex-less counter
    println!("\n--- Section 2: what does NOT compile ---");
    println!("  let mut counter = 0;");
    println!("  for _ in 0..10 {{ thread::spawn(|| counter += 1); }} // COMPILE ERROR");
    println!("  Mutex<T> or Arc<T> alone won't fix it either — you need BOTH:");
    println!("  Arc for shared ownership across threads, Mutex for exclusive access to the data.");

    // Section 3: RwLock<T> — many readers OR one writer, better than Mutex when reads dominate
    println!("\n--- Section 3: RwLock<T> ---");
    let config = Arc::new(RwLock::new(String::from("initial-config")));
    let mut readers = Vec::new();
    for id in 0..3 {
        let config = Arc::clone(&config);
        readers.push(thread::spawn(move || {
            let value = config.read().unwrap(); // many concurrent readers allowed
            println!("  reader {id} sees: {value}");
        }));
    }
    for r in readers {
        r.join().unwrap();
    }
    {
        let mut writer = config.write().unwrap(); // exclusive — blocks all readers and other writers
        *writer = String::from("updated-config");
    }
    println!("after write: {}", config.read().unwrap());

    // Section 4: a poisoned lock — what happens if a thread panics while holding one
    println!("\n--- Section 4: lock poisoning ---");
    let shared = Arc::new(Mutex::new(0));
    let shared_clone = Arc::clone(&shared);
    let result = thread::spawn(move || {
        let _guard = shared_clone.lock().unwrap();
        panic!("oops, panicking while holding the lock");
    })
    .join();
    println!("thread panicked: {}", result.is_err());
    match shared.lock() {
        Ok(_) => println!("lock still fine"),
        Err(poisoned) => println!(
            "lock is POISONED — recoverable via poisoned.into_inner(): {}",
            poisoned.into_inner()
        ),
    }

    println!("\nNotes:");
    println!("  - Arc<T>: thread-safe shared ownership (atomic refcounts); Rc<T> from exercise 16 is NOT thread-safe.");
    println!("  - MutexGuard releases the lock automatically on drop — impossible to forget an unlock().");
    println!("  - RwLock allows concurrent readers, exclusive writers — use it when reads vastly outnumber writes.");
    println!("  - A panic while holding a lock POISONS it; later lockers get Err, forcing you to notice and handle it.");
}
