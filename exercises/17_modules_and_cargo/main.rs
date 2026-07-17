// Exercise 17: Modules, Crates & Cargo
//
// Demonstrates: `mod`/`pub`/`use`, privacy rules, an inline module vs a
// file-backed module (`greeting.rs`, pulled in below), and what Cargo.toml
// controls — the Rust analog of Go's `go.mod` and package system.

mod greeting; // pulls in ./greeting.rs as the `greeting` module

// An inline module — same privacy rules as a file-backed one, just declared
// in place. Useful for small, tightly-scoped groupings within one file.
mod shapes {
    // Private by default — pub is required at EVERY level to be visible
    // from outside, unlike Go where capitalization alone controls export.
    pub struct Circle {
        pub radius: f64,
    }

    impl Circle {
        pub fn new(radius: f64) -> Self {
            Self { radius }
        }

        pub fn area(&self) -> f64 {
            self.helper_area() // private helper, callable from within the module
        }

        fn helper_area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }

    // pub(crate) — visible anywhere in this crate, but not to external crates
    // that might one day depend on this one as a library.
    pub(crate) fn crate_visible_note() -> &'static str {
        "visible within this crate, not outside it"
    }
}

use shapes::Circle; // bring Circle into scope so we can write `Circle` instead of `shapes::Circle`

fn main() {
    println!("=== Exercise 17: Modules, Crates & Cargo ===");

    // Section 1: a file-backed module
    println!("\n--- Section 1: file-backed module (greeting.rs) ---");
    println!("{}", greeting::hello("Ferris"));
    println!("{}", greeting::describe());
    // greeting::internal_helper(); // COMPILE ERROR: private to greeting.rs

    // Section 2: an inline module, privacy, and `use`
    println!("\n--- Section 2: inline module and privacy ---");
    let c = Circle::new(2.0); // works because of `use shapes::Circle` above
    println!("area={:.2}", c.area());
    println!("{}", shapes::crate_visible_note());
    // shapes::Circle::helper_area is private — inaccessible even with the full path

    // Section 3: what Cargo.toml controls
    println!("\n--- Section 3: Cargo.toml ---");
    println!("[package] name/version/edition; [[bin]] name+path per exercise (see this project's Cargo.toml)");
    println!("[dependencies] would list crates.io packages — this project intentionally has none");
    println!("Cargo.lock pins exact resolved versions, generated automatically, normally committed for binaries");

    println!("\nNotes:");
    println!("  - Everything is private by default; `pub` is required at every level to cross a module boundary.");
    println!("  - A module can be a file (mod greeting; -> greeting.rs) or inline (mod shapes {{ ... }}).");
    println!("  - `use` brings a path into scope for brevity — it does not change visibility, only naming.");
    println!("  - Cargo.toml + Cargo.lock together are Rust's go.mod + go.sum: manifest and pinned resolution.");
}
