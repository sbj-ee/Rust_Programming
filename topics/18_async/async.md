# Async/Await — Cheat Sheet

## `async fn` Compiles to a State Machine

```rust
async fn compute() -> i32 {
    let a = 5;
    let b = 10;
    a + b
}
```

Calling `compute()` does **not** run the body — it returns a value implementing `Future`,
inert until something polls it. This is the opposite of Go, where `go f()` immediately
schedules `f` to run concurrently; Rust's `async fn` is lazy by construction.

## `std` Ships the Trait, Not an Executor

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

`Future`, `Poll`, `Context`, and `Waker` all live in `std`/`core` — but **no executor** does.
This is deliberate: an embedded target, a game engine, and a web server all want different
scheduling policies, so the language gives you the trait and lets the ecosystem (`tokio`,
`async-std`, `smol`) supply the runtime. Go made the opposite tradeoff: goroutines +
scheduler are baked into every binary, with no choice of runtime.

## `.await` — a Suspend Point

```rust
async fn delayed_greeting(name: &str) -> String {
    some_delay().await;                       // suspends here until the awaited future is Ready
    format!("Hello, {name}, after a delay")
}
```

Each `.await` is a point where this function's state machine can pause and return control to
whatever is driving it, and later resume exactly where it left off — conceptually similar to
a C# `await` or a JS `await`, and to what a goroutine's blocking channel receive does
implicitly under Go's scheduler.

## `Poll::Pending` and the `Waker`

```rust
impl Future for Delay {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if ready_now() {
            Poll::Ready(())
        } else {
            register_to_wake_later(cx.waker().clone());   // "call me when you should poll me again"
            Poll::Pending
        }
    }
}
```

A `Future` that isn't ready returns `Poll::Pending` and stashes a clone of the `Waker` it was
given — the executor's contract is: don't bother polling this future again until its waker
fires. A hand-rolled `block_on` (exercise 29) demonstrates the whole loop in ~15 lines: poll,
and if `Pending`, park the thread until `Waker::wake()` unparks it.

## No `.await` Concurrency Without a Crate

```rust
async fn two_things() {
    thing_one().await;   // sequential — thing_two doesn't start until thing_one finishes
    thing_two().await;
}
```

`std` alone gives you no `join!`/`select!` — those macros, and real non-blocking I/O
(sockets, files) integrated with a reactor, come from `tokio`/`futures`/`async-std`. This
project's exercise 29 stays sequential and dependency-free deliberately; a real async
project should add `tokio` rather than reinvent this.

## Comparison to C / C++ / Go

| Concern | C | C++ | Go | Rust |
|---|---|---|---|---|
| Concurrency primitive | None built in (raw threads/callbacks) | Coroutines (C++20, `co_await`), still minimal stdlib support | Goroutines — lightweight, scheduler built in | `async fn`/`.await` — compiles to a state machine |
| Runtime/scheduler | N/A | Not provided by std | Built into every Go binary, no choice | Not provided by std — pick tokio/async-std/smol, or write your own |
| Starts running when | N/A | On `co_await`/explicit resume | Immediately on `go f()` | Only when polled by an executor |
| Structured concurrency (`join`) | Manual | Manual, or via a library | `sync.WaitGroup`, or channels | `tokio::join!`/`futures::join!` (a crate, not std) |
