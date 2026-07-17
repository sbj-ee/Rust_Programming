# Rust Programming

A structured Rust programming learning project covering fundamentals through concurrent,
networked, and systems programming. Designed for someone who already knows C and/or C++
(see `../C_Programming` and `../CPP_Programming`) — and ideally has also seen `../Go_Programming`
— and wants to learn idiomatic Rust: ownership and borrowing instead of manual memory
management or garbage collection.

## Layout

```
Rust_Programming/
├── Cargo.toml       # Single package; every exercise is an explicit [[bin]] target
├── Introduction.md  # Dev environment, build tools, exercise progression
├── Foreword.md      # Rust's origins at Mozilla, ownership, fearless concurrency
├── exercises/       # Progressive programs, each building on the last
│   ├── 01_hello_world/ … 30_benchmarking/
└── topics/              # Markdown reference sheets by concept
    ├── 01_types/                    scalar/compound types, overflow, casts, const vs static
    ├── 02_ownership_and_borrowing/  moves, Copy, the aliasing rule, NLL — the core idea
    ├── 03_memory/                   stack vs heap, Drop/RAII, no GC, memory-safety bug table
    ├── 04_strings/                  String vs &str, UTF-8 enforcement, char vs byte
    ├── 05_structs_and_traits/       structs, impl blocks, explicit trait impls, dispatch
    ├── 06_error_handling/           Option/Result, ?, panic!, custom error types
    ├── 07_concurrency/              threads, Send/Sync, channels, Arc<Mutex<T>>
    ├── 08_generics/                 trait bounds, monomorphization, where clauses
    ├── 09_testing/                  #[test], table-driven tests, #[should_panic]
    ├── 10_file_io/                  fs, Read/Write traits, BufReader/BufWriter
    ├── 11_collections/              Vec, HashMap, BTreeMap, HashSet, VecDeque
    ├── 12_lifetimes/                annotations, elision, structs holding references
    ├── 13_smart_pointers/           Box, Rc, RefCell, Weak
    ├── 14_cargo_and_modules/        mod/pub/use, Cargo.toml, Cargo.lock, workspaces
    ├── 15_closures_and_iterators/   Fn/FnMut/FnOnce, lazy adapter chains
    ├── 16_unsafe_rust/              raw pointers, FFI, the 5 things unsafe unlocks
    ├── 17_macros/                   macro_rules!, fragment specifiers, hygiene
    ├── 18_async/                    Future, Poll/Waker, why std ships no executor
    └── 19_sockets_and_networking/   TcpListener/TcpStream, no built-in HTTP
```

## Build

```bash
# Build every exercise
make

# Build and run one exercise directly
cargo run --bin 07_slices_and_arrays

# Build one exercise to a binary
cargo build --bin 07_slices_and_arrays
./target/debug/07_slices_and_arrays

# Remove all build artifacts
make clean
```

## Requirements

- `rustc`/`cargo` 1.75+ (stable channel; nothing here needs nightly)
- No external dependencies — every exercise uses only the standard library, matching the
  rule `../Go_Programming` and `../CPP_Programming` follow

## Exercises

| # | Topic | Key Concepts |
|---|-------|--------------|
| 01 | Hello World | `fn main`, `println!`/`print!`/`eprintln!`, `cargo run` vs `cargo build` |
| 02 | Variables & Types | `let`/`mut`, shadowing, scalar/compound types, `const`, explicit `as` casts |
| 03 | Control Flow | `if`/`match` as expressions, `loop`/`while`/`for`, `break` with a value, labels |
| 04 | Functions | expression vs statement (semicolon rule), tuple returns, closures |
| 05 | Ownership | move semantics, `Copy`, `clone()`, scope-based `Drop` — no GC, no `free` |
| 06 | Borrowing & References | `&`/`&mut`, the aliasing rule, non-lexical lifetimes, no dangling refs |
| 07 | Slices & Arrays | `[T; N]` vs `&[T]`, bounds-checked indexing, `&str`/`String` preview |
| 08 | Structs | struct literals, `impl` blocks, `&self`/`&mut self`/`self`, tuple/unit structs |
| 09 | Enums & Pattern Matching | data-carrying enums, exhaustive `match`, `Option<T>`, `if let`/`while let` |
| 10 | Error Handling | `Result<T, E>`, `?` propagation, `unwrap`/`expect`, `panic!` |
| 11 | Traits | explicit `impl Trait for Type`, default methods, static vs `dyn` dispatch |
| 12 | Generics | trait bounds, monomorphization, `where` clauses, generic structs |
| 13 | Collections | `Vec`, `HashMap`, `HashSet`, `BTreeMap`, `VecDeque` |
| 14 | Closures & Iterators | `Fn`/`FnMut`/`FnOnce`, lazy adapter chains, zero-cost fusion |
| 15 | Lifetimes | `'a` annotations, elision rules, structs holding references |
| 16 | Smart Pointers | `Box`, `Rc`, `RefCell`, `Weak` — breaking reference cycles |
| 17 | Modules, Crates & Cargo | `mod`/`pub`/`use`, file-backed vs inline modules, `Cargo.toml` |
| 18 | Testing | `#[test]`, `assert_eq!`, table-driven tests, `#[should_panic]` |
| 19 | Threads | `thread::spawn`, `move` closures, `Send`/`Sync`, compile-time race prevention |
| 20 | Channels | `std::sync::mpsc`, multi-producer/single-consumer, `try_recv` |
| 21 | Shared State | `Arc<Mutex<T>>`/`Arc<RwLock<T>>`, `MutexGuard`, lock poisoning |
| 22 | File I/O | `std::fs`, `Read`/`Write`, `BufReader`/`BufWriter`, `io::Error` |
| 23 | Custom Error Types | `Display` + `std::error::Error`, `From` conversions, `Box<dyn Error>` |
| 24 | JSON From Scratch | a hand-written recursive-descent JSON parser — what `serde_json` automates |
| 25 | TCP Sockets | `TcpListener`/`TcpStream`, a concurrent echo server, no leaked file descriptors |
| 26 | CLI Args & Subprocesses | `env::args`/`env::var`, `process::Command`, piping stdin/stdout |
| 27 | Unsafe Rust | raw pointers, `extern "C"` FFI, mutable statics, safe abstractions over `unsafe` |
| 28 | Macros | `macro_rules!`, fragment specifiers, repetition, hygiene |
| 29 | Async/Await | `async fn`, a hand-rolled `block_on` executor, `Future`/`Poll`/`Waker` |
| 30 | Benchmarking & Profiling | a `std::time::Instant` micro-bench harness, `black_box`, why there's no built-in `cargo bench` |

---

## Appendix A: The Cargo.toml `[[bin]]` Layout

Unlike `C_Programming` and `CPP_Programming`, there is no per-exercise Makefile and no
linker flags to manage — and unlike `Go_Programming`'s `go build ./...` wildcard, Cargo does
not auto-discover binaries outside `src/bin/`. Each exercise is declared explicitly:

```toml
[[bin]]
name = "07_slices_and_arrays"
path = "exercises/07_slices_and_arrays/main.rs"
```

```makefile
.PHONY: all build test fmt clippy lint clean

all: build

build:
	cargo build

test:
	cargo test

fmt:
	cargo fmt --check

clippy:
	cargo clippy --all-targets -- -D warnings

lint: fmt clippy

clean:
	cargo clean
```

### Common invocations

```bash
make                                              # build everything
cargo build --bin 11_traits                        # build one exercise
cargo run --bin 11_traits                           # build+run without a separate step
make test                                          # cargo test (runs every #[cfg(test)] mod)
make clippy                                        # cargo clippy --all-targets -- -D warnings
cargo fmt                                           # apply formatting in place
make fmt                                           # list files that need formatting (--check)
```

---

## Appendix B: The Rust Toolchain (replacing valgrind / gdb / nm / objdump)

Rust compiles to native machine code and manages memory without a garbage collector — much
closer to C/C++'s toolchain story than Go's. Most of the classic C/C++ tools (gdb, valgrind,
nm, objdump) work on a Rust binary completely unchanged; Rust's toolchain adds a few tools
on top that C/C++ don't have.

### `cargo clippy` — replacing (and far exceeding) `-Wall -Wextra -Wpedantic`

```bash
cargo clippy --all-targets -- -D warnings
```

Clippy ships hundreds of lints across correctness, style, performance, and complexity —
broader than `go vet`'s fixed correctness-only checklist, and far more opinionated than any
GCC/Clang warning flag set. `-D warnings` promotes every lint hit to a hard error, the same
posture `-Werror` gives you in C/C++.

### `cargo fmt` — replacing `clang-format`

```bash
cargo fmt --check     # list files that would be reformatted
cargo fmt              # apply
```

One canonical style, configured (rarely) via `rustfmt.toml` — the same "stop arguing about
brace placement" philosophy as `gofmt`, applied to a language with a much larger grammar.

### `cargo test` / `cargo miri` — replacing manual test runners and adding UB detection

```bash
cargo test                    # runs every #[test] fn
cargo +nightly miri run       # interprets the program, flags undefined behavior in `unsafe` code
```

`miri` has no C/C++ analog with the same precision — ASan/UBSan catch a subset of UB at
runtime on real hardware; `miri` runs your program in an interpreter that tracks Rust's
specific aliasing and validity rules and flags violations even in code paths that happen not
to crash.

### `gdb`/`lldb` — unchanged from C/C++

```bash
rust-gdb ./target/debug/07_slices_and_arrays   # a gdb wrapper with Rust-aware pretty-printers
```

Rust uses OS threads (1:1, see exercise 19/topics/07), so there's no goroutine-aware `dlv`
equivalent needed the way `Go_Programming`'s appendix covers — a normal `gdb`/`lldb` session
already understands every thread in a Rust process the same way it understands a C++ one.

### `valgrind` — still works, because there's no GC to confuse it

```bash
valgrind --leak-check=full ./target/debug/16_smart_pointers
valgrind --tool=helgrind ./target/debug/21_shared_state
```

Leaks are rare in safe Rust (an `Rc`/`Arc` reference cycle is the main way to cause one, see
exercise 16) but not impossible, and `valgrind` finds them exactly as it would in C/C++.
Data races in *safe* Rust are compile errors (topics/07), so `helgrind` mostly has nothing to
find there — but `unsafe` code can still race, and `helgrind`/`miri` both remain useful.

### Inspecting the compiled binary

```bash
nm target/debug/07_slices_and_arrays | grep main   # symbol table — same tool as C/C++
objdump -d target/release/30_benchmarking            # disassemble — same tool as C/C++
cargo asm exercise_fn                                 # (external tool: cargo-asm) annotated per-function asm
cargo bloat --release                                  # (external tool) binary size broken down by function
strip target/release/30_benchmarking                   # remove debug symbols, same as any ELF/Mach-O binary
```

### Quick reference: C/C++ tool → Rust equivalent

| C / C++ tool | Purpose | Rust equivalent |
|---|---|---|
| `valgrind --leak-check=full` | find leaked heap blocks | Same tool, still works — leaks are rare but possible (`Rc` cycles) |
| `valgrind --tool=helgrind` / `drd` | detect data races | Compile error in safe code; `helgrind`/`miri` still useful for `unsafe` |
| `gdb`/`lldb` | interactive debugger | Same tools; `rust-gdb`/`rust-lldb` add Rust-aware pretty-printers |
| `nm` | list symbols | `nm` — unchanged |
| `objdump -d` | disassemble | `objdump -d` — unchanged |
| `clang-format` / style guide | enforce style | `cargo fmt` (rustfmt) — one canonical style |
| `-Wall -Wextra -Wpedantic` | compiler warnings | `cargo clippy -- -D warnings` — much broader lint set |
| ASan / UBSan | catch UB at runtime | `cargo miri` — catches UB in an interpreter, not just at runtime on real hardware |
| `ar`/`.a`, `.so` + `LD_LIBRARY_PATH` | static/shared libraries | `.rlib` (static, Rust-only) / `.so`/`.dylib` via `crate-type = ["dylib"]` |
| `strings` | find readable text in a binary | `strings` — unchanged, Rust binaries are still ELF/Mach-O |
