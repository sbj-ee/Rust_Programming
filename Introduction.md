# Introduction

## Who This Is For

This project is for programmers who already know C and/or C++ (see `C_Programming` and
`CPP_Programming` in this monorepo) — and ideally have also seen `Go_Programming` — and want
to learn idiomatic Rust. If you can read a `struct`, follow a pointer, and use a Makefile,
you are ready. No prior Rust experience is assumed.

The exercises do not re-teach programming fundamentals from zero — they assume you already
understand variables, control flow, functions, and memory at a conceptual level. What they
teach is how Rust's central idea (ownership, checked entirely at compile time, with no
garbage collector and no manual `free`) changes the way you solve the same problems you
already know from C, C++, and possibly Go.

## Philosophy

Each exercise is a complete, runnable binary declared as its own `[[bin]]` target in
`Cargo.toml`. There are no stub files to fill in. Read the source, build it, run it, then
change something — in particular, try the things the comments say *won't* compile, and read
the compiler's error message. Understanding Rust comes largely from arguing with the borrow
checker and learning to read what it's telling you.

Comments explain the *why*, not the *what* — particularly where Rust's behavior surprises
someone coming from C/C++/Go (e.g., a moved-from variable becoming unusable, `Send`/`Sync`
turning a data race into a compile error, `Option<T>` replacing null entirely).

## Development Environment

```bash
# Toolchain — install via rustup (https://rustup.rs), not a package manager, so
# `rustup update` can manage stable/beta/nightly and components in one place
rustc --version      # 1.75+ required; nothing here needs nightly
cargo --version

# Formatter, linter, docs — installed as rustup "components"
cargo fmt --check     # list files that need formatting
cargo clippy --all-targets -- -D warnings   # static analysis, far broader than go vet/-Wall
cargo doc --open       # build and browse API docs, including for the standard library

# Optional but useful
rustup component add miri    # UB detector for unsafe code (exercise 27)
rust-gdb --version            # gdb with Rust-aware pretty-printers
```

## Building

```bash
# Build every exercise
make

# Build and run one exercise directly
cargo run --bin 09_enums_and_pattern_matching

# Build one exercise to a binary
cargo build --bin 09_enums_and_pattern_matching
./target/debug/09_enums_and_pattern_matching

# Run the test suite (exercise 17's greeting.rs, exercise 18, and any other #[cfg(test)] mods)
make test

# Format-check and lint everything
make lint

# Remove build artifacts
make clean
```

There is no per-exercise Makefile. Every exercise is declared as an explicit `[[bin]]`
target in the single project-root `Cargo.toml`, pointing at
`exercises/<NN_name>/main.rs` — see the README's Appendix A for why this needs to be
explicit here, unlike `Go_Programming`'s `go build ./...` wildcard.

## Exercise Progression

The 30 exercises form four progressive tiers.

### Tier 1 — Rust Fundamentals & Ownership (01–10)

Direct analogs to C/C++/Go concepts, with ownership — the concept that has no analog in any
of them — introduced as soon as the basics are in place.

| # | Topic | What's New Versus C/C++/Go |
|---|-------|------------------------------|
| 01 | Hello World | `fn main`, no header/prototype split, `cargo run` vs `cargo build` |
| 02 | Variables & Types | Immutable by default (`mut` opts in); shadowing; no implicit conversions, ever |
| 03 | Control Flow | `if`/`match`/`loop` are expressions; only `loop` yields a value via `break VALUE` |
| 04 | Functions | Tail expression (no semicolon) = return value; closures capture by ref/mut/value |
| 05 | Ownership | Every value has ONE owner; assignment moves (not copies) unless the type is `Copy` |
| 06 | Borrowing & References | One `&mut` XOR many `&`, enforced at compile time; no dangling references, ever |
| 07 | Slices & Arrays | `[T; N]` (length in the type) vs `&[T]` (a borrowed view); bounds-checked always |
| 08 | Structs | No `class`; methods in a separate `impl` block; `&self`/`&mut self`/`self` receivers |
| 09 | Enums & Pattern Matching | Enums carry data (tagged unions); `match` is exhaustive; `Option<T>` replaces null |
| 10 | Error Handling | `Result<T, E>` + `?`; not exceptions; `unwrap`/`expect`/`panic!` for the unrecoverable case |

### Tier 2 — Idiomatic Rust (11–18)

The features that make Rust distinct from a "C++ with a friendlier compiler."

| # | Topic | Core Idea |
|---|-------|-----------|
| 11 | Traits | EXPLICIT `impl Trait for Type` (no implicit satisfaction like Go); static vs `dyn` dispatch |
| 12 | Generics | Trait bounds + monomorphization — zero-cost, unlike Go's partial runtime dispatch |
| 13 | Collections | `Vec`, `HashMap`, `BTreeMap`, `HashSet`, `VecDeque` — explicit capacity/growth control |
| 14 | Closures & Iterators | `Fn`/`FnMut`/`FnOnce`; lazy adapter chains that compile to a hand-written loop |
| 15 | Lifetimes | `'a` annotations proving a reference never outlives its data — elided most of the time |
| 16 | Smart Pointers | `Box` (single owner), `Rc`/`Weak` (shared, cycle-safe), `RefCell` (runtime-checked mutability) |
| 17 | Modules, Crates & Cargo | `pub` required at every level (unlike Go's capitalization); `Cargo.toml`/`Cargo.lock` |
| 18 | Testing | `#[test]` + `#[cfg(test)]`, next to the code, like Go's `_test.go` but same-file |

### Tier 3 — Concurrency & Standard Library (19–26)

The same systems tasks as the POSIX exercises in `C_Programming`/`CPP_Programming` and the
`net`/`os`-based ones in `Go_Programming`, done with Rust's standard library.

| # | Topic | Standard Library |
|---|-------|-------------------|
| 19 | Threads | `std::thread` — OS threads (1:1), data races are a COMPILE error via `Send`/`Sync` |
| 20 | Channels | `std::sync::mpsc` — Rust's `chan`, minus a built-in multi-channel `select` |
| 21 | Shared State | `Arc<Mutex<T>>`/`Arc<RwLock<T>>` — "share memory by locking it," compiler-enforced |
| 22 | File I/O | `std::fs`, `Read`/`Write` traits, `BufReader`/`BufWriter` |
| 23 | Custom Error Types | `Display` + `std::error::Error` + `From` — the boilerplate `thiserror` normally generates |
| 24 | JSON From Scratch | A hand-written parser — what `serde_json` + derive macros automate in a real project |
| 25 | TCP Sockets | `std::net::{TcpListener, TcpStream}` — sockets close automatically on `Drop` |
| 26 | CLI Args & Subprocesses | `std::env`, `std::process::Command` — the Rust analog of `os/exec`/`fork`+`exec` |

### Tier 4 — Advanced Rust (27–30)

| # | Topic | Core Idea |
|---|-------|-----------|
| 27 | Unsafe Rust | Raw pointers, `extern "C"` FFI, mutable statics — the 5 things `unsafe` unlocks, nothing more |
| 28 | Macros | `macro_rules!` — hygienic, syntax-aware, unlike C's `#define` text substitution |
| 29 | Async/Await | `async fn`/`.await`, and why `std` ships `Future` but deliberately no executor |
| 30 | Benchmarking & Profiling | A hand-rolled `Instant`-based harness — why stable Rust has no built-in `cargo bench` |

## Key Differences from C_Programming / CPP_Programming / Go_Programming

| Concern | C | C++ | Go | Rust |
|---------|---|-----|-----|------|
| Memory | `malloc`/`free` | RAII + smart pointers | Garbage collected | Ownership, compile-time checked; `Drop` on scope exit |
| Null safety | `NULL`, unchecked | `nullptr`, unchecked | `nil`, typed-nil pitfall | No null — `Option<T>`, exhaustively matched |
| Generics | `void *` + macros | Templates | Type parameters (1.18+), partial runtime dispatch | Type parameters + trait bounds, fully monomorphized |
| Polymorphism | Function pointer tables | `virtual`/`override` | Implicit interfaces | Traits, EXPLICIT `impl Trait for Type` |
| Error handling | Return codes + `errno` | Exceptions | `error` return values | `Result<T, E>` + `?`, `#[must_use]`-enforced |
| Concurrency | `pthread_t` + mutexes | `std::thread` + mutexes | Goroutines + channels | OS threads; data races are COMPILE errors |
| Build | Makefile + linker flags | Makefile + linker flags | `go build ./...`, no linker flags | `cargo build`, explicit `[[bin]]` per exercise here |
| Formatting | Style guide (`clang-format`) | Style guide (`clang-format`) | `gofmt` — one true format | `cargo fmt` (rustfmt) — one true format |
| Linting | `-Wall -Wextra -Wpedantic` | same, plus clang-tidy | `go vet` (correctness only) | `cargo clippy` (correctness + style + perf, huge lint set) |
| Compilation | Object files + linking | Object files + linking | Single static binary, no `.so` needed | Native binary; static by default, same linker model as C/C++ |

## Every File Has

- A single `main.rs` with `fn main()` and at least four named sections
- Output produced by running it — `cargo run --bin NN_topic`
- Zero warnings from `cargo clippy --all-targets -- -D warnings`
- Formatting that matches `cargo fmt` exactly (enforced, not a style suggestion)
