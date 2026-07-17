# Unsafe Rust — Cheat Sheet

`unsafe` doesn't disable the borrow checker, move semantics, or type checking — all of that
still applies. It unlocks exactly five additional operations the compiler cannot verify are
safe on its own, and asks *you* to uphold the invariants instead.

## The Five Things `unsafe` Unlocks

```rust
unsafe {
    let x = *raw_ptr;                 // 1. dereference a raw pointer
    some_unsafe_fn();                  // 2. call an unsafe fn (or extern "C" fn)
    STATIC_MUT += 1;                    // 3. mutate a mutable static
    // 4. implement an unsafe trait (e.g. `unsafe impl Send for MyType {}`)
    // 5. access a union field
}
```

## Raw Pointers

```rust
let value = 42;
let ptr: *const i32 = &value;        // CREATING a raw pointer is safe
unsafe { println!("{}", *ptr); }      // DEREFERENCING it requires unsafe

let arr = [10, 20, 30];
let p = arr.as_ptr();
unsafe { *p.offset(1) };               // pointer arithmetic — no bounds check, your responsibility
```

Unlike a Rust reference (`&T`), a raw pointer (`*const T`/`*mut T`) can be null, dangling, or
unaligned, and the compiler tracks none of it — this is exactly a C pointer, with C's
sharp edges restored deliberately, in an explicitly marked block.

## FFI — Calling Into C

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}
unsafe { abs(-7) };   // calling any extern fn is unsafe — its safety contract is outside Rust
```

The Rust analog of a C `extern` declaration or a Go `cgo` call — `unsafe` here reflects that
the compiler has no way to verify the C side upholds any invariant at all.

## Mutable Statics

```rust
static mut COUNTER: i32 = 0;
unsafe { COUNTER += 1; }
```

A `static mut` is a global variable the compiler cannot prove is race-free — mutating it
from multiple threads without synchronization is a data race `unsafe` does **not** protect
you from; it just marks that you, not the compiler, are now responsible.

## The Real Purpose: Building Safe Abstractions

```rust
let mut data = [1, 2, 3, 4, 5, 6];
let (left, right) = data.split_at_mut(3);   // two non-overlapping &mut slices
left[0] = 100;
right[0] = 200;
```

The standard library's own `split_at_mut` uses `unsafe` internally (it must — the borrow
checker alone cannot prove two slices from the same array don't overlap) but exposes a
**100% safe** public API. This is the idiomatic pattern: push `unsafe` to the smallest
possible internal surface, verify its invariants by hand once, and let everyone else use a
safe wrapper. Most Rust programmers write `unsafe` rarely, if ever.

## Undefined Behavior Still Exists — `unsafe` Just Moves the Burden

Violating an `unsafe` invariant (dereferencing a dangling pointer, creating two `&mut`
references to the same data via raw pointers, reading uninitialized memory) is undefined
behavior — exactly as bad as in C, and the compiler is free to assume it never happens,
which can produce surprising miscompilations far from the actual bug.

```bash
cargo miri run     # (nightly tool) interprets your program and flags UB in unsafe code
                     # that a normal run might not visibly break on — the closest Rust
                     # equivalent to valgrind's Memcheck, but UB-aware, not just leak-aware
```

## Comparison to C / C++

| Concern | C | C++ | Rust (safe) | Rust (`unsafe`) |
|---|---|---|---|---|
| Pointer dereference | Always allowed, unchecked | Always allowed, unchecked | Not applicable — no raw pointers | Allowed, unchecked — your responsibility |
| Bounds checking | None | None (unless `.at()`) | Always (`Vec`/slice indexing panics) | None if you use raw pointer arithmetic |
| Marking risky code | Not distinguished from safe code | Not distinguished from safe code | N/A | Explicitly delimited by an `unsafe` block/fn |
| UB detection tools | valgrind, ASan, UBSan | valgrind, ASan, UBSan | Not needed (UB impossible in safe code) | `cargo miri`, plus the C/C++ tools still work |
