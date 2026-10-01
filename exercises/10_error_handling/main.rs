// Exercise 10: Error Handling
//
// Demonstrates: `Result<T, E>` for recoverable errors, the `?` operator for
// propagation, `unwrap`/`expect`, and `panic!` for unrecoverable errors.
// Custom error types and `std::error::Error` get full depth in exercise 23.

use std::num::ParseIntError;

// A function that can fail returns Result<T, E> — the error is an ordinary
// value in the signature, not a hidden exception path.
fn parse_positive(s: &str) -> Result<i32, String> {
    let n: i32 = s.parse().map_err(|e: ParseIntError| e.to_string())?; // ? propagates the Err early
    if n <= 0 {
        return Err(format!("{n} is not positive"));
    }
    Ok(n)
}

// ? works across error types too, as long as From<ParseIntError> is available
// for the return type's error — here both sides are String so it's direct.
fn sum_all(inputs: &[&str]) -> Result<i32, String> {
    let mut total = 0;
    for s in inputs {
        total += parse_positive(s)?; // any Err short-circuits sum_all immediately
    }
    Ok(total)
}

fn main() {
    println!("=== Exercise 10: Error Handling ===");

    // Section 1: matching a Result directly
    println!("\n--- Section 1: matching Result ---");
    match parse_positive("42") {
        Ok(n) => println!("parsed: {n}"),
        Err(e) => println!("error: {e}"),
    }
    match parse_positive("-5") {
        Ok(n) => println!("parsed: {n}"),
        Err(e) => println!("error: {e}"),
    }

    // Section 2: the ? operator propagates errors up the call stack
    println!("\n--- Section 2: ? propagation ---");
    match sum_all(&["1", "2", "3"]) {
        Ok(total) => println!("sum_all(1,2,3) = {total}"),
        Err(e) => println!("sum_all failed: {e}"),
    }
    match sum_all(&["1", "oops", "3"]) {
        Ok(total) => println!("sum_all(1,oops,3) = {total}"),
        Err(e) => println!("sum_all failed: {e}"),
    }

    // Section 3: unwrap/expect — for cases you're CERTAIN can't fail (or a
    // prototype where any failure should crash loudly)
    println!("\n--- Section 3: unwrap and expect ---");
    let n = parse_positive("100").unwrap(); // panics with a generic message if Err
    println!("unwrap: {n}");
    let n2 = parse_positive("7").expect("literal 7 should always parse"); // panics with YOUR message
    println!("expect: {n2}");

    // Section 4: Option also has unwrap/expect and combinators
    println!("\n--- Section 4: Option combinators ---");
    let maybe: Option<i32> = "12".parse().ok(); // Result -> Option, discarding the error
    println!("unwrap_or default: {}", maybe.unwrap_or(0));
    let none: Option<i32> = None;
    #[allow(clippy::unnecessary_literal_unwrap)]
    // None is a literal here only to illustrate unwrap_or's fallback
    let fallback = none.unwrap_or(-1);
    println!("unwrap_or on None: {fallback}");

    // Section 5: panic! — for programmer errors and invariant violations,
    // not for routine, expected failure (that's what Result is for)
    println!("\n--- Section 5: panic! (not invoked here) ---");
    println!(
        "panic!(\"message\") unwinds the CURRENT THREAD's stack, running Drop impls as it goes."
    );
    println!(
        "If that thread is main, the process then exits with status 101; a panic on any other"
    );
    println!("thread ends only that thread (join() returns Err — see exercise 21). With");
    println!(
        "`panic = \"abort\"` in a Cargo profile, it aborts immediately instead, with no unwinding."
    );
    println!("unwrap()/expect() on an Err or None call panic! internally");

    println!("\nNotes:");
    println!("  - Result<T, E> is the Go-style 'error as a value' convention, made exhaustive by the type system.");
    println!("  - `?` is NOT exceptions — it's sugar for 'match Ok(v)=>v, Err(e)=>return Err(e.into())'.");
    println!("  - Reach for unwrap/expect only when failure is a bug, not an expected runtime condition.");
    println!("  - See exercise 23 for custom error types and Box<dyn std::error::Error>.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_positive_cases() {
        let cases = [
            ("42", Ok(42)),
            ("1", Ok(1)),
            ("2147483647", Ok(i32::MAX)),
            ("0", Err("0 is not positive".to_string())),
            ("-5", Err("-5 is not positive".to_string())),
        ];
        for (input, expected) in cases {
            assert_eq!(parse_positive(input), expected, "input was {input:?}");
        }
    }

    #[test]
    fn parse_positive_rejects_non_numbers() {
        for input in ["", "abc", "4.2", " 7", "2147483648"] {
            assert!(parse_positive(input).is_err(), "should reject {input:?}");
        }
    }

    #[test]
    fn sum_all_cases() {
        assert_eq!(sum_all(&[]), Ok(0));
        assert_eq!(sum_all(&["1", "2", "3"]), Ok(6));
        // The first failure short-circuits, and its message is propagated as-is.
        assert_eq!(
            sum_all(&["1", "-4", "oops"]),
            Err("-4 is not positive".to_string())
        );
        assert!(sum_all(&["1", "oops", "3"]).is_err());
    }
}
