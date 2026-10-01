// Exercise 28: Macros
//
// Demonstrates: declarative macros (`macro_rules!`) — pattern-matching
// code generation at compile time. This is Rust's answer to C's
// text-substitution `#define` macros, but hygienic for local variables (no
// accidental capture) and matched against token trees parsed as Rust syntax
// fragments, not substituted as raw text.

// A macro with a fixed pattern — no arguments, just a reusable code snippet.
macro_rules! greet {
    () => {
        println!("Hello from a macro!")
    };
}

// A macro taking one expression argument, bound to the metavariable $val.
// `expr` is a FRAGMENT SPECIFIER — it tells the macro matcher to parse $val
// as a whole expression, not just grab tokens blindly like a C macro would.
// Because $val is substituted as ONE already-parsed expression node,
// square!(2 + 3) means (2 + 3) * (2 + 3) with no parentheses needed — the
// operator-precedence bug of C's `#define SQUARE(x) x * x` cannot happen.
//
// Precedence is not the only C macro trap, though: writing `$val * $val`
// would still EVALUATE the argument twice, so square!(next()) would call
// next() twice. Binding it to a local once fixes that — and hygiene
// guarantees this `v` can't collide with a `v` at the call site.
macro_rules! square {
    ($val:expr) => {{
        let v = $val;
        v * v
    }};
}

// The double-evaluating version, kept only to demonstrate the bug in Section 2.
macro_rules! square_naive {
    ($val:expr) => {
        $val * $val
    };
}

// Repetition: `$(...)`,* matches zero or more comma-separated expressions —
// the same repetition syntax std's own vec! uses to accept any number of
// elements. (The real vec! expands to a boxed array converted into a Vec,
// allocating once, rather than pushing one element at a time like this.)
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
    println!("square!(2 + 3) = {}", square!(2 + 3)); // 25: $val is one expression node, not text

    // Side effects: the naive version evaluates its argument twice.
    let mut calls = 0;
    let mut next = || {
        calls += 1;
        calls + 1 // returns 2, then 3, ...
    };
    let naive = square_naive!(next()); // expands to next() * next() -> 2 * 3
    let fixed = square!(next()); // expands to { let v = next(); v * v } -> 4 * 4
    println!("square_naive!(next()) = {naive}  (argument evaluated twice: 2 * 3)");
    println!("square!(next())       = {fixed} (argument evaluated once: 4 * 4)");
    println!("next() was called {calls} times in total");

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
        "vec![1,2,3] is a macro_rules! macro in std, using the same repetition syntax as my_vec!."
    );
    println!("format!(...) is a macro_rules! wrapper too, but around format_args!, which is a");
    println!("compiler built-in — not every std macro can be written in macro_rules! yourself.");

    println!("\nNotes:");
    println!(
        "  - macro_rules! is HYGIENIC for local variables and labels: a `let` inside a macro can't"
    );
    println!(
        "    clash with the caller's variables, unlike C's #define. (Item names like fns are not"
    );
    println!(
        "    hygienic — make_getter! above deliberately defines methods visible to the caller.)"
    );
    println!("  - Bind an expression argument to a local once (`let v = $val;`) so side effects run once.");
    println!("  - Fragment specifiers ($x:expr, $x:ident, $x:ty, ...) restrict what a metavariable can match.");
    println!("  - `$(...),*` / `$(...),+` handle repetition — zero-or-more / one-or-more separated groups.");
    println!("  - Procedural macros (#[derive(...)], attribute macros) are a separate, more powerful mechanism");
    println!("    that runs actual Rust code at compile time — out of scope here, but worth knowing exists.");
}
