# Generics — Cheat Sheet

## Generic Functions

```rust
fn largest<T: PartialOrd + Copy>(values: &[T]) -> T {
    let mut max = values[0];
    for &v in values {
        if v > max { max = v; }
    }
    max
}

largest(&[3, 7, 1]);        // T = i32
largest(&[3.5, 1.2, 9.9]);  // T = f64, a SEPARATE compiled function
```

`T: PartialOrd + Copy` is a **trait bound** — without it, `>` and the implicit copy on
`values[0]` wouldn't compile, because not every type supports ordering or is cheap to copy.
This is closer to a C++ template's implicit requirements made explicit, or to Go's
`[T cmp.Ordered]` constraint syntax.

## Monomorphization — Why Rust Generics Are Zero-Cost

Each call site above compiles a **separate, fully specialized copy** of `largest` — one for
`i32`, one for `f64` — at compile time. There is no runtime type dispatch, no boxing, no
vtable: generic code runs exactly as fast as if you'd hand-written each version. The cost is
paid at compile time (larger binaries, longer builds), not at runtime — the same tradeoff
C++ templates make, and a stronger guarantee than Go's generics (implemented with some
runtime dictionary-passing for cases the compiler can't fully specialize).

## Generic Structs

```rust
struct Point<T> { x: T, y: T }
struct Pair<A, B> { first: A, second: B }   // independently-typed parameters

impl<T: std::fmt::Display> Point<T> {
    fn describe(&self) -> String { format!("({}, {})", self.x, self.y) }
}
```

`Point<i32>` and `Point<f64>` are different types after monomorphization, the same way
`[i32; 4]` and `[i32; 5]` are different types (topics/01).

## `where` Clauses

```rust
fn describe_pair<A, B>(pair: &Pair<A, B>) -> String
where
    A: std::fmt::Display,
    B: std::fmt::Display,
{
    format!("{} / {}", pair.first, pair.second)
}
```

Equivalent to inline bounds (`<A: Display, B: Display>`); preferred once there are several
bounds or the bounds involve associated types, for readability.

## Trait Bounds Constrain Capability, Not Just Shape

```rust
fn sum_and_print<T: std::iter::Sum + Copy + std::fmt::Display>(values: &[T]) {
    let total: T = values.iter().copied().sum();
    println!("{total}");
}
```

Unlike C's `void*` (gives up type safety AND capability entirely — you can do anything to
the bytes, safely or not) or an unconstrained template misuse in C++ (fails late, often with
an unreadable error deep in the instantiation), a Rust trait bound fails **immediately, at
the generic function's own definition**, with a clear "the trait bound `T: Foo` is not
satisfied" — you find out you're missing a capability before anyone ever calls the function
with a bad type.

## `impl Trait` vs Generic Parameter — Same Thing, Different Syntax

```rust
fn print_it(x: impl std::fmt::Display) { println!("{x}"); }
fn print_it2<T: std::fmt::Display>(x: T) { println!("{x}"); }
```

These compile to the same monomorphized code; `impl Trait` is shorthand for a single-use
anonymous type parameter, useful when you don't need to name `T` or refer to it more than
once in the signature.

## Comparison to C / C++ / Go

| Concern | C | C++ | Go | Rust |
|---|---|---|---|---|
| Mechanism | `void*` + macros | Templates | Type parameters (1.18+) | Type parameters + trait bounds |
| Type safety | None | Full, but errors can be cryptic | Full | Full, errors point at the missing bound directly |
| Dispatch | None — you cast blindly | Compile-time (monomorphized) | Mostly compile-time, some runtime dictionary passing | Compile-time (monomorphized) — zero-cost |
| Constraining behavior | Not possible | Concepts (C++20) or SFINAE (pre-20) | Constraint interfaces (`[T cmp.Ordered]`) | Trait bounds (`T: Trait`) |
| Binary size tradeoff | N/A | Larger (code bloat from instantiation) | Smaller (some runtime dispatch) | Larger (full monomorphization) |
