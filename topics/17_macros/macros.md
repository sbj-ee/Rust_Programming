# Macros — Cheat Sheet

## Declarative Macros (`macro_rules!`)

```rust
macro_rules! square {
    ($val:expr) => { $val * $val };
}
square!(5);        // expands to 5 * 5 at compile time
square!(2 + 3);     // expands to (2 + 3) * (2 + 3) — the matched expr is parenthesized implicitly
```

Pattern-matches against **Rust syntax trees**, not raw text — unlike C's `#define`, which is
naive token substitution with no awareness of expressions, precedence, or scope.

## Fragment Specifiers

```rust
macro_rules! make_getter {
    ($name:ident, $field:ident, $ty:ty) => {
        fn $name(&self) -> $ty { self.$field }
    };
}
```

| Specifier | Matches |
|---|---|
| `expr` | An expression |
| `ident` | An identifier or keyword |
| `ty` | A type |
| `pat` | A pattern |
| `block` | A `{ ... }` block |
| `item` | An item (fn, struct, impl, ...) |
| `tt` | A single token tree — the escape hatch for anything else |

## Repetition

```rust
macro_rules! my_vec {
    ($($item:expr),* $(,)?) => {{
        let mut v = Vec::new();
        $(v.push($item);)*
        v
    }};
}
my_vec![1, 2, 3];   // this is literally how std's own vec! macro is implemented
```

`$(...)* ` / `$(...)+ ` / `$(...)? ` match zero-or-more, one-or-more, and zero-or-one
repetitions of a comma- (or other token-) separated group — the mechanism behind every
variadic-looking macro in the standard library (`vec!`, `format!`, `println!`, `matches!`).

## Multiple Match Arms — Recursive Expansion

```rust
macro_rules! max {
    ($a:expr) => { $a };
    ($a:expr, $($rest:expr),+) => {{
        let a = $a;
        let rest_max = max!($($rest),+);
        if a > rest_max { a } else { rest_max }
    }};
}
max!(3, 7, 2, 9, 5);   // expands recursively, arm by arm, at compile time
```

## Generating Items, Not Just Expressions

```rust
struct Point { x: i32, y: i32 }
impl Point {
    make_getter!(get_x, x, i32);   // expands to a whole `fn get_x(&self) -> i32 { ... }`
}
```

A macro can generate entire declarations — functions, struct fields, trait impls — something
`#define` cannot safely do (C macros generating a function still just paste tokens, with no
syntactic awareness of what a "function" even is).

## Hygiene

```rust
macro_rules! using_a_temp {
    ($e:expr) => {{
        let temp = $e;   // this `temp` CANNOT collide with a `temp` at the call site
        temp * 2
    }};
}
```

Identifiers introduced *inside* a macro body live in their own hygiene context — they don't
accidentally capture or shadow identifiers from the code that invoked the macro. This is a
specific, longstanding source of C macro bugs (`#define SWAP(a, b) { int temp = a; a = b; b
= temp; }` breaks if the caller has a variable named `temp`) that `macro_rules!` cannot
reproduce.

## Procedural Macros (Beyond `macro_rules!`)

Not covered by an exercise in this project (they require a separate `proc-macro = true`
crate), but worth knowing about: `#[derive(Debug)]` and friends are **derive macros**, one
of three kinds of procedural macro (the others: function-like `my_macro!(...)`, and
attribute macros like `#[tokio::main]`). They run actual Rust code against a token stream at
compile time — strictly more powerful than `macro_rules!`, and how crates like `serde`
generate `Serialize`/`Deserialize` implementations.

## Comparison to C / C++ / Go

| Concern | C | C++ | Go | Rust |
|---|---|---|---|---|
| Text-substitution macros | `#define`, naive tokens | `#define` (discouraged; `constexpr`/templates preferred) | None — no macro system at all | `macro_rules!`, syntax-aware, hygienic |
| Code generation beyond macros | None | Templates (type-level only) | `go generate` (external tool, runs any program) | Procedural macros (compile-time Rust code) |
| Variable capture bugs | Common (`SWAP` example above) | Same risk, if `#define` is used | N/A | Prevented by hygiene |
