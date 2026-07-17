// Exercise 05: Ownership
//
// Demonstrates: the rule that has no analog in C, C++, or Go — every value
// has exactly one owner, and when the owner goes out of scope, the value is
// dropped. Move semantics, the `Copy` trait, and `clone()`.

#[derive(Debug)]
struct Document {
    title: String,
}

// Taking `Document` by value TAKES OWNERSHIP — the caller can no longer use it.
fn consume(doc: Document) {
    println!("consumed: {}", doc.title);
} // `doc` is dropped here — its memory is freed, deterministically, no GC involved

// Returning a value MOVES ownership out to the caller.
fn create(title: &str) -> Document {
    Document {
        title: title.to_string(),
    }
}

fn main() {
    println!("=== Exercise 05: Ownership ===");

    // Section 1: move semantics for heap-backed types
    println!("\n--- Section 1: moves ---");
    let s1 = String::from("hello"); // s1 owns this heap allocation
    let s2 = s1; // the allocation MOVES to s2 — s1 is no longer valid
                 // println!("{s1}"); // COMPILE ERROR: value borrowed after move
    println!("s2={s2}");
    println!("(s1 is no longer usable — this is a compile-time check, not a runtime one)");

    // Section 2: Copy types don't move — they're duplicated instead
    println!("\n--- Section 2: Copy types ---");
    let x = 5; // i32 implements Copy: cheap, fixed-size, stack-only
    let y = x; // this COPIES, it does not move
    println!(
        "x={x} y={y} (both still valid — integers, bools, chars, and tuples of these are Copy)"
    );

    // Section 3: clone() for an explicit, possibly expensive duplicate
    println!("\n--- Section 3: clone ---");
    let original = String::from("duplicate me");
    let cloned = original.clone(); // deep copy of the heap buffer, opt-in and visible
    println!("original={original} cloned={cloned} (both valid — clone left original intact)");

    // Section 4: ownership through function calls
    println!("\n--- Section 4: ownership and functions ---");
    let doc = create("Q3 Report");
    consume(doc); // ownership moves into consume()
                  // println!("{doc:?}"); // COMPILE ERROR: doc was moved into consume()
    println!("(doc was moved into consume() and dropped when that call returned)");

    // Section 5: scope-based drop, deterministic and visible
    println!("\n--- Section 5: scope-based Drop ---");
    struct Noisy(&'static str);
    impl Drop for Noisy {
        fn drop(&mut self) {
            println!("dropping {}", self.0);
        }
    }
    {
        let _a = Noisy("first");
        let _b = Noisy("second");
        println!("end of inner scope, about to drop in REVERSE declaration order");
    } // _b drops, then _a — deterministic, no GC pause, no finalizer queue

    println!("\nNotes:");
    println!("  - Every value has exactly one owner; assignment MOVES it unless the type is Copy.");
    println!("  - A moved-from binding is a compile error to use — Go/C++ would let you touch stale data.");
    println!("  - `Drop` runs deterministically at end of scope, in reverse declaration order.");
    println!("  - This is what 'no garbage collector, no manual free, no use-after-free' means in practice.");
}
