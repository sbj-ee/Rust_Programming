// Exercise 28: Macros
//
// Demonstrates: declarative macros (`macro_rules!`) — pattern-matching
// code generation at compile time. This is Rust's answer to C's
// text-substitution `#define` macros, but hygienic (no accidental variable
// capture) and matched against Rust syntax, not raw tokens.

// A macro with a fixed pattern — no arguments, just a reusable code snippet.
macro_rules! greet {
    () => {
        println!("Hello from a macro!")
    };
}

// A macro taking one expression argument, bound to the metavariable $val.
// `expr` is a FRAGMENT SPECIFIER — it tells the macro matcher to parse $val
// as a whole expression, not just grab tokens blindly like a C macro would.
macro_rules! square {
    ($val:expr) => {
        $val * $val
    };
}

// Repetition: `$(...)`,* matches zero or more comma-separated expressions —
// this is how vec![1, 2, 3] itself is implemented in the standard library.
macro_rules! my_vec {
    ($($item:expr),* $(,)?) => {{
        let mut v = Vec::new();
        $(v.push($item);)*
        v
    }};
}

// Multiple patterns — a macro can behave differently depending on how many
// arguments (or what shape) it's invoked with, like function overloading
// resolved at expansion time instead of by the type system.
macro_rules! max {
    ($a:expr) => { $a };
    ($a:expr, $($rest:expr),+) => {
        {
            let a = $a;
            let rest_max = max!($($rest),+);
            if a > rest_max { a } else { rest_max }
        }
    };
}

// A macro that generates an item (a function), not just an expression —
// macros can produce whole declarations, which #define can never do safely.
macro_rules! make_getter {
    ($name:ident, $field:ident, $ty:ty) => {
        fn $name(&self) -> $ty {
            self.$field
        }
    };
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    make_getter!(get_x, x, i32);
    make_getter!(get_y, y, i32);
}

fn main() {
    println!("=== Exercise 28: Macros ===");

    // Section 1: a zero-argument macro
    println!("\n--- Section 1: no-argument macro ---");
    greet!();

    // Section 2: an expression-taking macro, expanded at each call site
    println!("\n--- Section 2: expression macro ---");
    println!("square!(5) = {}", square!(5));
    println!("square!(2 + 3) = {}", square!(2 + 3)); // note: expands to (2+3)*(2+3) via the parenthesized $val

    // Section 3: repetition — variadic-style macros
    println!("\n--- Section 3: repetition ---");
    #[allow(clippy::vec_init_then_push)] // the push-in-a-loop IS the point my_vec! demonstrates
    let v = my_vec![10, 20, 30, 40];
    println!("my_vec! = {v:?}");

    // Section 4: multiple match arms, recursive expansion
    println!("\n--- Section 4: recursive macro ---");
    println!("max!(3, 7, 2, 9, 5) = {}", max!(3, 7, 2, 9, 5));

    // Section 5: generating items (methods), not just expressions
    println!("\n--- Section 5: item-generating macro ---");
    let p = Point { x: 3, y: 4 };
    println!("get_x()={} get_y()={}", p.get_x(), p.get_y());

    // Section 6: standard library macros you already use, demystified
    println!("\n--- Section 6: familiar macros are just macro_rules! too ---");
    println!(
        "vec![1,2,3] and format!(...) are declarative macros defined this same way in std/core."
    );

    println!("\nNotes:");
    println!("  - macro_rules! is HYGIENIC: identifiers introduced inside a macro can't accidentally clash");
    println!(
        "    with the caller's variables, unlike C's #define — a real source of C macro bugs."
    );
    println!("  - Fragment specifiers ($x:expr, $x:ident, $x:ty, ...) restrict what a metavariable can match.");
    println!("  - `$(...),*` / `$(...),+` handle repetition — zero-or-more / one-or-more separated groups.");
    println!("  - Procedural macros (#[derive(...)], attribute macros) are a separate, more powerful mechanism");
    println!("    that runs actual Rust code at compile time — out of scope here, but worth knowing exists.");
}
