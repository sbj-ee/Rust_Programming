// Exercise 07: Slices & Arrays
//
// Demonstrates: fixed-size arrays `[T; N]`, slices `&[T]` as a view into
// contiguous memory (array, Vec, or another slice), and the `&str`/`String`
// relationship, which is exactly the array/slice relationship applied to text.

fn largest(values: &[i32]) -> i32 {
    // &[i32] accepts a slice of ANY length, backed by an array, Vec, or another
    // slice — this is the Rust analog of a Go slice parameter or a C
    // (pointer, length) pair, but bounds-checked and borrow-checked.
    let mut max = values[0];
    for &v in values {
        if v > max {
            max = v;
        }
    }
    max
}

fn main() {
    println!("=== Exercise 07: Slices & Arrays ===");

    // Section 1: fixed-size arrays — the length is part of the type
    println!("\n--- Section 1: arrays ---");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a={a:?} len={} type=[i32; 5]", a.len());
    // let b: [i32; 5] = a; // arrays of Copy types are themselves Copy — this copies

    // Section 2: slicing an array with a range
    println!("\n--- Section 2: slicing ---");
    let middle = &a[1..4]; // a view: elements at indices 1,2,3 — does not copy
    println!("&a[1..4] = {middle:?}");
    println!(
        "&a[..2] = {:?}  &a[3..] = {:?}  &a[..] = {:?}",
        &a[..2],
        &a[3..],
        &a[..]
    );

    // Section 3: out-of-bounds access panics, it does not read adjacent memory
    println!("\n--- Section 3: bounds checking ---");
    // let oops = a[10]; // PANIC at runtime: "index out of bounds" — never silent UB
    println!("(indexing out of range panics with a message — no silent buffer overread)");
    println!(
        "get(10) returns an Option instead of panicking: {:?}",
        a.get(10)
    );
    println!("get(2) = {:?}", a.get(2));

    // Section 4: Vec — the growable counterpart, slices work on it identically
    println!("\n--- Section 4: Vec and slices ---");
    let v: Vec<i32> = vec![9, 3, 7, 1, 8];
    println!("v={v:?} largest(&v)={}", largest(&v)); // &Vec<T> coerces to &[T]
    println!("largest(&a)={}", largest(&a)); // &[T; N] coerces to &[T] too — same function, no overload needed

    // Section 5: &str is a slice of UTF-8 bytes; String owns a growable buffer of them
    println!("\n--- Section 5: &str vs String ---");
    let owned: String = String::from("hello, rust");
    let borrowed: &str = &owned[0..5]; // a slice INTO owned's buffer — no copy
    println!("owned={owned:?} borrowed_slice={borrowed:?}");
    // Slicing at a non-char-boundary panics — see topics/04_strings for the UTF-8 details.

    println!("\nNotes:");
    println!("  - Array length is part of the type: [i32; 5] and [i32; 6] are different types.");
    println!("  - A slice &[T] is a (pointer, length) view — the Vec/array it points into must outlive it.");
    println!("  - Indexing panics on out-of-range; `.get()` returns Option<T> for the non-panicking form.");
    println!("  - &str is to String what &[T] is to Vec<T> — full depth in topics/04_strings.");
}
