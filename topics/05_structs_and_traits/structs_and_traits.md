# Structs & Traits — Cheat Sheet

## Structs

```rust
struct Rectangle { width: f64, height: f64 }

let r1 = Rectangle { width: 4.0, height: 3.0 };  // keyed literal, always required (no positional form)
let width = 4.0;
let r2 = Rectangle { width, height: 3.0 };        // field init shorthand when names match

struct Point(f64, f64);   // tuple struct — access via .0, .1
struct Marker;             // unit struct — zero-sized, no fields at all
```

No `class` keyword; a struct is plain data. Unlike Go, there is no positional literal form —
`Rectangle { width: 4.0, height: 3.0 }` always names its fields (or uses shorthand).

## Methods (`impl` Blocks)

```rust
impl Rectangle {
    fn new(width: f64, height: f64) -> Self {           // associated function — no `self`
        Self { width, height }                            // convention for a constructor
    }
    fn area(&self) -> f64 { self.width * self.height }   // &self: read-only borrow
    fn scale(&mut self, f: f64) { self.width *= f; self.height *= f; } // &mut self: can mutate
    fn into_square(self) -> Rectangle { /* ... */ self } // self: consumes the receiver
}

let r = Rectangle::new(4.0, 3.0);   // Type::function(...), not a `new` keyword
r.area();                            // method call syntax auto-borrows/derefs as needed
```

Methods are declared in a separate `impl` block, never inside the struct body — unlike a
C++ class where methods live inside the class definition.

## Traits — Rust's Interfaces

```rust
trait Shape {
    fn area(&self) -> f64;                              // required — every implementer must define this
    fn describe(&self) -> String {                        // DEFAULT method — implementers get it for free
        format!("area = {:.2}", self.area())
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 { self.width * self.height }   // explicit `impl Trait for Type`
    // describe() not overridden — uses the default
}
```

Unlike Go's **implicit/structural** interface satisfaction (a type satisfies an interface
just by having the right methods, no declaration needed), Rust requires an explicit
`impl Trait for Type` block — the compiler never guesses that a type "happens to match."

## Static vs Dynamic Dispatch

```rust
fn print_area(s: &impl Shape) { println!("{}", s.describe()); }   // static: monomorphized per call site
fn total(shapes: &[Box<dyn Shape>]) -> f64 {                        // dynamic: one function, vtable calls
    shapes.iter().map(|s| s.area()).sum()
}
```

`impl Trait` (or `<T: Shape>`) picks static dispatch: zero runtime cost, one compiled copy
per concrete type. `dyn Trait` picks dynamic dispatch: one compiled function, a vtable
lookup per call, but lets you put different concrete types in the same `Vec`.

## Common Derivable Traits

```rust
#[derive(Debug, Clone, PartialEq, Default)]
struct Point { x: i32, y: i32 }
```

| Derive | Gives you |
|---|---|
| `Debug` | `{:?}` formatting |
| `Clone` | `.clone()` — explicit deep copy |
| `Copy` | implicit bitwise copy instead of move (only for stack-only data) |
| `PartialEq`/`Eq` | `==`/`!=` |
| `PartialOrd`/`Ord` | `<`, `>`, `.sort()` |
| `Default` | `Point::default()` |
| `Hash` | usable as a `HashMap`/`HashSet` key |

## The Typed-Nil Problem Rust Doesn't Have

Go's `var p *T; var err error = p; err == nil` is famously `false` — a typed nil inside an
interface is not the nil interface. Rust has no null at all in safe code (see
topics/06_error_handling's `Option<T>`), so this entire class of bug doesn't exist.

## Comparison to C++ / Go

| C++ | Go | Rust |
|---|---|---|
| `class`, private by default | `struct`, plain data; export via capitalization | `struct`, plain data; export via `pub` |
| Constructor/destructor | No ctor; `NewX()` convention; GC handles cleanup | No ctor; `Type::new()` convention; `Drop` for cleanup |
| `virtual`/`override`, vtable | Interfaces, implicit satisfaction, itable dispatch | Traits, EXPLICIT `impl Trait for Type`, vtable via `dyn` |
| Abstract base class (`= 0`) | Interface with no default implementation | Trait with no default method implementation |
| `class X : public Base` | Embedding (composition, not inheritance) | No inheritance at all — composition + traits only |
| `dynamic_cast<T*>` | Type assertion `v.(T)` | `Any` + `downcast_ref::<T>()`, or a match on an enum |
