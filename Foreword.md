# Foreword

## The Language and Its Origins

In 2006, Graydon Hoare started Rust as a personal project. Mozilla began sponsoring it in
2009, and by 2010 it was a public, corporate-backed effort with a specific, urgent purpose:
Firefox's C++ codebase — millions of lines old, deeply concurrent, and full of the exact bug
classes that dominate C/C++ CVE histories — needed a systems language that could be as fast
as C++ without inheriting its memory-safety problems. The project that eventually shipped
some of that work, Servo (an experimental browser engine), was Rust's proving ground: could
a language with no garbage collector make "fearless concurrency" — writing multithreaded
code without the constant fear of a data race — actually true, not just aspirational?

Rust 1.0 shipped in May 2015, with its own compatibility promise: code written for 1.0 still
compiles on modern Rust, edition changes (2015 → 2018 → 2021 → 2024) are opt-in via
`Cargo.toml`, and old editions keep compiling indefinitely. Unlike Go's single, unversioned
language spec, Rust's edition system lets the language evolve syntax while every existing
crate keeps building.

> *"Rust is a systems programming language that runs blazingly fast, prevents segfaults,
> and guarantees thread safety."*
> — The Rust Book's opening line, and the whole design brief in one sentence

## Ownership: The Idea With No Precedent in C, C++, or Go

Every other language in this monorepo picks one of two strategies for memory: manual
management (C, and C++ without discipline) or garbage collection (Go). Rust picks a third
option nobody had shipped at this scale before: an **ownership system**, checked entirely at
compile time, that tracks who is responsible for freeing every value and proves — before the
program ever runs — that nothing is used after it's freed, freed twice, or mutated while
someone else is reading it.

```rust
let s1 = String::from("hello");
let s2 = s1;             // ownership MOVES to s2
// println!("{s1}");     // compile error, not a runtime bug waiting to happen
```

C++ RAII (smart pointers, destructors) gets close to this discipline through *convention* —
a disciplined C++ programmer avoids most of these bugs, but the compiler doesn't enforce it,
and one raw `new`/`delete` or dangling reference anywhere in a large codebase breaks the
invariant silently. Rust's borrow checker makes the discipline **mandatory** and checks it
mechanically, for every line, every time.

## Fearless Concurrency

The payoff for the ownership system's upfront cost shows up hardest in concurrent code. In
C and C++, a data race is a runtime bug: two threads touch the same memory, at least one
writes, nothing serializes them, and the result is undefined behavior you might not observe
until production, at scale, under load. In Rust, the same setup is a **compile error** — the
`Send`/`Sync` marker traits (topics/07) extend the ownership and borrowing rules across
thread boundaries, so the compiler rejects code that could race before it ever runs.

> *"Fearless concurrency" means Rust enables you to write code that's free of subtle bugs
> and is easy to refactor without introducing new ones.*
> — The Rust Book

Go's goroutines + channels make concurrent code *easier to write*; Rust's ownership +
`Send`/`Sync` make concurrent code *provably correct*, at the cost of a stricter compiler you
must satisfy before your program runs at all.

## Zero-Cost Abstractions

Rust's other governing principle, inherited directly from C++: **what you don't use, you
don't pay for; what you do use, you couldn't hand-code any better.** Iterators (exercise 14)
compile to the same tight loop as a hand-written index loop. Generics (exercise 12) are
monomorphized — a fully specialized, separately compiled copy per concrete type — with no
runtime dispatch cost, the same tradeoff C++ templates make and a stronger guarantee than
Go's generics, which still involve some runtime dictionary-passing in cases the compiler
can't fully specialize.

## No Exceptions, No Null

Two more decisions this project's exercises lean on constantly: Rust has no exceptions
(`Result<T, E>` and `?`, exercise 10, cover recoverable errors the way Go's error-return
convention does, but made *exhaustive* by the type system) and no null (`Option<T>` replaces
it everywhere, exercise 09, closing off an entire category of bug C's `NULL` and even Go's
typed-nil-in-an-interface footgun leave open).

## The Toolchain Is Also the Language

Rust ships `rustfmt`, `clippy`, `cargo test`, `cargo doc`, and a dependency/build system
(`Cargo`) in the box, the same "no argument about which build system this project uses"
philosophy Go's toolchain has — applied to a language with a much larger surface area than
Go's deliberately small one. Where `Go_Programming` needed no per-exercise Makefile because
`go build ./...` auto-discovers every `package main`, this project needs an explicit
`[[bin]]` entry per exercise in `Cargo.toml` (Appendix A of the README) because Cargo's
auto-discovery only looks in `src/bin/` — a small but real difference in how much the two
toolchains assume about your project layout.

## Rust, C, C++, and Go Together

This project is a companion to `C_Programming`, `CPP_Programming`, and `Go_Programming` in
this monorepo. Where `C_Programming` explores manual memory management and `CPP_Programming`
explores RAII and zero-cost abstraction with a garbage-collector-free runtime, and
`Go_Programming` explores the same systems topics with a garbage collector and structural
interfaces, this project explores them through an ownership system that gets you C++'s
performance profile with compile-time-proven memory and thread safety — at the cost of a
steeper initial learning curve than any of the other three. The POSIX-adjacent exercises —
sockets, subprocesses, file I/O — reappear here using `std::net`, `std::process`, and
`std::fs`, so you can compare, line for line, how the same problem looks when the compiler
itself is doing the bookkeeping C leaves to you and the discipline C++ leaves to convention.

> *"The Rust compiler ... plays gatekeeper by refusing to compile code with these elusive
> bugs, including concurrency bugs. By working alongside the compiler, the team can spend
> their time focusing on the program's logic rather than chasing down bugs."*
> — The Rust Book

## What This Project Is

Every exercise is a complete, runnable binary — no stub files. Build it, run it, read the
output, then change something and see what the compiler says. The comments explain *why* a
piece of code is idiomatic Rust — particularly where its behavior differs from what a C,
C++, or Go programmer would expect — not *what* the syntax does; the code itself, and
`cargo doc`, already tell you that.

> *"Rust's ownership rules ... give you memory safety guarantees without needing a garbage
> collector, so it's important to understand how ownership works."*
> — The Rust Book
