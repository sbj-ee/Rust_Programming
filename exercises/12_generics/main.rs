// Exercise 12: Generics
//
// Demonstrates: generic functions and structs, trait bounds, `where`
// clauses, and monomorphization — the compile-time expansion that makes
// Rust generics zero-cost, unlike Go's generics (still specialized, but a
// younger, less-optimized implementation) or a C `void*` + macro approach.

use std::fmt::Display;

// T: PartialOrd is a TRAIT BOUND — without it, `>` would not compile because
// not every type supports ordering. Compare to C's void* (no bound at all,
// no safety) or Go's `[T cmp.Ordered]` (structurally similar to this).
fn largest<T: PartialOrd + Copy>(values: &[T]) -> T {
    let mut max = values[0];
    for &v in values {
        if v > max {
            max = v;
        }
    }
    max
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
        "largest(ints)={} largest(floats)={}",
        largest(&ints),
        largest(&floats)
    );
    // Each call above compiles a SEPARATE largest::<i32> / largest::<f64> —
    // this is monomorphization, and it's why there's no runtime dispatch cost.

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
    println!("  - Go's generics work structurally like this; C's void*+macros give up type safety entirely.");
}
