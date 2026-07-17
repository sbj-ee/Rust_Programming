# Cargo & Modules — Cheat Sheet

## `mod`, `pub`, and `use`

```rust
mod shapes {                      // inline module
    pub struct Circle { pub radius: f64 }   // pub needed at EVERY level to be externally visible
    impl Circle {
        pub fn area(&self) -> f64 { self.helper() }
        fn helper(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }  // private
    }
}
mod greeting;   // file-backed: pulls in ./greeting.rs (or ./greeting/mod.rs)

use shapes::Circle;   // brings a path into scope — doesn't change visibility, just naming
```

Everything is **private by default**. Unlike Go (capitalization controls export) or C++ (no
built-in module privacy at all, only translation-unit `static`/anonymous namespaces), Rust
requires an explicit `pub` at each level a path crosses to be visible from outside.

## Privacy Levels

```rust
pub fn visible_everywhere() {}
pub(crate) fn visible_in_this_crate_only() {}
pub(super) fn visible_to_parent_module_only() {}
fn visible_in_this_module_only() {}   // no modifier: private
```

## File-Backed vs Inline Modules

A module can be declared inline (`mod foo { ... }`, all in one file) or backed by a separate
file (`mod foo;`, resolved to `foo.rs` or `foo/mod.rs` next to the declaring file). Both have
identical privacy rules — the split is purely organizational, unlike Go where a package
**is** a directory.

## `Cargo.toml` — the Manifest

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }

[[bin]]              # an explicit binary target (used throughout this project)
name = "my_tool"
path = "src/bin/my_tool.rs"
```

Rust's analog of Go's `go.mod`: declares the package name, edition (a Rust "language
version" opt-in, not a compiler version), and dependencies with semver ranges.

## `Cargo.lock` — Pinned, Resolved Versions

Generated automatically; the analog of `go.sum`. For **binaries**, commit it (reproducible
builds); for **libraries**, it's typically left uncommitted (the downstream binary's lock
file is what matters). This project has no dependencies, so `Cargo.lock` is trivial either
way.

## Crate Types

- **Binary crate** (`src/main.rs`, or a `[[bin]]` target): compiles to an executable.
- **Library crate** (`src/lib.rs`): compiles to something other crates depend on.
- **Workspace**: a `Cargo.toml` with `[workspace]` grouping multiple crates that share one
  `Cargo.lock` and `target/` directory — the analog of a Go multi-module workspace
  (`go.work`).

## Common Commands

```bash
cargo new my_project        # scaffold a new binary crate
cargo build                  # compile
cargo run                    # compile (if needed) + run
cargo test                   # run #[test] functions
cargo doc --open             # build and open API docs, including for dependencies
cargo add serde              # add a dependency to Cargo.toml (requires cargo-edit or Cargo 1.62+)
cargo publish                # push to crates.io
```

## Comparison to C / C++ / Go

| Concern | C | C++ | Go | Rust |
|---|---|---|---|---|
| Manifest | None (Makefile is ad hoc) | None standard (CMakeLists.txt, ad hoc) | `go.mod` | `Cargo.toml` |
| Lock file | None | None standard | `go.sum` | `Cargo.lock` |
| Namespacing unit | File (extern-linkage globals) | `namespace` | Directory = package | `mod` (file or inline), independent of directory layout |
| Export control | `static` hides; no export keyword | `namespace`, access specifiers on class members | Capitalized identifier = exported | `pub` (and `pub(crate)`/`pub(super)`) required explicitly |
| Package registry | None (vendor manually) | None standard (vcpkg/Conan are third-party) | proxy.golang.org, module paths = URLs | crates.io |
