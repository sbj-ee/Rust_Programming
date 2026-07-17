# Testing — Cheat Sheet

## The Basics

```rust
fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]           // compiled ONLY when testing — adds nothing to `cargo build`
mod tests {
    use super::*;        // pull in everything from the enclosing scope

    #[test]
    fn add_basic() {
        assert_eq!(add(2, 3), 5);
    }
}
```

Run with `cargo test`. Tests live right next to the code they test, gated by
`#[cfg(test)]`, unlike Go's separate `_test.go` files or C++'s separate test binary — though
Rust *also* supports separate integration-test files for library crates (see below).

## Assertion Macros

```rust
assert!(1 + 1 == 2);                          // any bool expression
assert_eq!(2 + 2, 4);                          // equality; prints BOTH sides on failure
assert_ne!(2 + 2, 5);                          // inequality
assert_eq!(a, b, "custom message: {a} vs {b}"); // optional formatted message
```

`assert_eq!`/`assert_ne!` print both operands on failure (requires `Debug`), which is
usually more useful than a bare `assert!(a == b)` — you see the actual values without
adding your own `println!` first.

## Table-Driven Tests

```rust
#[test]
fn palindrome_cases() {
    let cases = [("racecar", true), ("hello", false), ("", true)];
    for (input, expected) in cases {
        assert_eq!(is_palindrome(input), expected, "input was {input:?}");
    }
}
```

The idiomatic replacement for a hand-rolled test-table runner (same idea as Go's
table-driven tests with `t.Run` subtests) — one `#[test]` fn, a loop, and a per-case
message so a failure tells you which row broke.

## `#[should_panic]`

```rust
#[test]
#[should_panic(expected = "division by zero")]
fn divide_by_zero_panics() {
    divide(1, 0);
}
```

Asserts the function panics — and with the `expected` substring present in the panic
message. Without `#[should_panic]`, a panicking test is a *failure*, not a pass.

## Integration Tests (Library Crates)

```
my_crate/
├── src/lib.rs
└── tests/
    └── some_integration_test.rs   # compiled as its own crate, only sees `pub` items
```

Each file in `tests/` is a separate crate that links against your library's public API only
— this is Rust's "black box" integration test tier, analogous to a separate test binary in
C/C++ or a package_test in Go. This project's exercises are binaries (`[[bin]]` targets, not
a `[lib]`), so integration-style `tests/` files aren't used here — `#[cfg(test)] mod tests`
inside each `main.rs` covers unit-level testing.

## Other Useful Flags

```bash
cargo test                      # run everything
cargo test add_basic            # run tests whose name contains "add_basic"
cargo test -- --nocapture       # show println! output even for passing tests
cargo test -- --test-threads=1  # run serially (tests run in parallel by default)
```

## Comparison to C / C++ / Go

| Concern | C | C++ | Go | Rust |
|---|---|---|---|---|
| Built-in framework | None (roll your own, or use a library) | None in the language; libraries (Catch2, gtest) | `testing` package, built in | `#[test]`, built in |
| Test location | Separate files/binary | Separate files/binary | `_test.go` files, same package | `#[cfg(test)] mod tests`, same file (unit); `tests/` (integration) |
| Table-driven pattern | Manual loop over a struct array | Manual, or parameterized tests (gtest) | `t.Run` subtests over a table | Plain loop with `assert_eq!` per case |
| Parallel by default | No | Depends on the framework | No (sequential unless `t.Parallel()`) | Yes — tests run in parallel by default |
