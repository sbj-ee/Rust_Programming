# Smart Pointers — Cheat Sheet

## `Box<T>` — Single Ownership, Heap-Allocated

```rust
let b: Box<i32> = Box::new(5);
```

The simplest smart pointer: one owner, heap allocation, deallocated when the `Box` drops.
Needed for:
- **Recursive types** — a struct can't contain itself by value (infinite size); `Box`
  breaks the cycle by putting a fixed-size pointer where the recursive field would be.
- **Trait objects** — `Box<dyn Trait>` for dynamic dispatch (topics/05).
- Avoiding large stack copies of infrequently-moved data.

```rust
enum List { Cons(i32, Box<List>), Nil }   // without Box, this wouldn't have a finite size
```

## `Rc<T>` — Multiple Ownership, Single-Threaded

```rust
use std::rc::Rc;
let a = Rc::new(String::from("shared"));
let b = Rc::clone(&a);      // bumps a refcount — cheap, NOT a deep copy
Rc::strong_count(&a);        // 2
```

For when a value needs more than one owner — a graph or tree where a node has multiple
parents, for example. `Rc<T>` is **not** thread-safe (its refcount isn't atomic); sending one
to another thread is a compile error. Use `Arc<T>` (topics/07) for the multi-threaded
equivalent.

## `RefCell<T>` — Interior Mutability, Runtime-Checked

```rust
use std::cell::RefCell;
let cell = RefCell::new(5);
*cell.borrow_mut() += 1;    // panics at RUNTIME if another borrow is already active
```

Normally the aliasing rule (topics/02) is enforced at compile time. `RefCell<T>` moves that
check to runtime: `.borrow()`/`.borrow_mut()` panic instead of failing to compile if you
violate the one-writer-XOR-many-readers rule. Used when the compiler can't statically prove
a borrow pattern is safe but you can — commonly paired with `Rc`:

```rust
let shared = Rc::new(RefCell::new(0));
let a = Rc::clone(&shared);
*a.borrow_mut() += 1;   // mutate shared state despite `a`/`shared` being "immutable" bindings
```

## `Weak<T>` — Breaking Reference Cycles

```rust
use std::rc::{Rc, Weak};
struct Node { parent: RefCell<Weak<Node>>, children: RefCell<Vec<Rc<Node>>> }
```

Two `Rc`s pointing at each other (a parent holding an `Rc` to its child, and the child
holding an `Rc` back to its parent) never reach a refcount of zero — a leak, the one memory
leak that's straightforward to write in *safe* Rust. `Weak<T>` is a non-owning reference
(doesn't increment the strong count); `.upgrade()` gives you an `Option<Rc<T>>` — `Some` if
the value is still alive, `None` if it's already been dropped.

## Choosing Among Them

| Need | Reach for |
|---|---|
| One owner, heap-allocated | `Box<T>` |
| Multiple owners, single-threaded | `Rc<T>` |
| Multiple owners, multi-threaded | `Arc<T>` (topics/07) |
| Mutate through a shared/immutable reference | `RefCell<T>` (single-threaded) or `Mutex<T>`/`RwLock<T>` (multi-threaded) |
| Shared + mutable + single-threaded | `Rc<RefCell<T>>` |
| Shared + mutable + multi-threaded | `Arc<Mutex<T>>` |
| Break an `Rc`/`Arc` cycle | `Weak<T>` |

## Comparison to C++

| C++ | Rust |
|---|---|
| `std::unique_ptr<T>` | `Box<T>` |
| `std::shared_ptr<T>` | `Rc<T>` (single-threaded) or `Arc<T>` (thread-safe) |
| `std::weak_ptr<T>` | `Weak<T>` |
| `mutable` keyword for interior mutability | `Cell<T>`/`RefCell<T>` — explicit wrapper types, not a keyword |
| Manual cycle-breaking discipline | Same discipline, but `Weak<T>` makes the non-owning intent explicit in the type |
