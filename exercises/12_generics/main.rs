// Exercise 12: Generics
//
// Demonstrates: generic functions and structs, trait bounds, `where`
// clauses, and monomorphization — the compile-time expansion that makes
// Rust generics zero-cost. Go's generics are only partly specialized: Go
// compiles one copy per GC "shape" (types with the same memory layout share
// code) and passes a runtime dictionary for the type-specific operations.
// C's `void*` + macro approach gives up type checking entirely.

use std::fmt::Display;

// T: PartialOrd is a TRAIT BOUND — without it, `>` would not compile because
// not every type supports ordering. Compare to C's void* (no bound at all,
// no safety) or Go's `[T cmp.Ordered]` (structurally similar to this).
//
// Returns Option<T> because an empty slice has no largest element (exercise
// 07 makes the same point for i32). PartialOrd rather than Ord lets this work
// for f64 too; `Iterator::max` needs Ord, which f64 lacks because of NaN.
fn largest<T: PartialOrd + Copy>(values: &[T]) -> Option<T> {
    let (&first, rest) = values.split_first()?;
    let mut max = first;
    for &v in rest {
        if v > max {
            max = v;
        }
    }
    Some(max)
}

// A generic struct — Point<i32>, Point<f64>, Point<String> are all
// DIFFERENT, separately compiled types after monomorphization.
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Display> Point<T> {
    fn describe(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

// Multiple type parameters, each independently bound.
struct Pair<A, B> {
    first: A,
    second: B,
}

// `where` clause — equivalent bound, more readable once there are several.
fn describe_pair<A, B>(pair: &Pair<A, B>) -> String
where
    A: Display,
    B: Display,
{
    format!("{} / {}", pair.first, pair.second)
}

fn main() {
    println!("=== Exercise 12: Generics ===");

    // Section 1: a generic function instantiated at different types
    println!("\n--- Section 1: generic functions ---");
    let ints = [3, 7, 1, 9, 4];
    let floats = [3.5, 1.2, 9.9, 0.1];
    println!(
        "largest(ints)={:?} largest(floats)={:?}",
        largest(&ints),
        largest(&floats)
    );
    // Each call above compiles a SEPARATE largest::<i32> / largest::<f64> —
    // this is monomorphization, and it's why there's no runtime dispatch cost.
    let no_chars: [char; 0] = [];
    println!("largest(&[] as &[char]) = {:?}", largest(&no_chars));

    // Section 2: generic structs
    println!("\n--- Section 2: generic structs ---");
    let int_point = Point { x: 1, y: 2 };
    let float_point = Point { x: 1.5, y: 2.5 };
    println!(
        "int_point={} float_point={}",
        int_point.describe(),
        float_point.describe()
    );

    // Section 3: multiple type parameters and a `where` clause
    println!("\n--- Section 3: multiple type parameters ---");
    let pair = Pair {
        first: "count",
        second: 42,
    };
    println!("pair: {}", describe_pair(&pair));

    // Section 4: trait bounds constrain what you can do, not just types you can accept
    println!("\n--- Section 4: bounds enforce capability, not just shape ---");
    fn sum_and_print<T: std::iter::Sum + Copy + Display>(values: &[T]) {
        let total: T = values.iter().copied().sum();
        println!("sum = {total}");
    }
    sum_and_print(&[1, 2, 3, 4]);
    sum_and_print(&[1.1, 2.2, 3.3]);

    println!("\nNotes:");
    println!("  - Monomorphization compiles a separate copy of generic code per concrete type — zero-cost.");
    println!("  - Trait bounds (T: Trait) are required to use ANY behavior beyond move/drop — no implicit '.compare()'.");
    println!(
        "  - `where` clauses are equivalent to inline bounds, preferred once bounds get numerous."
    );
    println!("  - Go's constraints look similar, but Go shares one copy per memory 'shape' plus a runtime");
    println!("    dictionary instead of fully monomorphizing; C's void*+macros give up type safety entirely.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_ints() {
        let cases: [(&[i32], Option<i32>); 4] = [
            (&[], None),
            (&[3, 7, 1, 9, 4], Some(9)),
            (&[-1], Some(-1)),
            (&[-8, -3, -5], Some(-3)),
        ];
        for (input, expected) in cases {
            assert_eq!(largest(input), expected, "input was {input:?}");
        }
    }

    #[test]
    fn largest_other_types() {
        assert_eq!(largest(&[3.5, 1.2, 9.9, 0.1]), Some(9.9));
        assert_eq!(largest(&['r', 'u', 's', 't']), Some('u'));
        assert_eq!(largest(&["pear", "apple", "zebra"]), Some("zebra"));
        let empty: [f64; 0] = [];
        assert_eq!(largest(&empty), None);
    }
}
