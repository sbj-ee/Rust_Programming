// Exercise 15: Lifetimes
//
// Demonstrates: lifetime annotations on functions and structs, the elision
// rules that let you omit them most of the time, and why a struct holding
// a reference needs an explicit lifetime parameter. This concept has no
// analog in Go (GC) or C (no compile-time borrow tracking at all).

// The lifetime 'a says: the returned reference lives at most as long as the
// SHORTER of x and y's lifetimes. Without this annotation the compiler
// cannot know which input the output reference is tied to.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// A struct that HOLDS a reference must declare a lifetime parameter — the
// struct cannot outlive the data it borrows. This is the direct cost of
// avoiding a garbage collector: the compiler must prove borrows don't dangle.
struct Excerpt<'a> {
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    // Elision rule: a method taking &self returns a reference with self's
    // lifetime automatically — no annotation needed here despite returning
    // a reference derived from a lifetime-parameterized struct.
    fn first_word(&self) -> &str {
        self.text.split_whitespace().next().unwrap_or("")
    }
}

fn main() {
    println!("=== Exercise 15: Lifetimes ===");

    // Section 1: why the annotation is needed
    println!("\n--- Section 1: longest() ---");
    let s1 = String::from("a long string");
    let result;
    {
        let s2 = String::from("short");
        result = longest(s1.as_str(), s2.as_str());
        println!("longest inside inner scope: {result}"); // fine — s2 still alive here
    }
    // println!("{result}"); // would be a COMPILE ERROR here: s2 (and thus a
    //                        // possible source of `result`) has been dropped —
    //                        // the borrow checker rejects the dangling case
    //                        // even though THIS particular call returned s1.

    // Section 2: elision — most functions need no annotation at all
    println!("\n--- Section 2: lifetime elision ---");
    fn first_char(s: &str) -> Option<char> {
        s.chars().next() // one input reference, one output reference: elided automatically
    }
    println!("first_char(\"rust\") = {:?}", first_char("rust"));

    // Section 3: a struct holding a reference
    println!("\n--- Section 3: structs with a lifetime parameter ---");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt = Excerpt { text: &novel }; // Excerpt<'_> cannot outlive `novel`
    println!("first word: {}", excerpt.first_word());
    // `novel` must stay alive as long as `excerpt` is used — the compiler enforces this.

    // Section 4: 'static — the special lifetime meaning "for the whole program"
    println!("\n--- Section 4: 'static ---");
    let literal: &'static str = "string literals are baked into the binary, always 'static";
    println!("{literal}");

    println!("\nNotes:");
    println!("  - A lifetime is a compile-time-only annotation — it costs nothing at runtime.");
    println!("  - Most functions never need one: the elision rules cover single-input, method (&self) cases.");
    println!("  - A struct holding &T must carry that T's lifetime as a type parameter — see Excerpt<'a> above.");
    println!("  - This is the price of no GC: the compiler proves every reference outlives its use, statically.");
}
