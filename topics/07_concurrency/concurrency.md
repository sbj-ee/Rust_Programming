# Concurrency — Cheat Sheet

## Threads

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("running on a spawned thread");
    42
});
let result = handle.join().unwrap();   // blocks until the thread finishes; unwrap the panic-or-value
```

`thread::spawn` creates a real OS thread — 1:1, unlike Go's M:N goroutines scheduled onto a
small pool of OS threads. Spawning is more expensive per-call than a goroutine, and the
correctness bar (`Send`/`Sync`, below) is stricter, but there is no runtime scheduler to
reason about.

## `move` Closures

```rust
let data = vec![1, 2, 3];
thread::spawn(move || {                 // `move` transfers ownership of `data` INTO the closure
    println!("{}", data.iter().sum::<i32>());
});
```

Without `move`, the closure would try to *borrow* `data`, and the compiler can't prove the
borrow outlives the spawned thread (which might run after the current function returns) —
so it refuses to compile. This is the ownership system from topics/02 extended across
threads, not a separate mechanism.

## `Send` and `Sync` — the Traits That Make This Safe

- **`Send`**: a type is safe to *move* to another thread.
- **`Sync`**: a type is safe to *share by reference* (`&T`) across threads.

Both are auto-derived by the compiler for types built entirely from `Send`/`Sync` pieces —
you almost never write `impl Send for MyType` by hand. `Rc<T>` is deliberately **not**
`Send`/`Sync` (its refcount isn't atomic); `Arc<T>` is (it uses atomic operations). Trying to
send an `Rc<T>` to another thread is a **compile error**, not a race you discover in
production.

## Channels (`std::sync::mpsc`)

```rust
use std::sync::mpsc;
let (tx, rx) = mpsc::channel();
thread::spawn(move || tx.send("hi").unwrap());
println!("{}", rx.recv().unwrap());
```

Rust's direct analog of Go's `chan` — "share memory by communicating." `mpsc` = multi
producer (clone `tx`), single consumer (`rx`). Unlike Go, `std` has no built-in multi-channel
`select`; the `crossbeam-channel` crate adds one (not used in this project — zero deps).

## Shared State: `Arc<Mutex<T>>` / `Arc<RwLock<T>>`

```rust
use std::sync::{Arc, Mutex};
let counter = Arc::new(Mutex::new(0));
let c = Arc::clone(&counter);
thread::spawn(move || { *c.lock().unwrap() += 1; });  // guard auto-releases the lock on drop
```

`Arc<T>` (atomic Rc) gives multiple threads shared ownership; `Mutex<T>`/`RwLock<T>` guard
the data itself. Unlike a raw `pthread_mutex_t`, you cannot forget to unlock — the
`MutexGuard` releases the lock when it drops, and you cannot even *read* the data without
going through the guard the type system requires.

## What the Compiler Prevents

```rust
let mut counter = 0;
// thread::spawn(|| counter += 1);  // COMPILE ERROR: may outlive borrowed value `counter`
```

A data race requires two threads touching the same memory with at least one write and no
synchronization. The combination of ownership (topics/02) + `Send`/`Sync` makes the *setup*
for a data race a compile error — there is no race detector needed to catch it at runtime,
because it cannot compile in the first place. `unsafe` code can still introduce races; safe
code cannot.

## Comparison to C / C++ / Go

| Concern | C | C++ | Go | Rust |
|---|---|---|---|---|
| Thread model | `pthread_t`, 1:1 | `std::thread`, 1:1 | Goroutines, M:N | `std::thread`, 1:1 |
| Data race detection | None built-in (valgrind/helgrind) | None built-in (TSan) | `go run -race` (runtime) | Compile-time, via the type system |
| Shared mutable state | Raw pointer + manual mutex | `std::mutex` + `lock_guard` | Shared memory + `sync.Mutex`, or channels | `Arc<Mutex<T>>`; guard enforces access |
| Message passing | Manual (pipes, queues) | Manual, or a library | `chan` — built into the language | `std::sync::mpsc` — stdlib, not core syntax |
| Forgetting to unlock | Common bug | RAII (`lock_guard`) prevents it | N/A (channels) or `defer mu.Unlock()` | Impossible — guard drops automatically |
