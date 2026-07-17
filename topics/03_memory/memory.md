# Memory — Cheat Sheet

## Stack vs Heap

```rust
let x = 5;                    // stack: fixed size known at compile time
let s = String::from("hi");   // stack holds {ptr, len, cap}; the bytes "hi" live on the heap
let b = Box::new(42);         // stack holds a pointer; the i32 lives on the heap
```

Just like C/C++: the stack is for fixed-size, scope-bound data; the heap is for anything
whose size isn't known at compile time or that must outlive the current stack frame. The
difference from C is *who* frees the heap allocation and *when*.

## No GC, No Manual `free` — `Drop` Instead

```rust
struct Resource(String);
impl Drop for Resource {
    fn drop(&mut self) {
        println!("releasing {}", self.0);
    }
}
{
    let _r = Resource("file handle".into());
} // _r.drop() runs HERE, deterministically, when the scope ends
```

Every heap allocation is owned by exactly one binding (see topics/02); when that binding's
scope ends, `Drop::drop` runs immediately — not "eventually, when the GC gets around to it"
(Go), and not "whenever you remember to call `free`" (C). This is RAII, the same discipline
C++ uses, but the compiler enforces it is never skipped and never doubled (no double-free).

## Escape Analysis, Roughly

Unlike Go, Rust does not silently promote a stack value to the heap when a reference to it
escapes — that scenario is a **compile error** (see topics/02's `dangle()` example), not a
runtime decision. If a value needs to outlive its creating scope, you make that explicit:
return it by value (ownership moves out), or heap-allocate it deliberately with `Box::new`.

## `Box<T>` — the Simplest Heap Allocation

```rust
let boxed: Box<i32> = Box::new(5);   // single owner, heap-allocated
println!("{}", *boxed);               // deref to read the value
```

Used for: recursive types (a struct can't contain itself by value — `Box` breaks the
infinite size), large values you don't want copied around the stack, and trait objects
(`Box<dyn Trait>`). Full depth in topics/13_smart_pointers.

## Memory Safety Bugs Rust Eliminates at Compile Time

| Bug class | Cause in C/C++ | Why Rust can't have it |
|---|---|---|
| Use-after-free | Dangling pointer still dereferenced | Moved/dropped values can't be used — compile error |
| Double-free | `free()` called twice on the same pointer | Only the owner can drop; ownership can't be duplicated |
| Buffer overread/overwrite | No bounds check on raw pointer/array access | Indexing panics; slices carry their length everywhere |
| Data race | Two threads touch memory, no lock, ≥1 write | The aliasing rule + Send/Sync forbid the setup entirely |
| Null pointer dereference | `NULL` is a valid pointer value | No null in safe Rust — `Option<T>` instead (topics/06) |

None of these require a garbage collector — they're caught by the borrow checker at compile
time, at zero runtime cost. `unsafe` code (topics/16) can still produce all of the above; it
is the explicit, narrow escape hatch where you take over these guarantees yourself.

## Inspecting What the Compiler Decided

```bash
cargo build --release            # optimized build, worth comparing sizes/behavior against debug
cargo bloat --release            # (external tool) breaks down binary size by function
valgrind --leak-check=full ./target/debug/some_binary   # still works — Rust has no GC to confuse it
```

Leaks are rare but not impossible in *safe* Rust: an `Rc` reference cycle (topics/13) or a
forgotten background thread holding data alive will leak, same conceptually as a Go
goroutine leak — `valgrind` or manual auditing, not a "GC will handle it" assumption, is
still the right tool.

## Comparison to C / C++ / Go

| Concern | C | C++ | Go | Rust |
|---|---|---|---|---|
| Allocation | `malloc`/`free` | `new`/`delete`, RAII/smart pointers | GC-managed | Owner-tracked, `Drop` on scope exit |
| Deallocation timing | Whenever you call `free` | Deterministic (destructor at scope exit) | Nondeterministic (GC decides) | Deterministic (drop at scope exit) |
| Leak risk | High (forget to free) | Low with RAII, high with raw `new` | Goroutine/reference leaks still possible | Low; `Rc` cycles are the main risk |
| Runtime cost | None | Near-zero (RAII) | GC pause + write barriers | None — same as C++ RAII |
