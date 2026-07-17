// Exercise 02: Variables & Types
//
// Demonstrates: `let` bindings, immutability by default, `mut`, shadowing,
// scalar types, compound types, `const`, and explicit casts with `as`.

fn main() {
    println!("=== Exercise 02: Variables & Types ===");

    // Section 1: immutability by default
    println!("\n--- Section 1: let vs mut ---");
    let x = 5; // immutable — this is the DEFAULT, unlike every C-family language
               // x = 6; // COMPILE ERROR: cannot assign twice to immutable variable
    let mut y = 5; // opt into mutability explicitly
    y += 1;
    println!("x={x} y={y}");

    // Section 2: shadowing — a NEW binding, not a mutation
    println!("\n--- Section 2: shadowing ---");
    let z = 5;
    let z = z * 2; // shadows the old `z`; can even change type
    let z = format!("value is {z}"); // now a String, still called `z`
    println!("{z}");

    // Section 3: scalar types
    println!("\n--- Section 3: scalar types ---");
    let a: i32 = -42; // signed 32-bit, the default integer type
    let b: u8 = 255; // unsigned 8-bit
    let c: i64 = 9_000_000_000; // underscores are just visual separators
    let d: f64 = 19.75; // 64-bit float, the default float type
    let e: bool = true;
    let f: char = 'R'; // ALWAYS 4 bytes, a Unicode scalar value — not a byte like C's char
    println!("a={a} b={b} c={c} d={d} e={e} f={f}");
    println!(
        "isize/usize are pointer-width: usize={} bytes",
        std::mem::size_of::<usize>()
    );

    // Section 4: overflow behavior — checked in debug, wraps in release
    println!("\n--- Section 4: overflow ---");
    let max: u8 = u8::MAX;
    println!("u8::MAX = {max}");
    println!(
        "wrapping_add(1) = {} (debug build would instead panic on plain `+`)",
        max.wrapping_add(1)
    );

    // Section 5: compound types — tuples and arrays
    println!("\n--- Section 5: compound types ---");
    let point: (f64, f64, &str) = (1.0, 2.0, "origin-ish");
    println!("tuple: ({}, {}, {})", point.0, point.1, point.2);
    let (px, py, _) = point; // destructuring
    println!("destructured: px={px} py={py}");

    let arr: [i32; 4] = [10, 20, 30, 40]; // fixed size, part of the type
    let zeros = [0; 5]; // [0, 0, 0, 0, 0]
    println!("arr={arr:?} zeros={zeros:?} arr.len()={}", arr.len());

    // Section 6: const — must have an explicit type, must be a constant expression
    println!("\n--- Section 6: const ---");
    const MAX_RETRIES: u32 = 3; // inlined at every use site, no fixed memory address
    println!("MAX_RETRIES={MAX_RETRIES}");

    // Section 7: explicit casts — Rust never converts numeric types implicitly
    println!("\n--- Section 7: casting with `as` ---");
    let big: i64 = 300;
    let truncated = big as u8; // 300 % 256 = 44 — silent truncation, not an error
    println!("300i64 as u8 = {truncated}");
    let f_to_i = 3.9_f64 as i32; // truncates toward zero, does not round
    println!("3.9f64 as i32 = {f_to_i}");

    println!("\nNotes:");
    println!("  - `let` bindings are immutable unless marked `mut` — the opposite default from C/C++/Go.");
    println!("  - Shadowing creates a new binding (can change type); `mut` reuses the same storage (cannot).");
    println!("  - `char` is a 4-byte Unicode scalar value, not a byte — see topics/04_strings.");
    println!("  - No implicit numeric conversions anywhere, ever — every widen/narrow is an explicit `as`.");
}
