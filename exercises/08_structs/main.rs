// Exercise 08: Structs
//
// Demonstrates: struct definitions, tuple structs, unit structs, `impl`
// blocks, methods (`&self`, `&mut self`, `self`), and associated functions
// (the `Type::new(...)` constructor convention — there is no `new` keyword).

#[derive(Debug)] // auto-generates a Debug impl so {:?} works
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // Associated function — no `self` parameter, called as Rectangle::new(...).
    // This is the idiomatic constructor; Rust has no `new` keyword or built-in ctor.
    fn new(width: f64, height: f64) -> Self {
        Self { width, height } // field init shorthand: `width` means `width: width`
    }

    // &self — an immutable borrow of the receiver. Read-only methods use this.
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // &mut self — a mutable borrow. Required to modify fields.
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }

    // self (by value) — consumes the receiver. Used for conversions/builders
    // where the original should no longer be usable afterward.
    fn into_square(self) -> Rectangle {
        let side = (self.width + self.height) / 2.0;
        Rectangle {
            width: side,
            height: side,
        }
    }
}

// Tuple struct — fields have no names, accessed by position (.0, .1, ...).
struct Point(f64, f64);

// Unit struct — no fields at all; useful as a marker type or trait target.
struct Marker;

fn main() {
    println!("=== Exercise 08: Structs ===");

    // Section 1: struct literals and field init shorthand
    println!("\n--- Section 1: struct literals ---");
    let width = 4.0;
    let r1 = Rectangle { width, height: 3.0 }; // shorthand: `width` instead of `width: width`
    println!("r1={r1:?}");

    // Section 2: associated function as constructor
    println!("\n--- Section 2: associated functions ---");
    let r2 = Rectangle::new(6.0, 2.0);
    println!("r2={r2:?}");

    // Section 3: &self methods
    println!("\n--- Section 3: &self methods ---");
    println!("r1.area()={} r2.area()={}", r1.area(), r2.area());

    // Section 4: &mut self methods
    println!("\n--- Section 4: &mut self methods ---");
    let mut r3 = Rectangle::new(1.0, 1.0);
    r3.scale(5.0);
    println!("r3 after scale(5.0) = {r3:?}");

    // Section 5: self by value — consumes the original
    println!("\n--- Section 5: consuming self ---");
    let square = r2.into_square();
    // println!("{r2:?}"); // COMPILE ERROR: r2 was moved into into_square()
    println!("square={square:?} (r2 is no longer usable)");

    // Section 6: tuple structs and unit structs
    println!("\n--- Section 6: tuple structs and unit structs ---");
    let origin = Point(0.0, 0.0);
    println!("origin=({}, {})", origin.0, origin.1);
    let _marker = Marker; // zero-sized, exists only at compile time

    // Section 7: struct update syntax
    println!("\n--- Section 7: struct update syntax ---");
    let base = Rectangle::new(10.0, 10.0);
    let modified = Rectangle {
        width: 20.0,
        ..base
    }; // takes remaining fields from base
    println!("modified={modified:?}");

    println!("\nNotes:");
    println!("  - No `class` keyword or constructor syntax; `Type::new(...)` is convention, not language.");
    println!(
        "  - &self / &mut self / self are just sugar for the receiver's first parameter type."
    );
    println!("  - #[derive(Debug)] generates {{:?}} support — see topics/05_structs_and_traits.");
    println!(
        "  - Tuple structs (Point(f64, f64)) and unit structs (Marker) have no field names at all."
    );
}
