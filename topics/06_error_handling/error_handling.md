# Error Handling — Cheat Sheet

## `Option<T>` — No Null

```rust
enum Option<T> { Some(T), None }

fn find(v: &[i32], target: i32) -> Option<usize> {
    v.iter().position(|&x| x == target)
}

match find(&[1, 2, 3], 2) {
    Some(i) => println!("found at {i}"),
    None => println!("not found"),
}
```

There is no null pointer/reference in safe Rust. Anywhere another language would use `null`,
`nil`, or a sentinel value, Rust uses `Option<T>` — and the compiler forces you to handle
the `None` case, because `match` must be exhaustive.

## `Result<T, E>` — Recoverable Errors as Values

```rust
fn parse(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}

match parse("42") {
    Ok(n) => println!("{n}"),
    Err(e) => println!("error: {e}"),
}
```

Same philosophy as Go's `(value, error)` return convention, but made exhaustive: you cannot
accidentally ignore the `Err` case the way you can forget to check Go's `err != nil` — using
a `Result` without handling it at all produces a compiler warning (`#[must_use]`).

## `?` — Propagation, Not Exceptions

```rust
fn sum_all(inputs: &[&str]) -> Result<i32, std::num::ParseIntError> {
    let mut total = 0;
    for s in inputs {
        total += s.parse::<i32>()?;   // Err returns immediately; Ok unwraps the value
    }
    Ok(total)
}
```

`?` is sugar for `match result { Ok(v) => v, Err(e) => return Err(e.into()) }` — it is
visible control flow at the call site (like Go's `if err != nil { return err }`), not a
hidden stack unwind (like a C++ `throw`). The `.into()` means `?` can cross error types as
long as a `From` conversion exists (see topics below and exercise 23).

## `unwrap` / `expect` — For "This Can't Fail"

```rust
let n = parse("42").unwrap();                        // panics with a generic message on Err
let n2 = parse("42").expect("literal should parse");  // panics with YOUR message on Err
```

Use these only when failure indicates a bug, not an expected runtime condition — a
prototype, a test, or a value you've already validated. Routine, expected failure belongs
in a `Result` your caller handles, not an `unwrap()`.

## `panic!` — Unrecoverable Errors

```rust
panic!("invariant violated: {reason}");
```

Unwinds the stack (running `Drop` impls along the way, by default), prints the message and
location, and aborts the thread (the whole process, if it was the main thread, or if
`panic = "abort"` is set in `Cargo.toml`). This is Rust's analog of C++'s uncaught exception
or Go's unrecovered panic — for programmer errors, not routine control flow.

## Custom Error Types

```rust
#[derive(Debug)]
enum ConfigError { MissingField(String), InvalidNumber(std::num::ParseIntError) }

impl std::fmt::Display for ConfigError { /* ... */ }
impl std::error::Error for ConfigError {}
impl From<std::num::ParseIntError> for ConfigError {
    fn from(e: std::num::ParseIntError) -> Self { ConfigError::InvalidNumber(e) }
}
```

`Debug` + `Display` + `std::error::Error` is the trio that makes a type a "proper" Rust
error. `impl From<X> for MyError` is what makes `?` able to convert `X` into `MyError`
automatically. See exercise 23 for the full pattern, and note that real projects usually
reach for the `thiserror` crate (generates this boilerplate) and `anyhow` (a ready-made
`Box<dyn Error>`-like catch-all with `.context()`) rather than hand-writing it every time.

## Comparison to C / C++ / Go

| Concern | C | C++ | Go | Rust |
|---|---|---|---|---|
| Recoverable errors | Return code + `errno` | Exceptions (`throw`/`catch`) | `error` return value | `Result<T, E>` return value |
| Can you ignore an error? | Yes, silently | Yes, if uncaught it terminates | Yes, silently (`_ = err`) | Compiler warns (`#[must_use]`) |
| Propagation | Manual `if (err) return err;` | Automatic stack unwind | Manual `if err != nil { return err }` | `?` — visible, one character |
| Null/missing value | `NULL` (a valid, dereferenceable-looking pointer) | `nullptr` | `nil` | No null — `Option<T>`, exhaustively matched |
| Unrecoverable errors | `abort()` | uncaught exception | `panic()` | `panic!()` |
