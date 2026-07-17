// Exercise 01: Hello World
//
// Demonstrates: the `main` entry point, `println!`/`print!`/`eprintln!`,
// line vs block comments, and how `cargo run`/`cargo build` differ from a
// bare `rustc` invocation.

fn main() {
    println!("=== Exercise 01: Hello World ===");

    // Section 1: the basic macro trio
    println!("\n--- Section 1: printing ---");
    println!("println! adds a trailing newline");
    print!("print! does not");
    println!(); // so we add one ourselves
    eprintln!("eprintln! writes to stderr, not stdout");

    // Section 2: string formatting — {} calls Display, {:?} calls Debug
    println!("\n--- Section 2: formatting ---");
    let name = "Ferris";
    let year = 2015;
    println!("{name} shipped Rust 1.0 in {year}"); // captured identifiers (Rust 2021+)
    println!("{} shipped Rust 1.0 in {}", name, year); // positional, also fine
    println!("{:?}", vec![1, 2, 3]); // Debug formatting — no Display impl needed

    // Section 3: comments
    // A line comment, like this one.
    /* A block comment, which /* can nest */, unlike C's /* */. */
    println!("\n--- Section 3: comments compile away, nothing to print ---");

    println!("\nNotes:");
    println!("  - No header/prototype split: one file, `fn main()` is the entry point.");
    println!(
        "  - `cargo run` compiles (if stale) and runs in one step; `cargo build` only compiles."
    );
    println!("  - `rustc main.rs` works too, but skips Cargo's dependency and target management.");
    println!("  - Semicolons matter: they turn an expression into a statement (see exercise 04).");
}
