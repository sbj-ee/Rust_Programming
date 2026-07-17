// Exercise 23: Custom Error Types
//
// Demonstrates: hand-writing an error enum that implements
// `std::error::Error` + `Display`, `From` conversions so `?` can cross
// error types, and `Box<dyn Error>` as a catch-all return type for a
// function that can fail in more than one way. (A real project would often
// reach for the `thiserror`/`anyhow` crates to remove this boilerplate —
// this exercise writes it by hand, in keeping with the zero-dependency rule.)

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum ConfigError {
    MissingField(String),
    InvalidNumber(ParseIntError),
    OutOfRange { field: String, value: i32 },
}

// Display supplies the human-readable message — what {} and to_string() use.
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingField(name) => write!(f, "missing field: {name}"),
            ConfigError::InvalidNumber(e) => write!(f, "invalid number: {e}"),
            ConfigError::OutOfRange { field, value } => {
                write!(f, "{field}={value} is out of range")
            }
        }
    }
}

// std::error::Error is mostly a marker; its default source() returning None
// is fine unless you want to expose a wrapped cause explicitly (see below).
impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::InvalidNumber(e) => Some(e),
            _ => None,
        }
    }
}

// From lets `?` auto-convert a ParseIntError into a ConfigError at the call
// site — this is what makes `?` work across error types, not just within one.
impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::InvalidNumber(e)
    }
}

fn parse_port(raw: Option<&str>) -> Result<u16, ConfigError> {
    let raw = raw.ok_or_else(|| ConfigError::MissingField("port".to_string()))?;
    let value: i32 = raw.parse()?; // ParseIntError -> ConfigError via the From impl above
    if !(1..=65535).contains(&value) {
        return Err(ConfigError::OutOfRange {
            field: "port".to_string(),
            value,
        });
    }
    Ok(value as u16)
}

// A function that can fail for reasons from MULTIPLE unrelated error types
// returns Box<dyn Error> — the "any error" catch-all, at the cost of losing
// the specific type at the call site (downcast_ref can recover it if needed).
fn load_and_validate(raw: &str) -> Result<u16, Box<dyn Error>> {
    let port = parse_port(Some(raw))?; // ConfigError -> Box<dyn Error> via a blanket From impl in std
    Ok(port)
}

fn main() {
    println!("=== Exercise 23: Custom Error Types ===");

    // Section 1: the happy path
    println!("\n--- Section 1: success ---");
    println!("{:?}", parse_port(Some("8080")));

    // Section 2: each error variant, with Display output
    println!("\n--- Section 2: each failure mode ---");
    for case in [None, Some("not-a-number"), Some("99999")] {
        match parse_port(case) {
            Ok(p) => println!("ok: {p}"),
            Err(e) => println!("error: {e}"),
        }
    }

    // Section 3: source() exposes the wrapped cause for diagnostics/logging
    println!("\n--- Section 3: error source chain ---");
    if let Err(e) = parse_port(Some("bad")) {
        println!("display: {e}");
        if let Some(source) = e.source() {
            println!("source:  {source}");
        }
    }

    // Section 4: Box<dyn Error> as a uniform return type
    println!("\n--- Section 4: Box<dyn Error> ---");
    match load_and_validate("443") {
        Ok(p) => println!("loaded port {p}"),
        Err(e) => println!("failed: {e}"),
    }

    println!("\nNotes:");
    println!("  - A custom error type needs Debug + Display + std::error::Error to slot into the ecosystem.");
    println!(
        "  - `impl From<X> for MyError` is what lets `?` convert X into MyError automatically."
    );
    println!(
        "  - Box<dyn Error> is the 'any error' return type — trades specificity for uniformity."
    );
    println!("  - In real projects, `thiserror` generates this Display/Error boilerplate; `anyhow` gives you");
    println!("    a ready-made Box<dyn Error>-like type with context() — both are worth adopting past this exercise.");
}
