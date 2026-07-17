# Closures & Iterators — Cheat Sheet

## The Three Closure Traits

```rust
fn call_fn(f: impl Fn(i32) -> i32, x: i32) -> i32 { f(x) }           // borrows captures
fn call_fn_mut(mut f: impl FnMut() -> i32) -> i32 { f() }             // mutably borrows captures
fn call_fn_once(f: impl FnOnce() -> String) -> String { f() }          // takes ownership, one call
```

| Trait | Captures by | Can call |
|---|---|---|
| `Fn` | `&T` (shared reference) | Any number of times |
| `FnMut` | `&mut T` (mutable reference) | Any number of times, but exclusively |
| `FnOnce` | `T` (by value) | Exactly once |

Every `Fn` is also `FnMut`, and every `FnMut` is also `FnOnce` — the compiler infers the
*most permissive* trait a closure qualifies for based on how it uses its captures. This has
no Go equivalent (Go closures always capture by reference to the enclosing variable, full
stop, GC handles the rest); it's closer to C++'s lambda `[&]` vs `[=]` vs `[x = std::move(x)]`
capture lists, but checked and inferred rather than manually specified.

## `move` — Force Capture by Value

```rust
let owned = String::from("data");
let closure = move || owned;   // without `move`, this would try to borrow `owned`
```

Required whenever a closure must outlive the scope that created it — most commonly, handing
a closure to `thread::spawn` (topics/07).

## Iterators Are Lazy

```rust
let iter = vec![1, 2, 3].iter().map(|n| { println!("mapping {n}"); n * n });
// nothing has printed yet — building the pipeline does no work
let squares: Vec<i32> = iter.collect();   // NOW it runs, once per element
```

An iterator adapter chain builds a description of work, not the work itself. A *consumer*
(`collect`, `for`, `sum`, `fold`, `any`, ...) drives it. This is more thoroughly lazy than
Go's iteration model — a Go `for range` over a slice has no adapter chain to be lazy about
in the first place.

## Common Adapters

```rust
v.iter().map(|x| x * 2)              // transform each element
 .filter(|x| x % 2 == 0)              // keep matching elements
 .filter_map(|x| x.checked_sub(1))    // filter + map in one step, skipping None
 .fold(0, |acc, x| acc + x)           // reduce to one value, with a seed
 .take(3)                              // first n elements
 .skip(2)                              // drop first n elements
 .zip(other_iter)                      // pair up two iterators
 .enumerate()                          // pair each element with its index
 .rev()                                 // reverse (needs DoubleEndedIterator)
 .chain(other_iter)                     // concatenate two iterators
 .any(|x| x > 5)  .all(|x| x > 0)      // short-circuiting predicates
 .find(|x| *x > 5)                      // first match, or None
 .sum::<i32>()  .count()  .min()  .max()
```

## Zero-Cost — This Compiles to a Tight Loop

```rust
let total: i32 = (0..1_000_000).filter(|n| n % 3 == 0).map(|n| n * n).sum();
```

Despite looking like it allocates intermediate collections at each step, the whole chain
above compiles down to a single loop with no allocations — LLVM inlines and fuses the
adapters. This is the same "the abstraction costs nothing" promise C++'s STL algorithms make,
delivered through the same mechanism (monomorphization + inlining, topics/08).

## Custom Iterators

```rust
struct Counter { count: u32 }
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.count < 5 { self.count += 1; Some(self.count) } else { None }
    }
}
```

Implement `next()` and get the entire adapter chain (`.map`, `.filter`, `.collect`, ...) for
free via the `Iterator` trait's default methods — you write one method, inherit dozens.

## Comparison to C / C++ / Go

| Concern | C | C++ | Go | Rust |
|---|---|---|---|---|
| Anonymous functions | Not in the language (function pointers only) | Lambdas (`[capture](args){...}`) | Closures, always capture by reference | Closures, `Fn`/`FnMut`/`FnOnce` inferred |
| Iteration abstraction | Manual index loops | Iterators + `<algorithm>` (`std::transform`, etc.) | `for range`, or manual index loops | Iterator trait + adapter chains, lazy |
| Laziness | N/A | Range-v3/ranges (C++20) add it; classic STL is eager | Not applicable | Always lazy — nothing runs until consumed |
