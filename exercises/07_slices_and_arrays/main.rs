// Exercise 07: Slices & Arrays
//
// Demonstrates: fixed-size arrays `[T; N]`, slices `&[T]` as a view into
// contiguous memory (array, Vec, or another slice), and the `&str`/`String`
// relationship, which is exactly the array/slice relationship applied to text.

// &[i32] accepts a slice of ANY length, backed by an array, Vec, or another
// slice — this is the Rust analog of a Go slice parameter or a C
// (pointer, length) pair, but bounds-checked and borrow-checked.
//
// "Any length" includes ZERO. An empty slice has no largest element, so the
// honest return type is Option<i32>: indexing `values[0]` here instead would
// panic on `&[]`. The caller is forced to handle the empty case.
fn largest(values: &[i32]) -> Option<i32> {
    let (&first, rest) = values.split_first()?; // None for an empty slice
    let mut max = first;
    for &v in rest {
        if v > max {
            max = v;
        }
    }
    Some(max)
    // Idiomatic one-liner once you know iterators (exercise 14):
    //     values.iter().copied().max()
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
    println!("v={v:?} largest(&v)={:?}", largest(&v)); // &Vec<T> coerces to &[T]
    println!("largest(&a)={:?}", largest(&a)); // &[T; N] coerces to &[T] too — same function, no overload needed
    println!("largest(&[])={:?}", largest(&[])); // empty slice: None, not a panic

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_cases() {
        let cases: [(&[i32], Option<i32>); 6] = [
            (&[], None),
            (&[7], Some(7)),
            (&[9, 3, 7, 1, 8], Some(9)),
            (&[1, 2, 3], Some(3)),
            (&[-5, -2, -9], Some(-2)),
            (&[4, 4, 4], Some(4)),
        ];
        for (input, expected) in cases {
            assert_eq!(largest(input), expected, "input was {input:?}");
        }
    }

    #[test]
    fn largest_accepts_arrays_vecs_and_subslices() {
        let a = [1, 5, 3];
        let v = vec![2, 8, 6];
        assert_eq!(largest(&a), Some(5));
        assert_eq!(largest(&v), Some(8));
        assert_eq!(largest(&v[..1]), Some(2));
    }
}
