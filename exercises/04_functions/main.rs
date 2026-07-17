// Exercise 04: Functions
//
// Demonstrates: `fn` syntax, expressions vs. statements (the semicolon
// rule), returning tuples for "multiple return values", early `return`,
// and closures capturing their environment.

// Type annotations on parameters and return type are ALWAYS required for
// free functions — unlike closures (Section 5), there is no inference here.
fn add(a: i32, b: i32) -> i32 {
    a + b // no semicolon: this is the tail EXPRESSION, and becomes the return value
}

#[allow(clippy::needless_return)] // deliberately shown for contrast with add()'s tail expression
fn add_explicit(a: i32, b: i32) -> i32 {
    return a + b; // `return` works too, but is idiomatic only for early exits
}

// "Multiple return values" — Go has real syntax for this; Rust returns a tuple.
fn div_mod(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

fn classify(n: i32) -> &'static str {
    if n < 0 {
        return "negative"; // early return — skips the rest of the function
    }
    if n == 0 {
        return "zero";
    }
    "positive" // tail expression covers the remaining case
}

// Variadic-ish: Rust has no varargs, but a slice parameter covers most uses.
fn sum(values: &[i32]) -> i32 {
    values.iter().sum() // iterator adapter — full depth in exercise 14
}

fn main() {
    println!("=== Exercise 04: Functions ===");

    // Section 1: a statement (semicolon) vs an expression (no semicolon)
    println!("\n--- Section 1: expressions vs statements ---");
    let block_value = {
        let a = 3;
        let b = 4;
        a * a + b * b // tail expression — no semicolon, this is the block's value
    };
    println!("block_value={block_value}");
    println!(
        "add(2,3)={} add_explicit(2,3)={}",
        add(2, 3),
        add_explicit(2, 3)
    );

    // Section 2: tuple returns for multiple values
    println!("\n--- Section 2: tuple returns ---");
    let (q, r) = div_mod(17, 5);
    println!("17 / 5 = {q} remainder {r}");

    // Section 3: early return
    println!("\n--- Section 3: early return ---");
    for n in [-3, 0, 8] {
        println!("classify({n}) = {}", classify(n));
    }

    // Section 4: slice parameter instead of varargs
    println!("\n--- Section 4: slice parameter ---");
    println!("sum(&[1,2,3,4]) = {}", sum(&[1, 2, 3, 4]));

    // Section 5: closures — anonymous functions that CAPTURE their environment
    println!("\n--- Section 5: closures ---");
    let factor = 10;
    let scale = |n: i32| n * factor; // captures `factor` by reference automatically
    println!("scale(5) = {}", scale(5));

    let mut total = 0;
    let mut accumulate = |n: i32| total += n; // captures `total` mutably
    accumulate(1);
    accumulate(2);
    accumulate(3);
    #[allow(clippy::drop_non_drop)] // dropping the closure ends its mutable borrow of `total`
    drop(accumulate);
    println!("total after closures = {total}");

    // Section 6: passing a function or closure as an argument
    println!("\n--- Section 6: functions as values ---");
    fn apply(f: impl Fn(i32) -> i32, x: i32) -> i32 {
        f(x)
    }
    println!(
        "apply(add is wrong-arity; use a closure) = {}",
        apply(|x| x + 1, 41)
    );

    println!("\nNotes:");
    println!(
        "  - Omit the semicolon on the last line to return it — this is idiomatic, not a trick."
    );
    println!(
        "  - `return` is for early exits only; using it on the last line draws a clippy warning."
    );
    println!("  - Closures infer parameter/return types from usage; named `fn`s never do.");
    println!(
        "  - `Fn`/`FnMut`/`FnOnce` describe HOW a closure captures — full depth in exercise 14."
    );
}
