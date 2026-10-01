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
    let out_of_range = || ConfigError::OutOfRange {
        field: "port".to_string(),
        value,
    };
    if value == 0 {
        return Err(out_of_range()); // port 0 means "any port", not a valid config value
    }
    // u16::try_from is the CHECKED conversion: it fails for anything outside
    // 0..=65535 instead of silently truncating the way `value as u16` would.
    u16::try_from(value).map_err(|_| out_of_range())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_port_accepts_valid_ports() {
        for (raw, expected) in [("1", 1), ("80", 80), ("8080", 8080), ("65535", 65535)] {
            assert_eq!(
                parse_port(Some(raw)).ok(),
                Some(expected),
                "input was {raw:?}"
            );
        }
    }

    #[test]
    fn parse_port_missing_field() {
        assert!(matches!(
            parse_port(None),
            Err(ConfigError::MissingField(ref f)) if f == "port"
        ));
    }

    #[test]
    fn parse_port_invalid_number_keeps_source() {
        for raw in ["", "not-a-number", "80.5", "99999999999"] {
            let err = parse_port(Some(raw)).unwrap_err();
            assert!(
                matches!(err, ConfigError::InvalidNumber(_)),
                "input was {raw:?}"
            );
            assert!(
                err.source().is_some(),
                "InvalidNumber should expose its cause"
            );
        }
    }

    #[test]
    fn parse_port_out_of_range() {
        for (raw, value) in [("0", 0), ("-1", -1), ("65536", 65536), ("99999", 99999)] {
            match parse_port(Some(raw)) {
                Err(ConfigError::OutOfRange { field, value: v }) => {
                    assert_eq!(field, "port");
                    assert_eq!(v, value);
                }
                other => panic!("{raw:?}: expected OutOfRange, got {other:?}"),
            }
        }
    }

    #[test]
    fn display_messages() {
        assert_eq!(
            parse_port(None).unwrap_err().to_string(),
            "missing field: port"
        );
        assert_eq!(
            parse_port(Some("70000")).unwrap_err().to_string(),
            "port=70000 is out of range"
        );
    }

    #[test]
    fn boxed_error_path() {
        assert_eq!(load_and_validate("443").ok(), Some(443));
        let err = load_and_validate("x").unwrap_err();
        assert!(err.downcast_ref::<ConfigError>().is_some());
    }
}
