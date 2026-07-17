// Exercise 13: Collections
//
// Demonstrates: `Vec<T>`, `HashMap<K, V>`, `HashSet<T>`, `BTreeMap<K, V>`,
// and `VecDeque<T>` — the workhorses that replace hand-rolled dynamic
// arrays and hash tables from C, and cover the same ground as Go's
// slice/map builtins with an explicit, richer API.

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

fn main() {
    println!("=== Exercise 13: Collections ===");

    // Section 1: Vec<T> — growable array, the default sequence type
    println!("\n--- Section 1: Vec ---");
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    v.insert(1, 15); // insert at index 1: [10, 15, 20, 30]
    v.remove(0); // remove index 0: [15, 20, 30]
    println!("v={v:?} len={}", v.len());
    println!("contains(20)={}", v.contains(&20));

    // Section 2: HashMap<K, V> — no guaranteed iteration order
    println!("\n--- Section 2: HashMap ---");
    let mut ages: HashMap<String, u32> = HashMap::new();
    ages.insert("alice".to_string(), 30);
    ages.insert("bob".to_string(), 25);
    *ages.entry("alice".to_string()).or_insert(0) += 1; // entry API: update-or-insert
    match ages.get("bob") {
        Some(age) => println!("bob is {age}"),
        None => println!("bob not found"),
    }
    println!("alice after birthday: {:?}", ages.get("alice"));

    // Section 3: HashSet<T> — uniqueness, set operations
    println!("\n--- Section 3: HashSet ---");
    let a: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
    let b: HashSet<i32> = [3, 4, 5, 6].into_iter().collect();
    let mut intersection: Vec<&i32> = a.intersection(&b).collect();
    intersection.sort(); // sort for deterministic printing — HashSet order is not stable
    println!("a={a:?} b={b:?} intersection={intersection:?}");

    // Section 4: BTreeMap<K, V> — like HashMap, but always iterates in key order
    println!("\n--- Section 4: BTreeMap ---");
    let mut scores: BTreeMap<&str, i32> = BTreeMap::new();
    scores.insert("charlie", 88);
    scores.insert("alice", 95);
    scores.insert("bob", 72);
    for (name, score) in &scores {
        // guaranteed sorted-by-key order, unlike HashMap
        println!("  {name}: {score}");
    }

    // Section 5: VecDeque<T> — a ring buffer, O(1) push/pop at both ends
    println!("\n--- Section 5: VecDeque ---");
    let mut queue: VecDeque<i32> = VecDeque::new();
    queue.push_back(1);
    queue.push_back(2);
    queue.push_front(0);
    println!("queue={queue:?}");
    println!("pop_front={:?} remaining={:?}", queue.pop_front(), queue);

    // Section 6: building a Vec from an iterator with collect()
    println!("\n--- Section 6: collect() ---");
    let squares: Vec<i32> = (1..=5).map(|n| n * n).collect();
    println!("squares={squares:?}");

    println!("\nNotes:");
    println!("  - Vec<T> is the default sequence — like a Go slice, but growth/capacity are explicit methods.");
    println!("  - HashMap iteration order is unspecified and randomized per-process; use BTreeMap for sorted order.");
    println!(
        "  - `entry(key).or_insert(default)` avoids a separate contains-key check-then-insert."
    );
    println!(
        "  - collect() needs a target type — usually inferred from the `let` binding's annotation."
    );
}
