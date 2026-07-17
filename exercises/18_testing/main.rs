// Exercise 18: Testing
//
// Demonstrates: `#[test]`, `#[cfg(test)]`, the `assert!`/`assert_eq!`/
// `assert_ne!` macros, table-driven tests, and `#[should_panic]`. Run with
// `cargo test --bin 18_testing`.

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero");
    }
    a / b
}

fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();
    cleaned.chars().eq(cleaned.chars().rev())
}

fn main() {
    println!("=== Exercise 18: Testing ===");
    println!(
        "This exercise's interesting content is in `cargo test --bin 18_testing`, not `cargo run`."
    );
    println!(
        "add(2,3)={} divide(10,2)={} is_palindrome(\"racecar\")={}",
        add(2, 3),
        divide(10, 2),
        is_palindrome("racecar")
    );

    println!("\nNotes:");
    println!("  - #[cfg(test)] compiles the `tests` module ONLY when testing — it adds nothing to `cargo build`.");
    println!(
        "  - assert_eq!/assert_ne! print both sides on failure; assert! takes any bool expression."
    );
    println!("  - #[should_panic] asserts a test function panics; without it, a panic means the test failed.");
    println!("  - Binaries (src/bin-style targets like this one) support #[test] but not a separate tests/ dir —");
    println!(
        "    that integration-test layout is for library crates (a `[lib]` with a src/lib.rs)."
    );
}

// The convention: tests live in a `mod tests` gated by #[cfg(test)], right
// next to the code they test. `use super::*;` pulls in everything from the
// enclosing scope (add, divide, is_palindrome above).
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_basic() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn add_negative() {
        assert_eq!(add(-1, -1), -2);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn divide_by_zero_panics() {
        divide(1, 0);
    }

    // Table-driven test — the idiomatic replacement for a hand-rolled test
    // runner loop; each case is just an entry in the array.
    #[test]
    fn palindrome_cases() {
        let cases = [
            ("racecar", true),
            ("A man a plan a canal Panama", true),
            ("hello", false),
            ("", true),
        ];
        for (input, expected) in cases {
            assert_eq!(is_palindrome(input), expected, "input was {input:?}");
        }
    }

    // Subtests via #[test] fns grouped in one mod give the same organization
    // Go gets from t.Run subtests — there is no nested-test macro needed.
    #[test]
    fn assert_variants() {
        assert!(add(1, 1) == 2); // arbitrary boolean condition
        assert_eq!(add(2, 2), 4); // equality, with a readable failure message
        assert_ne!(add(2, 2), 5); // inequality
    }
}
