// A separate file, pulled in by `mod greeting;` in main.rs — the same
// pattern as ../Go_Programming's exercises/30_modules_and_packages/greeting
// subpackage, but a module here is a file, not a directory-as-namespace.

pub fn hello(name: &str) -> String {
    format!("Hello, {name}, from the greeting module!")
}

// Not `pub` — only visible inside this module and its children, never from main.rs.
fn internal_helper() -> &'static str {
    "internal_helper is private to greeting.rs"
}

pub fn describe() -> String {
    internal_helper().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_includes_name() {
        assert!(hello("Ferris").contains("Ferris"));
    }
}
