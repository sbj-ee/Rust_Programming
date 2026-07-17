# Lifetimes — Cheat Sheet

A compile-time-only annotation — costs nothing at runtime. No analog in Go (GC means the
compiler never needs to prove a reference's validity) or C (no compile-time reference
tracking at all).

## Why the Annotation Exists

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

`'a` says: the returned reference lives at most as long as the **shorter** of `x` and `y`'s
lifetimes. Without it, the compiler has no way to know which input the output reference is
tied to — and it refuses to guess, because guessing wrong would let a dangling reference
through.

```rust
let s1 = String::from("long string");
let result;
{
    let s2 = String::from("short");
    result = longest(s1.as_str(), s2.as_str());   // fine — s2 still alive
}
// println!("{result}");   // COMPILE ERROR here: s2 (a possible source of
                            // `result`) was dropped — even though THIS call
                            // happened to return s1, the compiler can't
                            // know that without running the function
```

## Elision — Most Functions Need No Annotation

```rust
fn first_char(s: &str) -> Option<char> { s.chars().next() }   // one input ref, one output ref: elided
```

Three elision rules cover the overwhelming majority of functions: a single reference
parameter's lifetime is assumed for the output; a `&self`/`&mut self` method's output is
assumed to borrow from `self`. You only write `'a` explicitly when a function takes
**multiple** reference parameters and returns a reference derived from more than one of
them ambiguously.

## Structs Holding References

```rust
struct Excerpt<'a> {
    text: &'a str,   // Excerpt cannot outlive the &str it borrows
}

impl<'a> Excerpt<'a> {
    fn first_word(&self) -> &str {   // elided: output borrows from &self
        self.text.split_whitespace().next().unwrap_or("")
    }
}
```

A struct that borrows data must declare a lifetime parameter — this is the direct,
syntactic cost of having no garbage collector: the compiler must be able to prove, from the
struct's *type* alone, that it never outlives the data it points into.

## `'static`

```rust
let s: &'static str = "compiled into the binary";
```

The special lifetime meaning "valid for the entire program." All string literals are
`'static`. `'static` on a generic bound (`T: 'static`) means "contains no non-`'static`
references," not "lives forever" — a common point of confusion for newcomers.

## Comparison to C / C++ / Go

| Concern | C | C++ | Go | Rust |
|---|---|---|---|---|
| Reference validity check | None (your problem) | None (your problem; `-Wdangling` catches a few cases) | Not needed — GC keeps referents alive | Proven at compile time via lifetimes |
| Annotation needed in source | Never (no such concept) | Never (no such concept) | Never (no such concept) | Rarely — elision covers most functions |
| Runtime cost | None | None | GC overhead to keep everything reachable alive | None — purely a compile-time check |
| Failure mode when wrong | Silent UB, crash, or corruption | Silent UB, crash, or corruption | Impossible (GC prevents it) | Compile error, before the program ever runs |
