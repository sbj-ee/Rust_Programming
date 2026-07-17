# Ownership & Borrowing — Cheat Sheet

The one concept with no direct analog in C, C++, or Go. Everything else in Rust — no GC, no
manual `free`, "fearless concurrency" — falls out of these rules.

## The Three Rules

1. Every value has exactly one owner.
2. When the owner goes out of scope, the value is dropped.
3. Ownership can be moved (transferred) or borrowed (temporarily lent), never duplicated
   silently.

```rust
let s1 = String::from("hello");
let s2 = s1;          // MOVES the heap allocation to s2
// println!("{s1}");  // COMPILE ERROR: value borrowed after move
```

Unlike C++ copy constructors (implicit, silent, can be expensive) or Go (GC tracks all of
this for you), Rust makes the transfer explicit at compile time and free at runtime — a move
is a bitwise copy of the stack representation (pointer/len/cap for a `String`), and the
compiler statically forbids using the old binding again.

## `Copy` Types Don't Move

```rust
let x = 5;    // i32: Copy
let y = x;    // copies, doesn't move — both x and y are valid
```

Types are `Copy` if they're cheap, fixed-size, and stack-only: integers, floats, `bool`,
`char`, and tuples/arrays of `Copy` types. Anything owning a heap allocation (`String`,
`Vec<T>`, `Box<T>`) cannot be `Copy`.

## Borrowing: `&T` and `&mut T`

```rust
fn len(s: &String) -> usize { s.len() }        // borrows — caller keeps ownership
fn shout(s: &mut String) { s.push_str("!"); }   // mutable borrow — can modify

let s = String::from("hi");
len(&s);           // s still usable after
let mut s2 = String::from("hi");
shout(&mut s2);    // s2 still usable, and now modified
```

## The Aliasing Rule (Enforced at Compile Time)

At any point, for a given value: **one mutable reference, OR any number of immutable
references — never both.**

```rust
let mut v = vec![1, 2, 3];
let r1 = &v;
let r2 = &v;          // fine: multiple immutable borrows
println!("{r1:?} {r2:?}");
// r1, r2's last use was above — NLL ends their borrow there
let r3 = &mut v;      // fine NOW — no immutable borrow is still active
r3.push(4);
```

This single rule is what makes data races impossible in safe Rust: a race requires two
threads touching the same memory with at least one write, and the aliasing rule forbids a
live mutable reference from coexisting with any other reference at all — including across
threads (enforced there via `Send`/`Sync`, see topics/07_concurrency).

## Non-Lexical Lifetimes (NLL)

A borrow ends at its **last use**, not at the end of the enclosing block — the compiler
does real liveness analysis, not naive scope-based tracking. This is why the `r1`/`r2`/`r3`
example above compiles: `r1` and `r2` are dead (never used again) by the time `r3` is
created.

## Why Dangling References Are Impossible

```rust
fn dangle() -> &String {
    let s = String::from("gone");
    &s   // COMPILE ERROR: `s` is dropped at the end of this function;
}        // the reference would outlive its data — rejected before it can run
```

C happily returns `&local_var` and lets you find out at runtime (if you're lucky — a crash;
if you're unlucky, silent memory corruption). Go's escape analysis would just move `s` to
the heap and let the GC keep it alive. Rust's borrow checker instead proves, statically,
that this can never happen — the function must return an owned value instead.

## Comparison to C / C++ / Go

| Concern | C | C++ | Go | Rust |
|---|---|---|---|---|
| Dangling pointer/reference | Silent UB | Silent UB (unless using smart pointers carefully) | Impossible — GC keeps referents alive | Compile error — the borrow checker rejects it |
| Data races | Your problem | Your problem (`std::mutex` if you remember) | Possible; `go run -race` finds them at runtime | Compile error in safe code — Send/Sync + the aliasing rule |
| Use-after-free | Silent UB | Silent UB (mitigated by RAII/smart pointers) | Impossible — GC | Compile error — moved-from values can't be used |
| Cost of the safety | None (you get none) | RAII: near-zero; smart pointers: refcount overhead | GC pause/overhead | Zero at runtime — all checking is at compile time |
