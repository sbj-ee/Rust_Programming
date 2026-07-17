// Exercise 09: Enums & Pattern Matching
//
// Demonstrates: enums that carry data (algebraic data types — no C/Go
// equivalent), exhaustive `match`, `Option<T>` as the replacement for null,
// `if let`/`while let`, and match guards.

// Each variant can carry different, differently-shaped data — this is a
// "sum type" / tagged union with compiler-enforced exhaustive handling,
// unlike a C enum (just named integers) or a Go const-iota block.
#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
            // No `default`/`_` needed — all three variants are covered, and the
            // compiler rejects this function if a fourth variant is ever added
            // and left unhandled.
        }
    }
}

fn find(haystack: &[i32], needle: i32) -> Option<usize> {
    for (i, &v) in haystack.iter().enumerate() {
        if v == needle {
            return Some(i); // Option::Some wraps a found value
        }
    }
    None // Option::None — there is no null in safe Rust
}

fn main() {
    println!("=== Exercise 09: Enums & Pattern Matching ===");

    // Section 1: enums with data, and matching them
    println!("\n--- Section 1: enums carrying data ---");
    let shapes = [
        Shape::Circle { radius: 2.0 },
        Shape::Rectangle {
            width: 3.0,
            height: 4.0,
        },
        Shape::Triangle {
            base: 6.0,
            height: 2.0,
        },
    ];
    for s in &shapes {
        println!("{s:?} area={:.2}", s.area());
    }

    // Section 2: Option<T> — the replacement for null/nil pointers
    println!("\n--- Section 2: Option<T> ---");
    let data = [10, 20, 30, 40];
    match find(&data, 30) {
        Some(i) => println!("found 30 at index {i}"),
        None => println!("not found"),
    }
    match find(&data, 99) {
        Some(i) => println!("found 99 at index {i}"),
        None => println!("99 not found"),
    }

    // Section 3: if let — shorthand for a match with one interesting arm
    println!("\n--- Section 3: if let ---");
    let maybe_index = find(&data, 20);
    if let Some(i) = maybe_index {
        println!("if let: found at {i}");
    } else {
        println!("if let: not found");
    }

    // Section 4: while let — drain something until a pattern stops matching
    println!("\n--- Section 4: while let ---");
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        print!("{top} ");
    }
    println!("(stack now empty: {stack:?})");

    // Section 5: match guards and binding patterns
    println!("\n--- Section 5: match guards ---");
    for n in [-5, 0, 3, 42] {
        let desc = match n {
            n if n < 0 => "negative".to_string(),
            0 => "zero".to_string(),
            n if n % 2 == 0 => format!("positive even ({n})"),
            n => format!("positive odd ({n})"),
        };
        println!("{n} -> {desc}");
    }

    // Section 6: Option combinators instead of manual matching
    println!("\n--- Section 6: Option combinators ---");
    let result = find(&data, 40).map(|i| i * 100).unwrap_or(0);
    println!("map/unwrap_or result={result}");

    println!("\nNotes:");
    println!("  - Rust enums are tagged unions: each variant can hold different data.");
    println!("  - `match` must be exhaustive — adding a variant without updating a match is a compile error.");
    println!("  - Option<T> replaces null entirely; you cannot forget to check it, the compiler forces a match.");
    println!(
        "  - `if let`/`while let` are sugar for a match with exactly one pattern of interest."
    );
}
