// Exercise 06: Borrowing & References
//
// Demonstrates: `&T` and `&mut T`, the aliasing rule the borrow checker
// enforces (one mutable reference XOR any number of immutable references,
// never both at once), and why dangling references are impossible.

// &str, not &String, is the idiomatic parameter type — it accepts a &String via
// deref coercion (as in Section 1 below) AND a plain string literal, so it borrows
// more generally without asking the caller to have a String in the first place.
fn len(s: &str) -> usize {
    s.len() // borrows s — does not take ownership, caller keeps using it after
}

fn shout(s: &mut String) {
    s.push_str("!!!"); // a mutable borrow CAN modify the caller's data
}

// This would not compile if uncommented — included as a topic reference:
//
// fn dangle() -> &String {
//     let s = String::from("gone");
//     &s // COMPILE ERROR: `s` is dropped at the end of this function; the
//        // reference would outlive the data it points to. C/C++ would
//        // happily hand you a dangling pointer here.
// }

fn main() {
    println!("=== Exercise 06: Borrowing & References ===");

    // Section 1: immutable borrow — read without taking ownership
    println!("\n--- Section 1: immutable references ---");
    let s = String::from("hello");
    let n = len(&s); // &s borrows; ownership stays with `s`
    println!("len({s:?}) = {n} (s is still usable)");

    // Section 2: mutable borrow — the caller opts in with `&mut`
    println!("\n--- Section 2: mutable references ---");
    let mut greeting = String::from("hi");
    shout(&mut greeting);
    println!("after shout: {greeting}");

    // Section 3: the aliasing rule — one mutable XOR many immutable, enforced at compile time
    println!("\n--- Section 3: the aliasing rule ---");
    let mut value = String::from("data");
    let r1 = &value;
    let r2 = &value; // multiple immutable borrows: fine
    println!("r1={r1} r2={r2}");
    // r1, r2's last use was the line above — non-lexical lifetimes (NLL) end
    // their borrow there, so a mutable borrow is legal starting here.
    let r3 = &mut value;
    r3.push_str("-modified");
    println!("r3={r3}");
    // let r4 = &value;      // would be fine here too — r3's borrow already ended
    // let bad = &mut value; // COMPILE ERROR if r1/r2 were still in use: cannot
    //                        // borrow `value` as mutable because it is also
    //                        // borrowed as immutable

    // Section 4: references to elements while iterating
    println!("\n--- Section 4: iterating by reference ---");
    let numbers = vec![1, 2, 3, 4];
    let mut total = 0;
    for n in &numbers {
        // &numbers avoids moving the Vec into the loop
        total += n;
    }
    println!("numbers={numbers:?} total={total} (numbers still owned by the outer scope)");

    // Section 5: mutating through a mutable iterator
    println!("\n--- Section 5: mutating in place ---");
    let mut scores = vec![10, 20, 30];
    for s in &mut scores {
        *s += 1; // dereference to write through the reference
    }
    println!("scores after +1 each: {scores:?}");

    println!("\nNotes:");
    println!("  - `&T` borrows for reading; `&mut T` borrows for writing — never both at once.");
    println!("  - The borrow checker rejects dangling references at compile time — see the commented-out dangle().");
    println!(
        "  - No pointer arithmetic on references, unlike C; see exercise 27 for raw pointers."
    );
    println!("  - NLL (non-lexical lifetimes) ends a borrow at its LAST use, not at the end of the block.");
}
