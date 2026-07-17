# Collections — Cheat Sheet

## `Vec<T>` — the Default Sequence

```rust
let mut v: Vec<i32> = Vec::new();
v.push(1); v.push(2);
v.insert(1, 99);       // insert at index
v.remove(0);            // remove at index, returns the removed value
v.contains(&2);
v.sort(); v.reverse();
let v2 = vec![1, 2, 3]; // macro form, like a literal
```

Like a Go slice, but capacity/growth are explicit methods (`Vec::with_capacity`,
`.reserve()`) rather than hidden behind `append`. `&v` coerces to `&[T]` (a slice, see
exercise 07) anywhere a slice is expected.

## `HashMap<K, V>` — Unordered

```rust
use std::collections::HashMap;
let mut ages: HashMap<String, u32> = HashMap::new();
ages.insert("alice".into(), 30);
*ages.entry("alice".into()).or_insert(0) += 1;   // update-or-insert, one call
ages.get("bob");                                    // Option<&u32>
```

Iteration order is unspecified **and randomized per process** (a deliberate DoS mitigation)
— never rely on it. This is stricter than Go's map (also unordered, but not adversarially
randomized) or C++'s `std::unordered_map` (implementation-defined, typically stable within a
run).

## `BTreeMap<K, V>` — Ordered

```rust
use std::collections::BTreeMap;
let mut scores: BTreeMap<&str, i32> = BTreeMap::new();
scores.insert("charlie", 88);
scores.insert("alice", 95);
for (k, v) in &scores {   // always iterates in KEY order
    println!("{k}: {v}");
}
```

Reach for this whenever you need deterministic, sorted iteration — the direct analog of
C++'s `std::map` (also a balanced tree) vs `std::unordered_map`.

## `HashSet<T>` / `BTreeSet<T>`

```rust
use std::collections::HashSet;
let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
let b: HashSet<i32> = [2, 3, 4].into_iter().collect();
a.intersection(&b);   // iterator over shared elements
a.union(&b);
a.difference(&b);
```

## `VecDeque<T>` — a Ring Buffer

```rust
use std::collections::VecDeque;
let mut q: VecDeque<i32> = VecDeque::new();
q.push_back(1); q.push_front(0);   // O(1) at BOTH ends, unlike Vec (O(n) at the front)
q.pop_front();
```

## Building From an Iterator: `collect()`

```rust
let squares: Vec<i32> = (1..=5).map(|n| n * n).collect();
let set: HashSet<i32> = squares.iter().copied().collect();
```

`collect()` needs a target type — usually inferred from the `let` binding's annotation or a
turbofish (`.collect::<Vec<_>>()`). It works for any collection implementing
`FromIterator`, which includes all of the above plus `String`, `Result<Vec<T>, E>`, and more.

## Comparison to C / C++ / Go

| Concern | C | C++ | Go | Rust |
|---|---|---|---|---|
| Growable array | Manual `realloc` | `std::vector` | `[]T` (slice) + `append` | `Vec<T>` |
| Hash map | Not in the language (roll your own) | `std::unordered_map` | `map[K]V`, built-in syntax | `HashMap<K, V>` |
| Ordered map | Not in the language | `std::map` | Not built-in (sort keys manually) | `BTreeMap<K, V>` |
| Update-or-insert | Manual check-then-insert | `operator[]` default-constructs | `m[k]++` works due to zero values | `.entry(k).or_insert(default)` |
| Set type | Not in the language | `std::set`/`std::unordered_set` | `map[T]struct{}` idiom | `HashSet<T>`/`BTreeSet<T>` |
