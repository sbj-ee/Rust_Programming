// Exercise 27: Unsafe Rust
//
// Demonstrates: raw pointers (`*const T`/`*mut T`), the five things
// `unsafe` unlocks, a minimal `extern "C"` FFI declaration, and why
// `unsafe` narrows the trust boundary instead of disabling the type system.
// This is where Rust's guarantees become something YOU promise the
// compiler, the same promise C makes on every line.

// `unsafe fn` — calling it is itself an unsafe operation; the signature
// documents that the caller must uphold some invariant the compiler can't check.
unsafe fn read_at(ptr: *const i32, index: isize) -> i32 {
    // Pointer arithmetic + dereference — this is exactly what C does on
    // every array access, with none of the bounds checking exercise 07 has.
    *ptr.offset(index)
}

// FFI: declaring a C function so Rust can call it. No `libc` crate needed —
// `abs` is provided by the platform's C runtime, already linked into any Rust binary.
extern "C" {
    fn abs(input: i32) -> i32;
}

static mut COUNTER: i32 = 0; // a mutable static — inherently unsafe to touch

fn main() {
    println!("=== Exercise 27: Unsafe Rust ===");

    // Section 1: raw pointers can be created in safe code, only DEREFERENCED unsafely
    println!("\n--- Section 1: raw pointers ---");
    let value = 42;
    let raw_ptr: *const i32 = &value; // creating a raw pointer is safe
    unsafe {
        println!("dereferenced raw_ptr: {}", *raw_ptr); // dereferencing requires unsafe
    }

    // Section 2: pointer arithmetic — no bounds checking, the caller's responsibility
    println!("\n--- Section 2: pointer arithmetic ---");
    let arr = [10, 20, 30, 40];
    let ptr = arr.as_ptr();
    unsafe {
        for i in 0..arr.len() as isize {
            print!("{} ", read_at(ptr, i));
        }
    }
    println!();
    println!("(read_at(ptr, 10) would compile and likely crash or return garbage — no bounds check exists)");

    // Section 3: calling into C via FFI
    println!("\n--- Section 3: extern \"C\" FFI ---");
    unsafe {
        println!("abs(-7) via libc = {}", abs(-7));
    }

    // Section 4: mutable statics — a global the compiler cannot prove is race-free
    println!("\n--- Section 4: mutable statics ---");
    unsafe {
        COUNTER += 1;
        COUNTER += 1;
        println!("COUNTER = {COUNTER}");
    }
    println!("(mutating a `static mut` from multiple threads is a data race unsafe does NOT protect you from)");

    // Section 5: split_at_mut — a safe API built on unsafe internals
    println!("\n--- Section 5: safe abstractions over unsafe code ---");
    let mut data = [1, 2, 3, 4, 5, 6];
    let (left, right) = data.split_at_mut(3); // the stdlib uses unsafe internally to hand out
    left[0] = 100; // two non-overlapping &mut slices — the borrow checker alone
    right[0] = 200; // cannot prove `left` and `right` don't alias, so the stdlib
    println!("left={left:?} right={right:?}"); // asserts it via a bounds check + unsafe, once, here

    println!("\nNotes:");
    println!("  - unsafe unlocks exactly 5 things: deref raw pointers, call unsafe fn/extern fn,");
    println!(
        "    mutate a static, impl an unsafe trait, access a union field. Nothing else changes."
    );
    println!("  - unsafe does NOT disable the borrow checker or turn off move semantics — those still apply.");
    println!("  - The goal of unsafe code is almost always to build a SAFE abstraction on top (see split_at_mut).");
    println!("  - `cargo miri run` (nightly tool) catches undefined behavior in unsafe code that a normal run won't.");
}
