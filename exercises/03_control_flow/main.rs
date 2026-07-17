// Exercise 03: Control Flow
//
// Demonstrates: `if` as an expression, the single `loop` construct plus
// `while` and `for`, `break` with a value, and loop labels. A first taste
// of `match` appears here too — full pattern-matching depth is exercise 09.

fn main() {
    println!("=== Exercise 03: Control Flow ===");

    // Section 1: if is an EXPRESSION, not a statement
    println!("\n--- Section 1: if as an expression ---");
    let n = 7;
    let parity = if n % 2 == 0 { "even" } else { "odd" }; // both arms must match types
    println!("{n} is {parity}");

    // Section 2: for — the only loop that iterates a range/collection
    println!("\n--- Section 2: for ---");
    for i in 0..5 {
        // 0..5 is a half-open Range; 0..=5 is inclusive
        print!("{i} ");
    }
    println!();
    let names = ["alice", "bob", "carol"];
    for name in &names {
        // iterate by reference — see exercise 06 for why this matters
        print!("{name} ");
    }
    println!();

    // Section 3: while
    println!("\n--- Section 3: while ---");
    let mut countdown = 3;
    while countdown > 0 {
        print!("{countdown} ");
        countdown -= 1;
    }
    println!("liftoff");

    // Section 4: loop — unconditional, and the only loop that can RETURN a value
    println!("\n--- Section 4: loop with break value ---");
    let mut attempts = 0;
    let result = loop {
        attempts += 1;
        if attempts == 4 {
            break attempts * 10; // the value after `break` becomes loop's value
        }
    };
    println!("result={result} after {attempts} attempts");

    // Section 5: labeled loops — break/continue a specific outer loop
    println!("\n--- Section 5: labeled loops ---");
    let mut found = None;
    'outer: for x in 0..5 {
        for y in 0..5 {
            if x * y == 6 {
                found = Some((x, y));
                break 'outer; // without the label this would only break the inner loop
            }
        }
    }
    println!("first pair with product 6: {found:?}");

    // Section 6: match — a preview; exhaustiveness is enforced by the compiler
    println!("\n--- Section 6: match preview ---");
    let code = 404;
    let meaning = match code {
        200 => "OK",
        404 => "Not Found",
        500..=599 => "Server Error", // inclusive range pattern
        _ => "Unknown",              // `_` is required unless every case is covered
    };
    println!("{code} -> {meaning}");

    println!("\nNotes:");
    println!("  - `if`/`match`/`loop` are expressions — they can produce a value used in a `let`.");
    println!(
        "  - `for` is the only loop with syntax sugar for ranges/iterators; no C-style `for(;;)`."
    );
    println!(
        "  - Only `loop` can yield a value via `break VALUE`; `while`/`for` always yield `()`."
    );
    println!(
        "  - `match` must be exhaustive — the compiler rejects a missing case at compile time."
    );
}
