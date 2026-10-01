// Exercise 27: Unsafe Rust
//
// Demonstrates: raw pointers (`*const T`/`*mut T`), the operations
// `unsafe` unlocks, a minimal `extern "C"` FFI declaration, and why
// `unsafe` narrows the trust boundary instead of disabling the type system.
// This is where Rust's guarantees become something YOU promise the
// compiler, the same promise C makes on every line.

use std::sync::atomic::{AtomicI32, Ordering};

/// Reads the `i32` that is `index` elements past `ptr`.
///
/// `unsafe fn` — calling it is itself an unsafe operation; the signature
/// documents that the caller must uphold invariants the compiler can't check.
///
/// # Safety
///
/// The caller must guarantee that:
/// - `ptr` is non-null, properly aligned, and points into a single live
///   allocation of initialized `i32`s (e.g. an array or `Vec` buffer), and
/// - `ptr.add(index)` stays inside that same allocation — i.e. `index` is
///   less than the number of elements that follow `ptr`.
///
/// Violating either is undefined behavior, not a panic.
unsafe fn read_at(ptr: *const i32, index: usize) -> i32 {
    // Pointer arithmetic + dereference — this is exactly what C does on
    // every array access, with none of the bounds checking exercise 07 has.
    // `add(usize)` is the idiomatic form of `offset(isize)` for forward steps.
    *ptr.add(index)
}

// FFI: declaring a C function so Rust can call it. No `libc` crate needed —
// `abs` is provided by the platform's C runtime, already linked into any Rust binary.
// (In edition 2024 this block must be spelled `unsafe extern "C" { ... }`, making
// explicit that YOU vouch for these signatures matching the C side.)
extern "C" {
    fn abs(input: i32) -> i32;
}

// A mutable static — inherently unsafe to touch, because nothing stops two
// threads from writing it at once. Even taking a shared `&STATIC_COUNTER`
// (which `println!("{STATIC_COUNTER}")` does implicitly) is discouraged: the
// compiler's `static_mut_refs` lint flags it, since any later write while that
// reference lives is undefined behavior.
static mut STATIC_COUNTER: i32 = 0;

// The safe replacement for almost every `static mut`: an atomic. No `unsafe`
// needed, and concurrent increments from many threads are well-defined.
static ATOMIC_COUNTER: AtomicI32 = AtomicI32::new(0);

fn main() {
    println!("=== Exercise 27: Unsafe Rust ===");

    // Section 1: raw pointers can be created in safe code, only DEREFERENCED unsafely
    println!("\n--- Section 1: raw pointers ---");
    let value = 42;
    let raw_ptr: *const i32 = std::ptr::addr_of!(value); // creating a raw pointer is safe
    unsafe {
        println!("dereferenced raw_ptr: {}", *raw_ptr); // dereferencing requires unsafe
    }

    // Section 2: pointer arithmetic — no bounds checking, the caller's responsibility
    println!("\n--- Section 2: pointer arithmetic ---");
    let arr = [10, 20, 30, 40];
    let ptr = arr.as_ptr();
    unsafe {
        for i in 0..arr.len() {
            // SAFETY: `ptr` comes from `arr.as_ptr()` and `i < arr.len()`, so
            // every read stays inside `arr`'s initialized elements.
            print!("{} ", read_at(ptr, i));
        }
    }
    println!();
    println!(
        "(read_at(ptr, 10) would compile — and be undefined behavior: no bounds check exists)"
    );

    // Section 3: calling into C via FFI
    println!("\n--- Section 3: extern \"C\" FFI ---");
    unsafe {
        println!("abs(-7) via libc = {}", abs(-7));
    }

    // Section 4: mutable statics — a global the compiler cannot prove is race-free
    println!("\n--- Section 4: mutable statics (and the atomic alternative) ---");
    // Both READING and WRITING a `static mut` require `unsafe`. Copy the value
    // out (an i32 is Copy) instead of formatting the static directly, so no
    // reference to it is ever created.
    let snapshot = unsafe {
        STATIC_COUNTER += 1;
        STATIC_COUNTER += 1;
        STATIC_COUNTER
    };
    println!("STATIC_COUNTER = {snapshot}");
    println!("(mutating a `static mut` from multiple threads is a data race unsafe does NOT protect you from)");

    // The idiomatic fix: an atomic static — safe code, safe from any thread.
    let handles: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                ATOMIC_COUNTER.fetch_add(1, Ordering::Relaxed);
            })
        })
        .collect();
    for h in handles {
        h.join().expect("counter thread panicked");
    }
    println!(
        "ATOMIC_COUNTER after 4 threads = {} (no unsafe anywhere)",
        ATOMIC_COUNTER.load(Ordering::Relaxed)
    );

    // Section 5: split_at_mut — a safe API built on unsafe internals
    println!("\n--- Section 5: safe abstractions over unsafe code ---");
    let mut data = [1, 2, 3, 4, 5, 6];
    let (left, right) = data.split_at_mut(3); // the stdlib uses unsafe internally to hand out
    left[0] = 100; // two non-overlapping &mut slices — the borrow checker alone
    right[0] = 200; // cannot prove `left` and `right` don't alias, so the stdlib
    println!("left={left:?} right={right:?}"); // asserts it via a bounds check + unsafe, once, here

    println!("\nNotes:");
    println!(
        "  - unsafe unlocks a short, fixed list: deref raw pointers, call unsafe fn/extern fn,"
    );
    println!("    read or write a `static mut`, impl an unsafe trait, read a union field.");
    println!("    (Edition 2024 adds `unsafe extern` blocks and `unsafe` attributes.) Nothing else changes.");
    println!(
        "  - Prefer atomics, Mutex, or OnceLock over `static mut` — they need no unsafe at all."
    );
    println!("  - unsafe does NOT disable the borrow checker or turn off move semantics — those still apply.");
    println!("  - The goal of unsafe code is almost always to build a SAFE abstraction on top (see split_at_mut).");
    println!("  - `cargo +nightly miri run` (nightly-only tool) catches undefined behavior in unsafe code that a normal run won't.");
}
