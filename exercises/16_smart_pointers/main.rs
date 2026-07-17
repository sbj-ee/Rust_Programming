// Exercise 16: Smart Pointers
//
// Demonstrates: `Box<T>` (single ownership, heap allocation), `Rc<T>`
// (multiple ownership via reference counting), `RefCell<T>` (interior
// mutability — runtime-checked borrowing), and `Weak<T>` (non-owning
// references that break reference cycles).

use std::cell::RefCell;
use std::rc::{Rc, Weak};

// A recursive type MUST go through a pointer — without Box<T>, the compiler
// cannot compute a finite size for List (this mirrors why C needs `struct
// Node *next` instead of `struct Node next`).
#[derive(Debug)]
#[allow(dead_code)] // fields are only ever read through the {:?} Debug impl below
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // Weak: does not keep the parent alive
    #[allow(dead_code)] // written to build the tree; not read again in this exercise
    children: RefCell<Vec<Rc<Node>>>, // Rc: shared ownership, ref-counted
}

fn main() {
    println!("=== Exercise 16: Smart Pointers ===");

    // Section 1: Box<T> — single ownership, heap-allocated, enables recursive types
    println!("\n--- Section 1: Box<T> ---");
    use List::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{list:?}");

    // Section 2: Rc<T> — multiple owners, reference-counted, single-threaded only
    println!("\n--- Section 2: Rc<T> ---");
    let shared = Rc::new(String::from("shared data"));
    println!("count after creation: {}", Rc::strong_count(&shared));
    let a = Rc::clone(&shared); // bumps the refcount — cheap, no deep copy
    let b = Rc::clone(&shared);
    println!("count after two clones: {}", Rc::strong_count(&shared));
    println!("a={a} b={b}");
    drop(a);
    println!(
        "count after dropping one clone: {}",
        Rc::strong_count(&shared)
    );

    // Section 3: RefCell<T> — interior mutability, borrow rules checked at RUNTIME
    println!("\n--- Section 3: RefCell<T> ---");
    let cell = RefCell::new(5);
    *cell.borrow_mut() += 10; // borrow_mut() would PANIC if another borrow were active
    println!("cell={}", cell.borrow());
    // A second simultaneous borrow_mut() while one is held panics at runtime
    // instead of being rejected at compile time — the tradeoff for mutating
    // through a shared (Rc<RefCell<T>>) reference.

    // Section 4: Rc<RefCell<T>> — the common combo for shared, mutable state
    // in a single-threaded program (see Arc<Mutex<T>> in exercise 21 for the
    // multi-threaded equivalent).
    println!("\n--- Section 4: Rc<RefCell<T>> ---");
    let shared_counter = Rc::new(RefCell::new(0));
    let c1 = Rc::clone(&shared_counter);
    let c2 = Rc::clone(&shared_counter);
    *c1.borrow_mut() += 1;
    *c2.borrow_mut() += 1;
    println!("shared_counter={}", shared_counter.borrow());

    // Section 5: Weak<T> — breaks reference cycles that Rc alone would leak
    println!("\n--- Section 5: Weak<T> and parent/child cycles ---");
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // weak, non-owning link back up
    println!(
        "leaf.value={} leaf.parent.value={:?}",
        leaf.value,
        leaf.parent.borrow().upgrade().map(|p| p.value)
    );
    println!("(if parent held an Rc instead of Weak, leaf<->branch would leak forever)");

    println!("\nNotes:");
    println!("  - Box<T>: single owner, heap-allocated — the default choice, and how recursive types compile.");
    println!("  - Rc<T>: multiple owners via refcounting, single-threaded only (Arc<T> is the thread-safe version).");
    println!("  - RefCell<T>: interior mutability with runtime-checked borrows — panics instead of compile error.");
    println!("  - Weak<T>: a non-owning reference, used to break parent/child Rc cycles that would otherwise leak.");
}
