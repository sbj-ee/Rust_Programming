// Exercise 11: Traits
//
// Demonstrates: trait definitions, default methods, trait bounds on
// functions (`impl Trait` / generic + `where`), and trait objects (`dyn
// Trait`) for runtime polymorphism — Rust's answer to Go's interfaces and
// C++'s virtual functions, but with an explicit `impl Trait for Type`.

trait Shape {
    fn area(&self) -> f64;

    // Default method — types get this for free unless they override it.
    // Unlike Go interfaces (zero methods can have bodies), trait default
    // methods work like a C++ virtual function with a base implementation.
    fn describe(&self) -> String {
        format!("a shape with area {:.2}", self.area())
    }
}

struct Circle {
    radius: f64,
}

// Unlike Go's implicit satisfaction, Rust requires an explicit `impl Trait
// for Type` block — the compiler will not treat a type as implementing a
// trait just because it happens to have matching methods.
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    // describe() not overridden — uses the trait's default.
}

struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }

    fn describe(&self) -> String {
        format!("a square with side {}", self.side) // overrides the default
    }
}

// Static dispatch: `impl Trait` in argument position — monomorphized per
// call site at compile time, zero runtime overhead, but each instantiation
// is a separate compiled copy (see exercise 12 for the generics version).
fn print_area(shape: &impl Shape) {
    println!("{}", shape.describe());
}

// Dynamic dispatch: `&dyn Trait` — one compiled function, a vtable lookup
// per call, but lets a single Vec hold different concrete types.
fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn main() {
    println!("=== Exercise 11: Traits ===");

    // Section 1: implementing a trait, using default and overridden methods
    println!("\n--- Section 1: trait impls ---");
    let c = Circle { radius: 2.0 };
    let s = Square { side: 3.0 };
    println!("{}", c.describe()); // uses the default
    println!("{}", s.describe()); // uses Square's override

    // Section 2: static dispatch via impl Trait
    println!("\n--- Section 2: static dispatch (impl Trait) ---");
    print_area(&c);
    print_area(&s);

    // Section 3: dynamic dispatch via dyn Trait — heterogeneous collection
    println!("\n--- Section 3: dynamic dispatch (dyn Trait) ---");
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Square { side: 2.0 }),
    ];
    for shape in &shapes {
        println!("{}", shape.describe());
    }
    println!("total_area = {:.2}", total_area(&shapes));

    // Section 4: standard library traits you already use without noticing
    println!("\n--- Section 4: familiar stdlib traits ---");
    #[derive(Debug, Clone, PartialEq, Default)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone(); // Clone trait
    println!("p1={p1:?} p2={p2:?} equal={}", p1 == p2); // PartialEq trait
    println!("default={:?}", Point::default()); // Default trait

    println!("\nNotes:");
    println!(
        "  - `impl Trait for Type` is explicit — no accidental interface satisfaction like Go."
    );
    println!("  - Default trait methods are opt-out (override), unlike Go where every method is separately defined.");
    println!("  - `impl Trait` args = static dispatch (monomorphized); `dyn Trait` = dynamic dispatch (vtable).");
    println!("  - #[derive(...)] auto-implements common traits (Debug, Clone, PartialEq, Default) — no boilerplate.");
}
